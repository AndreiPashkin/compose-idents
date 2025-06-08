use compose_idents::compose_idents;

compose_idents!(
    for (prefix, (suffix, ty)) in [(foo, (baz, u32)), (bar, (qux, u64))]
    my_fn = concat(prefix, _, suffix, _, ty),
    {
        fn my_fn() -> u32 { 42 }
    }
);

fn main() {
    assert_eq!(foo_baz_u32(), 42);
    assert_eq!(bar_qux_u64(), 42);
}
