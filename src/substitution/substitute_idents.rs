//! Provides [`substitute_idents`] - a helper that performs alias-substitution within an AST node
//! of an arbitrary type while using [`StreamVisitor`] internally.

use crate::ast::Value;
use crate::error::Error;
use crate::substitution::{
    format_string, StreamVisitor, StreamVisitorAction, StreamWalker, VisitorCtx,
};
use crate::util::log::debug;
use proc_macro2::{Ident, Literal, Span};
use quote::ToTokens;
use std::any::type_name;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use syn::parse::Parse;
use syn::LitStr;

/// A visitor compatible with [`StreamWalker`] that substitutes identifiers and formats
/// string literals.
struct SubstituteIdentsVisitor<N: Parse> {
    substitutions: HashMap<String, Rc<Value>>,
    error_data: Option<(String, String, Span)>,
    node_type: PhantomData<N>,
}

impl<N: Parse> SubstituteIdentsVisitor<N> {
    pub fn new(substitutions: HashMap<String, Rc<Value>>) -> Self {
        Self {
            substitutions,
            error_data: None,
            node_type: PhantomData,
        }
    }
}

impl<N: Parse> StreamVisitor for SubstituteIdentsVisitor<N> {
    fn visit_ident_mut(
        &mut self,
        _: &VisitorCtx,
        ident: &Ident,
    ) -> Result<StreamVisitorAction, Error> {
        if let Some(value) = self.substitutions.get(&ident.to_string()) {
            let substitution = value.to_token_stream();
            self.error_data = Some((
                ident.to_string(),
                substitution.clone().to_string(),
                ident.span(),
            ));

            Ok(StreamVisitorAction::Replace(substitution))
        } else {
            Ok(StreamVisitorAction::Continue)
        }
    }
    fn visit_literal_mut(
        &mut self,
        _: &VisitorCtx,
        literal: &Literal,
    ) -> Result<StreamVisitorAction, Error> {
        let Ok(lit_str) = syn::parse2::<LitStr>(literal.into_token_stream()) else {
            return Ok(StreamVisitorAction::Continue);
        };
        let formatted = format_string(lit_str.value().as_str(), &self.substitutions);
        let lit_str = LitStr::new(&formatted, lit_str.span());

        Ok(StreamVisitorAction::Replace(lit_str.to_token_stream()))
    }
    fn after_replace_mut(&mut self, ctx: &VisitorCtx) -> Result<(), Error> {
        let stream = ctx.current_stream();
        match syn::parse2::<N>(stream.clone()) {
            Ok(_) => Ok(()),
            Err(err) => {
                debug!("Error validating stream after token replacement: {}", err);
                let error_data = match self.error_data.take() {
                    Some(err) => err,
                    None => {
                        panic!("Error data is always expected to be set at this point.");
                    }
                };
                Err(Error::SubstitutionError(
                    error_data.0,
                    error_data.1,
                    err,
                    error_data.2,
                ))
            }
        }
    }
}

/// Substitutes identifiers within the provided AST node's token-stream representation and returns a
/// new node based on the resulting token-stream.
pub fn substitute_idents<N: ToTokens + Parse>(
    node: &N,
    substitutions: &HashMap<String, Rc<Value>>,
) -> Result<N, Error> {
    let mut visitor = SubstituteIdentsVisitor::<N>::new(substitutions.clone());
    let mut walker = StreamWalker::new(&mut visitor);
    let stream = node.to_token_stream();
    let new_stream = walker.walk(stream)?;

    match syn::parse2::<N>(new_stream.clone()) {
        Ok(new_node) => Ok(new_node),
        Err(err) => {
            unreachable!(
                "Stream is expected to be valid after all token replacements for node \"{}\" of type \"{}\":\n{}\n{}",
                node.to_token_stream(),
                type_name::<N>(),
                new_stream,
                err,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::make_substitutions;
    use super::substitute_idents;
    use crate::ast::Value;
    use crate::error::Error;
    use proc_macro2::{Ident, Span};
    use quote::ToTokens;
    use rstest::rstest;
    use std::collections::HashMap;
    use std::rc::Rc;
    use syn::parse_quote;

    /// Various token substitution cases.
    #[rstest]
    #[case::substituting_single_token(
        parse_quote!{{
            fn foo() -> u32 { 1 }
        }},
        parse_quote!{{
            fn bar() -> u32 { 1 }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    #[case::substituting_multiple_tokens(
        parse_quote!{{
            let foo = 1;
            let bar = foo + 1;
        }},
        parse_quote!{{
            let baz = 1;
            let bar = baz + 1;
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("baz", Span::call_site())),
        ),
    )]
    #[case::substituting_with_multiple_tokens(
        parse_quote!{{
            fn foo() -> T { 1 }
        }},
        parse_quote!{{
            fn foo() -> Result<u32, String> { 1 }
        }},
        make_substitutions!(
            "T" => Value::from_type(syn::parse_str::<syn::Type>("Result<u32, String>").unwrap()),
        ),
    )]
    #[case::string_formatting(
        parse_quote!{{
            fn foo() -> &str { "Hello, % name %!" }
        }},
        parse_quote!{{
            fn foo() -> &str { "Hello, World!" }
        }},
        make_substitutions!(
            "name" => Value::from_ident(Ident::new("World", Span::call_site())),
        ),
    )]
    fn substitution(
        #[case] input: syn::Block,
        #[case] expected: syn::Block,
        #[case] substitutions: HashMap<String, Rc<Value>>,
    ) {
        let result = substitute_idents(&input, &substitutions);
        assert!(result.is_ok());

        let actual = result.unwrap();
        assert_eq!(actual, expected);
    }

    /// Substitution producing invalid AST node should yield an error.
    #[test]
    fn substitution_yields_error_on_invalid_ast_node() {
        let input_fn: syn::ItemFn = parse_quote! {
            fn f() -> T { 0 }
        };
        let type_: syn::LitInt = parse_quote!(123);

        let value = Value::from_lit_int(type_);
        let subs = make_substitutions!(
            "T" => value.clone(),
        );

        let result = substitute_idents(&input_fn, &subs);
        assert!(result.is_err());

        let err = result.unwrap_err();

        assert!(matches!(err, Error::SubstitutionError(_, _, _, _)));

        let Error::SubstitutionError(actual_original, actual_replacement, _, _) = err else {
            unreachable!()
        };

        assert_eq!(actual_original, "T");
        assert_eq!(actual_replacement, value.to_token_stream().to_string());
    }
}
