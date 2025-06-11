//! Tests assignment of a bare argument to an alias.
use compose_idents::compose_idents;

compose_idents!(my_static = qux, {
    static my_static: u32 = 42;
},);

fn main() {
    assert_eq!(qux, 42);
}
