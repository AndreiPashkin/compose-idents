//! hash() should generate unique identifiers without collisions across scopes (and a scope is
//! a single macro invocation).
use compose_idents::compose_idents;

compose_idents!(
    my_unique_var = concat(foo, _, hash(1)),
    my_var = concat(SPAM, _, EGGS),
    {
        const my_unique_var: u32 = 42;
        const my_var: u32 = my_unique_var;
    }
);

compose_idents!(
    my_unique_var = concat(foo, _, hash(1)),
    my_var = concat(BORK, _, GORK),
    {
        const my_unique_var: u32 = 42;
        const my_var: u32 = my_unique_var;
    }
);

macro_rules! my_macro {
    () => {
        compose_idents!(
            my_local = concat(foo, _, hash(1)),
            my_same_local = concat(foo, _, hash(1)),
            my_other_local = concat(foo, _, hash(2)),
            {
                let my_local: u32 = 42;
                let my_other_local: u32 = my_same_local;
                let comparison: bool = my_local == my_other_local;

                assert!(comparison);
            }
        );
    };
}

fn main() {
    assert_eq!(SPAM_EGGS, 42);
    assert_eq!(BORK_GORK, 42);

    my_macro!();
}
