use compose_idents::compose;

compose!(
    MY_NORMALIZED_ALIAS = concat(my, _, normalize(&'static str)),
    {
        static MY_NORMALIZED_ALIAS: &str = "This alias is made from a normalized argument";
    }
);

assert_eq!(
    my_static_str,
    "This alias is made from a normalized argument"
);
