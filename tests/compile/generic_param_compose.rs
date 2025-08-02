//! Composed aliases should be usable as const generic parameters.
use compose_idents::compose_idents;

compose_idents!(DATA_LEN = concat(FOO, _, BAR), {
    fn my_fn<T, const DATA_LEN: usize>(_data: &[T; DATA_LEN]) -> usize {
        DATA_LEN
    }
});

fn main() {
    assert_eq!(my_fn(&[1, 2, 3]), 3);
}
