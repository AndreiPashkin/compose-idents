//! Tests for to_expr() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_expr,
    (
        ident,
        { alias = to_expr(foo) },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = foo; }
        },
        None,
    ),
    (
        path,
        { alias = to_expr(foo::bar) },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = foo::bar; }
        },
        None,
    ),
    (
        expr,
        { alias = to_expr(1 + 2) },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = 1 + 2; }
        },
        None,
    ),
    (
        int,
        { alias = to_expr(42) },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = 42; }
        },
        None,
    ),
    (
        str,
        { alias = to_expr("foo") },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = "foo"; }
        },
        None,
    ),
    (
        tokens_failure,
        { alias = to_expr(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        valid_function_call,
        { alias = to_expr(lower(FOO)) },
        {
            fn my_fn() { alias }
        },
        {
            fn my_fn() { foo }
        },
        None,
    ),
);
