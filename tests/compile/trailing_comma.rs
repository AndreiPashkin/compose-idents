//! A trailing comma after the item block should be allowed.
use compose_idents::compose;

compose!(my_fn = concat(foo, _, bar), {
    fn my_fn() -> u32 {
        42
    }
},);

fn main() {
    assert_eq!(foo_bar(), 42);
}
