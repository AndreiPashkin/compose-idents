//! Tests for alias definition functionality.
use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    aliases,

    // Alias definition with a value expression.
    (
        value_expr,
        { alias = foo },
        { fn alias() -> u32 { 1 } },
        { fn foo() -> u32 { 1 } },
        None,
    ),
    // Alias definition with a func-call expression.
    (
        call_expr,
        { alias = concat(foo, _, bar) },
        { fn alias() -> u32 { 1 } },
        { fn foo_bar() -> u32 { 1 } },
        None,
    ),
    // Multiple aliases.
    (
        multiple_aliases,
        { alias1 = foo, alias2 = concat(foo, _, bar) },
        { fn alias1() -> u32 { alias2 } },
        { fn foo() -> u32 { foo_bar } },
        None,
    ),
    // Alias reuse.
    (
        alias_reuse,
        { alias1 = foo, alias2 = concat(alias1, _, bar) },
        { fn alias1() -> u32 { alias2 } },
        { fn foo() -> u32 { foo_bar } },
        None,
    ),
    // Duplicate aliases.
    (
        duplicate_aliases,
        { alias = foo, alias = bar },
        { fn alias() -> u32 { alias2 } },
        { },
        Some(ErrorType::RedefinedNameError),
    ),
    // Alias re-use coercion.
    //
    // This is a very subtle case:
    //  - `s` in isolation has type `ident`, while `"foo bar"` has type `str`.
    //  - When `s` is re-used, but its target type is still set to `ident` (which would be a bug)
    //    then the coercion during the eval-phase would fail, because coercion from `str` to `ident`
    //    is not possible.
    //  - But if the resolve-phase correctly accounts for alias re-use - it should set the target
    //    type of `s` to `str`, and then the coercion during the eval-phase should succeed.
    (
        reuse_coercion,
        { s = "foo bar", alias = s },
        { fn my_fn() -> &str { alias } },
        { fn my_fn() -> &str { "foo bar" } },
        None,
    ),

);
