use crate::ast::{Alias, AliasSpecItem, AliasValue};
use crate::util::log::debug;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for AliasSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        debug!("Parsing AliasSpecItem: {:?}...", input);

        debug!("Parsing alias...");
        let alias: Alias = input.parse()?;
        input.parse::<Token![=]>()?;

        debug!("Parsing alias value...");
        let value: AliasValue = input.parse()?;
        debug!("Parsing AliasValue is successful.");

        Ok(AliasSpecItem::new(
            next_unique_id(),
            Rc::new(alias),
            Rc::new(value),
        ))
    }
}
