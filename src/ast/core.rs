use proc_macro2::Span;
use std::collections::HashMap;

/// An AST node that has a syntactic span.
pub trait Ast {
    /// Returns the span of the item.
    fn span(&self) -> Span;
}

/// Lexical scope.
#[derive(Default, Clone)]
pub struct Scope<'a> {
    aliases: HashMap<String, &'a dyn Ast>,
}

impl<'a> Scope<'a> {
    pub fn names_mut(&mut self) -> &mut HashMap<String, &'a dyn Ast> {
        &mut self.aliases
    }
}
