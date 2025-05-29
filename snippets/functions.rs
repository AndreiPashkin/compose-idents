use compose_idents::compose_idents;

compose_idents!(
    my_const = [upper(foo), _, lower(BAR)],
    // Function calls can be arbitrarily nested and combined.
    my_static = [upper(lower(BAZ))],
    {
        const my_const: u8 = 1;
        static my_static: &str = "hello";
    }
);

assert_eq!(FOO_bar, 1);
assert_eq!(BAZ, "hello");
