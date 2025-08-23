use crate::core::Type;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

impl Parse for Type {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "ident" => Ok(Type::Ident),
            "type" => Ok(Type::Type),
            "path" => Ok(Type::Path),
            "str" => Ok(Type::LitStr),
            "int" => Ok(Type::LitInt),
            "tokens" => Ok(Type::Tokens),
            _ => Err(syn::Error::new(
                ident.span(),
                format!("Unknown type: {}", ident),
            )),
        }
    }
}
