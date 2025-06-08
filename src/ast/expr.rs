use crate::ast::{Ast, Spanned};
use proc_macro2::{Ident, Span, TokenStream};

/// Argument to the [`compose_idents`] macro.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub enum Arg {
    Ident(Ident),
    LitStr(Span, String),
    LitInt(Span, u64),
    Tokens(Span, TokenStream),
    Underscore(Span),
}

impl Spanned for Arg {
    fn span(&self) -> Span {
        match self {
            Arg::Ident(ident) => ident.span(),
            Arg::LitStr(span, _) => *span,
            Arg::LitInt(span, _) => *span,
            Arg::Tokens(span, _) => *span,
            Arg::Underscore(span) => *span,
        }
    }
}

impl Ast for Arg {}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub enum Func {
    Upper(Expr),
    Lower(Expr),
    SnakeCase(Expr),
    CamelCase(Expr),
    PascalCase(Expr),
    Hash(Expr),
    Normalize(Expr),
    Concat(Vec<Expr>),
    SignatureMismatch(String),
    Undefined,
}

impl Spanned for Func {
    fn span(&self) -> Span {
        match self {
            Func::Upper(expr) => expr.span(),
            Func::Lower(expr) => expr.span(),
            Func::SnakeCase(expr) => expr.span(),
            Func::CamelCase(expr) => expr.span(),
            Func::PascalCase(expr) => expr.span(),
            Func::Hash(expr) => expr.span(),
            Func::Normalize(expr) => expr.span(),
            Func::Concat(exprs) => exprs
                .first()
                .map(|e| e.span())
                .unwrap_or_else(Span::call_site),
            Func::SignatureMismatch(_) => Span::call_site(),
            Func::Undefined => Span::call_site(),
        }
    }
}

impl Ast for Func {}

/// Expression in form of an argument or a function call.
#[derive(Debug, Clone)]
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::ArgExpr(arg) => arg.span(),
            Expr::FuncCallExpr(func) => func.span(),
        }
    }
}

impl Ast for Expr {}
