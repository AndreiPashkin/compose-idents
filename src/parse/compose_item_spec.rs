use crate::ast::{AliasSpec, ComposeItemSpec, LoopSpec};
use crate::util::deprecation::DeprecationService;
use crate::util::unique_id::next_unique_id;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for ComposeItemSpec {
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

        let deprecation_service = DeprecationService::scoped();
        if let Some(spec) = &spec {
            if spec.is_comma_used().is_some_and(|v| !v) {
                deprecation_service.add_semicolon_separator_warning();
            }
        }

        let has_comma = input.peek(Token![,]);
        let has_semicolon = input.peek(Token![;]);
        let has_separator = has_comma || has_semicolon;

        if has_separator {
            if spec.is_some() {
                if has_comma {
                    let _ = input.parse::<Token![,]>()?;
                } else {
                    let _ = input.parse::<Token![;]>()?;
                }
            } else if loops.is_some() {
                return Err(input
                    .error("Trailing separator after loops is not allowed in #[compose_item]."));
            }
        }

        Ok(ComposeItemSpec::new(
            next_unique_id(),
            loops.map(std::rc::Rc::new),
            spec.map(std::rc::Rc::new),
        ))
    }
}
