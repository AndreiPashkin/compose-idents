use proc_macro2::Ident;
use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::LitStr;

/// Visitor that replaces aliases in the provided code block with their definitions.
pub struct AliasSubstitutionVisitor {
    replacements: HashMap<Ident, Ident>,
}

impl AliasSubstitutionVisitor {
    pub fn new(replacements: HashMap<Ident, Ident>) -> Self {
        Self { replacements }
    }
}

impl VisitMut for AliasSubstitutionVisitor {
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
