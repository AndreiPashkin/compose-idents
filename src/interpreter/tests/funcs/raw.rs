//! Tests for raw() function.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    raw,
    // An argument with a trailing comma.
    (
        trailing_comma,
        { alias = raw(u64,) },
        {
            fn my_fn() -> (alias) { (1,) }
        },
        {
            fn my_fn() -> (u64,) { (1,) }
        },
        None,
    ),
    // Module.
    (
        module,
        { alias = raw(mod foo { fn bar() -> u32 { 0 } }) },
        {
            alias
        },
        {
            mod foo { fn bar() -> u32 { 0 } }
        },
        None,
    ),
);
