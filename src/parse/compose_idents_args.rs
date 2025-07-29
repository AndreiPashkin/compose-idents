use crate::ast::{AliasSpec, ComposeIdentsArgs};
use crate::parse::MIXING_SEP_ERROR;
use crate::util::deprecation::DeprecationService;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Token};

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec: AliasSpec = input.parse()?;
        let block: Block = input.parse()?;
        let deprecation_service = DeprecationService::scoped();

        if spec.is_comma_used().is_some_and(|value| !value) {
            deprecation_service.add_semicolon_separator_warning();
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

        if let (Some(is_comma_current_sep), Some(is_comma_used)) =
            (is_comma_current_sep, spec.is_comma_used())
        {
            if is_comma_current_sep ^ is_comma_used {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

        Ok(ComposeIdentsArgs::new(
            next_unique_id(),
            Rc::new(spec),
            block,
        ))
    }
}
