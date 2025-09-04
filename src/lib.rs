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

use crate::ast::{ComposeItemSpec, RawAST};
use crate::core::Environment;
use crate::interpreter::Interpreter;
use crate::util::unique_id::next_unique_id;
use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::parse_macro_input;
use util::deprecation::DeprecationService;

enum InvocationType {
    Func(TokenStream),
    Attr(TokenStream, TokenStream),
}

fn compose_core(prefix: &'static str, invocation: InvocationType) -> TokenStream {
    let deprecation_service = DeprecationService::new_rc(prefix);
    DeprecationService::maybe_set_global(deprecation_service);
    let deprecation_service_scope = DeprecationService::scoped();

    let environment = Rc::new(Environment::new_initialized(next_unique_id()));
    Environment::maybe_set_global(environment.clone());

    let interpreter = Interpreter::new(environment, deprecation_service_scope);

    let args = match invocation {
        InvocationType::Func(input) => parse_macro_input!(input as RawAST),
        InvocationType::Attr(attr, item) => {
            // Parse attribute prefix tailored for the attribute macro form
            let spec: ComposeItemSpec = match syn::parse(attr) {
                Ok(v) => v,
                Err(err) => return TokenStream::from(err.into_compile_error()),
            };

            // Treat the decorated item as the block
            let item: proc_macro2::TokenStream = item.into();
            let block_tokens: proc_macro2::TokenStream = quote!({ #item });
            let block: syn::Block = match syn::parse2(block_tokens) {
                Ok(v) => v,
                Err(err) => return TokenStream::from(err.into_compile_error()),
            };

            RawAST::from_compose_item_spec(&spec, block)
        }
    };
    match interpreter.execute(args) {
        Ok(ts) => ts.into(),
        Err(err) => {
            let syn_err: syn::Error = err.into();
            TokenStream::from(syn_err.into_compile_error())
        }
    }
}

/// Compose identifiers from the provided parts and replace their aliases in the decorated item.
///
/// This attribute macro is equivalent to [`compose!`], but treats the annotated item as the
/// code block.
///
/// # Example
///
/// ```rust
/// use compose_idents::compose_item;
///
/// #[compose_item(
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
/// )]
/// // String placeholders `% my_alias %` are expanded inside literals and doc attributes.
/// #[doc = "Greets: % greeting %"]
/// fn my_fn() -> &'static str { greeting }
///
/// assert_eq!(foo_1_bar(), "Hello, world!");
/// assert_eq!(foo_1_baz(), "Hallo, welt!");
/// ```
///
/// # Reference
///
#[doc = include_str!("../snippets/reference_h2.md")]
#[proc_macro_attribute]
pub fn compose_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    compose_core("compose_item!: ", InvocationType::Attr(attr, item))
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
    compose_core("compose!: ", InvocationType::Func(input))
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
    compose_core("compose_idents!: ", InvocationType::Func(input))
}
