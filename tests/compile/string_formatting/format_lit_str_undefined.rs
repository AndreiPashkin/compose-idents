//! String-formatting should skip aliases that are undefined.
use compose_idents::compose_idents;

compose_idents!(my_alias2 = abc, {
    static MY_STRING: &str = "Hello, %my_alias%!";
});

fn main() {
    assert_eq!(MY_STRING, "Hello, %my_alias%!");
}
