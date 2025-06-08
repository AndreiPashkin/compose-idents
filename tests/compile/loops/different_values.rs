use compose_idents::compose_idents;

compose_idents!(
    for part in [foo, "bar", 42, _, concat(baz, qux)]

    my_fn = concat(foo, _, part),
    {
        fn my_fn() -> u32 { 42 }
    }
);

fn main() {
    assert_eq!(foo_foo(), 42);
    assert_eq!(foo_bar(), 42);
    assert_eq!(foo_42(), 42);
    assert_eq!(foo__(), 42);
    assert_eq!(foo_bazqux(), 42);
}
