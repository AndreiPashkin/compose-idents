use compose_idents::compose_idents;

compose_idents!(alias = lower(42), {
    fn alias() -> u32 {
        42
    }
});

fn main() {
    assert_eq!(qux_foo_baz(), 42);
}
