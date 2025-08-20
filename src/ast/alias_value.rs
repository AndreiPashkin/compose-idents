use crate::ast::{Ast, Expr, NodeId};
use proc_macro2::Span;
use std::rc::Rc;

/// Alias value, which is a sequence of expressions that form the value of the alias.
#[derive(Debug)]
pub struct AliasValue {
    id: NodeId,
    span: Span,
    expr: Rc<Expr>,
}

impl AliasValue {
    /// Creates a new [`AliasValue`] with the given expressions.
    pub fn new(id: NodeId, expr: Rc<Expr>, span: Span) -> Self {
        Self { id, span, expr }
    }

    /// Reads the expressions.
    pub fn expr(&self) -> Rc<Expr> {
        self.expr.clone()
    }

    /// Reads the span of the alias value.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Ast for AliasValue {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.span
    }
}
