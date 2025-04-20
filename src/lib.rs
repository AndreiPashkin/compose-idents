#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

mod core;
mod eval;
mod funcs;
mod parse;

use crate::core::{ComposeIdentsArgs, ComposeIdentsVisitor, State};
use proc_macro::TokenStream;
use quote::quote;
use std::sync::Mutex;
use syn::{parse_macro_input, visit_mut::VisitMut};

static COUNTER: Mutex<u64> = Mutex::new(0);

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
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    let state = State { seed: *counter };
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
