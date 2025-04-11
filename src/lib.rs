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
    Block, Ident, LitStr, Token,
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

// Note: the parsing code handles both commas or semicolons as argument separators
// this is done for backwards-compatibility with <= 0.0.4 versions.
const MIXING_SEP_ERROR: &str = r#"Mixing "," and ";" as separators is not allowed"#;

struct IdentSpec {
    items: Vec<IdentSpecItem>,
    is_comma_used: Option<bool>,
}

impl Parse for IdentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        let mut is_comma_used = None;

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

            let is_comma_current_sep = if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                true
            } else if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                false
            } else {
                return Err(input.error(r#"Expected "," or ";""#));
            };

            if let Some(is_comma_used) = is_comma_used {
                if is_comma_used != is_comma_current_sep {
                    return Err(input.error(MIXING_SEP_ERROR));
                }
            } else {
                is_comma_used = Some(is_comma_current_sep);
            }

            if !input.peek(Ident) {
                break;
            }
        }

        Ok(IdentSpec {
            items,
            is_comma_used,
        })
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

        let is_comma_current_sep = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(true)
        } else if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            Some(false)
        } else {
            None
        };

        if let (Some(is_comma_current_sep), Some(is_comma_used)) =
            (is_comma_current_sep, spec.is_comma_used)
        {
            if is_comma_current_sep ^ is_comma_used {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

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

static mut COUNTER: u64 = 0;

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// In addition to replacing identifier aliases it replaces tokens like `%alias%` in string
/// literals (including in doc-attributes).
///
/// # Syntax
///
/// ```rust,ignore
/// use compose_idents::compose_idents;
///
/// compose_idents!(
///     // Alias is defined one or more arguments that are supposed to be concatenated
///     alias1 = [part1, part2],
///     // Multiple aliases could be defined
///     // and they could be composed from arbitrary number of arguments
///     // Which could be identifiers, strings, numbers or underscores
///     alias2 = [part3, _, "part4", _, 1],
///     // Functions could applied to the arguments, calls to functions could be nested
///     alias3 = [some_func(part5), outer_func(inner_func(part6))],
///     // ... more aliases
///     {
///         // Code block that uses aliases as identifiers
///         // The aliases will be replaced with their replacements when the code is expanded
///         let alias1 = 42;
///
///         fn alias2() -> u32 { 42 }
///
///         // Aliases could be also used for string-formatting using %alias% syntax
///         #[doc = "Documentation for %alias3%"]
///         fn alias3() -> u32 { 42 }
///     },
/// );
/// ```
///
/// Semicolons could also be used as separators between the macro arguments for
/// backwards-compatibility. Mixing separator styles in the same macro invocation is not allowed.
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
