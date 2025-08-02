//! Numeric literals should be usable as compose_idents! arguments.
use compose_idents::compose_idents;

compose_idents!(my_fn = concat(foo, _, 1, _, bar), {
    fn my_fn() -> u32 {
        42
    }
});

fn main() {
    assert_eq!(foo_1_bar(), 42);
}
