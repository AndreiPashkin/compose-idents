//! Tests nested loops.
use compose_idents::compose_idents;

compose_idents!(
    for prefix in [foo, bar]
    for suffix in [baz, qux]

    my_fn = concat(prefix, _, suffix),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(foo_baz(), 42);
    assert_eq!(foo_qux(), 42);
    assert_eq!(bar_baz(), 42);
    assert_eq!(bar_qux(), 42);
}
