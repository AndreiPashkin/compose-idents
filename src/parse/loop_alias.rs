use crate::ast::{Alias, Ast, LoopAlias, Tuple};
use crate::error::combine_errors;
use syn::parse::{discouraged::Speculative, Parse, ParseStream};
use syn::token::Paren;

impl Parse for LoopAlias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();

        let fork = input.fork();
        if fork.peek(Paren) {
            match fork.parse::<Tuple<Alias>>() {
                Ok(tuple) => {
                    input.advance_to(&fork);

                    return Ok(LoopAlias::from_tuple(tuple));
                }
                Err(err) => errors.push(err),
            }
        }

        let fork = input.fork();
        match fork.parse::<Alias>() {
            Ok(alias) => {
                input.advance_to(&fork);

                return Ok(LoopAlias::from_simple(alias));
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Failed to parse loop alias (see errors below)",
            input.span(),
            errors,
        ))
    }
}
