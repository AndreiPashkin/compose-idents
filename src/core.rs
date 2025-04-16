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
pub(super) struct IdentSpec {
    pub(super) items: Vec<AliasSpecItem>,
    pub(super) is_comma_used: Option<bool>,
}

impl AliasSpecItem {
    fn replacement(&self, state: &State) -> Ident {
        let ident = self.exprs.iter().fold("".to_string(), |acc, item| {
            format!("{}{}", acc, item.eval(state))
        });
        format_ident!("{}", ident)
    }
}

/// Arguments to the [`compose_idents`] macro.
pub(super) struct ComposeIdentsArgs {
    pub(super) spec: IdentSpec,
    pub(super) block: Block,
}

impl IdentSpec {
    pub(super) fn replacements(&self, state: &State) -> HashMap<Ident, Ident> {
        self.items
            .iter()
            .map(|item| (item.alias.clone(), item.replacement(state)))
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
