//! Tests for snake_case() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    snake_case,
    (
        ident,
        { alias = snake_case(FooBarBaz) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn foo_bar_baz() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = snake_case("FooBar") },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "foo_bar" }
        },
        None,
    ),
    (
        path_failure,
        { alias = snake_case(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        expr_failure,
        { alias = snake_case(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        int_failure,
        { alias = snake_case(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        tokens_failure,
        { alias = snake_case(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
);
