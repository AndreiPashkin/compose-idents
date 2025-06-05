use compose_idents::compose_idents;

compose_idents!(
    my_fn = concat(foo, _, "baz"),
    my_fn = concat(gork, _, bork),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(foo_baz(), 42);
}
