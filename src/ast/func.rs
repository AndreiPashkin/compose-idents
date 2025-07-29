use crate::ast::{Ast, Expr, NodeId};
use proc_macro2::{Ident, Span};
use std::fmt::Display;
use std::rc::Rc;

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub struct Func {
    id: NodeId,
    args: Option<Vec<Rc<Expr>>>,
    tokens: Option<Rc<Expr>>,
    name: Ident,
    span: Span,
}

impl Func {
    /// Creates a new [`Func`] with the given name and arguments.
    pub fn new(
        id: NodeId,
        name: Ident,
        args: Option<Vec<Rc<Expr>>>,
        tokens: Option<Rc<Expr>>,
        span: Span,
    ) -> Self {
        Self {
            id,
            name,
            args,
            tokens,
            span,
        }
    }

    /// The function's name.
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// The function's arguments.
    pub fn args(&self) -> Option<&[Rc<Expr>]> {
        self.args.as_ref().map(|x| x as _)
    }

    /// The function's arguments as raw tokens.
    pub fn tokens(&self) -> Option<Rc<Expr>> {
        self.tokens.as_ref().map(|tokens| tokens.clone())
    }
}

impl Ast for Func {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.span
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name(),
            match (self.args(), self.tokens()) {
                (Some(args), _) => args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                (_, Some(tokens)) => tokens.to_string(),
                _ => String::new(),
            },
        )
    }
}

/// Function type - signatures of built-in functions.
#[derive(Debug, Clone)]
pub enum FuncInner {
    Upper(Rc<Expr>),
    Lower(Rc<Expr>),
    SnakeCase(Rc<Expr>),
    CamelCase(Rc<Expr>),
    PascalCase(Rc<Expr>),
    Hash(Rc<Expr>),
    Normalize(Rc<Expr>),
    Concat(Vec<Rc<Expr>>),
}

/// Metadata for storing resolved function information
#[derive(Debug, Clone)]
pub struct FuncMetadata {
    pub inner: Rc<FuncInner>,
}
