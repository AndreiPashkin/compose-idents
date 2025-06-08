//! Tests various loop-related features.
use compose_idents::compose_idents;

compose_idents!(
    for base in [foo, bar]
    for (prefix, suffix) in [("gork", bork), (lower(SPAM), normalize(My::Enum))]

    my_fn = concat(prefix, _, base, _, suffix),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(gork_foo_bork(), 42);
    assert_eq!(spam_foo_My_Enum(), 42);
    assert_eq!(gork_bar_bork(), 42);
    assert_eq!(spam_bar_My_Enum(), 42);
}
