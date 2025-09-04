//! Using `compose_idents!` should trigger a deprecation warning.
#![deny(warnings)]
use compose_idents::compose_idents;

compose_idents!(name = foo, {
    fn name() {}
},);

fn main() {}
