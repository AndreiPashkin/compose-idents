use compose_idents::compose_idents;

compose_idents!(my_var = camel_case(foo_bar), {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(fooBar, 42);
}
