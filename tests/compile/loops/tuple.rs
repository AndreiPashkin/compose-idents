//! Tests loops that use tuples.
use compose_idents::compose_idents;

compose_idents!(
    for (prefix, ty) in [(foo, u32), (bar, u64)]

    my_fn = concat(prefix, _, ty, _gork),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(foo_u32_gork(), 42);
    assert_eq!(bar_u64_gork(), 42);
}
