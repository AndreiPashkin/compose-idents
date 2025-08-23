//! Tests for to_path() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_path,
    (
        ident,
        { alias = to_path(foo) },
        {
            fn f(_: alias) {}
        },
        {
            fn f(_: foo) {}
        },
        None,
    ),
    (
        path,
        { alias = to_path(foo::bar) },
        {
            fn f(_: alias) {}
        },
        {
            fn f(_: foo::bar) {}
        },
        None,
    ),
    (
        expr_failure,
        { alias = to_path(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        int_failure,
        { alias = to_path(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        str_failure,
        { alias = to_path("foo") },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        tokens_failure,
        { alias = to_path(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        valid_function_call,
        { alias = to_path(lower(FOO)) },
        {
            fn my_fn() { alias }
        },
        {
            fn my_fn() { foo }
        },
        None,
    ),
);
