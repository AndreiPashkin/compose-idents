//! Tests various types of values in loops value-lists.
use compose_idents::compose_idents;

compose_idents!(
    for part in [foo, "bar", concat(baz, _, lower(QUX)), normalize(Foo::Bar), normalize(&'static str)]

    my_fn = concat(foo, _, part),
    {
        fn my_fn() -> u32 { 42 }
    }
);

fn main() {
    assert_eq!(foo_foo(), 42);
    assert_eq!(foo_bar(), 42);
    assert_eq!(foo_baz_qux(), 42);
    assert_eq!(foo_Foo_Bar(), 42);
    assert_eq!(foo_static_str(), 42);
}
