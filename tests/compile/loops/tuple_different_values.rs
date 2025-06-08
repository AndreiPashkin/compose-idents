use compose_idents::compose_idents;

compose_idents!(
    for (suffix, val) in [
        (foo, 1u32),
        ("bar", 2u32),
        (42, 3u32),
        (_, 4u32),
        (concat(baz, qux), 5u32)
    ]

    my_static = concat(my_, normalize(suffix)),
    {
        const my_static: u32 = 42;
    }
);

fn main() {
    assert_eq!(my_foo, 42u32);
    assert_eq!(my_bar, 42u32);
    assert_eq!(my__42, 42u32);
    assert_eq!(my__, 42u32);
    assert_eq!(my_bazqux, 42u32);
}
