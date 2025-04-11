use compose_idents::compose_idents;

compose_idents!(my_fn = [foo, _, "baz"], {
    #[allow(dead_code)]
    #[doc = "My doc comment for %my_fn%"]
    fn my_fn() -> u32 {
        42
    }
});

fn main() {}
