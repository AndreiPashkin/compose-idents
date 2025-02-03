![Build](https://github.com/AndreiPashkin/compose-idents/actions/workflows/build.yml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/compose-idents)](https://crates.io/crates/compose-idents)
[![docs.rs](https://img.shields.io/docsrs/compose-idents)](https://docs.rs/compose-idents)

# compose-idents

A procedural macro that allows to construct identifiers from one or more arbitrary parts.

## Motivation

Rust's declarative macros do not allow generating new identifiers, because they are designed to operate on
the syntactic level (as opposed to the lexical level) using simple pattern matching.

For example the following code won't work:
```rust,ignore
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


compose_idents!(my_fn_1 = [foo, _, "baz"]; my_fn_2 = [spam, _, eggs]; {
    fn my_fn_1() -> u32 {
        123
    }

    fn my_fn_2() -> u32 {
        321
    }
});


assert_eq!(foo_baz(), 123);
assert_eq!(spam_eggs(), 321);
```

For more usage examples look into `examples/` and `tests/` directories of the repository.

## Alternatives

There some other projects dedicated to identifier manipulation:

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
