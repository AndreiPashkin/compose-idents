use crate::ast::{AliasSpec, Ast, LoopSpec, NodeId};
use proc_macro2::Span;
use std::rc::Rc;
use syn::spanned::Spanned;
use syn::Block;

/// Root AST produced by the parse phase, containing optional loops, optional alias spec, and a user block.
#[derive(Debug)]
pub struct RawAST {
    id: NodeId,
    loops: Option<Rc<LoopSpec>>,
    spec: Option<Rc<AliasSpec>>,
    block: Block,
}

impl Ast for RawAST {
    fn id(&self) -> NodeId {
        self.id
    }
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

impl RawAST {
    /// Creates new RawAST with the given components.
    pub fn new(
        id: NodeId,
        loops: Option<Rc<LoopSpec>>,
        spec: Option<Rc<AliasSpec>>,
        block: Block,
    ) -> Self {
        Self {
            id,
            loops,
            spec,
            block,
        }
    }

    /// Reads the loop specification if any.
    pub fn loops(&self) -> Option<Rc<LoopSpec>> {
        self.loops.clone()
    }

    /// Reads the alias specification if any.
    pub fn spec(&self) -> Option<Rc<AliasSpec>> {
        self.spec.clone()
    }

    /// Reads an immutable reference to the code block.
    pub fn block(&self) -> &Block {
        &self.block
    }
}
