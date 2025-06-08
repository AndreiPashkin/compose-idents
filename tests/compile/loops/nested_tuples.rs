//! Tests nested loops that use loops.
use compose_idents::compose_idents;

compose_idents!(
    for (part_a, part_b) in [(foo, bar), (baz, qux)]
    for (prefix, suffix) in [(gork, bork), (spam, eggs)]

    my_fn = concat(prefix, _, part_a, _, part_b, _, suffix),
    {
        fn my_fn() -> u32 {
            42
        }
    }
);

fn main() {
    assert_eq!(gork_foo_bar_bork(), 42);
    assert_eq!(spam_foo_bar_eggs(), 42);
    assert_eq!(gork_baz_qux_bork(), 42);
    assert_eq!(spam_baz_qux_eggs(), 42);
}
