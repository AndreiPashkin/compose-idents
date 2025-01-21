use compose_idents::compose_idents;

compose_idents!(my_fn_1 = [foo, _, "baz"]; my_fn_2 = [spam, _, eggs]; {
    fn my_fn_1() -> u32 {
        123
    }

    fn my_fn_2() -> u32 {
        321
    }
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

fn main() {
    assert_eq!(foo_baz(), 123);
    assert_eq!(spam_eggs(), 321);
    assert_eq!(nested_foo(), 42);
}
