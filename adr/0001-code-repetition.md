---
id: 0001
status: accepted
date: 2025-08-31
parent: "0000"
---

# Code Repetition

## Context and Problem Statement

Currently, `compose_idents!` is often combined with `macro_rules!` to generate multiple code instances. This is
cumbersome and reduces readability.
The goal is to introduce a built‑in capability to generate multiple variations of the provided code directly in the
library.

The key challenge is in type resolution when repetition is driven by user-provided values of different types:

  - How to resolve type coercion of function arguments for each generated variant?
  - How to resolve function overloading for each generated variant?
  - How to resolve type coercion of reused aliases for each generated variant?

## Decision Drivers

1. Functional completeness.

   Code repetition is the primary goal:

     - Substitutions with values of different types should be supported for each code variant. For example, if the
       return type of a function is substituted across variants, users should be able to supply values of different
       types (e.g., `u32`, `Option<u32>`) for different variants.
     - Nested repetition specifications should be supported, with Cartesian product semantics.

   At the same time, it should not interfere with existing features:

     - Function overload resolution should be performed **per iteration**, according to the alias's type in that
       iteration.
     - Function calls and value coercion should also be resolved **per iteration**.
     - Coercion of reused aliases should also be resolved **per iteration**.

2. Simplicity of syntax.

   Keep the syntax simple and minimal.
3. Backward compatibility.

   Existing code should continue to work without changes.
4. Debuggability.

   Maintain the current approach of providing precise, verbose error messages.
5. Extensibility.

   The design should be open to extending code repetition with specialized functions such as `zip()`, `enumerate()`,
   and `map()`.

## Considered Options

1. Internal representation and analysis
   1. AST expansion (desugaring).

      Add a pass after parsing and before name resolution that takes the high‑level AST produced by the parsing phase
      and rewrites it into an intermediate AST without repetition constructs, replacing them with multiple regular
      substitution constructs.

      The expand phase should preserve original spans and alias names so desugaring does not interfere with
      debuggability.

      Pros:
        - Simplicity of implementation - the existing type resolution and evaluation logic can be reused with
          minimal changes.
        - Expansion-phase could be used for future features that would require new high-level syntactic constructs.
      Cons:
        - Does not handle constructs more complex than static arrays (e.g., transformations or function calls that
          produce variation data, or non‑static variation data).

   2. Generic polymorphism.

      Perform type resolution statically during the resolve phase and treat a heterogeneous
      sequence of values as a generic sequence (similar to Haskell's `HList`).

      1. Monomorphization.

         This approach comes down to emitting versions of the code specialized for each combination of generic arguments
         passed to the generic functions or to syntactic constructs.

         Pros:
           - Avoids indirection and can enable better performance.
         Cons:
           - More compatible with a bytecode/VM execution model than AST walking due to simpler code generation.
           - Larger memory overhead and potential code bloat.

      2. Witness-passing.

         Preserve a single body of generic code, but generate hidden auxiliary data structures that carry type‑resolution
         information. The resolve phase would generate these auxiliary data used by the eval phase.

         Pros:
           - Fits well with the existing AST‑metadata infrastructure and AST‑walking execution model.
           - More memory‑efficient.
         Cons:
           - More indirection and potential performance overhead.

   3. Dynamic type resolution.

      Move all type resolution to the eval phase, effectively making the language dynamically typed.

      Pros:
        - Much simpler on the implementation side.
      Cons:
        - Steers the project toward a dynamically typed language, which was not the original intent.
        - Makes errors harder to debug as the syntax grows more complex.
        - Hurts performance.

2. Syntax

   1. `for ... in ...`.

      Allow an optional header with a for‑in–like syntax:
      ```
      for (foo, bar) in [(42, u32), (None, Option<u32>)]

      fn_name = concat(my, _, fn, _, normalize(bar)),
      {
        pub fn fn_name() -> bar {
          foo
        }
      }
      ```

      Pros:
        - Familiar syntax.
        - Clear semantics.
        - Aesthetic.
      Cons:
        - Would require introducing an entirely new syntactic construct.

   2. `iter()` function.

      Provide a built‑in function `iter()` that takes a list of values and generates as many code variants as there are
      values in the list. Multiple invocations produce a Cartesian product:
      ```
      return_type = iter([u32, Option<u32>]),
      value = iter([42, None]),
      fn_name = concat(my, _, fn, _, normalize(return_type)),
      {
        pub fn fn_name() -> return_type {
          value
        }
      }
      ```

## Decision Outcome

1.1 Internal representation and analysis: AST expansion (desugaring) — Tentatively accepted. It is the simplest
    option and covers the basic repetition use case. However, it cannot handle more complex scenarios where functions
    are applied to input arrays or variation data is generated from function calls. This is an ad‑hoc, temporary
    solution.
1.2 Generic polymorphism — Tentatively rejected. This approach is promising but requires significantly more effort, so
    it is deferred to the backlog for now.
1.3 Dynamic type resolution — Rejected. It goes against the original intent and makes errors harder to debug.
2.1 Syntax: `for ... in ...` — Accepted. It is familiar and aesthetic, and makes it clearer what the macro does and
    what results to expect.

Other options are rejected.

## Consequences

1. Functional completeness.

   Provides first‑class code repetition via a Rust‑like `for ... in ...` header, with nested loops (Cartesian product
   semantics).
   Type coercion, function overloading, and alias reuse are resolved per iteration; each generated variant is analyzed
   independently.
   Desugaring intentionally limits sources to static, user‑specified sequences. Applying functions to produce variation
   data, or generating variation data from function calls, is not supported at this time.

2. Simplicity of syntax.

   The loop header keeps the surface syntax readable and obvious, avoiding delimiter-heavy constructs.

3. Backwards compatibility.

   Existing macro invocations without a loop header continue to work unchanged.

4. Debuggability.

   Desugaring preserves original spans and does not interfere with error messages.

5. Extensibility.

   The chosen desugaring approach is not very extensible; extensibility was traded for implementation simplicity.

## More Information

There is a popular crate with similar functionality: [duplicate](https://crates.io/crates/duplicate)
