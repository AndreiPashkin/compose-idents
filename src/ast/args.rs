use crate::ast::{AliasSpec, Ast, LoopSpec};
use proc_macro2::Span;
use syn::Block;

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    loops: Option<LoopSpec>,
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
    pub fn new(loops: Option<LoopSpec>, spec: AliasSpec, block: Block) -> Self {
        Self { loops, spec, block }
    }

    /// Reads the loop specification if any.
    pub fn loops(&self) -> Option<&LoopSpec> {
        self.loops.as_ref()
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
