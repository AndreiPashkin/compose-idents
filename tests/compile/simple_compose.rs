use compose_idents::compose_idents;

compose_idents!(my_fn = [foo, _, "baz"]; {
    fn my_fn() -> u32 {
        42
    }
});

fn main() {
    assert_eq!(foo_baz(), 42);
}
