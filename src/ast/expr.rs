use crate::ast::{Arg, ArgInner, Ast, Func, NodeId};
use proc_macro2::Span;
use std::fmt::Display;

/// Expression in form of an argument or a function call.
#[derive(Debug, Clone)]
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::ArgExpr(arg) => match arg.inner() {
                ArgInner::Ident(ident) => write!(f, "{}", ident),
                ArgInner::LitStr(value) => write!(f, "\"{}\"", value.value()),
                ArgInner::LitInt(value) => write!(f, "{}", value),
                ArgInner::Tokens(tokens) => write!(f, "{}", tokens),
                ArgInner::Underscore(_) => write!(f, "_"),
            },
            Expr::FuncCallExpr(func) => write!(f, "{}", func),
        }
    }
}

impl Ast for Expr {
    fn id(&self) -> NodeId {
        match self {
            Expr::ArgExpr(arg) => arg.id(),
            Expr::FuncCallExpr(func) => func.id(),
        }
    }
    fn span(&self) -> Span {
        match self {
            Expr::ArgExpr(arg) => arg.span(),
            Expr::FuncCallExpr(func) => func.span(),
        }
    }
}
