use crate::ast::{Alias, AliasValue, Ast, NodeId};
use proc_macro2::Span;
use std::rc::Rc;

/// A single alias specification.
#[derive(Debug)]
pub struct AliasSpecItem {
    id: NodeId,
    alias: Rc<Alias>,
    value: Rc<AliasValue>,
}

impl Ast for AliasSpecItem {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.alias.span()
    }
}

impl AliasSpecItem {
    /// Creates a new [`AliasSpecItem`] with the given alias and expressions.
    pub fn new(id: NodeId, alias: Rc<Alias>, value: Rc<AliasValue>) -> Self {
        Self { id, alias, value }
    }

    /// Reads the alias identifier.
    pub fn alias(&self) -> Rc<Alias> {
        self.alias.clone()
    }

    /// Reads the alias value.
    pub fn value(&self) -> Rc<AliasValue> {
        self.value.clone()
    }
}
