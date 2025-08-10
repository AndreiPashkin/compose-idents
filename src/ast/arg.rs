use crate::ast::{Ast, NodeId};
use crate::util::unique_id::next_unique_id;
use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{LitInt, LitStr};

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
    LitStr(LitStr),
    LitInt(LitInt),
    Tokens(TokenStream),
    Underscore(Span),
}

impl Arg {
    pub fn new(id: NodeId, inner: ArgInner) -> Self {
        Self { id, inner }
    }

    pub fn from_ident(ident: Ident) -> Self {
        Self::new(next_unique_id(), ArgInner::Ident(ident))
    }

    pub fn from_underscore(span: Span) -> Self {
        Self::new(next_unique_id(), ArgInner::Underscore(span))
    }

    pub fn from_lit_str(lit_str: LitStr) -> Self {
        Self::new(next_unique_id(), ArgInner::LitStr(lit_str))
    }

    pub fn from_lit_int(lit_int: LitInt) -> Self {
        Self::new(next_unique_id(), ArgInner::LitInt(lit_int))
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
            ArgInner::LitStr(lit_str) => lit_str.span(),
            ArgInner::LitInt(lit_int) => lit_int.span(),
            ArgInner::Tokens(tokens) => tokens.span(),
            ArgInner::Underscore(span) => *span,
        }
    }
}
