use crate::ast::{Ast, NodeId};
use crate::util::unique_id::next_unique_id;
use proc_macro2::{Ident, Span, TokenStream};

/// Argument to the [`compose_idents`] macro.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub struct Arg {
    id: NodeId,
    inner: ArgInner,
}

#[derive(Debug, Clone)]
pub enum ArgInner {
    Ident(Ident),
    LitStr(String),
    LitInt(u64),
    Tokens(TokenStream),
    Underscore,
}

impl Arg {
    pub fn new(id: NodeId, inner: ArgInner) -> Self {
        Self { id, inner }
    }

    pub fn from_ident(ident: Ident) -> Self {
        Self::new(next_unique_id(), ArgInner::Ident(ident))
    }

    pub fn from_underscore() -> Self {
        Self::new(next_unique_id(), ArgInner::Underscore)
    }

    pub fn from_lit_str(value: String) -> Self {
        Self::new(next_unique_id(), ArgInner::LitStr(value))
    }

    pub fn from_lit_int(value: u64) -> Self {
        Self::new(next_unique_id(), ArgInner::LitInt(value))
    }

    pub fn from_tokens(tokens: TokenStream) -> Self {
        Self::new(next_unique_id(), ArgInner::Tokens(tokens))
    }

    pub fn inner(&self) -> &ArgInner {
        &self.inner
    }
}

impl Ast for Arg {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        match &self.inner {
            ArgInner::Ident(ident) => ident.span(),
            ArgInner::LitStr(_) => Span::call_site(),
            ArgInner::LitInt(_) => Span::call_site(),
            ArgInner::Tokens(_) => Span::call_site(),
            ArgInner::Underscore => Span::call_site(),
        }
    }
}
