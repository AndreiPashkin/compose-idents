#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../snippets/docs.md")]

mod ast;
mod core;
mod error;
mod eval;
mod expand;
mod funcs;
mod interpreter;
mod parse;
mod resolve;
mod substitution;
mod util;

use crate::ast::RawAST;
use crate::core::Environment;
use crate::interpreter::Interpreter;
use crate::util::unique_id::next_unique_id;
use proc_macro::TokenStream;
use std::rc::Rc;
use syn::parse_macro_input;
use util::deprecation::DeprecationService;

fn compose_core(input: TokenStream, prefix: &'static str) -> TokenStream {
    let deprecation_service = DeprecationService::new_rc(prefix);
    DeprecationService::maybe_set_global(deprecation_service);
    let deprecation_service_scope = DeprecationService::scoped();

    let environment = Rc::new(Environment::new_initialized(next_unique_id()));
    Environment::maybe_set_global(environment.clone());

    let interpreter = Interpreter::new(environment, deprecation_service_scope);

    let args = parse_macro_input!(input as RawAST);
    match interpreter.execute(args) {
        Ok(ts) => ts.into(),
        Err(err) => {
            let syn_err: syn::Error = err.into();
            TokenStream::from(syn_err.into_compile_error())
        }
    }
}

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// In addition to replacing identifier aliases it replaces tokens like `% alias %` in string
/// literals (including in doc-attributes).
///
/// # Example
///
/// ```rust
/// use compose_idents::compose;
///
/// compose!(
///     // For-in loops could be used to generate multiple variations of the code.
///     for (suffix, (interjection, noun)) in [
///         (BAR, (Hello, "world")),
///         (baz, ("Hallo", "welt")),
///     ]
///
///     // A simple alias definition.
///     my_fn = concat(foo, _, 1, _, lower(suffix)),
///     // Many functions are overloaded support different input argument types.
///     greeting = concat(to_str(interjection), ", ", noun, "!"),
///     {
///         // String placeholders `% my_alias %` are expanded inside literals and doc attributes.
///         #[doc = "Greets: % greeting %"]
///         fn my_fn() -> &'static str { greeting }
///     },
/// );
///
/// assert_eq!(foo_1_bar(), "Hello, world!");
/// assert_eq!(foo_1_baz(), "Hallo, welt!");
/// ```
///
/// # Reference
///
#[doc = include_str!("../snippets/reference_h2.md")]
#[proc_macro]
pub fn compose(input: TokenStream) -> TokenStream {
    compose_core(input, "compose!: ")
}

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// In addition to replacing identifier aliases it replaces tokens like `% alias %` in string
/// literals (including in doc-attributes).
///
/// # Deprecation
///
/// The macro is deprecated. Please use [`compose!`] instead.
///
/// # Example
///
/// ```rust
/// use compose_idents::compose_idents;
///
/// compose_idents!(
///     // For-in loops could be used to generate multiple variations of the code.
///     for (suffix, (interjection, noun)) in [
///         (BAR, (Hello, "world")),
///         (baz, ("Hallo", "welt")),
///     ]
///
///     // A simple alias definition.
///     my_fn = concat(foo, _, 1, _, lower(suffix)),
///     // Many functions are overloaded support different input argument types.
///     greeting = concat(to_str(interjection), ", ", noun, "!"),
///     {
///         // String placeholders `% my_alias %` are expanded inside literals and doc attributes.
///         #[doc = "Greets: % greeting %"]
///         fn my_fn() -> &'static str { greeting }
///     },
/// );
///
/// assert_eq!(foo_1_bar(), "Hello, world!");
/// assert_eq!(foo_1_baz(), "Hallo, welt!");
/// ```
///
/// # Reference
///
#[doc = include_str!("../snippets/reference_h2_compose_idents.md")]
#[deprecated(
    since = "0.3.0",
    note = "Renamed to compose!. Use compose!(...) instead."
)]
#[proc_macro]
pub fn compose_idents(input: TokenStream) -> TokenStream {
    compose_core(input, "compose_idents!: ")
}
