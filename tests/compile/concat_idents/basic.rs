//! Basic usage of `compose_idents!` should work as expected.
use compose_idents::compose_idents;

compose_idents!(
    // For-in loops could be used to generate multiple variations of the code.
    for (suffix, (interjection, noun)) in [
        (BAR, (Hello, "world")),
        (baz, ("Hallo", "welt")),
    ]

    // A simple alias definition.
    my_fn = concat(foo, _, 1, _, lower(suffix)),
    // Many functions are overloaded support different input argument types.
    greeting = concat(to_str(interjection), ", ", noun, "!"),
    {
        // String placeholders `% my_alias %` are expanded inside literals and doc attributes.
        #[doc = "Greets: % greeting %"]
        fn my_fn() -> &'static str { greeting }
    },
);

fn main() {
    assert_eq!(foo_1_bar(), "Hello, world!");
    assert_eq!(foo_1_baz(), "Hallo, welt!");
}
