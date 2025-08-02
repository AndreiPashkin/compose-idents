//! Nested function calls should be supported.
use compose_idents::compose_idents;

compose_idents!(my_var = lower(upper(FOO)), {
    const my_var: u32 = 42;
});

fn main() {
    assert_eq!(foo, 42);
}
