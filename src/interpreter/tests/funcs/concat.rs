//! Tests for concat() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    concat,
    (
        idents,
        { alias = concat(foo, Bar, Baz) },
        {
            fn alias() -> u32 {
                42
            }
        },
        {
            fn fooBarBaz() -> u32 {
                42
            }
        },
        None,
    ),
    (
        idents_and_tokens,
        { alias = concat(foo, _, 1, _, bar) },
        {
            fn alias() -> u32 {
                42
            }
        },
        {
            fn foo_1_bar() -> u32 {
                42
            }
        },
        None,
    ),
    (
        idents_and_tokens_failure,
        { alias = concat(foo, _, 1, _, &'static str) },
        {
            fn alias() -> u32 {
                42
            }
        },
        { },
        Some(ErrorType::EvalError),
    ),
    (
        strs,
        { alias = concat("foo", "bar", "baz") },
        {
            fn my_fn() -> &str {
                alias
            }
        },
        {
            fn my_fn() -> &str {
                "foobarbaz"
            }
        },
        None,
    ),
    (
        ints,
        { alias = concat(1, 2, 3) },
        {
            fn my_fn() -> u32 {
                alias
            }
        },
        {
            fn my_fn() -> u32 {
                123
            }
        },
        None,
    ),
    (
        tokens,
        // Notice - raw() is used to fence comma-containing argument.
        { alias = concat(Result<, raw(u32,), String, >) },
        {
            fn my_fn() -> alias {
                Ok(42)
            }
        },
        {
            fn my_fn() -> Result<u32, String> {
                Ok(42)
            }
        },
        None,
    ),
);
