use crate::ast::Ast;
use proc_macro2::{Ident, Span, TokenStream};

/// Argument to the [`compose_idents`] macro.
///
/// Its [`Parse`] impl parses the input entirely, until the end.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub enum Arg {
    Ident(Ident),
    LitStr(String),
    LitInt(u64),
    Tokens(TokenStream),
    Underscore,
}

impl Ast for Arg {
    fn span(&self) -> Span {
        match self {
            Arg::Ident(ident) => ident.span(),
            Arg::LitStr(_) => Span::call_site(),
            Arg::LitInt(_) => Span::call_site(),
            Arg::Tokens(_) => Span::call_site(),
            Arg::Underscore => Span::call_site(),
        }
    }
}

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

impl Ast for Func {
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

/// Expression in form of an argument or a function call.
///
/// Just like [`Arg`] - parses the input entirely, until the end.
#[derive(Debug, Clone)]
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

impl Ast for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::ArgExpr(arg) => arg.span(),
            Expr::FuncCallExpr(func) => func.span(),
        }
    }
}
