#![doc = include_str!("../README.md")]

mod core;
mod eval;
mod funcs;

use crate::core::{Expr, State};
use crate::eval::Eval;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    visit_mut::VisitMut,
    Block, Ident, Token,
};

struct IdentSpecItem {
    alias: Ident,
    exprs: Vec<Expr>,
}

impl IdentSpecItem {
    fn replacement(&self, state: &State) -> Ident {
        let ident = self.exprs.iter().fold("".to_string(), |acc, item| {
            format!("{}{}", acc, item.eval(state))
        });
        format_ident!("{}", ident)
    }
}

struct IdentSpec {
    items: Vec<IdentSpecItem>,
}

impl Parse for IdentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        loop {
            let alias: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let content;
            bracketed!(content in input);
            let mut exprs = Vec::new();
            loop {
                match content.parse::<Expr>() {
                    Ok(expr) => exprs.push(expr),
                    Err(err) => return Err(err),
                }
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }
            items.push(IdentSpecItem { alias, exprs });
            input.parse::<Token![;]>()?;
            if !input.peek(Ident) {
                break;
            }
        }
        Ok(IdentSpec { items })
    }
}

impl IdentSpec {
    fn replacements(&self, state: &State) -> HashMap<Ident, Ident> {
        self.items
            .iter()
            .map(|item| (item.alias.clone(), item.replacement(state)))
            .collect()
    }
}

struct ComposeIdentsArgs {
    spec: IdentSpec,
    block: Block,
}

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec: IdentSpec = input.parse()?;
        let block: Block = input.parse()?;
        Ok(ComposeIdentsArgs { spec, block })
    }
}

struct ComposeIdentsVisitor {
    replacements: HashMap<Ident, Ident>,
}

impl VisitMut for ComposeIdentsVisitor {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(replacement) = self.replacements.get(ident) {
            *ident = replacement.clone();
        }
    }
}

static mut COUNTER: u64 = 0;

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../snippets/usage.rs")]
/// ```
#[proc_macro]
pub fn compose_idents(input: TokenStream) -> TokenStream {
    let state = State {
        seed: unsafe {
            COUNTER += 1;
            COUNTER
        },
    };
    let args = parse_macro_input!(input as ComposeIdentsArgs);
    let mut visitor = ComposeIdentsVisitor {
        replacements: args.spec.replacements(&state),
    };
    let mut block = args.block;
    visitor.visit_block_mut(&mut block);

    let block_content = block.stmts;

    let expanded = quote! { #(#block_content)* };
    TokenStream::from(expanded)
}
