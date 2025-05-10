use compose_idents::compose_idents;

compose_idents!(alias_1 = [foo, _, "baz"], alias_2 = [qux, _, alias_1], {
    fn alias_2() -> u32 {
        42
    }
});

fn main() {
    assert_eq!(qux_foo_baz(), 42);
}
