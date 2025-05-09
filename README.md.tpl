![Build](https://github.com/AndreiPashkin/compose-idents/actions/workflows/build.yml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/compose-idents)](https://crates.io/crates/compose-idents)
[![docs.rs](https://img.shields.io/docsrs/compose-idents)](https://docs.rs/compose-idents)

# compose-idents

A macro for generating new identifiers (names of variables, functions, traits, etc.) by concatenating one or more
arbitrary parts and applying other manipulations.

It was created as an alternative to `macro_rules!` that doesn't allow creating new identifiers from the macro arguments
and [`concat_idents!`][1] macro from the nightly Rust, which is limited in capabilities and has not been stabilized
[since 2015][2].

[1]: https://doc.rust-lang.org/std/macro.concat_idents.html
[2]: https://github.com/rust-lang/rust/issues/29599

## Features

- **Identifier generation**

  Identifiers can be generated via concatenation of multiple parts. Arguments of the outer macro
  definitions and literals are supported for identifier definitions.
- **Functions**

  Functions can be applied when defining new identifiers for changing case and style.
- **String formatting**

  Strings can be formatted with `%alias%` syntax, which is useful for generating doc-attributes.

## Usage

This section contains various usage examples. For even more usage examples look into `tests/` directory
of the repository.

### Quick start example

`compose_idents!` works by accepting definitions of aliases and a code block where aliases
could be used as normal identifiers. When the macro is expanded, the aliases are replaced with their
definitions:
```rust
use compose_idents::compose_idents;

// We generate separate const-functions for each type as a workaround
// since Rust doesn't allow us to use `core::ops::Add` in `const fn`.

macro_rules! gen_const_add {
    ($T:ty) => {
        compose_idents!(
            Type = [upper($T)],   // Alias for the type - make it uppercase in addition
            add_fn = [add_, $T],  // Alias for the function name
            {
                // Strings (including in doc-attributes) can be formatted with %alias% syntax.
                #[doc = "Adds two arguments of type `%Type%` at compile time."]
                pub const fn add_fn(a: $T, b: $T) -> $T {  // Aliases are used as normal identifiers
                    a + b
                }
            }
        );
    };
}

gen_const_add!(u32);  // Expands into `add_u32()` function.
gen_const_add!(u64);  // Expands into `add_u64()` function.

assert_eq!(add_u32(2_u32, 2_u32), 4_u32);
assert_eq!(add_u64(2_u64, 2_u64), 4_u64);
```

### Generating tests for different types

Another practical example for how to auto-generate names for macro-generated tests for different data types:
```rust
use std::ops::Add;
use compose_idents::compose_idents;

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
  x + y
}

macro_rules! generate_add_tests {
    ($($type:ty),*) => {
      $(
        compose_idents!(test_fn = [test_add_, $type], {
          fn test_fn() {
            let actual = add(2 as $type, 2 as $type);
            let expected = (2 + 2) as $type;

            assert_eq!(actual, expected);
          }
        });
      )*
    };
}

// Generates tests for u8, u32 and u64 types
generate_add_tests!(u8, u32, u64);

test_add_u8();
test_add_u32();
test_add_u64();
```

### Reference example

This example includes all the features of the macro:
```rust
{{ file.Read "snippets/usage.rs" -}}
```

## Functions

| Function          | Description                                                          |
|-------------------|----------------------------------------------------------------------|
| `upper(arg)`      | Converts the `arg` to upper case.                                    |
| `lower(arg)`      | Converts the `arg` to lower case.                                    |
| `snake_case(arg)` | Converts the `arg` to snake_case.                                    |
| `camel_case(arg)` | Converts the `arg` to camelCase.                                     |
| `pascal_case(arg)`| Converts the `arg` to PascalCase.                                   |
| `hash(arg)`       | Hashes the `arg` deterministically within a single macro invocation. |


## Alternatives

There some other tools and projects dedicated to identifier manipulation:

- A macro from Nightly Rust that allows to concatenate identifiers. It is limited in functionality and nowhere near
  to be stabilized:
  <https://doc.rust-lang.org/std/macro.concat_idents.html>
- A very similar macro that doesn't support multiple aliases and is not maintained:
  <https://github.com/DzenanJupic/concat-idents>
- A macro that allows to define and refer to unique temporary variables:
  <https://crates.io/crates/templest>

## Development

The following standards are followed to maintain the project:
- <https://www.conventionalcommits.org/en/v1.0.0/>
- <https://semver.org/>
- <https://keepachangelog.com/en/1.1.0/>
- <https://adr.github.io/madr/>
