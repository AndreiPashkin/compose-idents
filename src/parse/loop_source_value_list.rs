use crate::ast::{Expr, LoopSourceValue, LoopSourceValueList, Tuple};
use crate::error::combine_errors;
use crate::util::unique_id::next_unique_id;
use syn::parse::{discouraged::Speculative, Parse, ParseStream};
use syn::token::Paren;
use syn::{bracketed, Token};

impl Parse for LoopSourceValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();

        let fork = input.fork();
        if fork.peek(Paren) {
            match fork.parse::<Tuple<Expr>>() {
                Ok(tuple) => {
                    input.advance_to(&fork);

                    return Ok(LoopSourceValue::from_tuple(tuple));
                }
                Err(err) => errors.push(err),
            }
        }

        let fork = input.fork();
        match fork.parse::<Expr>() {
            Ok(expr) => {
                input.advance_to(&fork);

                return Ok(LoopSourceValue::from_value(expr));
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Failed to parse loop source value (see errors below)",
            input.span(),
            errors,
        ))
    }
}

impl Parse for LoopSourceValueList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let content;
        bracketed!(content in input);

        let punctuated = content.parse_terminated(LoopSourceValue::parse, Token![,])?;
        let source_values: Vec<LoopSourceValue> = punctuated.into_iter().collect();

        Ok(LoopSourceValueList::new(
            next_unique_id(),
            source_values,
            span,
        ))
    }
}
