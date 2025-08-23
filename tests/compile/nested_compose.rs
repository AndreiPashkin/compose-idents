//! compose_idents! should work when invoked inside a macro_rules! macro.
use compose_idents::compose_idents;

macro_rules! outer_macro {
    ($name:tt) => {
        compose_idents!(my_fn = concat(foo, _, $name), {
            fn my_fn() -> u32 {
                42
            }
        });
    };
}

outer_macro!(baz);
outer_macro!(bar);

fn main() {
    assert_eq!(foo_baz(), 42);
    assert_eq!(foo_bar(), 42);
}
