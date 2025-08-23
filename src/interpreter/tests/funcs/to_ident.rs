//! Tests for to_ident() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_ident,
    (
        ident,
        { alias = to_ident(fooBar) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn fooBar() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = to_ident("fooBar") },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn fooBar() -> u32 { 1 }
        },
        None,
    ),
    (
        path_failure,
        { alias = to_ident(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        expr_failure,
        { alias = to_ident(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        int_failure,
        { alias = to_ident(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        str_failure,
        { alias = to_ident("&foo") },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        tokens_failure,
        { alias = to_ident(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        valid_function_call_failure,
        { alias = to_ident(lower(FOO)) },
        {
            fn my_fn() { alias }
        },
        {
            fn my_fn() { foo }
        },
        None,
    ),
);
