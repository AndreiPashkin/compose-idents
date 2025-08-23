use crate::ast::{AliasValue, Call, Expr, TerminatedExpr};
use crate::util::combined::combine;
use crate::util::deprecation::DeprecationService;
use crate::util::log::debug;
use crate::util::terminated::Terminated;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, Ident, Token};

impl Parse for AliasValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        debug!("Parsing AliasValue...");

        let span = input.span();

        debug!("Parsing Terminated<Expr, ...>...");
        let terminated = input.parse::<TerminatedExpr<combine!(Token![,], Token![;])>>()?;
        debug!("Parsing Terminated<Expr, ...> is successful.");
        let expr = terminated.into_expr();

        Ok(AliasValue::new(next_unique_id(), Rc::new(expr), span))
    }
}
