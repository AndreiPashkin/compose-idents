use crate::ast::{Tuple, TupleValue};
use crate::util::unique_id::next_unique_id;
use std::fmt::Debug;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Token};
//
//
// impl <V>Tuple<V>
// where
//     V: Parse + Clone + Debug,
// {
//     pub fn parse_with(
//         input: ParseStream<'_>,
//         parser: fn(ParseStream<'_>) -> syn::Result<V>,
//     ) -> syn::Result<Self> {
//         let span = input.span();
//         let content;
//
//         parenthesized!(content in input);
//
//         let punctuated =
//             content.parse_terminated(TupleValue::<V>::parse, Token![,])?;
//         let values: Vec<TupleValue<V>> = punctuated.into_iter().collect();
//
//         Ok(Tuple::<V>::new(next_unique_id(), values, span))
//     }
// }

impl<V> Parse for Tuple<V>
where
    V: Parse + Clone + Debug,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let content;

        parenthesized!(content in input);

        let punctuated = content.parse_terminated(TupleValue::<V>::parse, Token![,])?;
        let values: Vec<TupleValue<V>> = punctuated.into_iter().collect();

        Ok(Tuple::<V>::new(next_unique_id(), values, span))
    }
}

impl<V> Parse for TupleValue<V>
where
    V: Parse + Clone + Debug,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        if input.peek(Paren) {
            let tuple = input.parse::<Tuple<V>>()?;
            Ok(TupleValue::from_tuple(next_unique_id(), tuple, span))
        } else {
            let value = input.parse::<V>()?;
            Ok(TupleValue::from_value(
                next_unique_id(),
                Rc::new(value),
                span,
            ))
        }
    }
}

// impl <V>TupleValue<V>
// where
//     V: Parse + Clone + Debug,
// {
//     pub fn parse_with(
//         input: ParseStream<'_>,
//         parser: fn(ParseStream<'_>) -> syn::Result<V>,
//     ) -> syn::Result<Self> {
//
//     }
// }
