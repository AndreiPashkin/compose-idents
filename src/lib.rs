#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../snippets/docs.md")]

mod ast;
mod core;
mod error;
mod eval;
mod funcs;
mod interpreter;
mod parse;
mod resolve;
mod substitution;
mod util;

use crate::ast::ComposeIdentsArgs;
use crate::core::Environment;
use crate::interpreter::Interpreter;
use crate::util::unique_id::next_unique_id;
use proc_macro::TokenStream;
use std::rc::Rc;
use syn::parse_macro_input;
use util::deprecation::DeprecationService;

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// In addition to replacing identifier aliases it replaces tokens like `% alias %` in string
/// literals (including in doc-attributes).
///
/// # Example
///
/// ```rust
/// use compose_idents::compose_idents;
///
/// compose_idents!(
///     // A simple alias definition.
///     my_fn = concat(foo, _, 1, _, lower(BAR)),
///     // Many functions are overloaded support different input argument types.
///     greeting = concat(to_str(Hello), ", ", "world!"),
///     {
///         // String placeholders `% my_alias %` are expanded inside literals and doc attributes.
///         #[doc = "Greets: % greeting %"]
///         fn my_fn() -> &'static str { greeting }
///     },
/// );
///
/// assert_eq!(foo_1_bar(), "Hello, world!");
/// ```
///
/// # Reference
///
#[doc = include_str!("../snippets/reference_h2.md")]
#[proc_macro]
pub fn compose_idents(input: TokenStream) -> TokenStream {
    let deprecation_service = DeprecationService::scoped();

    let environment = Rc::new(Environment::new_initialized(next_unique_id()));
    Environment::maybe_set_global(environment.clone());

    let interpreter = Interpreter::new(environment, deprecation_service);

    let args = parse_macro_input!(input as ComposeIdentsArgs);
    match interpreter.execute(args) {
        Ok(ts) => ts.into(),
        Err(err) => {
            let syn_err: syn::Error = err.into();
            TokenStream::from(syn_err.into_compile_error())
        }
    }
}
