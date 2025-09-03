use crate::ast::{AliasSpec, LoopSpec, RawAST};
use crate::parse::MIXING_SEP_ERROR;
use crate::util::deprecation::DeprecationService;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Token};

impl Parse for RawAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let loops = if input.peek(Token![for]) {
            Some(input.parse::<LoopSpec>()?)
        } else {
            None
        };

        let spec = if input.peek(syn::Ident) {
            Some(input.parse::<AliasSpec>()?)
        } else {
            None
        };

        let block: Block = input.parse()?;
        let deprecation_service = DeprecationService::scoped();

        match &spec {
            Some(spec) if spec.is_comma_used().is_some_and(|v| !v) => {
                deprecation_service.add_semicolon_separator_warning();
            }
            _ => {}
        }

        let is_comma_current_sep = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(true)
        } else if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            Some(false)
        } else {
            None
        };

        if let (Some(is_comma_current_sep), Some(spec)) = (is_comma_current_sep, &spec) {
            if is_comma_current_sep ^ spec.is_comma_used().is_some_and(|value| value) {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

        Ok(RawAST::new(
            next_unique_id(),
            loops.map(Rc::new),
            spec.map(Rc::new),
            block,
        ))
    }
}
