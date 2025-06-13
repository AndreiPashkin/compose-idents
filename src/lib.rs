#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../snippets/docs.md")]

mod ast;
mod core;
mod deprecation;
mod error;
mod eval;
mod funcs;
mod interpreter;
mod parse;
mod resolve;
mod unique_id;

use crate::ast::ComposeIdentsArgs;
use crate::deprecation::DeprecationService;
use crate::interpreter::Interpreter;
use proc_macro::TokenStream;
use std::convert::TryInto;
use syn::parse_macro_input;

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
///     // Alias is defined as an expression that could be a value or a function call that evaluates
///     // in a valid identifier.
///     alias1 = concat(foo, _, bar),
///     alias2 = foo,
///     // Function calls could be nested.
///     alias3 = upper(snake_case(fooBarBaz)),
///     // Values could be idents, string or integer literals, underscores or free-form tokens.
///     alias4 = concat(foo, _, "bar", _, 42),
///     // Values could be converted to valid identifiers using `normalize()` function.
///     alias5 = concat(my, _, fn, _, normalize(My::Enum)),
///     // Bracket-based legacy syntax is still supported but deprecated.
///     alias6 = [part3, _, "part4", _, 1],
///     // ... more alias definitions
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
/// # Reference
///
#[doc = include_str!("../snippets/reference_h2.md")]
#[proc_macro]
pub fn compose_idents(input: TokenStream) -> TokenStream {
    let deprecation_service = DeprecationService::scoped();
    let args = parse_macro_input!(input as ComposeIdentsArgs);
    let interpreter = Interpreter::new(args, deprecation_service);
    match interpreter.execute() {
        Ok(ts) => ts.into(),
        Err(err) => {
            let syn_err: syn::Error = err.try_into().unwrap_or_else(|_| {
                syn::Error::new(proc_macro2::Span::call_site(), "Unknown error")
            });
            TokenStream::from(syn_err.into_compile_error())
        }
    }
}
