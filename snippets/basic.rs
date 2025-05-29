use compose_idents::compose_idents;

compose_idents!(
    // Literal strings are accepted as arguments and their content is parsed.
    my_fn_1 = [foo, _, "bar"],
    // The same applies to literal integers, underscores or free-form token sequences.
    my_fn_2 = [spam, _, 1, _, eggs],
    {
        fn my_fn_1() -> u32 {
            42
        }

        fn my_fn_2() -> u32 {
            42
        }
    },
);

assert_eq!(foo_bar(), 42);
assert_eq!(spam_1_eggs(), 42);
