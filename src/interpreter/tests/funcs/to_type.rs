//! Tests for to_type() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_type,
    (
        ident,
        { alias = to_type(Foo) },
        {
            fn f(_: alias) {}
        },
        {
            fn f(_: Foo) {}
        },
        None,
    ),
    (
        path,
        { alias = to_type(foo::bar) },
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
        { alias = to_type(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        int_failure,
        { alias = to_type(42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        str_failure,
        { alias = to_type("foo") },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        tokens_failure,
        { alias = to_type(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        valid_function_call,
        { alias = to_type(lower(FOO)) },
        {
            fn my_fn() { alias }
        },
        {
            fn my_fn() { foo }
        },
        None,
    ),
);
