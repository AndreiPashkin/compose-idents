![Build](https://github.com/AndreiPashkin/compose-idents/actions/workflows/build.yml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/compose-idents)](https://crates.io/crates/compose-idents)
[![docs.rs](https://img.shields.io/docsrs/compose-idents)](https://docs.rs/compose-idents)

# compose-idents

A procedural macro that allows to construct identifiers from one or more arbitrary parts.

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
```

This is why there is a need for a macro that allows to construct new identifiers.

## Usage

Here is how the macro works:
```rust
use compose_idents::compose_idents;


compose_idents!(
  my_fn = [foo, _, "baz"];
  MY_UPPER_STATIC = [upper(spam), _, upper("eggs")];
  MY_LOWER_STATIC = [lower(GORK), _, lower(BORK)];
  MY_SNAKE_CASE_STATIC = [snake_case(snakeCase)];
  MY_CAMEL_CASE_STATIC = [camel_case(camel_case)]; {
    fn my_fn() -> u32 {
        123
    }

    static MY_UPPER_STATIC: u32 = 321;
    static MY_LOWER_STATIC: u32 = 321123;
    static MY_SNAKE_CASE_STATIC: u32 = 123321;
    static MY_CAMEL_CASE_STATIC: u32 = 333333;
});


assert_eq!(foo_baz(), 123);
assert_eq!(SPAM_EGGS, 321);
assert_eq!(gork_bork, 321123);
assert_eq!(snake_case, 123321);
assert_eq!(camelCase, 333333);
```

Here is a more practical example for how to auto-generate names for macro-generated tests for different data types:
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

For more usage examples look into `examples/` and `tests/` directories of the repository.

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
