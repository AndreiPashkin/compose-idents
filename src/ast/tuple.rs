//! Contains AST-elements used for defining tuples and contained values.
use crate::ast::{Ast, Spanned};
use proc_macro2::Span;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::FromIterator;

/// A value of a form `(foo, (bar, baz))`.
#[derive(Debug, Clone)]
pub struct Tuple<V>
where
    V: Debug + Clone,
{
    values: Vec<TupleValue<V>>,
    span: Span,
}

impl<V: Debug + Clone> Tuple<V> {
    /// Creates a new tuple.
    pub fn new(values: Vec<TupleValue<V>>, span: Span) -> Self {
        Self { values, span }
    }

    pub fn values(&self) -> &[TupleValue<V>] {
        &self.values
    }

    pub fn iter_recursive(&self) -> TupleValuesRecursiveIterator<V> {
        TupleValuesRecursiveIterator::new(self)
    }
    pub fn iter_destructuring(&self) -> TupleValuesDestructuringIterator<V> {
        TupleValuesDestructuringIterator::new(self)
    }
}

/// Recursively iterates over the elements of the tuple and nested tuples.
#[derive(Debug, Clone)]
pub struct TupleValuesRecursiveIterator<V>
where
    V: Debug + Clone,
{
    stack: VecDeque<TupleValue<V>>,
}

impl<V> TupleValuesRecursiveIterator<V>
where
    V: Debug + Clone,
{
    pub fn new(tuple: &Tuple<V>) -> Self {
        Self {
            stack: VecDeque::from_iter(tuple.values().iter().cloned()),
        }
    }
}

impl<V> Iterator for TupleValuesRecursiveIterator<V>
where
    V: Debug + Clone,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front() {
            Some(TupleValue::Value(value)) => Some(value),
            Some(TupleValue::Tuple(tuple)) => {
                self.stack.extend(tuple.values().iter().cloned());
                self.next()
            }
            None => None,
        }
    }
}

impl<V> Spanned for Tuple<V>
where
    V: Debug + Clone,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<V: Debug + Clone> Ast for Tuple<V> {}

/// Allowed values in a tuple.
#[derive(Debug, Clone)]
pub enum TupleValue<V>
where
    V: Debug + Clone,
{
    Tuple(Tuple<V>),
    Value(V),
}

impl<V> Spanned for TupleValue<V>
where
    V: Debug + Clone + Ast,
{
    fn span(&self) -> Span {
        match self {
            TupleValue::Tuple(tuple) => tuple.span(),
            TupleValue::Value(value) => value.span(),
        }
    }
}

impl<V: Debug + Clone + Ast> Ast for TupleValue<V> {}

/// Recursively iterates over [`TupleValue`] elements - both tuples or values.
///
/// This allows to compare structures of multiple nested tuples without writing recursive code and
/// instead operating with flat iterators.
#[derive(Debug, Clone)]
pub struct TupleValuesDestructuringIterator<V>
where
    V: Debug + Clone,
{
    stack: VecDeque<TupleValue<V>>,
}

impl<V> TupleValuesDestructuringIterator<V>
where
    V: Debug + Clone,
{
    pub fn new(tuple: &Tuple<V>) -> Self {
        Self {
            stack: VecDeque::from_iter(tuple.values().iter().cloned()),
        }
    }
}

impl<V> Iterator for TupleValuesDestructuringIterator<V>
where
    V: Debug + Clone,
{
    type Item = TupleValue<V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front() {
            Some(value @ TupleValue::Value(_)) => Some(value),
            Some(TupleValue::Tuple(values)) => {
                self.stack.extend(values.values().iter().cloned());
                Some(TupleValue::Tuple(values))
            }
            None => None,
        }
    }
}
