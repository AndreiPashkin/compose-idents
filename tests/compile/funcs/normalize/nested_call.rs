//! normalize() should treat nested function calls as arbitrary token-sequences.
use compose_idents::compose_idents;

compose_idents!(my_var = concat(my_, normalize(foo())), {
    static my_var: u32 = 42;
});

fn main() {
    assert_eq!(my_foo, 42);
}
