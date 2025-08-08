//! String-formatting should be possible for string literals.
use compose_idents::compose_idents;

compose_idents!(my_alias = concat(foo, _, "baz"), {
    static MY_STRING: &str = "Hello, %my_alias%!";
    static MY_STRING_WHITESPACE: &str = "Hello, % my_alias %!";
    static MY_STRING_IRREGULAR_WHITESPACE: &str = "Hello, % my_alias  %!";
});

fn main() {
    assert_eq!(MY_STRING, "Hello, foo_baz!");
    assert_eq!(MY_STRING_WHITESPACE, "Hello, foo_baz!");
    assert_eq!(MY_STRING_IRREGULAR_WHITESPACE, "Hello, foo_baz!");
}
