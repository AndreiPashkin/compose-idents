use compose_idents::compose_idents;

compose_idents!(my_fn = [foo, _, "bar"]; {
    fn my_fn() -> u32 {
        1
    }
});

compose_idents!(
    my_fn_1 = [foo, _, "baz"];
    my_fn_2 = [spam, _, eggs]; {
    fn my_fn_1() -> u32 {
        2
    }

    fn my_fn_2() -> u32 {
        3
    }
});

fn main() {
    assert_eq!(foo_bar(), 1);
    assert_eq!(foo_baz(), 2);
    assert_eq!(spam_eggs(), 3);
}
