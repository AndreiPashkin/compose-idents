//! Contains AST-elements related to for-loops.
use crate::ast::{Alias, Ast, Expr, Tuple};
use proc_macro2::Span;

/// Alias declared by a loop.
///
/// In an expression like `for (a, b) in [(1, 2), (3, 4)]`, the alias would be `(a, b)`.
pub enum LoopAlias {
    Simple(Alias),
    Tuple(Tuple<Alias>),
}

/// Source value of a loop.
pub enum LoopSourceValue {
    Simple(Expr),
    Tuple(Tuple<Expr>),
}

impl Ast for LoopSourceValue {
    fn span(&self) -> Span {
        match self {
            LoopSourceValue::Simple(expr) => expr.span(),
            LoopSourceValue::Tuple(tuple) => tuple.span(),
        }
    }
}

/// A list of source values for a loop.
///
/// In an expression like `for (a, b) in [(1, 2), (3, 4)]`, the value list
/// would be `[(1, 2), (3, 4)]`.
pub struct LoopSourceValueList {
    values: Vec<LoopSourceValue>,
    span: Span,
}

impl LoopSourceValueList {
    /// Creates a new [`LoopSourceValueList`] with the given values.
    pub fn new(values: Vec<LoopSourceValue>, span: Span) -> Self {
        Self { values, span }
    }

    /// Reads the source values.
    pub fn values(&self) -> &[LoopSourceValue] {
        &self.values
    }
}

impl Ast for LoopSourceValueList {
    fn span(&self) -> Span {
        self.span
    }
}

/// A single loop.
pub struct LoopSpecItem {
    alias: LoopAlias,
    list: LoopSourceValueList,
    span: Span,
}

impl LoopSpecItem {
    pub fn new(alias: LoopAlias, list: LoopSourceValueList, span: Span) -> Self {
        Self { alias, list, span }
    }

    /// Reads the loop's alias.
    pub fn alias(&self) -> &LoopAlias {
        &self.alias
    }

    /// Reads the loop's source value list.
    pub fn list(&self) -> &LoopSourceValueList {
        &self.list
    }
}

impl Ast for LoopSpecItem {
    fn span(&self) -> Span {
        self.span
    }
}

/// Specification of all loops.
pub struct LoopSpec {
    loops: Vec<LoopSpecItem>,
}

impl LoopSpec {
    /// Creates a new [`LoopSpec`] with the given loops.
    pub fn new(loops: Vec<LoopSpecItem>) -> Self {
        Self { loops }
    }
    /// Reads the loops.
    pub fn loops(&self) -> &[LoopSpecItem] {
        &self.loops
    }
}

impl Ast for LoopSpec {
    fn span(&self) -> Span {
        if let Some(first) = self.loops.first() {
            first.span()
        } else {
            Span::call_site()
        }
    }
}
