//! Provides [`LoopSourceValueList`] - a list of source values of a loop.
use crate::ast::{Ast, Expr, NodeId, Tuple};
use proc_macro2::Span;
use std::rc::Rc;

/// A list of source values of a loop.
#[derive(Debug, Clone)]
pub enum LoopSourceValue {
    Value(Rc<Expr>),
    Tuple(Tuple<Expr>),
}

impl LoopSourceValue {
    /// Creates a new [`LoopSourceValue`] from a singular value.
    pub fn from_value(expr: Expr) -> Self {
        Self::Value(Rc::new(expr))
    }
    /// Creates a new [`LoopSourceValue`] from a tuple.
    pub fn from_tuple(tuple: Tuple<Expr>) -> Self {
        Self::Tuple(tuple)
    }
}

impl Ast for LoopSourceValue {
    fn id(&self) -> NodeId {
        match self {
            LoopSourceValue::Value(expr) => expr.id(),
            LoopSourceValue::Tuple(tuple) => tuple.id(),
        }
    }

    fn span(&self) -> Span {
        match self {
            LoopSourceValue::Value(expr) => expr.span(),
            LoopSourceValue::Tuple(tuple) => tuple.span(),
        }
    }
}

/// A list of source values for a loop.
///
/// In an expression like `for (a, b) in [(1, 2), (3, 4)]`, the value list
/// would be `[(1, 2), (3, 4)]`.
#[derive(Debug, Clone)]
pub struct LoopSourceValueList {
    id: NodeId,
    values: Vec<LoopSourceValue>,
    span: Span,
}

impl LoopSourceValueList {
    /// Creates a new [`LoopSourceValueList`] with the given values.
    pub fn new(id: NodeId, values: Vec<LoopSourceValue>, span: Span) -> Self {
        Self { id, values, span }
    }

    /// Reads the source values.
    pub fn values(&self) -> &[LoopSourceValue] {
        &self.values
    }
}

impl Ast for LoopSourceValueList {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        self.span
    }
}
