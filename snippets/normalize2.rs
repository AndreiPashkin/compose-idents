use compose_idents::compose;

compose!(
    // Path -> ident
    A = normalize2(Foo::Bar),
    // Type with lifetime -> ident
    B = normalize2(&'static str),
    // Tokens (via raw fencing) -> ident
    C = normalize2(raw(Result<u32, String>)),
    {
        fn A() -> u32 { 1 }
        fn B() -> u32 { 2 }
        fn C() -> u32 { 3 }
    }
);

assert_eq!(Foo_Bar(), 1);
assert_eq!(static_str(), 2);
assert_eq!(Result_u32_String(), 3);
