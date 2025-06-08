use compose_idents::compose_idents;

compose_idents!(
    for prefix in [foo, bar]
    for suffix in [baz, qux]

    my_fn = concat(prefix, _, suffix),
    my_static = concat(prefix, _, suffix, _static),
    {
        const my_static: u32 = 42;
        fn my_fn() -> u32 { my_static }
    }
);

fn main() {
    assert_eq!(foo_baz_static, 42u32);
    assert_eq!(foo_baz(), 42u32);
    assert_eq!(foo_qux_static, 42u32);
    assert_eq!(foo_qux(), 42u32);
    assert_eq!(bar_baz_static, 42u32);
    assert_eq!(bar_baz(), 42u32);
    assert_eq!(bar_qux_static, 42u32);
    assert_eq!(bar_qux(), 42u32);
}
