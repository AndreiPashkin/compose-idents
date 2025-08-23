//! Tests for normalize() function.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    normalize,
    (
        lifetime,
        { alias = normalize(&'static str) },
        {
            fn alias() -> u32 { 6 }
        },
        {
            fn static_str() -> u32 { 6 }
        },
        None,
    ),
    (
        generic_type,
        { alias = normalize(Result<T, E>) },
        {
            fn my_fn() { let alias = 7; }
        },
        {
            fn my_fn() { let Result_T_E = 7; }
        },
        None,
    ),
    (
        path,
        { alias = normalize(Foo::Bar) },
        {
            fn my_fn() { let alias = 7; }
        },
        {
            fn my_fn() { let Foo_Bar = 7; }
        },
        None,
    ),
    (
        path_turbofish,
        { alias = normalize(HashMap::<String, i32>) },
        {
            fn my_fn() { let alias = 7; }
        },
        {
            fn my_fn() { let HashMap_String_i32 = 7; }
        },
        None,
    ),
    (
        path_tuple,
        { alias = normalize(HashMap::<(u64, u64), i32>) },
        {
            fn my_fn() { let alias = 7; }
        },
        {
            fn my_fn() { let HashMap_u64_u64_i32 = 7; }
        },
        None,
    ),
    (
        valid_function_call,
        { alias = normalize(lower(FOO)) },
        {
            fn my_fn() { let alias = 7; }
        },
        {
            fn my_fn() { let lower_FOO = 7; }
        },
        None,
    ),
);
