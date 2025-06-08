//! Tests simple loop usage.
use compose_idents::compose_idents;

compose_idents!(
    for suffix in [foo, bar]

    my_static = suffix,
    my_fn = concat(my, _, suffix),
    {
        static my_static: u32 = 42;

        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(foo, 42);
    assert_eq!(my_foo(), 42);
    assert_eq!(bar, 42);
    assert_eq!(my_bar(), 42);
}
