//! Tests for normalize2() function.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    normalize2,
    // Ident input.
    (
        ident,
        { alias = normalize2(foo) },
        { fn alias() -> u32 { 6 } },
        { fn foo() -> u32 { 6 } },
        None,
    ),
    // String input.
    (
        string,
        { alias = normalize2("foo bar") },
        { fn alias() -> u32 { 6 } },
        { fn foo_bar() -> u32 { 6 } },
        None,
    ),
    // Integer input.
    (
        int,
        { alias = normalize2(123) },
        { fn alias() -> u32 { 6 } },
        { fn _123() -> u32 { 6 } },
        None,
    ),
    // Path input.
    (
        path,
        { alias = normalize2(Foo::Bar) },
        { fn alias() -> u32 { 6 } },
        { fn Foo_Bar() -> u32 { 6 } },
        None,
    ),
    // Type input.
    (
        type_,
        { alias = normalize2(&'static str) },
        { fn alias() -> u32 { 6 } },
        { fn static_str() -> u32 { 6 } },
        None,
    ),
    // Expr input.
    (
        expr,
        { alias = normalize2(1 + 2) },
        { fn alias() -> u32 { 6 } },
        { fn _1_2() -> u32 { 6 } },
        None,
    ),
    // Tokens input (via raw() fencing).
    (
        tokens_via_raw,
        { alias = normalize2(raw(Result<u32, String>)) },
        { fn alias() -> u32 { 6 } },
        { fn Result_u32_String() -> u32 { 6 } },
        None,
    ),
);
