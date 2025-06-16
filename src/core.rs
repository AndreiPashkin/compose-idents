//! Defines core types used throughout the project.

use crate::util::unique_id::next_unique_id;
use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::{Ident, LitStr};

/// State of a particular macro invocation.
///
/// Contains data useful for internal components and used within the scope of a single macro
/// invocation.
#[derive(Debug)]
pub struct State {
    /// Random seed.
    seed: u64,
}

impl State {
    /// Creates a new State with the given `seed`.
    pub fn new() -> Self {
        Self {
            seed: next_unique_id(),
        }
    }

    /// Reads the seed value.
    #[inline]
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

/// Visitor that replaces aliases in the provided code block with their definitions.
pub struct ComposeIdentsVisitor {
    replacements: HashMap<Ident, Ident>,
}

impl ComposeIdentsVisitor {
    pub fn new(replacements: HashMap<Ident, Ident>) -> Self {
        Self { replacements }
    }
}

impl VisitMut for ComposeIdentsVisitor {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(replacement) = self.replacements.get(ident) {
            *ident = replacement.clone();
        }
    }

    fn visit_lit_str_mut(&mut self, i: &mut LitStr) {
        let value = i.value();
        let mut formatted = i.value().clone();

        for (alias, repl) in self.replacements.iter() {
            formatted = formatted.replace(&format!("%{}%", alias), &repl.to_string());
        }

        if formatted != value {
            *i = LitStr::new(&formatted, i.span());
        }
    }
}
