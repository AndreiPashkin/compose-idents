use compose_idents::compose_idents;

compose_idents!(
    // Basic concatenation
    my_var_1 = [concat(foo, bar, baz)],
    // Mixed argument types
    my_var_2 = [concat("prefix", _, suffix, _, 42)],
    // Single argument
    my_var_3 = [concat(single)],
    // Nested in other function calls
    my_var_4 = [upper(concat(hello, _, world))],
    {
        const my_var_1: u32 = 1;
        const my_var_2: u32 = 2;
        const my_var_3: u32 = 3;
        const my_var_4: u32 = 4;
    }
);

fn main() {
    assert_eq!(foobarbaz, 1);
    assert_eq!(prefix_suffix_42, 2);
    assert_eq!(single, 3);
    assert_eq!(HELLO_WORLD, 4);
}
