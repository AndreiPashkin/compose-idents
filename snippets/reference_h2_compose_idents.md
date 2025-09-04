This is a complete reference to the functionality of this library split into thematic sections.

## Basic alias definition

You can define aliases with the syntax `alias = concat(arg1, normalize(arg2), ...)`, `alias = lower(ARG)`,
`alias = arg`, etc., where args may be identifiers, string literals, integers, underscores, or any arbitrary sequences
of tokens (like `&'static str`, `My::Enum` and so on - such values would be recognized as just tokens):
```rust
use compose_idents::compose_idents;

compose_idents!(
    // Literal strings are accepted as arguments and their content is parsed.
    my_fn_1 = concat(foo, _, bar),
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

## Code repetition

Multiple code variants could be generated with `for ... in [...]` syntax. The loop variable can be used directly inside
the block:
```rust
use compose_idents::compose_idents;

compose_idents!(for name in [foo, bar] {
    fn name() -> u32 {
        1
    }
});

assert_eq!(foo(), 1);
assert_eq!(bar(), 1);
```

## Attribute macro form

`#[compose_item(...)]` is an attribute macro equivalent to `compose_idents! { ... }`, except it treats the annotated item as
the code block. Otherwise, it works the same way:
```rust
use compose_idents::compose_item;

#[compose_item(
    my_fn = concat(foo, _, bar),
)]
pub fn my_fn() -> u32 {
    42
}

fn main() {
    assert_eq!(foo_bar(), 42);
}
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

`normalize2()` is similar to `normalize()`, but it evaluates its single input value first and then
normalizes the result into a valid identifier. Unlike `normalize()`, which operates on raw tokens,
`normalize2()` accepts values of different types — `ident`, `str`, `int`, `path`, `type`, `expr`, and
`tokens` — and always produces an `ident`:
```rust
use compose_idents::compose_idents;

compose_idents!(
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
```

## String formatting

Aliases could be used in string formatting with `% alias %` syntax. This is useful for generating doc-attributes:
```rust
use compose_idents::compose_idents;

compose_idents!(
    my_fn = concat(foo, _, bar),
    MY_FORMATTED_STR = concat(FOO, _, BAR),
    {
        static MY_FORMATTED_STR: &str = "This is % MY_FORMATTED_STR %";

        // You can use % alias % syntax to replace aliases with their definitions
        // in string literals and doc-attributes.
        #[doc = "This is a docstring for % my_fn %"]
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
    complex_fn = concat(to_ident("prefix_"), normalize(&'static str), _, snake_case(CamelCase)),
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

## Syntax

### Expressions

Expressions consist of values (`foo`, `Foo::Bar`, `1 + 1`, `"bar"`, `123`, etc) and
function calls (`concat(foo, _, bar)`, `camel_case(foo_bar)`, etc).

#### Values

A value can represent any sequence of tokens - it could be a simple identifier like `foo`, a path like `std::vec::Vec`,
a literal like `"foo"` or `42`, or more complex constructs.

Values are typed, types of values are detected automatically, values are silently coerced between _some_ of the types
(see the "Types" section below). Most of the time a user doesn't need to care about types or explicitly casting between
them. For explicit casting, see functions described in the "Functions" → "Type casting" section below.

Examples of values of different types could be found in the "Types" section.

##### String formatting

String literals could be formatted using `% alias %` syntax. This is especially useful for generating doc-attributes.

#### Function calls

A function call consists of a function name and the argument-list enclosed in parentheses. Arguments are separated by
commas. Arguments themselves are arbitrary expressions.

A reference of the available functions could be found in the "Functions" section below.

##### Comma-containing arguments

If an argument contains commas - the system would try hard to parse it correctly and determine the argument boundaries,
but if it's not possible - use `raw()` function to fence the complex argument.

##### Function overloading

Functions could be overloaded and have multiple signatures. For example `concat(...)` could work for strings, integers
and for arbitrary tokens as well. All overloads are listed in the "Functions" section.

### Aliases

An alias is an identifier assigned an arbitrary expression: `alias = <expr>`. Alias-definitions are separated by commas.

```plain,ignore
// Alias can be defined as any expression.

// It could be just a simple value.
alias1 = foo,
// Or a function call.
alias2 = concat(foo, _, bar),
// Function calls could be nested.
alias3 = upper(snake_case(fooBarBaz)),
// Complex (often - comma containing) expressions could be fenced using `raw()`.
alias4 = concat(Result<, raw(u32,), String>),
// Any value could be converted to valid identifiers using `normalize()` function.
alias5 = concat(my, _, fn, _, normalize(My::Enum)),
```

#### Alias re-use

Aliases could be re-used in subsequent (but not preceding) definitions of other aliases:

```plain,ignore
alias1 = foo,
alias2 = concat(alias1, _, bar), // alias1 is re-used here
```

### Types

| Type     | Example                              | Description                                                                                                                                                                                  |
|----------|--------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `ident`  | `foo`                                | Identifier type.                                                                                                                                                                             |
| `type`   | `Result<u32, Error>`                 | Type type.                                                                                                                                                                                   |
| `path`   | `foo::bar`                           | Path type.                                                                                                                                                                                   |
| `expr`   | `2 + 2`, `if c { 1 } else { 0 }`     | Expression type.                                                                                                                                                                             |
| `str`    | `"foo"`                              | Literal string type.                                                                                                                                                                         |
| `int`    | `123`                                | Literal integer type.                                                                                                                                                                        |
| `tokens` | `mod foo { fn bar() -> u32 { 0 } }`  | Arbitrary evaluated token-sequence. If used as a function argument - terminated by a comma, and if it contains expressions - they are evaluated.                                             |
| `raw`    | `mod foo { fn bar() -> u32 { 0 } }`  | Raw unevaluated token-sequence. Only used as a type of a single input argument - doesn't respect any delimiters, if contains expressions - they are treated as raw tokens and not evaluated. |

#### Coercion rules

Values are automatically coerced between compatible types when needed. Coercion is limited very limited by design, it doesn't encompass
all possible type conversion directions, only the most useful ones and the ones that are infallible.

| From    | To       | Description                                    |
|---------|----------|------------------------------------------------|
| `ident` | `path`   | Identifier to path (e.g., `foo` → `foo`)       |
| `ident` | `type`   | Identifier to type (e.g., `u32` → `u32`)       |
| `ident` | `expr`   | Identifier to expression (e.g., `foo` → `foo`) |
| any     | `tokens` | Any value to tokens                            |

### Functions

#### Case manipulation

Functions that change the case or style.

| Function                      | Description                                 | Example                  | Example Result |
|-------------------------------|---------------------------------------------|--------------------------|----------------|
| `upper(str) -> str`           | Converts the string argument to UPPER case. | `upper("foo")`           | `"FOO"`        |
| `upper(ident) -> ident`       | Converts the ident argument to UPPER case.  | `upper(foo)`             | `FOO`          |
| `lower(str) -> str`           | Converts the string argument to lower case. | `lower("FOO")`           | `"foo"`        |
| `lower(ident) -> ident`       | Converts the ident argument to lower case.  | `lower(FOO)`             | `foo`          |
| `snake_case(str) -> str`      | Converts the string argument to snake_case. | `snake_case("FooBar")`   | `"foo_bar"`    |
| `snake_case(ident) -> ident`  | Converts the ident argument to snake_case.  | `snake_case(FooBar)`     | `foo_bar`      |
| `camel_case(str) -> str`      | Converts the string argument to camelCase.  | `camel_case("foo_bar")`  | `"fooBar"`     |
| `camel_case(ident) -> ident`  | Converts the ident argument to camelCase.   | `camel_case(foo_bar)`    | `fooBar`       |
| `pascal_case(str) -> str`     | Converts the string argument to PascalCase. | `pascal_case("foo_bar")` | `"FooBar"`     |
| `pascal_case(ident) -> ident` | Converts the ident argument to PascalCase.  | `pascal_case(foo_bar)`   | `FooBar`       |

#### Token manipulation

General purpose functions that perform useful operations on tokens.

| Function                            | Description                                                                    | Example                                 | Example Result        |
|-------------------------------------|--------------------------------------------------------------------------------|-----------------------------------------|-----------------------|
| `normalize(raw) -> ident`           | Transforms raw input into a valid Rust identifier.                             | `normalize(&'static str)`               | `static_str`          |
| `normalize2(ident) -> ident`        | Evaluates the ident and transforms it to a valid identifier.                   | `normalize2(FooBar)`                    | `FooBar`              |
| `normalize2(str) -> ident`          | Evaluates the string literal and transforms it to a valid identifier.          | `normalize2("&'static str")`            | `static_str`          |
| `normalize2(int) -> ident`          | Evaluates the integer literal and transforms it to a valid identifier.         | `normalize2(123)`                       | `_123`                |
| `normalize2(path) -> ident`         | Evaluates the path and transforms it to a valid identifier.                    | `normalize2(Foo::Bar)`                  | `Foo_Bar`             |
| `normalize2(type) -> ident`         | Evaluates the type and transforms it to a valid identifier.                    | `normalize2(&'static str)`              | `static_str`          |
| `normalize2(expr) -> ident`         | Evaluates the expression and transforms it to a valid identifier.              | `normalize2(1 + 2)`                     | `_1_2`                |
| `normalize2(tokens) -> ident`       | Evaluates tokens and transforms them to a valid identifier.                    | `normalize2(raw(Result<u32, String>))`  | `Result_u32_String`   |
| `concat(ident...) -> ident`         | Concatenates multiple idents into a single identifier.                         | `concat(foo, _, bar)`                   | `foo_bar`             |
| `concat(ident, tokens...) -> ident` | Concatenates an ident and follow-up tokens arguments into a single identifier. | `concat(prefix, _, 123)`                | `prefix_123`          |
| `concat(str...) -> str`             | Concatenates multiple strings into a single string.                            | `concat("foo", "_", "bar")`             | `"foo_bar"`           |
| `concat(int...) -> int`             | Concatenates multiple integers into a single integer.                          | `concat(1, 2, 3)`                       | `123`                 |
| `concat(tokens...) -> tokens`       | Concatenates multiple tokens arguments into a single tokens value.             | `concat(Result<, raw(u32,), String, >)` | `Result<u32, String>` |

#### Special purpose

Functions for special use cases.

| Function                | Description                                                                    | Example           | Example Result |
|-------------------------|--------------------------------------------------------------------------------|-------------------|----------------|
| `hash(str) -> str`      | Hashes the string deterministically within a single macro invocation.          | `hash("input")`   | `"12345678"`   |
| `hash(ident) -> ident`  | Hashes the ident deterministically within a single macro invocation.           | `hash(input)`     | `__12345678`   |
| `hash(tokens) -> ident` | Hashes the tokens argument deterministically within a single macro invocation. | `hash(foo + bar)` | `__87654321`   |

#### Type casting

These functions are useful whenever you need to explicitly cast an arbitrary value to a particular type.

| Function                      | Description                                                                                             | Example                           | Example Result       |
|-------------------------------|---------------------------------------------------------------------------------------------------------|-----------------------------------|----------------------|
| `raw(raw) -> tokens`          | Converts raw unevaluated input to a tokens value. Useful for noisy inputs that contain separators, etc. | `raw(Result<u32, Error>)`         | `Result<u32, Error>` |
| `to_ident(tokens) -> ident`   | Converts the tokens argument to an identifier.                                                          | `to_ident(lower("FOO"))`          | `foo`                |
| `to_path(tokens) -> path`     | Converts the tokens argument to a path.                                                                 | `to_path(concat(std, ::, vec))`   | `std::vec`           |
| `to_type(tokens) -> type`     | Converts the tokens argument to a type.                                                                 | `to_type(concat(Vec, <, u32, >))` | `Vec<u32>`           |
| `to_expr(tokens) -> expr`     | Converts the tokens argument to an expression.                                                          | `to_expr(concat(1, +, 2))`        | `1 + 2`              |
| `to_str(tokens) -> str`       | Converts the tokens argument to a string.                                                               | `to_str(foo)`                     | `"foo"`              |
| `to_int(tokens) -> int`       | Converts the tokens argument to an integer.                                                             | `to_int(concat(4, 2))`            | `42`                 |
| `to_tokens(tokens) -> tokens` | Identity function for tokens - useful for converting any value to tokens.                               | `to_tokens(foo)`                  | `foo`                |
