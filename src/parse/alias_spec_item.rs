use crate::ast::{Alias, AliasSpecItem, AliasValue};
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for AliasSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let alias: Alias = input.parse()?;
        input.parse::<Token![=]>()?;

        let value: AliasValue = input.parse()?;

        Ok(AliasSpecItem::new(
            next_unique_id(),
            Rc::new(alias),
            Rc::new(value),
        ))
    }
}
