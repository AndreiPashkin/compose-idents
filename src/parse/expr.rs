use crate::ast::{Arg, Expr, Func};
use crate::error::combine_errors;
use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();
        let fork = input.fork();

        match fork.parse::<Func>() {
            Ok(func) => {
                input.advance_to(&fork);
                return Ok(Expr::FuncCallExpr(Box::new(func)));
            }
            Err(err) => errors.push(err),
        }

        match input.parse::<Arg>() {
            Ok(arg) => return Ok(Expr::ArgExpr(Box::new(arg))),
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Expected argument or function call (see the errors below)",
            Span::call_site(),
            errors,
        ))
    }
}
