use crate::ast::{Call, Expr, NodeId, Value};
use crate::core::Environment;
use crate::error::Error;
use crate::util::terminated::Terminated;
use crate::util::unique_id::next_unique_id;
use proc_macro2::TokenStream;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Token};

impl Parse for Call {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let name = input.parse::<Ident>()?;

        let Some(environment) = Environment::get_global() else {
            panic!("Environment is not set, cannot parse function call");
        };

        if !environment.has_func(name.to_string().as_str()) {
            return Err(Error::UndefinedFunctionError(name.to_string(), span).into());
        }

        let raw_args;
        parenthesized!(raw_args in input);
        let punctuated = raw_args
            .fork()
            .parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,]);
        let args = match punctuated {
            Ok(punctuated) => punctuated
                .into_iter()
                .map(|arg| Rc::new(arg.into_value()))
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        };
        let raw = raw_args
            .parse::<TokenStream>()
            .ok()
            .map(|tokens| Rc::new(Expr::from_value(Value::from_raw(tokens))));

        Ok(Call::new(next_unique_id() as NodeId, name, args, raw, span))
    }
}
