This is a complete reference to the functionality of this library split into thematic sections.

## Basic alias definition

You can define aliases with the syntax `alias = concat(arg1, normalize(arg2), ...)`, `alias = lower(ARG)`,
`alias = arg`, etc., where args may be identifiers, string literals, integers, underscores, or any arbitrary sequences
of tokens (like `&'static str`, `My::Enum` and so on - such values would be recognized as just tokens):
```rust
use compose_idents::compose_idents;

compose_idents!(
    // Literal strings are accepted as arguments and their content is parsed.
    my_fn_1 = concat(foo, _, "bar"),
    // The same applies to literal integers, underscores or free-form token sequences.
    my_fn_2 = concat(spam, _, 1, _, eggs),
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
```

## Alias reuse

Aliases could also be reused in definitions of other aliases:
```rust
use compose_idents::compose_idents;

compose_idents!(
    base_alias = FOO,
    derived_alias = concat(BAR, _, base_alias),
    {
        static base_alias: u32 = 1;
        static derived_alias: u32 = base_alias;
    },
);

assert_eq!(FOO, 1);
assert_eq!(BAR_FOO, 1);
```

## Functions

Functions can be applied to the arguments used for the alias definitions:
```rust
use compose_idents::compose_idents;

compose_idents!(
    my_const = concat(upper(foo), _, lower(BAR)),
    // Function calls can be arbitrarily nested and combined.
    my_static = upper(lower(BAZ)),
    {
        const my_const: u8 = 1;
        static my_static: &str = "hello";
    }
);

assert_eq!(FOO_bar, 1);
assert_eq!(BAZ, "hello");
```

You can find a complete description of all functions below under "Functions" heading.

## Casing manipulation

There are multiple functions for altering the naming convention of identifiers:
```rust
use compose_idents::compose_idents;

compose_idents!(
    MY_SNAKE_CASE_STATIC = snake_case(snakeCase),
    MY_CAMEL_CASE_STATIC = camel_case(camel_case),
    MY_PASCAL_CASE_STATIC = pascal_case(concat(pascal, _, case)),
    {
        static MY_SNAKE_CASE_STATIC: u32 = 1;
        static MY_CAMEL_CASE_STATIC: u32 = 2;
        static MY_PASCAL_CASE_STATIC: u32 = 3;
    },
);

assert_eq!(snake_case, 1);
assert_eq!(camelCase, 2);
assert_eq!(PascalCase, 3);
```

## Token normalization

`normalize()` function is useful for making valid identifiers out of arbitrary tokens:
```rust
use compose_idents::compose_idents;

compose_idents!(
    MY_NORMALIZED_ALIAS = concat(my, _, normalize(&'static str)),
    {
        static MY_NORMALIZED_ALIAS: &str = "This alias is made from a normalized argument";
    }
);

assert_eq!(
    my_static_str,
    "This alias is made from a normalized argument"
);
```

## String formatting

Aliases could be used in string formatting with `%alias%` syntax. This is useful for generating doc-attributes:
```rust
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
```

## Generating unique identifiers

`hash()` function deterministically hashes the input _within a single macro invocation_. It means that within the same
`compose_idents!` call `hash(foobar)` will always produce the same output. But in another call - the output would be
different (but also the same for the same input).

It could be used to avoid conflicts between identifiers of global variables, or any other items that are defined in
global scope.

```rust
use compose_idents::compose_idents;

macro_rules! create_static {
    () => {
        compose_idents!(
            MY_UNIQUE_STATIC = hash(1),
            MY_OTHER_UNIQUE_STATIC = hash(2),
            {
                static MY_UNIQUE_STATIC: u32 = 42;
                static MY_OTHER_UNIQUE_STATIC: u32 = 42;
            }
        );
    };
}

create_static!();
create_static!();
```

This example roughly expands to this:
```rust
use compose_idents::compose_idents;
static __5360156246018494022: u32 = 42;
static __1421539829453635175: u32 = 42;
static __17818851730065003648: u32 = 42;
static __10611722954104835980: u32 = 42;
```

## Concatenating multiple arguments

The `concat()` function takes multiple arguments and concatenates them together. It provides explicit concatenation
that can be either nested within other function calls or to aggregate results of other function calls:

```rust
use compose_idents::compose_idents;

compose_idents!(
    // Basic example
    basic_fn = concat(foo, _, bar, _, baz),
    // Mixed with other functions
    upper_fn = upper(concat(hello, _, world)),
    // Complex example
    complex_fn = concat("prefix_", normalize(&'static str), "_", snake_case(CamelCase)),
    {
        fn basic_fn() -> u32 { 1 }
        fn upper_fn() -> u32 { 2 }
        fn complex_fn() -> u32 { 3 }
    }
);

assert_eq!(foo_bar_baz(), 1);
assert_eq!(HELLO_WORLD(), 2);
assert_eq!(prefix_static_str_camel_case(), 3);
```

## Functions

| Function                  | Description                                                          |
|---------------------------|----------------------------------------------------------------------|
| `upper(arg)`              | Converts the `arg` to upper case.                                    |
| `lower(arg)`              | Converts the `arg` to lower case.                                    |
| `snake_case(arg)`         | Converts the `arg` to snake_case.                                    |
| `camel_case(arg)`         | Converts the `arg` to camelCase.                                     |
| `pascal_case(arg)`        | Converts the `arg` to PascalCase.                                    |
| `normalize(tokens)`       | Transforms a free-form sequence of `tokens` into a valid identifier. |
| `hash(arg)`               | Hashes the `arg` deterministically within a single macro invocation. |
| `concat(arg1, arg2, ...)` | Concatenates multiple arguments into a single identifier.            |
