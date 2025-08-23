//! The error message should correctly point to problematic location in the code-block and also
//! report source identifier and the substitution.
use compose_idents::compose_idents;

compose_idents!(my_fn = "foo bar", {
    fn my_fn() -> u32 {
        42
    }
},);

fn main() {}
