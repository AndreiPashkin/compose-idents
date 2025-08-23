use crate::ast::{Ast, Call, NodeId, Value};
use proc_macro2::Span;
use std::fmt::Display;
use std::marker::PhantomData;
use syn::parse::Parse;

/// Expression in form of an argument or a function call.
#[derive(Debug, Clone)]
pub struct Expr {
    inner: ExprInner,
}

#[derive(Debug, Clone)]
pub enum ExprInner {
    ValueExpr(Box<Value>),
    FuncCallExpr(Box<Call>),
}

impl Expr {
    pub fn new(inner: ExprInner) -> Self {
        Self { inner }
    }

    pub fn from_value(arg: Value) -> Self {
        Self::new(ExprInner::ValueExpr(Box::new(arg)))
    }

    pub fn from_call(func: Call) -> Self {
        Self::new(ExprInner::FuncCallExpr(Box::new(func)))
    }

    pub fn inner(&self) -> &ExprInner {
        &self.inner
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner() {
            ExprInner::ValueExpr(value) => write!(f, "{}", value),
            ExprInner::FuncCallExpr(call) => write!(f, "{}", call),
        }
    }
}

impl Ast for Expr {
    fn id(&self) -> NodeId {
        match self.inner() {
            ExprInner::ValueExpr(value) => value.id(),
            ExprInner::FuncCallExpr(func) => func.id(),
        }
    }
    fn span(&self) -> Span {
        match self.inner() {
            ExprInner::ValueExpr(arg) => arg.span(),
            ExprInner::FuncCallExpr(func) => func.span(),
        }
    }
}

/// Auxiliary type that represents an [`Expr`] terminated by a generic terminator-token.
#[derive(Debug, Clone)]
pub struct TerminatedExpr<Term>
where
    Term: Parse,
{
    expr: Expr,
    terminator_type: PhantomData<Term>,
}

impl<Term: Parse> TerminatedExpr<Term> {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr,
            terminator_type: PhantomData,
        }
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }
}

impl<Term: Parse> Ast for TerminatedExpr<Term> {
    fn id(&self) -> NodeId {
        self.expr.id()
    }
    fn span(&self) -> Span {
        self.expr.span()
    }
}
