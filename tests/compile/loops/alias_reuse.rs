//! Tests re-use of alias defined by outer loops in the inner loops.
use compose_idents::compose_idents;

compose_idents!(
    for base in [foo, bar]
    for name in [concat(spam, _, base), concat(eggs, _, base)]

    my_fn = name,
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
