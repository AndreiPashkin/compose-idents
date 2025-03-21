use compose_idents::compose_idents;

compose_idents!(
    my_alias = [foo, _, "baz"]; {
        static MY_STRING: &str = "Hello, %my_alias%!";
    };
);

fn main() {
    assert_eq!(MY_STRING, "Hello, foo_baz!");
}
