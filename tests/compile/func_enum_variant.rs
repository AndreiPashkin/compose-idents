use compose_idents::compose_idents;

enum Foo {
    Bar,
    Baz,
}

compose_idents!(my_var = lower(normalize(Foo::Bar)), {
    static my_var: u32 = 42;
});

fn main() {
    assert_eq!(foo_bar, 42);
}
