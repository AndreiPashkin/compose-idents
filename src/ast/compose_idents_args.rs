use crate::ast::{AliasSpec, Ast, NodeId};
use proc_macro2::Span;
use std::rc::Rc;
use syn::Block;

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    id: NodeId,
    spec: Rc<AliasSpec>,
    block: Block,
}

impl Ast for ComposeIdentsArgs {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.spec.span()
    }
}

impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(id: NodeId, spec: Rc<AliasSpec>, block: Block) -> Self {
        Self { id, spec, block }
    }

    /// Reads the alias specification.
    pub fn spec(&self) -> Rc<AliasSpec> {
        self.spec.clone()
    }

    /// Reads a mutable reference to the code block.
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}
