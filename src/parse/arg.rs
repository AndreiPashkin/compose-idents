use crate::ast::Arg;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_terminated = input.peek2(syn::parse::End);
        if input.peek(syn::Ident) && is_terminated {
            let ident = input.parse::<syn::Ident>()?;
            Ok(Arg::from_ident(ident))
        } else if input.peek(Token![_]) && is_terminated {
            input.parse::<Token![_]>()?;
            Ok(Arg::from_underscore())
        } else if input.peek(syn::LitStr) && is_terminated {
            let lit_str = input.parse::<syn::LitStr>()?;
            Ok(Arg::from_lit_str(lit_str.value()))
        } else if input.peek(syn::LitInt) && is_terminated {
            let lit_int = input.parse::<syn::LitInt>()?;
            Ok(Arg::from_lit_int(lit_int.base10_parse::<u64>()?))
        } else {
            let tokens = input.parse::<TokenStream>()?;
            Ok(Arg::from_tokens(tokens))
        }
    }
}
