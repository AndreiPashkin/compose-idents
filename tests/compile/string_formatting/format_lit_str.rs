//! String-formatting should be possible for string literals.
use compose_idents::compose_idents;

compose_idents!(my_alias = concat(foo, _, "baz"), {
    static MY_STRING: &str = "Hello, %my_alias%!";
});

fn main() {
    assert_eq!(MY_STRING, "Hello, foo_baz!");
}
