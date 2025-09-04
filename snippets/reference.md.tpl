{{- $h1 := tmpl.Exec "heading" (dict "headings_level" .headings_level "level" 0) -}}
{{- $h2 := tmpl.Exec "heading" (dict "headings_level" .headings_level "level" 1) -}}
{{- $h3 := tmpl.Exec "heading" (dict "headings_level" .headings_level "level" 2) -}}
{{- $h4 := tmpl.Exec "heading" (dict "headings_level" .headings_level "level" 3) -}}

This is a complete reference to the functionality of this library split into thematic sections.

{{ $h1 }} Basic alias definition

You can define aliases with the syntax `alias = concat(arg1, normalize(arg2), ...)`, `alias = lower(ARG)`,
`alias = arg`, etc., where args may be identifiers, string literals, integers, underscores, or any arbitrary sequences
of tokens (like `&'static str`, `My::Enum` and so on - such values would be recognized as just tokens):
```rust
{{ file.Read "snippets/basic.rs" -}}
```

{{ $h1 }} Alias reuse

Aliases could also be reused in definitions of other aliases:
```rust
{{ file.Read "snippets/alias_reuse.rs" -}}
```

{{ $h1 }} Code repetition

Multiple code variants could be generated with `for ... in [...]` syntax. The loop variable can be used directly inside
the block:
```rust
{{ file.Read "snippets/code_repetition.rs" -}}
```

{{ $h1 }} Attribute macro form

`#[compose_item(...)]` is an attribute macro equivalent to `compose! { ... }`, except it treats the annotated item as
the code block. Otherwise, it works the same way:
```rust
{{ file.Read "snippets/compose_item.rs" -}}
```

{{ $h1 }} Functions

Functions can be applied to the arguments used for the alias definitions:
```rust
{{ file.Read "snippets/functions.rs" -}}
```

You can find a complete description of all functions below under "Functions" heading.

{{ $h1 }} Casing manipulation

There are multiple functions for altering the naming convention of identifiers:
```rust
{{ file.Read "snippets/casing.rs" -}}
```

{{ $h1 }} Token normalization

`normalize()` function is useful for making valid identifiers out of arbitrary tokens:
```rust
{{ file.Read "snippets/normalize.rs" -}}
```

`normalize2()` is similar to `normalize()`, but it evaluates its single input value first and then
normalizes the result into a valid identifier. Unlike `normalize()`, which operates on raw tokens,
`normalize2()` accepts values of different types — `ident`, `str`, `int`, `path`, `type`, `expr`, and
`tokens` — and always produces an `ident`:
```rust
{{ file.Read "snippets/normalize2.rs" -}}
```

{{ $h1 }} String formatting

Aliases could be used in string formatting with `% alias %` syntax. This is useful for generating doc-attributes:
```rust
{{ file.Read "snippets/string_formatting.rs" -}}
```

{{ $h1 }} Generating unique identifiers

`hash()` function deterministically hashes the input _within a single macro invocation_. It means that within the same
`compose!` call `hash(foobar)` will always produce the same output. But in another call - the output would be
different (but also the same for the same input).

It could be used to avoid conflicts between identifiers of global variables, or any other items that are defined in
global scope.

```rust
{{ file.Read "snippets/hash.rs" -}}
```

This example roughly expands to this:
```rust
{{ file.Read "snippets/hash_expansion.rs" -}}
```

{{ $h1 }} Concatenating multiple arguments

The `concat()` function takes multiple arguments and concatenates them together. It provides explicit concatenation
that can be either nested within other function calls or to aggregate results of other function calls:

```rust
{{ file.Read "snippets/concat.rs" -}}
```

{{ $h1 }} Syntax

{{ $h2 }} Expressions

Expressions consist of values (`foo`, `Foo::Bar`, `1 + 1`, `"bar"`, `123`, etc) and
function calls (`concat(foo, _, bar)`, `camel_case(foo_bar)`, etc).

{{ $h3 }} Values

A value can represent any sequence of tokens - it could be a simple identifier like `foo`, a path like `std::vec::Vec`,
a literal like `"foo"` or `42`, or more complex constructs.

Values are typed, types of values are detected automatically, values are silently coerced between _some_ of the types
(see the "Types" section below). Most of the time a user doesn't need to care about types or explicitly casting between
them. For explicit casting, see functions described in the "Functions" → "Type casting" section below.

Examples of values of different types could be found in the "Types" section.

{{ $h4 }} String formatting

String literals could be formatted using `% alias %` syntax. This is especially useful for generating doc-attributes.

{{ $h3 }} Function calls

A function call consists of a function name and the argument-list enclosed in parentheses. Arguments are separated by
commas. Arguments themselves are arbitrary expressions.

A reference of the available functions could be found in the "Functions" section below.

{{ $h4 }} Comma-containing arguments

If an argument contains commas - the system would try hard to parse it correctly and determine the argument boundaries,
but if it's not possible - use `raw()` function to fence the complex argument.

{{ $h4 }} Function overloading

Functions could be overloaded and have multiple signatures. For example `concat(...)` could work for strings, integers
and for arbitrary tokens as well. All overloads are listed in the "Functions" section.

{{ $h2 }} Aliases

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

{{ $h3 }} Alias re-use

Aliases could be re-used in subsequent (but not preceding) definitions of other aliases:

```plain,ignore
alias1 = foo,
alias2 = concat(alias1, _, bar), // alias1 is re-used here
```

{{ $h2 }} Types

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

{{ $h3 }} Coercion rules

Values are automatically coerced between compatible types when needed. Coercion is limited very limited by design, it doesn't encompass
all possible type conversion directions, only the most useful ones and the ones that are infallible.

| From    | To       | Description                                    |
|---------|----------|------------------------------------------------|
| `ident` | `path`   | Identifier to path (e.g., `foo` → `foo`)       |
| `ident` | `type`   | Identifier to type (e.g., `u32` → `u32`)       |
| `ident` | `expr`   | Identifier to expression (e.g., `foo` → `foo`) |
| any     | `tokens` | Any value to tokens                            |

{{ $h2 }} Functions

{{ $h3 }} Case manipulation

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

{{ $h3 }} Token manipulation

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

{{ $h3 }} Special purpose

Functions for special use cases.

| Function                | Description                                                                    | Example           | Example Result |
|-------------------------|--------------------------------------------------------------------------------|-------------------|----------------|
| `hash(str) -> str`      | Hashes the string deterministically within a single macro invocation.          | `hash("input")`   | `"12345678"`   |
| `hash(ident) -> ident`  | Hashes the ident deterministically within a single macro invocation.           | `hash(input)`     | `__12345678`   |
| `hash(tokens) -> ident` | Hashes the tokens argument deterministically within a single macro invocation. | `hash(foo + bar)` | `__87654321`   |

{{ $h3 }} Type casting

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
