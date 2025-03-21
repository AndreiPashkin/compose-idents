![Build](https://github.com/AndreiPashkin/compose-idents/actions/workflows/build.yml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/compose-idents)](https://crates.io/crates/compose-idents)
[![docs.rs](https://img.shields.io/docsrs/compose-idents)](https://docs.rs/compose-idents)

# compose-idents

A macro for generating new identifiers (names of variables, functions, traits, etc) by concatenating one or more
arbitrary parts and applying other manipulations.

## Motivation

Rust's declarative macros do not allow generating new identifiers, because they are designed to operate on
the syntactic level (as opposed to the lexical level) using simple pattern matching.

For example the following code won't work:
```rust,compile_fail
macro_rules! my_macro {
    ($name:ident) => {
        my_$name_fn() -> u32 {
            42
        }
    };
}

my_macro!(foo);
assert_eq!(my_foo_fn(), 42);
```

`compose-idents` resolves this limitation:
```rust
use compose_idents::compose_idents;

macro_rules! my_macro {
    ($name:ident) => {
        compose_idents!(
            my_fn = [my, _, $name, _, "fn"]; {
                fn my_fn() -> u32 {
                    42
                }
            }
        )
    }
}

my_macro!(foo);
assert_eq!(my_foo_fn(), 42);
```

## Usage

This section contains various usage examples. For even more usage examples look into `tests/` directory
of the repository.

### Full example

This example includes all the features of the macro:
```rust
{{ file.Read "snippets/usage.rs" -}}
```

### Generating tests for different types

More practical example for how to auto-generate names for macro-generated tests for different data types:
```rust
use std::ops::Add;
use compose_idents::compose_idents;

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
  x + y
}

macro_rules! generate_add_tests {
    ($($type:ty),*) => {
      $(
        compose_idents!(test_fn = [test_add_, $type]; {
          fn test_fn() {
            let actual = add(2 as $type, 2 as $type);
            let expected = (2 + 2) as $type;

            assert_eq!(actual, expected);
          }
        });
      )*
    };
}

generate_add_tests!(u8, u32, u64);

test_add_u8();
test_add_u32();
test_add_u64();
```

### Formatting docstrings for generated functions

It's possible to format strings in doc-attributes (and also any literal strings) using `%alias%` syntax. It is useful
for generating docstrings for generated functions.

In this particular example we are generating addition functions that work at compile time for different types
(as `core::ops::Add` can't be used in generic const-functions). Notice, in addition to the function name,
the docstring is also formatted so that it mentions the type of the function:
```rust
use compose_idents::compose_idents;


macro_rules! generate_add {
    ($T:ty) => {
        compose_idents!(
            T = [$T];
            add_fn = [add, _, $T]; {
                #[doc = "Adds two arguments of type `%T%` at compile time."]
                const fn add_fn(a: $T, b: $T) -> $T {
                    a + b
                }
            }
        );
    };
}

generate_add!(u32);
generate_add!(u64);
```

The above example expands into this:
```rust
use compose_idents::compose_idents;

///Adds two arguments of type `u32` at compile time.
const fn add_u32(a: u32, b: u32) -> u32 {
  a + b
}

///Adds two arguments of type `u64` at compile time.
const fn add_u64(a: u64, b: u64) -> u64 {
  a + b
}
```

## Functions

| Function          | Description                                                          |
|-------------------|----------------------------------------------------------------------|
| `upper(arg)`      | Converts the `arg` to upper case.                                    |
| `lower(arg)`      | Converts the `arg` to lower case.                                    |
| `snake_case(arg)` | Converts the `arg` to snake_case.                                    |
| `camel_case(arg)` | Converts the `arg` to camelCase.                                     |
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
