use compose_idents::compose_idents;

compose_idents!(
    my_fn_1 = [foo, _, "baz"];
    my_fn_2 = [spam, _, 1, _, eggs];
    my_const = [upper(foo), _, lower(BAR)];
    my_static = [upper(lower(BAR))];
    MY_SNAKE_CASE_STATIC = [snake_case(snakeCase)];
    MY_CAMEL_CASE_STATIC = [camel_case(camel_case)]; {
    fn my_fn_1() -> u32 {
        123
    }

    fn my_fn_2() -> u32 {
        321
    }

    const my_const: u32 = 42;
    static my_static: u32 = 42;
    static MY_SNAKE_CASE_STATIC: u32 = 42;
    static MY_CAMEL_CASE_STATIC: u32 = 42;
});

macro_rules! outer_macro {
    ($name:tt) => {
        compose_idents!(my_nested_fn = [nested, _, $name]; {
            fn my_nested_fn() -> u32{
                42
            }
        });
    };
}

outer_macro!(foo);

assert_eq!(foo_baz(), 123);
assert_eq!(spam_1_eggs(), 321);
assert_eq!(nested_foo(), 42);
assert_eq!(FOO_bar, 42);
assert_eq!(BAR, 42);
assert_eq!(snake_case, 42);
assert_eq!(camelCase, 42);
