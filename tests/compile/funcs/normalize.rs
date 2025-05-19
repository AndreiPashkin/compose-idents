use compose_idents::compose_idents;

compose_idents!(my_var = [my_, normalize(&'static str)], {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(my_static_str, 42);
}
