use crate::ast::{AliasValue, TerminatedExpr};
use crate::util::combined::combine;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for AliasValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let terminated = input.parse::<TerminatedExpr<combine!(Token![,], Token![;])>>()?;
        let expr = terminated.into_expr();

        Ok(AliasValue::new(next_unique_id(), Rc::new(expr), span))
    }
}
