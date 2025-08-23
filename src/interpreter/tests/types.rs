//! Tests for substitution with values of different types.
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    types,

    // Ident cases.
    (
        ident,
        { alias = foo },
        { fn alias() -> u32 { 1 } },
        { fn foo() -> u32 { 1 } },
        None,
    ),
    (
        raw_keyword,
        { alias = r#type },
        { fn alias() -> u32 { 1 } },
        { fn r#type() -> u32 { 1 } },
        None,
    ),
    (
        ident_with_digits,
        { alias = foo123 },
        { fn alias() {} },
        { fn foo123() {} },
        None,
    ),
    (
        underscore,
        { alias = _ },
        { fn f() { let alias = 1; } },
        { fn f() { let _ = 1; } },
        None,
    ),

    // Path cases.
    (
        path_simple,
        { alias = foo::bar },
        { fn f(_: alias) {} },
        { fn f(_: foo::bar) {} },
        None,
    ),
    (
        path_with_generic_args,
        { alias = Result<i32, String> },
        { fn f(_: alias) {} },
        { fn f(_: Result<i32, String>) {} },
        None,
    ),
    (
        path_with_lifetime_generic_param,
        { alias = std::slice::Iter<'a, T> },
        { fn f<'a, T>(_: alias) {} },
        { fn f<'a, T>(_: std::slice::Iter<'a, T>) {} },
        None,
    ),

    // Type.
    (
        type_array,
        { alias = [u8; 32] },
        { fn f(_: alias) {} },
        { fn f(_: [u8; 32]) {} },
        None,
    ),
    (
        type_fn,
        { alias = fn(i32) -> i32 },
        { fn f(_: alias) {} },
        { fn f(_: fn(i32) -> i32) {} },
        None,
    ),
    (
        type_dyn_bounds,
        { alias = dyn Iterator<Item = u8> + Send + 'static },
        { fn f(_: alias) {} },
        { fn f(_: dyn Iterator<Item = u8> + Send + 'static) {} },
        None,
    ),

    // Expr.
    (
        expr_binary,
        { alias = 2 + 2 },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = 2 + 2; } },
        None,
    ),
    (
        expr_block,
        { alias = { let x = 1; x + 1 } },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = { let x = 1; x + 1 }; } },
        None,
    ),
    (
        expr_call,
        { alias = foo(1, 2) },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = foo(1, 2); } },
        None,
    ),

    // LitInt.
    (
        litint_dec,
        { alias = 123 },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = 123; } },
        None,
    ),
    (
        litint_underscore,
        { alias = 1_000_000 },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = 1_000_000; } },
        None,
    ),

    // LitStr.
    (
        litstr_simple,
        { alias = "hello" },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = "hello"; } },
        None,
    ),
    (
        litstr_raw_hashes,
        { alias = r#"he"llo"# },
        { fn f() { let _ = alias; } },
        { fn f() { let _ = r#"he"llo"#; } },
        None,
    ),

    // Tokens.
    (
        tokens_item_injection,
        { alias = #[derive(Default)] struct X; },
        { alias },
        { #[derive(Default)] struct X; },
        None,
    ),
    (
        tokens_let_stmt,
        { alias = let x = 1; },
        { fn f() { alias } },
        { fn f() { let x = 1; } },
        None,
    ),
);
