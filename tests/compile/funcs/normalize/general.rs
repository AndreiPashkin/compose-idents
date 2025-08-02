//! normalize() should be able to parse and sanitize complex syntactic constructs and turn them to
//! valid idents.
use compose_idents::compose_idents;

compose_idents!(
    my_var_1 = concat(my_, normalize(&'static str)),
    my_var_2 = concat(my_, normalize(Result<T, E>)),
    {
        const my_var_1: u32 = 42;
        const my_var_2: u32 = 42;
    }
);

fn main() {
    assert_eq!(my_static_str, 42);
    assert_eq!(my_Result_T_E, 42);
}
