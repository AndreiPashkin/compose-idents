use crate::ast::{FuncInner, FuncMetadata};
use proc_macro2::Span;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeId = u64;

/// An AST node that has a syntactic span.
pub trait Ast {
    /// Ast node identifier.
    fn id(&self) -> NodeId;
    /// Returns the span of the item.
    fn span(&self) -> Span;
}

#[derive(Debug, Clone, Default)]
pub struct AstMetadata {
    func_metadata: HashMap<NodeId, Rc<FuncMetadata>>,
}

impl AstMetadata {
    pub fn new() -> Self {
        Self {
            func_metadata: HashMap::new(),
        }
    }

    pub fn set_func_metadata(&mut self, id: NodeId, inner: Rc<FuncInner>) {
        self.func_metadata
            .insert(id, Rc::new(FuncMetadata { inner }));
    }

    pub fn get_func_metadata(&self, id: NodeId) -> Option<Rc<FuncMetadata>> {
        self.func_metadata.get(&id).cloned()
    }
}
