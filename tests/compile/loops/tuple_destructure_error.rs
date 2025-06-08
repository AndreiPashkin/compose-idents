//! Tests incorrect tuple destructuring producing correctly formatted compile-time errors.
use compose_idents::compose_idents;

compose_idents!(
    for (a, (b, c)) in [(foo, (bar, baz)), (gork, bork)]

    my_fn = concat(a, b, c),
    {
        fn my_fn() -> u32 { 42 }
    }
);

fn main() {}
