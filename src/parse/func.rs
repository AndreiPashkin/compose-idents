use crate::ast::{Expr, Func};
use crate::util::terminated::Terminated;
use crate::util::unique_id::next_unique_id;
use proc_macro2::TokenStream;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Token};

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let name = input.parse::<Ident>()?;
        let raw_args;
        parenthesized!(raw_args in input);
        let punctuated = raw_args
            .fork()
            .parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,]);
        let args = match punctuated {
            Ok(punctuated) => Some(
                punctuated
                    .into_iter()
                    .map(|arg| Rc::new(arg.into_value()))
                    .collect::<Vec<_>>(),
            ),
            Err(_) => None,
        };
        let tokens = raw_args.parse::<TokenStream>().ok().map(|tokens| {
            Rc::new(Expr::ArgExpr(Box::new(crate::ast::Arg::from_tokens(
                tokens,
            ))))
        });

        Ok(Func::new(next_unique_id(), name, args, tokens, span))
    }
}
