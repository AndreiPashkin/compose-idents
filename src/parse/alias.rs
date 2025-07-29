use crate::ast::Alias;
use crate::util::unique_id::next_unique_id;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

impl Parse for Alias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(Alias::new(next_unique_id(), ident))
    }
}
