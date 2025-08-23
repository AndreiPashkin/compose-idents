//! Tests for hash() function.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    hash,
    (
        ident,
        { alias = hash(FooBar) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            // We prefix the hash result for an ident-arg prefixed with "__".
            fn __1864179433826574950() -> u32 { 1 }
        },
        None,
    ),
    (
        str,
        { alias = hash("foo") },
        {
            fn my_fn() -> &str { alias }
        },
        {
            fn my_fn() -> &str { "10172337927241793445" }
        },
        None,
    ),
    (
        tokens,
        { alias = hash(let x = 1;) },
        {
            fn alias() -> u32 { 1 }
        },
        {
            fn __18050448427594546802() -> u32 { 1 }
        },
        None,
    ),
);
