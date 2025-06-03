//! Defines core types used throughout the project.

use crate::deprecation::DeprecationWarning;
use crate::unique_id::next_unique_id;
use std::collections::{BTreeSet, HashMap};
use syn::visit_mut::VisitMut;
use syn::{
    visit_mut, Attribute, Field, File, ForeignItem, Ident, Item, LitStr, TraitItem, Variant,
};

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

/// Processes the code block tries to add deprecations to existing syntactic elements.
pub struct DeprecationWarningVisitor {
    deprecation_warnings: BTreeSet<DeprecationWarning>,
    warning_prefix: String,
}

impl DeprecationWarningVisitor {
    pub fn new(deprecation_warnings: BTreeSet<DeprecationWarning>, warning_prefix: String) -> Self {
        Self {
            deprecation_warnings,
            warning_prefix,
        }
    }

    /// Try to place the deprecation attribute into the given attribute list.
    fn process_deprecations(&mut self, attrs: &mut Vec<Attribute>) {
        if self.deprecation_warnings.is_empty() {
            return;
        }
        for attr in attrs.iter() {
            if attr.path().is_ident("deprecated") {
                return;
            }
        }
        let warning = &self.deprecation_warnings.pop_first().unwrap();
        let attr = warning.with_prefix(&self.warning_prefix).to_attribute();
        attrs.push(attr);
    }
}

impl VisitMut for DeprecationWarningVisitor {
    fn visit_field_mut(&mut self, node: &mut Field) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_field_mut(self, node);
    }

    fn visit_file_mut(&mut self, node: &mut File) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_file_mut(self, node);
    }

    fn visit_foreign_item_mut(&mut self, node: &mut ForeignItem) {
        use ForeignItem::*;

        match node {
            Fn(item) => self.process_deprecations(&mut item.attrs),
            Static(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_foreign_item_mut(self, node);
    }

    fn visit_item_mut(&mut self, node: &mut Item) {
        use Item::*;

        match node {
            Const(item) => self.process_deprecations(&mut item.attrs),
            Enum(item) => self.process_deprecations(&mut item.attrs),
            ExternCrate(item) => self.process_deprecations(&mut item.attrs),
            Fn(item) => self.process_deprecations(&mut item.attrs),
            ForeignMod(item) => self.process_deprecations(&mut item.attrs),
            Impl(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            Mod(item) => self.process_deprecations(&mut item.attrs),
            Static(item) => self.process_deprecations(&mut item.attrs),
            Struct(item) => self.process_deprecations(&mut item.attrs),
            Trait(item) => self.process_deprecations(&mut item.attrs),
            TraitAlias(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Union(item) => self.process_deprecations(&mut item.attrs),
            Use(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_item_mut(self, node);
    }

    fn visit_trait_item_mut(&mut self, node: &mut TraitItem) {
        use TraitItem::*;

        match node {
            Const(item) => self.process_deprecations(&mut item.attrs),
            Fn(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_trait_item_mut(self, node);
    }

    fn visit_variant_mut(&mut self, node: &mut Variant) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_variant_mut(self, node);
    }
}
