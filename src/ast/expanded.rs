use crate::ast::{AliasSpec, Ast, NodeId};
use proc_macro2::Span;
use std::rc::Rc;
use syn::spanned::Spanned;

/// A single source code block and a set of alias-definitions.
#[derive(Debug, Clone)]
pub struct BlockRewrite {
    spec: Rc<AliasSpec>,
    block: syn::Block,
}

impl BlockRewrite {
    pub fn new(spec: Rc<AliasSpec>, block: syn::Block) -> Self {
        Self { spec, block }
    }
    pub fn spec(&self) -> &Rc<AliasSpec> {
        &self.spec
    }
    pub fn block(&self) -> &syn::Block {
        &self.block
    }
}

/// Simplified AST.
#[derive(Debug, Clone)]
pub struct ExpandedAST {
    id: NodeId,
    blocks: Vec<BlockRewrite>,
}

impl ExpandedAST {
    pub fn new(id: NodeId, invocations: Vec<BlockRewrite>) -> Self {
        Self {
            id,
            blocks: invocations,
        }
    }
    pub fn block_rewrite_items(&self) -> &[BlockRewrite] {
        &self.blocks
    }
}

impl Ast for ExpandedAST {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        if let Some(inv) = self.blocks.first() {
            inv.block().span()
        } else {
            Span::call_site()
        }
    }
}
