use crate::ast::{AliasSpec, Ast, LoopSpec, Spanned};
use proc_macro2::Span;
use syn::spanned::Spanned as SynSpanned;
use syn::Block;

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    loops: Option<LoopSpec>,
    spec: Option<AliasSpec>,
    block: Block,
}

impl Spanned for ComposeIdentsArgs {
    fn span(&self) -> Span {
        if let Some(loops) = &self.loops {
            loops.span()
        } else if let Some(spec) = &self.spec {
            spec.span()
        } else {
            self.block.span()
        }
    }
}

impl Ast for ComposeIdentsArgs {}

impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(loops: Option<LoopSpec>, spec: Option<AliasSpec>, block: Block) -> Self {
        Self { loops, spec, block }
    }

    /// Reads the loop specification if any.
    pub fn loops(&self) -> Option<&LoopSpec> {
        self.loops.as_ref()
    }

    /// Reads the alias specification.
    pub fn spec(&self) -> Option<&AliasSpec> {
        self.spec.as_ref()
    }

    /// Reads a mutable reference to the code block.
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}
