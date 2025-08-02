//! Using mixed separator styles (semicolon and comma) should result in a compile-time error.
use compose_idents::compose_idents;

compose_idents!(
    my_fn_1 = concat(foo, _, "baz"),
    my_fn_2 = concat(spam, _, eggs); {
    fn my_fn_1() -> u32 {
        123
    }

    fn my_fn_2() -> u32 {
        321
    }
});

fn main() {
    assert_eq!(foo_baz(), 123);
    assert_eq!(spam_eggs(), 321);
}
