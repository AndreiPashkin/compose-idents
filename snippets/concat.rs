use compose_idents::compose_idents;

compose_idents!(
    // Basic example
    basic_fn = [concat(foo, _, bar, _, baz)],
    // Mixed with other functions
    upper_fn = [upper(concat(hello, _, world))],
    // Complex example
    complex_fn = [concat("prefix_", normalize(&'static str), "_", snake_case(CamelCase))],
    {
        fn basic_fn() -> u32 { 1 }
        fn upper_fn() -> u32 { 2 }
        fn complex_fn() -> u32 { 3 }
    }
);

assert_eq!(foo_bar_baz(), 1);
assert_eq!(HELLO_WORLD(), 2);
assert_eq!(prefix_static_str_camel_case(), 3);
