//! Using macro-arguments of "type" type should be compatible with compose_idents!.
use compose_idents::compose;

macro_rules! outer_macro {
    ($t:ty) => {
        compose!(my_fn = concat(foo, _, $t), {
            fn my_fn() -> $t {
                42 as $t
            }
        });
    };
}

outer_macro!(u8);
outer_macro!(u32);

fn main() {
    assert_eq!(foo_u8(), 42_u8);
    assert_eq!(foo_u32(), 42_u32);
}
