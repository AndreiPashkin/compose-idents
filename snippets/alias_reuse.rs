use compose_idents::compose;

compose!(
    base_alias = FOO,
    derived_alias = concat(BAR, _, base_alias),
    {
        static base_alias: u32 = 1;
        static derived_alias: u32 = base_alias;
    },
);

assert_eq!(FOO, 1);
assert_eq!(BAR_FOO, 1);
