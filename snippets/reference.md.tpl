{{- define "heading" -}}
  {{- $hash := strings.Repeat (conv.ToInt (math.Add .headings_level .level)) "#" -}}
  {{ printf "%s" $hash }}
{{- end -}}

{{- $h1 := tmpl.Exec "heading" (dict "headings_level" .headings_level "level" 0) -}}

This is a complete reference to the functionality of this library split into thematic sections.

{{ $h1 }} Basic alias definition

You can define aliases with the syntax `alias = [arg1, arg2, …]`, where args may be identifiers, string literals,
integers, underscores, or any arbitrary sequences of tokens (like `&'static str`):
```rust
{{ file.Read "snippets/basic.rs" -}}
```

{{ $h1 }} Alias reuse

Aliases could also be reused in definitions of other aliases:
```rust
{{ file.Read "snippets/alias_reuse.rs" -}}
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

{{ $h1 }} String formatting

Aliases could be used in string formatting with `%alias%` syntax. This is useful for generating doc-attributes:
```rust
{{ file.Read "snippets/string_formatting.rs" -}}
```

{{ $h1 }} Generating unique identifiers

`hash()` function deterministically hashes the input _within a single macro invocation_. It means that within the same
`compose_idents!` call `hash(foobar)` will always produce the same output. But in another call - the output would be
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

{{ $h1 }} Functions

| Function            | Description                                                          |
|---------------------|----------------------------------------------------------------------|
| `upper(arg)`        | Converts the `arg` to upper case.                                    |
| `lower(arg)`        | Converts the `arg` to lower case.                                    |
| `snake_case(arg)`   | Converts the `arg` to snake_case.                                    |
| `camel_case(arg)`   | Converts the `arg` to camelCase.                                     |
| `pascal_case(arg)`  | Converts the `arg` to PascalCase.                                    |
| `normalize(tokens)` | Transforms a free-form sequence of `tokens` into a valid identifier. |
| `hash(arg)`         | Hashes the `arg` deterministically within a single macro invocation. |
