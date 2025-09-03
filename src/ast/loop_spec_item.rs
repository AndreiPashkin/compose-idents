//! Provides [`LoopSpecItem`] - a single for-loop.

use crate::ast::{Ast, LoopAlias, LoopSourceValueList, NodeId};
use proc_macro2::Span;
use std::rc::Rc;

/// A single loop.
#[derive(Debug, Clone)]
pub struct LoopSpecItem {
    id: NodeId,
    alias: Rc<LoopAlias>,
    list: Rc<LoopSourceValueList>,
    span: Span,
}

impl LoopSpecItem {
    pub fn new(
        id: NodeId,
        alias: Rc<LoopAlias>,
        list: Rc<LoopSourceValueList>,
        span: Span,
    ) -> Self {
        Self {
            id,
            alias,
            list,
            span,
        }
    }

    /// Reads the loop's alias.
    pub fn alias(&self) -> Rc<LoopAlias> {
        self.alias.clone()
    }

    /// Reads the loop's source value list.
    pub fn list(&self) -> Rc<LoopSourceValueList> {
        self.list.clone()
    }
}

impl Ast for LoopSpecItem {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        self.span
    }
}
