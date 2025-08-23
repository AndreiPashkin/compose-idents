use crate::ast::{CallMetadata, Expr, ValueMetadata};
use crate::core::{Func, Type};
use crate::util::log::debug;
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
    value_metadata: HashMap<NodeId, ValueMetadata>,
    call_metadata: HashMap<NodeId, CallMetadata>,
}

impl AstMetadata {
    pub fn set_value_metadata(&mut self, id: NodeId, target_type: Type, coercion_cost: u32) {
        debug!(
            "Setting value metadata for id: {}, type: {:?}, cost: {}",
            id, target_type, coercion_cost
        );
        self.value_metadata.insert(
            id,
            ValueMetadata {
                target_type,
                coercion_cost,
            },
        );
    }

    pub fn get_value_metadata(&self, id: NodeId) -> Option<&ValueMetadata> {
        debug!("Getting value metadata for id: {}", id);
        self.value_metadata.get(&id)
    }

    pub fn set_call_metadata(
        &mut self,
        id: NodeId,
        args: Vec<Rc<Expr>>,
        func: Rc<Func>,
        target_type: Type,
        coercion_cost: u32,
    ) {
        debug!(
            "Setting call metadata for id: {}, func: {:?}, args: {:?}, type: {:?}, cost: {}",
            id, func, args, target_type, coercion_cost
        );
        self.call_metadata.insert(
            id,
            CallMetadata {
                args,
                func,
                target_type,
                coercion_cost,
            },
        );
    }

    pub fn get_call_metadata(&self, id: NodeId) -> Option<&CallMetadata> {
        debug!("Getting call metadata for id: {}", id);
        self.call_metadata.get(&id)
    }
}
