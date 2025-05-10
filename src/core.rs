/// Defines core types used throughout the project.
use crate::eval::Eval;
use quote::format_ident;
use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::{Block, Ident, LitStr};

/// State of a particular macro invocation.
///
/// Contains data useful for internal components and used within the scope of a single macro
/// invocation.
#[derive(Debug)]
pub struct State {
    /// Random seed.
    pub(super) seed: u64,
}

/// Argument to the [`compose_idents`] macro in form of an ident, underscore or a string literal.
#[derive(Debug)]
pub struct Arg {
    pub(super) value: String,
}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug)]
pub enum Func {
    Upper(Box<Expr>),
    Lower(Box<Expr>),
    SnakeCase(Box<Expr>),
    CamelCase(Box<Expr>),
    PascalCase(Box<Expr>),
    Hash(Box<Expr>),
}

/// Expression in form of an argument or a function call.
#[derive(Debug)]
pub(super) enum Expr {
    ArgExpr { value: Box<Arg> },
    FuncCallExpr { value: Box<Func> },
}

/// A single alias specification.
pub(super) struct AliasSpecItem {
    pub(super) alias: Ident,
    pub(super) exprs: Vec<Expr>,
}

/// Specification of aliases provided to the [`compose_idents`] macro by the user.
pub(super) struct AliasSpec {
    pub(super) items: Vec<AliasSpecItem>,
    pub(super) is_comma_used: Option<bool>,
}

impl AliasSpecItem {
    pub(super) fn replacement(
        &self,
        state: &State,
        arg_replacements: &HashMap<String, String>,
    ) -> Ident {
        let ident = self.exprs.iter().fold("".to_string(), |acc, item| {
            let arg = item.eval(state);
            let replacement = arg_replacements.get(&arg);
            let arg = match replacement {
                Some(arg) => arg,
                None => &arg,
            };
            format!("{}{}", acc, arg)
        });
        format_ident!("{}", ident)
    }
}

/// Arguments to the [`compose_idents`] macro.
pub(super) struct ComposeIdentsArgs {
    pub(super) spec: AliasSpec,
    pub(super) block: Block,
}

impl AliasSpec {
    pub(super) fn replacements(&self, state: &State) -> HashMap<Ident, Ident> {
        let mut arg_replacements = HashMap::new();
        self.items
            .iter()
            .map(|item| {
                let replacement = item.replacement(state, &arg_replacements);
                arg_replacements.insert(format!("{}", item.alias), format!("{}", replacement));

                (item.alias.clone(), replacement)
            })
            .collect()
    }
}

/// Visitor that replaces aliases in the provided code block with their definitions.
pub(super) struct ComposeIdentsVisitor {
    pub(super) replacements: HashMap<Ident, Ident>,
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
