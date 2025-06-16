//! Provides [`Terminated`] type for parsing tokens until a terminator.
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::marker::PhantomData;
use syn::parse::{Parse, ParseStream};

/// Wraps the token-type `T` and parses it by consuming the input until the terminator `Term` or
/// the end if the input.
pub struct Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    value: T,
    terminator_type: PhantomData<Term>,
}

impl<T, Term> Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    pub(crate) fn into_value(self) -> T {
        self.value
    }
}

impl<T, Term> Parse for Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = TokenStream::new();
        while !input.is_empty() {
            let fork = input.fork();
            let is_terminator = fork.parse::<Term>().is_ok();
            if is_terminator {
                break;
            }
            let tt = input.parse::<TokenTree>()?;
            tokens.extend(tt.into_token_stream());
        }

        let value = syn::parse2::<T>(tokens)?;

        Ok(Terminated {
            value,
            terminator_type: PhantomData,
        })
    }
}
