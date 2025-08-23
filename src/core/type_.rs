//! Contains implementation of the little type system used by the library.

use crate::ast::Value;
use crate::core::Environment;
use crate::error::Error;
use crate::util::unique_id::next_unique_id;
use proc_macro2::Span;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

/// Type of value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Ident,
    Path,
    Type,
    Expr,
    LitStr,
    LitInt,
    Tokens,
    Raw,
    Variadic(Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Ident => write!(f, "ident"),
            Type::Path => write!(f, "path"),
            Type::Type => write!(f, "type"),
            Type::Expr => write!(f, "expr"),
            Type::LitStr => write!(f, "str"),
            Type::LitInt => write!(f, "int"),
            Type::Tokens => write!(f, "tokens"),
            Type::Raw => write!(f, "raw"),
            Type::Variadic(type_) => write!(f, "{}...", type_),
        }
    }
}

impl Type {
    pub fn coercion_cost_basic(from: &Type, to: &Type) -> Option<u32> {
        match (from, to) {
            (_, _) if from == to => Some(0),
            (Type::Ident, Type::Path) => Some(1),
            (Type::Ident, Type::Type) => Some(2),
            (Type::Ident, Type::Expr) => Some(3),
            (_, Type::Tokens) => Some(4),
            (_, Type::Raw) => Some(5),
            _ => None,
        }
    }

    /// Cost of the coercion from one type to another.
    ///
    /// Returns `None` if the casting is not possible.
    pub fn coercion_cost(from: &Type, to: &Type) -> Option<u32> {
        match (from, to) {
            (Type::Variadic(boxed_from), Type::Variadic(boxed_to)) => {
                Self::coercion_cost_basic(boxed_from.as_ref(), boxed_to.as_ref())
            }
            (from, to) => Self::coercion_cost_basic(from, to),
        }
    }
}

pub type FuncImpl = fn(&Func, &Environment, &Span, &[Rc<Value>]) -> Result<Value, Error>;

/// Function type.
///
/// Describes function's name, signature, output type, stores the pointer to implementation.
#[derive(Debug, Clone)]
pub struct Func {
    id: u64,
    name: String,
    arg_types: Vec<Type>,
    out_type: Type,
    func_impl: FuncImpl,
}

impl Func {
    pub fn new(name: String, arg_types: Vec<Type>, out_type: Type, func_impl: FuncImpl) -> Self {
        debug_assert!(
            arg_types
                .iter()
                .filter(|x| matches!(x, Type::Variadic(_)))
                .count()
                <= 1,
            "Function {} cannot have more than one variadic parameter",
            name,
        );
        debug_assert!(
            arg_types
                .iter()
                .enumerate()
                .find(|(_, x)| matches!(x, Type::Variadic(_)))
                .map_or(true, |(i, _)| i == arg_types.len() - 1),
            "Variadic parameter must be the last one in the function {}",
            name,
        );
        debug_assert!(
            if arg_types.iter().any(|a| a == &Type::Raw) {
                arg_types.len() == 1
            } else {
                true
            },
            "Function {} can have Raw type only if it is the only argument",
            name,
        );

        Self {
            id: next_unique_id(),
            name,
            arg_types,
            out_type,
            func_impl,
        }
    }

    /// The name of the function.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The function's signature.
    pub fn signature(&self) -> String {
        let args = self
            .arg_types
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}({}) -> {}", self.name, args, self.out_type)
    }

    /// Creates an error for incorrect types of provided arguments.
    pub fn arg_type_error(&self, args: &[Value], span: &Span) -> Error {
        Error::TypeError(
            format!(
                "Incorrect arguments for {}: {}",
                self.signature(),
                args.iter()
                    .map(|arg| format!("{:?}", arg.type_()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            *span,
        )
    }

    /// Argument types of the function.
    pub fn arg_types(&self) -> &[Type] {
        &self.arg_types
    }

    /// Output type of the function.
    pub fn out_type(&self) -> &Type {
        &self.out_type
    }

    /// Whether the function has at least one variadic parameter or not.
    pub fn is_variadic(&self) -> bool {
        self.arg_types
            .iter()
            .any(|t| matches!(t, Type::Variadic(_)))
    }

    pub fn non_variadic_arg_types(&self) -> Vec<Type> {
        self.arg_types
            .iter()
            .filter(|t| !matches!(t, Type::Variadic(_)))
            .cloned()
            .collect()
    }

    pub fn variadic_arg_type(&self) -> Option<Type> {
        self.arg_types
            .iter()
            .find(|t| matches!(t, Type::Variadic(_)))
            .map(|t| {
                if let Type::Variadic(boxed_type) = t {
                    *boxed_type.clone()
                } else {
                    unreachable!()
                }
            })
    }

    pub fn num_args(&self) -> usize {
        self.arg_types.len()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    /// Calls the function with the provided arguments.
    pub fn call(
        &self,
        args: &[Rc<Value>],
        environment: &Environment,
        span: Option<Span>,
    ) -> Result<Value, Error> {
        let span = span.unwrap_or_else(Span::call_site);
        (self.func_impl)(self, environment, &span, args)
    }
}
