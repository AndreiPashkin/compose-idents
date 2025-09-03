use compose_idents::compose_idents;

compose_idents!(for name in [foo, bar] {
    fn name() -> u32 {
        1
    }
});

assert_eq!(foo(), 1);
assert_eq!(bar(), 1);
