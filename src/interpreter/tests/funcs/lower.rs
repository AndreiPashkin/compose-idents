//! Tests for lower() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    lower,
    (
        ident,
        { alias = lower(FooBar) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn foobar() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = lower("FoO") },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "foo" }
        },
        None,
    ),
    (
        path_failure,
        { alias = lower(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        expr_failure,
        { alias = lower(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        int_failure,
        { alias = lower(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        tokens_failure,
        { alias = lower(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
);
