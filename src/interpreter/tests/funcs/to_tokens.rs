//! Tests for to_tokens() function.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    to_tokens,
    // An argument with a trailing comma.
    (
        trailing_comma,
        { alias = to_tokens(u64,) },
        {
            fn my_fn() -> alias { 1 }
        },
        {
            fn my_fn() -> u64 { 1 }
        },
        None,
    ),
    // Statement.
    (
        let_stmt,
        { alias = to_tokens(let x = 1;) },
        {
            fn f() { alias }
        },
        {
            fn f() { let x = 1; }
        },
        None,
    ),
    // Item.
    (
        item,
        { alias = to_tokens(#[derive(Default)] struct X;) },
        {
            alias
        },
        {
            #[derive(Default)] struct X;
        },
        None,
    ),
    // A valid function call evaluated.
    (
        valid_function_call,
        { alias = to_tokens(lower(FOO)) },
        {
            alias
        },
        {
            foo
        },
        None,
    ),
);
