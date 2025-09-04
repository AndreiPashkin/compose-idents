//! A trailing comma after alias spec should be allowed for `#[compose_item]`.
use compose_idents::compose_item;

#[compose_item(
    fn_name = concat(hello, _, world),
)]
fn fn_name() -> u32 {
    42
}

fn main() {
    assert_eq!(hello_world(), 42);
}
