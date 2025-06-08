use compose_idents::compose_idents;

compose_idents!(
    for (prefix1, ty1) in [(foo, u32), (bar, u64)]
    for (suffix2, ty2) in [(baz, i32), (qux, i64)]

    my_fn = concat(prefix1, _, suffix2, _, ty1, _, ty2),
    {
        fn my_fn() -> u32 { 42 }
    }
);

fn main() {
    assert_eq!(foo_baz_u32_i32(), 42);
    assert_eq!(foo_qux_u32_i64(), 42);
    assert_eq!(bar_baz_u64_i32(), 42);
    assert_eq!(bar_qux_u64_i64(), 42);
}
