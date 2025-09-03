//! Tests for loop functionality.
use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    loops,

    // Simple loop.
    (
        simple_loop,
        { for suffix in [foo, bar]

          my_static = suffix,
          my_fn = concat(my, _, suffix)
        },
        {
            static my_static: u32 = 42;

            fn my_fn() -> u32 {
                42
            }
        },
        {
            static foo: u32 = 42;
            fn my_foo() -> u32 {
                42
            }

            static bar: u32 = 42;
            fn my_bar() -> u32 {
                42
            }
        },
        None,
    ),

    // Nested loops - cartesian product semantics.
    (
        nested_cartesian,
        { for a in [x, y]
          for b in [1, 2]

          fn_name = concat(a, _, b)
        },
        {
            fn fn_name() -> u32 { 0 }
        },
        {
            fn x_1() -> u32 { 0 }
            fn x_2() -> u32 { 0 }
            fn y_1() -> u32 { 0 }
            fn y_2() -> u32 { 0 }
        },
        None,
    ),

    // Triple-nested loops (2 x 2 x 2).
    (
        triple_nested_cartesian,
        { for a in [x, y]
          for b in [1, 2]
          for c in [A, B]
          fn_name = concat(a, _, b, _, c)
        },
        {
            fn fn_name() -> u32 { 0 }
        },
        {
            fn x_1_A() -> u32 { 0 }
            fn x_1_B() -> u32 { 0 }
            fn x_2_A() -> u32 { 0 }
            fn x_2_B() -> u32 { 0 }
            fn y_1_A() -> u32 { 0 }
            fn y_1_B() -> u32 { 0 }
            fn y_2_A() -> u32 { 0 }
            fn y_2_B() -> u32 { 0 }
        },
        None,
    ),

    // Tuple destructuring across iterations.
    (
        tuple_destructuring,
        { for (name, type_, value) in [(foo, &'static str, "foo"), (bar, Option<u32>, None)]
          fn_name = concat(make_, name)
        },
        {
            fn fn_name() -> type_ { value }
        },
        {
            fn make_foo() -> &'static str { "foo" }
            fn make_bar() -> Option<u32> { None }
        },
        None,
    ),

    // Nested tuple destructuring.
    (
        nested_tuple_destructuring,
        { for (name, (return_type, param_type)) in [
              (fn1, (Result<&'static str, String>, Vec::<i32>)),
              (fn2, (Option<(i32, i32)>, std::collections::HashMap<u8, u16>))
          ]
        },
        {
            fn name(arg: param_type) -> return_type { panic!() }
        },
        {
            fn fn1(arg: Vec::<i32>) -> Result<&'static str, String> { panic!() }
            fn fn2(arg: std::collections::HashMap<u8, u16>) -> Option<(i32, i32)> { panic!() }
        },
        None,
    ),

    // Alias reuse in user spec (expected to be re-evaluated per-iteration).
    (
        alias_reuse_in_user_spec,
        {
          for suffix in [FOO, BAR]

          lower_suffix = lower(suffix),
          fn_name = concat(make_, lower_suffix)
        },
        {
            fn fn_name() -> u32 { 0 }
        },
        {
            fn make_foo() -> u32 { 0 }
            fn make_bar() -> u32 { 0 }
        },
        None,
    ),

    // Empty values list.
    (
        empty_source_list,
        {
          for suffix in []

          fn_name = concat(my_, suffix)
        },
        {
            fn fn_name() -> u32 { 0 }
        },
        { },
        None,
    ),

    // String formatting.
    (
        string_formatting,
        { for name in [foo, bar]
          fn_name = concat(test_, name)
        },
        {
            #[doc = "Docstring for % fn_name %"]
            fn fn_name() -> u32 { 0 }
        },
        {
            #[doc = "Docstring for test_foo"]
            fn test_foo() -> u32 { 0 }

            #[doc = "Docstring for test_bar"]
            fn test_bar() -> u32 { 0 }
        },
        None,
    ),

    // Only loops with no user alias spec – block uses loop alias directly
    (
        only_loops_no_user_spec,
        { for name in [alpha, beta] },
        {
            fn name() -> u32 { 0 }
        },
        {
            fn alpha() -> u32 { 0 }
            fn beta() -> u32 { 0 }
        },
        None,
    ),

);
