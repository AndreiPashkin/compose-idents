use crate::ast::Alias;
use proc_macro2::Span;
use std::collections::HashMap;

pub trait Spanned {
    /// Returns the span of the item.
    fn span(&self) -> Span;
}

/// An AST node that has a syntactic span.
pub trait Ast: Spanned {}

/// Lexical scope.
#[derive(Default, Clone)]
pub struct Scope<'a> {
    aliases: HashMap<Alias, &'a dyn Ast>,
}

impl<'a> Scope<'a> {
    pub fn names_mut(&mut self) -> &mut HashMap<Alias, &'a dyn Ast> {
        &mut self.aliases
    }
}
