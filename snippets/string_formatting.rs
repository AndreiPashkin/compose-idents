use compose_idents::compose_idents;

compose_idents!(
    my_fn = concat(foo, _, "baz"),
    MY_FORMATTED_STR = concat(FOO, _, BAR),
    {
        static MY_FORMATTED_STR: &str = "This is %MY_FORMATTED_STR%";

        // You can use %alias% syntax to replace aliases with their definitions
        // in string literals and doc-attributes.
        #[doc = "This is a docstring for %my_fn%"]
        fn my_fn() -> u32 {
            321
        }
    },
);

assert_eq!(FOO_BAR, "This is FOO_BAR");
