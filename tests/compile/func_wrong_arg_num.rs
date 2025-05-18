use compose_idents::compose_idents;

compose_idents!(my_fn = [snake_case(fooBar, barBaz)], {
    fn my_fn() -> u32 {
        42
    }
});

fn main() {
    assert_eq!(foo_baz(), 42);
}
