use crate::ast::{AliasSpecItem, Ast, NodeId};
use proc_macro2::Span;
use std::rc::Rc;

/// Specification of aliases provided to the [`compose_idents`] macro.
#[derive(Debug)]
pub struct AliasSpec {
    id: NodeId,
    items: Vec<Rc<AliasSpecItem>>,
    is_comma_used: Option<bool>,
}

impl Ast for AliasSpec {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.items
            .first()
            .map(|item| item.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl AliasSpec {
    /// Creates a new [`AliasSpec`] with the given items and separator information.
    pub fn new(id: NodeId, items: Vec<Rc<AliasSpecItem>>, is_comma_used: Option<bool>) -> Self {
        Self {
            id,
            items,
            is_comma_used,
        }
    }

    /// Reads the individual items in the alias specification.
    pub fn items(&self) -> &[Rc<AliasSpecItem>] {
        &self.items
    }

    /// Whether a comma is used as a separator.
    pub fn is_comma_used(&self) -> Option<bool> {
        self.is_comma_used
    }
}
