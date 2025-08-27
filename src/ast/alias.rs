use crate::ast::{Ast, NodeId};
use proc_macro2::{Ident, Span};

/// Alias declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alias {
    id: NodeId,
    ident: Ident,
}

impl Ast for Alias {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl Alias {
    /// Creates a new [`Alias`] with the given identifier.
    pub fn new(id: NodeId, ident: Ident) -> Self {
        Self { id, ident }
    }

    /// Reads the identifier.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}
