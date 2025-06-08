//! Tests that invalid aliases produce correct compile-errors.
use compose_idents::compose_idents;

compose_idents!(
    for suffix in [foo, 1]

    my_fn = concat(my, _, suffix),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(my_foo(), 42);
    assert_eq!(my_bar(), 42);
}
