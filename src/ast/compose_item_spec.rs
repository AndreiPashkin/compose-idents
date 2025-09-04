use crate::ast::{AliasSpec, LoopSpec, NodeId};
use proc_macro2::Span;
use std::rc::Rc;

use super::core::Ast;

/// Invocation of `#[compose_item]`.
///
/// It is identical to `RawAST` except it does not contain the code block.
#[derive(Debug, Clone)]
pub struct ComposeItemSpec {
    id: NodeId,
    loops: Option<Rc<LoopSpec>>,
    spec: Option<Rc<AliasSpec>>,
}

impl ComposeItemSpec {
    pub fn new(id: NodeId, loops: Option<Rc<LoopSpec>>, spec: Option<Rc<AliasSpec>>) -> Self {
        Self { id, loops, spec }
    }

    pub fn loops(&self) -> Option<Rc<LoopSpec>> {
        self.loops.clone()
    }

    pub fn spec(&self) -> Option<Rc<AliasSpec>> {
        self.spec.clone()
    }
}

impl Ast for ComposeItemSpec {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        if let Some(loops) = &self.loops {
            loops.span()
        } else if let Some(spec) = &self.spec {
            spec.span()
        } else {
            Span::call_site()
        }
    }
}
