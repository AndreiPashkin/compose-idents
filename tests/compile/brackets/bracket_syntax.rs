//! Bracket-based alias definition syntax should be supported.
use compose_idents::compose_idents;

compose_idents!(
    my_fn = [foo, _, "baz"],
    my_static = [concat(gork, _, bork)],
    {
        static my_static: u32 = 42;

        fn my_fn() -> u32 {
            42
        }
    },
);

fn main() {
    assert_eq!(gork_bork, 42);
    assert_eq!(foo_baz(), 42);
}
