//! Provides [`LoopSpec`], a specification of all loops.
use crate::ast::{Ast, LoopSpecItem, NodeId};
use proc_macro2::Span;

/// Specification of all loops.
#[derive(Debug)]
pub struct LoopSpec {
    id: NodeId,
    loops: Vec<LoopSpecItem>,
}

impl LoopSpec {
    /// Creates a new [`LoopSpec`] with the given loops.
    pub fn new(id: NodeId, loops: Vec<LoopSpecItem>) -> Self {
        Self { id, loops }
    }
    /// Reads the loops.
    pub fn loops(&self) -> &[LoopSpecItem] {
        &self.loops
    }
}

impl Ast for LoopSpec {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        if let Some(first) = self.loops.first() {
            first.span()
        } else {
            Span::call_site()
        }
    }
}
