//! Tests for to_str() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_str,
    (
        ident,
        { alias = to_str(fooBar) },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "fooBar" }
        },
        None,
    ),
    (
        str,
        { alias = to_str("foo") },
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
        { alias = to_str(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        expr_failure,
        { alias = to_str(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        int_failure,
        { alias = to_str(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        tokens_failure,
        { alias = to_str(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
);
