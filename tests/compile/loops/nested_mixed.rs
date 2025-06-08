//! Tests nested that use both simple values and tuples.
use compose_idents::compose_idents;

compose_idents!(
    for prefix in [foo, bar]
    for (suffix, ty) in [(baz, u32), (qux, u64)]

    my_fn = concat(prefix, _, suffix, _, ty),
    {
        fn my_fn() -> ty {
            42
        }
    }
);

fn main() {
    assert_eq!(foo_baz_u32(), 42);
    assert_eq!(foo_qux_u64(), 42);
    assert_eq!(bar_baz_u32(), 42);
    assert_eq!(bar_qux_u64(), 42);
}
