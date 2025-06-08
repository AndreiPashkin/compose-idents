//! Tests usage of tuples with different types of values.
use compose_idents::compose_idents;

compose_idents!(
    for (name, suffix) in [
        (foo, "bar"),
        (lower(GORK), concat(bork, _, lower(SPAM))),
        (normalize(Foo::Bar), normalize(&'static str)),
    ]

    my_static = concat(name, _, suffix),
    {
        const my_static: u32 = 42;
    }
);

fn main() {
    assert_eq!(foo_bar, 42);
    assert_eq!(gork_bork_spam, 42);
    assert_eq!(Foo_Bar_static_str, 42);
}
