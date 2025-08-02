//! Mixing bracket-based and expression-based syntax should be supported.
use compose_idents::compose_idents;

compose_idents!(
    my_static_1 = [foo, _, "bar"],
    my_static_2 = concat(gork, _, bork),
    {
        static my_static_1: u32 = 42;
        static my_static_2: u32 = 42;
    },
);

fn main() {
    assert_eq!(foo_bar, 42);
    assert_eq!(gork_bork, 42);
}
