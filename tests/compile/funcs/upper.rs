use compose_idents::compose_idents;

compose_idents!(my_var = concat(upper(foo), _, upper(bar)), {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(FOO_BAR, 42);
}
