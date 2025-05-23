use compose_idents::compose_idents;

compose_idents!(
    // Literal strings are accepted as arguments and their content is parsed.
    my_fn_1 = [foo, _, "baz"],
    // So as literal integers and underscores (or free-form token sequences).
    my_fn_2 = [spam, _, 1, _, eggs],
    // Functions can be applied to the arguments.
    my_const = [upper(foo), _, lower(BAR)],
    // Function calls can be arbitrarily nested and combined.
    my_static = [upper(lower(BAR))],
    MY_SNAKE_CASE_STATIC = [snake_case(snakeCase)],
    MY_CAMEL_CASE_STATIC = [camel_case(camel_case)],
    MY_PASCAL_CASE_STATIC = [pascal_case(pascal_case)],
    // normalize() allows to turn an arbitrary sequence of tokens into a valid identifier.
    MY_NORMALIZED_ALIAS = [my, _, normalize(&'static str)],
    // This function is useful to create identifiers that are unique across multiple macro invocations.
    // `hash(0b11001010010111)` will generate the same value even if called twice in the same macro call,
    // but will be different in different macro calls.
    MY_UNIQUE_STATIC = [hash(0b11001010010111)],
    MY_FORMATTED_STR = [FOO, _, BAR],
    MY_REUSED_ALIAS = [REUSED, _, FOO, _, my_static],
    {
        fn my_fn_1() -> u32 {
            123
        }

        // You can use %alias% syntax to replace aliases with their replacements
        // in string literals and doc-attributes.
        #[doc = "This is a docstring for %my_fn_2%"]
        fn my_fn_2() -> u32 {
            321
        }

        const my_const: u32 = 42;
        static my_static: u32 = 42;
        static MY_SNAKE_CASE_STATIC: u32 = 42;
        static MY_CAMEL_CASE_STATIC: u32 = 42;
        static MY_PASCAL_CASE_STATIC: u32 = 42;
        static MY_NORMALIZED_ALIAS: &'static str = "This alias is made from a normalized argument";
        static MY_UNIQUE_STATIC: u32 = 42;
        // This is an example of string literal formatting.
        static MY_FORMATTED_STR: &str = "This is %MY_FORMATTED_STR%";
        static MY_REUSED_ALIAS: u32 = 42;
    }
);

// It's possible to use arguments of declarative macros as parts of the identifiers.
macro_rules! outer_macro {
    ($name:tt) => {
        compose_idents!(my_nested_fn = [nested, _, $name], {
            fn my_nested_fn() -> u32 {
                42
            }
        });
    };
}

outer_macro!(foo);

macro_rules! global_var_macro {
    () => {
        // `my_static` is going to be unique in each invocation of `global_var_macro!()`.
        // But within the same invocation `hash(1)` will yield the same result.
        compose_idents!(my_static = [foo, _, hash(1)], {
            static my_static: u32 = 42;
        });
    };
}

global_var_macro!();
global_var_macro!();

assert_eq!(foo_baz(), 123);
assert_eq!(spam_1_eggs(), 321);
assert_eq!(nested_foo(), 42);
assert_eq!(FOO_bar, 42);
assert_eq!(BAR, 42);
assert_eq!(snake_case, 42);
assert_eq!(camelCase, 42);
assert_eq!(PascalCase, 42);
assert_eq!(
    my_static_str,
    "This alias is made from a normalized argument"
);
assert_eq!(FOO_BAR, "This is FOO_BAR");
assert_eq!(REUSED_FOO_BAR, 42);
