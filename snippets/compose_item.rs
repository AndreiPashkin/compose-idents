use compose_idents::compose_item;

#[compose_item(
    my_fn = concat(foo, _, bar),
)]
pub fn my_fn() -> u32 {
    42
}

fn main() {
    assert_eq!(foo_bar(), 42);
}
