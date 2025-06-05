use compose_idents::compose_idents;

compose_idents!(
    alias_1 = foo,
    alias_2 = concat("BAZ"),
    alias_3 = concat(qux, _, alias_1, _, lower(alias_2)),
    {
        fn alias_3() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(qux_foo_baz(), 42);
}
