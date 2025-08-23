//! Tests for upper() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    upper,
    (
        ident,
        { alias = upper(fooBar) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn FOOBAR() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = upper("foo") },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "FOO" }
        },
        None,
    ),
    (
        path_failure,
        { alias = upper(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        expr_failure,
        { alias = upper(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        int_failure,
        { alias = upper(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    (
        tokens_failure,
        { alias = upper(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
);
