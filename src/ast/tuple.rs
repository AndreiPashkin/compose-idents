//! Contains AST-elements used for defining tuples and contained values.
use crate::ast::{Ast, NodeId};
use proc_macro2::Span;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::rc::Rc;

/// A value of a form `(foo, (bar, baz))`.
#[derive(Debug, Clone)]
pub struct Tuple<V> {
    id: NodeId,
    values: Vec<TupleValue<V>>,
    span: Span,
}

impl<V> Tuple<V> {
    pub fn new(id: NodeId, values: Vec<TupleValue<V>>, span: Span) -> Self {
        Self { id, values, span }
    }

    pub fn values(&self) -> &[TupleValue<V>] {
        &self.values
    }

    pub fn iter_recursive(&self) -> TupleValuesRecursiveIterator<'_, V> {
        TupleValuesRecursiveIterator::new(self)
    }
    pub fn iter_destructuring(&self) -> TupleValuesDestructuringIterator<'_, V> {
        TupleValuesDestructuringIterator::new(self)
    }
}

/// Recursively iterates over the elements of the tuple and nested tuples.
#[derive(Debug, Clone)]
pub struct TupleValuesRecursiveIterator<'a, V> {
    stack: VecDeque<&'a TupleValue<V>>,
}

impl<'a, V> TupleValuesRecursiveIterator<'a, V> {
    pub fn new(tuple: &'a Tuple<V>) -> Self {
        Self {
            stack: VecDeque::from_iter(tuple.values().iter()),
        }
    }
}

impl<'a, V> Iterator for TupleValuesRecursiveIterator<'a, V> {
    type Item = Rc<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.stack.pop_front().map(|v| v.kind());
        match item {
            Some(TupleValueKind::Value(value)) => Some(value.clone()),
            Some(TupleValueKind::Tuple(tuple)) => {
                self.stack.extend(tuple.values().iter());
                self.next()
            }
            None => None,
        }
    }
}

impl<V> Ast for Tuple<V> {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        self.span
    }
}

/// A value of a tuple.
#[derive(Debug, Clone)]
pub struct TupleValue<V> {
    id: NodeId,
    span: Span,
    kind: TupleValueKind<V>,
}

impl<V> TupleValue<V> {
    /// Creates a new tuple value from a single value.
    pub fn from_value(id: NodeId, value: Rc<V>, span: Span) -> Self {
        Self {
            id,
            span,
            kind: TupleValueKind::Value(value),
        }
    }

    /// Creates a new tuple value from a nested tuple.
    pub fn from_tuple(id: NodeId, tuple: Tuple<V>, span: Span) -> Self {
        Self {
            id,
            span,
            kind: TupleValueKind::Tuple(tuple),
        }
    }

    /// The inner value of the tuple value.
    pub fn kind(&self) -> &TupleValueKind<V> {
        &self.kind
    }
}

impl<V> Ast for TupleValue<V> {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum TupleValueKind<V> {
    Tuple(Tuple<V>),
    Value(Rc<V>),
}

impl<V> Ast for TupleValueKind<V>
where
    V: Ast,
{
    fn id(&self) -> NodeId {
        match self {
            TupleValueKind::Tuple(t) => t.id(),
            TupleValueKind::Value(v) => v.id(),
        }
    }

    fn span(&self) -> Span {
        match self {
            TupleValueKind::Tuple(t) => t.span(),
            TupleValueKind::Value(v) => v.span(),
        }
    }
}

/// Recursively iterates over [`TupleValueKind`] elements - both tuples or values.
///
/// This allows to compare structures of multiple nested tuples without writing recursive code and
/// instead operating with flat iterators.
#[derive(Debug, Clone)]
pub struct TupleValuesDestructuringIterator<'a, V> {
    stack: VecDeque<&'a TupleValue<V>>,
}

impl<'a, V> TupleValuesDestructuringIterator<'a, V> {
    pub fn new(tuple: &'a Tuple<V>) -> Self {
        Self {
            stack: VecDeque::from_iter(tuple.values().iter()),
        }
    }
}

impl<'a, V> Iterator for TupleValuesDestructuringIterator<'a, V> {
    type Item = &'a TupleValueKind<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.stack.pop_front().map(|v| v.kind());
        match item {
            Some(value @ TupleValueKind::Value(_)) => Some(value),
            Some(tuple @ TupleValueKind::Tuple(values)) => {
                self.stack.extend(values.values().iter());
                Some(tuple)
            }
            None => None,
        }
    }
}
