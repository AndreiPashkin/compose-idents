use compose_idents::compose_idents;

compose_idents!(my_var = concat(foo, _, var), {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(foo_var, 42);
}
