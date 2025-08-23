//! Tests for to_int() function.

use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_int,
    (
        int,
        { alias = to_int(42) },
        {
            fn f() { let _ = alias; }
        },
        {
            fn f() { let _ = 42; }
        },
        None,
    ),
    (
        ident_failure,
        { alias = to_int(fooBar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        path_failure,
        { alias = to_int(foo::bar) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        expr_failure,
        { alias = to_int(|| 42) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        str_failure,
        { alias = to_int("42") },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
    (
        tokens_failure,
        { alias = to_int(let x = 1;) },
        {
            fn my_fn() -> &str { alias }
        },
        {},
        Some(ErrorType::TypeError),
    ),
);
