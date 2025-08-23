use crate::ast::{Ast, Expr, NodeId};
use crate::core::{Func, Type};
use proc_macro2::{Ident, Span};
use std::fmt::Display;
use std::rc::Rc;

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub struct Call {
    id: NodeId,
    /// Unprocessed raw arguments generating by the parser.
    raw_args: Vec<Rc<Expr>>,
    /// Unprocessed raw arguments generating by the parser as a token-sequence.
    raw_tokens: Option<Rc<Expr>>,
    name: Ident,
    span: Span,
}

impl Call {
    /// Creates a new [`Call`] with the given name and arguments.
    pub fn new(
        id: NodeId,
        name: Ident,
        raw_args: Vec<Rc<Expr>>,
        raw_tokens: Option<Rc<Expr>>,
        span: Span,
    ) -> Self {
        Self {
            id,
            name,
            raw_args,
            raw_tokens,
            span,
        }
    }

    /// The function's name.
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// The function call arguments.
    pub fn raw_args(&self) -> &[Rc<Expr>] {
        self.raw_args.as_ref()
    }

    /// The function call arguments as raw tokens.
    pub fn raw_tokens(&self) -> Option<Rc<Expr>> {
        self.raw_tokens.as_ref().map(|tokens| tokens.clone())
    }
}

impl Ast for Call {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.span
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name(),
            match (self.raw_args(), self.raw_tokens()) {
                (args, Some(tokens)) if args.len() == 1 => tokens.to_string(),
                (args, _) => args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            },
        )
    }
}

/// Metadata for storing resolved function call information
#[derive(Debug, Clone)]
pub struct CallMetadata {
    /// Resolved arguments - ready to be used in [`Func::call`].
    pub args: Vec<Rc<Expr>>,
    /// The resolved function type.
    pub func: Rc<Func>,
    /// The target coercion type.
    pub target_type: Type,
    /// The cost of coercing to the target type.
    pub coercion_cost: u32,
}
