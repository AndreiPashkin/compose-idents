use crate::ast::{LoopAlias, LoopSourceValueList, LoopSpecItem};
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for LoopSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        input.parse::<Token![for]>()?;

        let alias = input.parse::<LoopAlias>()?;

        input.parse::<Token![in]>()?;

        let list = input.parse::<LoopSourceValueList>()?;

        Ok(LoopSpecItem::new(
            next_unique_id(),
            Rc::new(alias),
            Rc::new(list),
            span,
        ))
    }
}
