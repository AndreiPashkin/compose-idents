//! Tests for pascal_case() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    pascal_case,
    (
        ident,
        { alias = pascal_case(foo_bar_baz) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn FooBarBaz() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = pascal_case("foo_bar") },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "FooBar" }
        },
        None,
    ),
    (
        path_failure,
        { alias = pascal_case(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        expr_failure,
        { alias = pascal_case(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        int_failure,
        { alias = pascal_case(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        tokens_failure,
        { alias = pascal_case(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
);
