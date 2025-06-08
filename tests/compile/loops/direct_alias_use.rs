//! Tests usage of aliases defined by loops directly in the code block.
use compose_idents::compose_idents;

compose_idents!(
    for base in [foo, bar]
    for my_fn in [concat(spam, _, base), concat(eggs, _, base)]
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(spam_foo(), 42);
    assert_eq!(eggs_foo(), 42);
    assert_eq!(spam_bar(), 42);
    assert_eq!(eggs_bar(), 42);
}
