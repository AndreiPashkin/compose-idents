use crate::ast::{AliasSpec, Ast};
use proc_macro2::Span;
use syn::Block;

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    spec: AliasSpec,
    block: Block,
}

impl Ast for ComposeIdentsArgs {
    fn span(&self) -> Span {
        self.spec.span()
    }
}

impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(spec: AliasSpec, block: Block) -> Self {
        Self { spec, block }
    }

    /// Reads the alias specification.
    pub fn spec(&self) -> &AliasSpec {
        &self.spec
    }

    /// Reads a mutable reference to the code block.
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}
