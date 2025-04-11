use compose_idents::compose_idents;

compose_idents!(my_var = [lower(FOO), _, lower(BAR)], {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(foo_bar, 42);
}
