use crate::ast::{AliasValue, Expr, Func};
// not used maybe but ok
use crate::util::combined::combine;
use crate::util::deprecation::DeprecationService;
use crate::util::terminated::Terminated;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, Ident, Token};

impl Parse for AliasValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let deprecation_service = DeprecationService::scoped();
        let span = input.span();
        let expr;

        if input.peek(Bracket) {
            // Fall back to the deprecated bracket-based syntax
            let mut exprs = Vec::new();
            let content;
            bracketed!(content in input);
            let punctuated =
                content.parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,])?;
            exprs.extend(
                punctuated
                    .into_iter()
                    .map(|arg| Rc::new(arg.into_value()))
                    .collect::<Vec<_>>(),
            );
            let func = Func::new(
                next_unique_id(),
                Ident::new("concat", span),
                Some(exprs),
                None,
                span,
            );
            expr = Expr::FuncCallExpr(Box::new(func));

            deprecation_service.add_bracket_syntax_warning();
        } else {
            let terminated = input.parse::<Terminated<Expr, combine!(Token![,], Token![;])>>()?;
            expr = terminated.into_value();
        }

        Ok(AliasValue::new(next_unique_id(), Rc::new(expr), span))
    }
}
