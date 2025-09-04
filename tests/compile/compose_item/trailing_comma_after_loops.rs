//! A trailing comma after loops (with no alias spec) should be rejected for `#[compose_item]`.
use compose_idents::compose_item;

#[compose_item(
    for name in [foo, bar],
)]
fn name() {}

fn main() {}
