//! Tests for function calls.
use crate::error::ErrorType;
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    calls,
    // Simple function call.
    (
        simple_call,
        { alias = upper(foo) },
        {
            fn alias() -> u32 {
                1
            }
        },
        {
            fn FOO() -> u32 {
                1
            }
        },
        None,
    ),
    // Nested function call.
    (
        nested_calls,
        { alias = lower(normalize(Foo::Bar)) },
        {
            fn alias() -> u32 {
                1
            }
        },
        {
            fn foo_bar() -> u32 {
                1
            }
        },
        None,
    ),
    // Function call with mixed arguments - values and other calls.
    (
        mixed_args,
        { alias = concat(foo, _, normalize(bar::baz)) },
        {
            fn alias() -> u32 {
                1
            }
        },
        {
            fn foo_bar_baz() -> u32 {
                1
            }
        },
        None,
    ),
    // Function call with wrong number of arguments.
    (
        wrong_num_args,
        { alias = lower(foo, bar) },
        {
            fn alias() -> u32 {
                1
            }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
    // Function call with missing arguments.
    (
        missing_args,
        { alias = lower() },
        {
            fn alias() -> u32 {
                1
            }
        },
        {},
        Some(ErrorType::SignatureError),
    ),
);
