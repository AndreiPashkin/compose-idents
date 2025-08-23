use proc_macro2::{TokenStream, TokenTree};
use std::iter::FromIterator;

pub trait TokenStreamExt {
    fn into_vec(self) -> Vec<TokenTree>;
    fn to_vec(&self) -> Vec<TokenTree>;
}

impl TokenStreamExt for TokenStream {
    fn into_vec(self) -> Vec<TokenTree> {
        self.into_iter().collect()
    }
    fn to_vec(&self) -> Vec<TokenTree> {
        self.clone().into_vec()
    }
}

pub trait TokenVecExt {
    fn into_token_stream(self) -> TokenStream;
    fn to_token_stream(&self) -> TokenStream;
}

impl TokenVecExt for Vec<TokenTree> {
    fn into_token_stream(self) -> TokenStream {
        TokenStream::from_iter(self)
    }

    fn to_token_stream(&self) -> TokenStream {
        self.clone().into_token_stream()
    }
}

impl TokenVecExt for &[TokenTree] {
    fn into_token_stream(self) -> TokenStream {
        TokenStream::from_iter(self.iter().cloned())
    }

    fn to_token_stream(&self) -> TokenStream {
        TokenStream::from_iter(self.iter().cloned())
    }
}
