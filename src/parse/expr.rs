use crate::ast::{Call, Expr, TerminatedExpr, TerminatedValue};
use crate::error::combine_errors;
use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::Token;

/// Just like impl of [`Parse`] for [`Value`] - parses the input either until the end or a comma.
impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();
        let fork = input.fork();

        match fork.parse::<Call>() {
            Ok(func) => {
                let expr = Expr::from_call(func);
                input.advance_to(&fork);
                return Ok(expr);
            }
            Err(err) => errors.push(err),
        }

        match input.parse::<TerminatedValue<Token![,]>>() {
            Ok(terminated_value) => {
                let expr = Expr::from_value(terminated_value.clone().into_value());
                return Ok(expr);
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Expected argument or function call (see the errors below)",
            Span::call_site(),
            errors,
        ))
    }
}

/// Parses an `Expr` followed by an optional terminator.
///
/// This differs from `Terminated<Expr, ...>` in that we let `syn` parse the `Expr`
/// first (so internal commas are handled properly), and only then we ensure that
/// either the input ended or the next token is the terminator `Term`. The terminator
/// is not consumed by this parser.
impl<Term: Parse> Parse for TerminatedExpr<Term> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let expr = fork.parse::<Expr>()?;

        let term_fork = fork.fork();
        if !term_fork.is_empty() {
            let _ = term_fork.parse::<Term>()?;
        }

        input.advance_to(&fork);
        Ok(TerminatedExpr::new(expr))
    }
}
