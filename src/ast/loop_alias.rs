//! Contains AST-elements related to for-loop aliases.
use crate::ast::{Alias, Ast, NodeId, Tuple};
use proc_macro2::Span;
use std::rc::Rc;

/// Alias declared by a loop.
///
/// In an expression like `for (a, b) in [(1, 2), (3, 4)]`, the alias would be `(a, b)`.
#[derive(Debug, Clone)]
pub enum LoopAlias {
    Simple(Rc<Alias>),
    Tuple(Tuple<Alias>),
}

impl LoopAlias {
    /// Creates a new [`LoopAlias`] from a singular value.
    pub fn from_simple(alias: Alias) -> Self {
        Self::Simple(Rc::new(alias))
    }
    /// Creates a new [`LoopAlias`] from a tuple.
    pub fn from_tuple(tuple: Tuple<Alias>) -> Self {
        Self::Tuple(tuple)
    }
}

impl Ast for LoopAlias {
    fn id(&self) -> NodeId {
        match self {
            LoopAlias::Simple(alias) => alias.id(),
            LoopAlias::Tuple(tuple) => tuple.id(),
        }
    }
    fn span(&self) -> Span {
        match self {
            LoopAlias::Simple(alias) => alias.span(),
            LoopAlias::Tuple(tuple) => tuple.span(),
        }
    }
}
