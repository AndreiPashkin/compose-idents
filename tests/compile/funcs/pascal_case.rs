//! pascal_case() should convert input values to PascalCase.
use compose_idents::compose_idents;

fn main() {
    compose_idents!(my_fn = pascal_case(foo_bar), {
        fn my_fn() -> u32 {
            42
        }
    });
    assert_eq!(FooBar(), 42);
}
