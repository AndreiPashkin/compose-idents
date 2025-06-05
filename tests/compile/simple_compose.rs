use compose_idents::compose_idents;

compose_idents!(
    my_fn = concat(foo, _, "bar"),
    my_static_1 = upper(baz),
    my_static_2 = qux,
    {
        static my_static_1: u32 = 42;
        static my_static_2: u32 = 42;

        fn my_fn() -> u32 {
            42
        }
    },
);

fn main() {
    assert_eq!(foo_bar(), 42);
    assert_eq!(BAZ, 42);
    assert_eq!(qux, 42);
}
