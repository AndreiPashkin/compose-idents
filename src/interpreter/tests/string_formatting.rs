//! Tests for formatting of string literals.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    string_formatting,
    // Basic string-formatting.
    (
        basic,
        { my_alias = concat(foo, _, bar) },
        {
            static MY_STRING: &str = "Hello, % my_alias %!";
            static MY_STRING_NO_WHITESPACE: &str = "Hello, %my_alias%!";
            static MY_STRING_IRREGULAR_WHITESPACE: &str = "Hello, % my_alias  %!";
        },
        {
            static MY_STRING: &str = "Hello, foo_bar!";
            static MY_STRING_NO_WHITESPACE: &str = "Hello, foo_bar!";
            static MY_STRING_IRREGULAR_WHITESPACE: &str = "Hello, foo_bar!";
        },
        None,
    ),
    // Formating in doc-attributes.
    (
        doc_attr,
        { my_fn = concat(foo, _, bar) },
        {
            #[doc = "My doc comment for % my_fn %"]
            fn my_fn() -> u32 {
                42
            }
        },
        {
            #[doc = "My doc comment for foo_bar"]
            fn foo_bar() -> u32 {
                42
            }
        },
        None,
    ),
    // Double percent '%%' escapes to a single percent '%'.
    (
        escape_percent,
        { my_alias = concat(foo, _, bar) },
        {
            static MY_STRING: &str = "Hello, % my_alias % %%";
        },
        {
            static MY_STRING: &str = "Hello, foo_bar %";
        },
        None,
    ),
    // Placeholders for undefined aliases should be kept as is.
    (
        undefined_alias,
        { my_other_alias = foo },
        {
            static MY_STRING: &str = "Hello, % my_alias %!";
        },
        {
            static MY_STRING: &str = "Hello, % my_alias %!";
        },
        None,
    ),
    // Unterminated placeholder should remain unchanged.
    (
        unterminated_placeholder,
        { my_alias = concat(foo, _, bar) },
        {
            static MY_STRING: &str = "Hello, % my_alias";
        },
        {
            static MY_STRING: &str = "Hello, % my_alias";
        },
        None,
    ),
    // Percent immediately followed by a non-placeholder should stay intact.
    (
        percent_non_placeholder,
        { my_alias = concat(foo, _, bar) },
        {
            static MY_STRING: &str = "Value: %x";
        },
        {
            static MY_STRING: &str = "Value: %x";
        },
        None,
    ),
);
