---
id: 0000
status: accepted
date: `2025-07-12`
parent: null
---

# Type System

## Context and Problem Statement

Currently `compose_idents!` macro is only able to handle substitution of identifiers with other identifiers (and not,
say, types, literals, etc). It limits applicability of the library in its current state and the planned code-repetition
feature.

In the current design aliases have only one type - ident. It implies that the value assigned to them must be a valid
identifier or convert to it, so something like `Result<u32, String>` would not be a valid value. But it's also clear
that it might be useful to be able to also do substitutions with types or with literals and so on.

There are several challenges:

1. Type system design - should it be strongly or weakly typed, dynamically or statically typed, unityped,
   optionally typed, etc?

   If non-unityped - what should be the types? Especially in light of the fact that idents, paths and types have large
   intersection in their possible values.
2. Parsing of complex values like `Result<u32, String>` and handling ambiguities of the syntax:
     - The comma that is both separator and part of the value.
     - `Result<u32, String>` could be parsed as both `syn::Type` and `syn::Path` without the context.
3. Resolving ambiguities during substitution - how to handle cases where a string literal is used as a function name or
   trait name, etc?
4. Substituting simple idents with multi-token values.

   If there is an `Ident` in a `TokenStream`, we may want to substitute it with another `Ident` or with a sequence of
   tokens (like `Result<u32, String>`). This could be easily done while working with a raw `TokenStream` directly,
   but then we would not have access to the contextual information (are we processing a function definition or a
   variable assignment, etc). Or it could be done inside a `syn`-based visitor, but then if we use
   `VisitMut::visit_ident_mut()` it would be impossible to substitute an `Ident` with a something other than another
   `Ident`. Or we would have to exhaustively implement every single `VisitMut` methods and code handling of every single
   language construct separately.
5. How to provide error messages with precise pointers to the causes of the errors in the user input?

   For example - single-pass substitution via sequential scan of a `TokenStream` wouldn't allow that, because it would
   make all the substitutions in one go, and then it would be up to the compiler to recognize the error, and it would
   print it as a resulting expansion of the macro with all the substitutions, and it would point to a syntax error in
   it without any references to the original user-input or the original code-block. It would be confusing and hard to
   debug.
6. Backwards compatibility - namely handling of string literals - what if an alias for function-name specified as a
   string literal?

## Decision Drivers

1. Functional completeness.

   The primary driver for this feature is desire to allow user to use library for more syntactic constructs,
   namely:
   1. Paths.
   2. Types.
   3. String literals.
   4. Integer literals.
   5. Identifiers.
   6. Raw tokens.
2. Simplicity of syntax.

   It is highly desirable to keep the syntax simple and minimal.
3. Backwards compatibility.

   It's generally not desirable to surprise the users with breaking API changes. Even in pre-1.0 versions.
4. Debuggability.

   Error messages should be non-generic and point to the specific place in the user input that caused the error.
5. Extensibility.

   How to avoid relying on ad-hoc solutions and maintain consistent architecture that provides
   obvious ways of adding new functionality in the future?
6. Performance - right now performance is not at a very high priority, but it is expected that at some point it will become
   more important. And it is desirable to not make early decisions that would block achieving better performance in the future.


## Considered Options

1. Type system design.
   1. Static typing.
      1. Static typing with explicit types.

         Pros:
           - Would be easier on the implementation side - there would be no need to implement type inference, coercion,
             casting.
         Cons:
           - Would introduce more syntactic clutter.
           - Would add to the mental overhead for the user - the primary purpose of the library for the user is
             substitution of identifiers by name. Adding explicit types doesn't help this purpose.
      2. Static typing with type inference.

         Type inference would save the user from the need of explicitly specifying types but would require implementing
         either introduction of new delimiter-heavy syntax or speculative parsing.

         Pros:
           - Less delimiter-heavy syntax, less mental overhead for the user.
         Cons:
           - Would require implementing speculative parsing.
   2. Dynamic typing.
   3. Optional typing.
   4. Weak typing.

      Weak typing means that the library would try to coerce any type to any type. For example literal strings into
      idents or integers, etc.

      Pros:
        - Would allow to avoid explicit casting or type specification.
      Cons:
        - Deviates from Rust's philosophy.
        - Implementation complexity.
        - It might be excessive - there is really no practical need to coerce between strings and idents or
          paths and strings.
   5. Strong typing.
      1. Strict strong typing.

         Strong typing means no silent conversions while giving user a way to explicitly convert types using casting
         functions (like `to_ident()`, `to_path()`, etc).

         Pros:
           - It's more explicit and consistent with Rust's philosophy.
           - It would be easier on the implementation side - no need to implement type coercion.
         Cons:
           - It would require user to explicitly cast types.
           - Would require introduction of special functions for type casting.
      2. Strong typing with restricted coercion.

         The idea is to keep strong typing in general but allow coercion in some cases - namely conversion between
         idents, paths, and types - types that have large intersection in their possible values. It applies to both
         function calls in alias definitions and also substitutions.

         For other cases of type conversion explicit casting would be used.

         Pros:
           - It's a sweet spot in terms of mental overhead and implementation complexity. User won't have to worry about
             type conversions that are obvious anyway (like ident to path, etc) and it won't be necessary to implement
             conversions for useless cases (like string to ident, etc).
         Cons:
           - User still would be required to use explicit casts in some cases.
   6. Unityped system.
2. Parsing.
   1. Introduction of new delimiter-heavy syntax.
   2. Speculative parsing.

      The idea is to attempt to parse complex values like `Result<u32, String>` as multiple different types and then
      chose based on priority and which attempt consumed the most tokens.

      The type interpretation priority (given equal count of consumed tokens) would be:
        - `Ident` - the most common type, unambiguous and "atomic".
        - `Path` - a composite type that is considered to be more "basic" compared to `Type` - it involves idents
                   separated by `::` with optional turbofish syntax for generic parameters.
        - `Type` - another composite type, which is more complex (includes tuple-types, array-types, slices,
                   impl Trait, and other forms).
        - `Expr` - even more complex type (includes function calls, control flow expressions, and many more).
        - `LitStr` - another atomic type.
        - `LitInt` - third atomic type.
        - `TokenStream` - a fallback type that is used when nothing else matches. It matches arbitrary tokens except
                          a comma which is used as an unconditional and unambiguous delimiter in this case.

      There might be ambiguous cases. For example every `Ident` is also a valid `Path`. And `Path` has a large
      intersection with `Type`. The answer to this is that it doesn't matter - the substitution is performed
      on `TokenStream` and `TokenTree` level anyway. What is important is disambiguation of arguments and delimiters
      without introducing delimiter-heavy syntax - this is the main purpose of speculative parsing.
3. Substitution.
   1. Simple substitution via a sequential scan of the `TokenStream`.

      Pros:
        - Very simple to implement.
        - Solves the problem of substituting simple idents with multiple tokens.
        - Delegates the problem of type coercion to the compiler - which is very good.
        - Doesn't require any notion of value type - all the work is done with low-level tokens represented as
          `TokenStream`.
      Cons:
        - While it delegates type coercion to the compiler, also delegates error reporting to the compiler,
          and the compiler wouldn't be able to provide detailed error message and reference the original user input and
          the original code-block that caused the error.

      1. Single substitution-reparse cycle per each alias-ident pair.

         The idea is to perform a seq. scan of the `TokenStream` and for each `Ident` that matches an alias, and for
         each match perform a complete reparsing of the `TokenStream` and in case of the error - use the span of the
         last substituted `Ident` from the code-block to create the error as well as the spans of the input values
         from the alias definition.

         Pros:
           - Would allow to provide rich and precise error messages with references to the original user input.
           - All the pros of the simple sequential scan.
         Cons:
           - Would imply `M * N` reparsing cycles, where `M` is the number of aliases and `N` is the number of
             matching `Ident`s in the `TokenStream`. In practice, it wouldn't be prohibitively slow, but still not
             the most efficient solution and not the most desirable one.

             1. Incremental editing via `syn`-based visitor.

                It's an optimization of the above idea. Everything stays the same, but happens within individual
                non-terminals (like function definitions, trait definitions, etc) - this way each reparsing cycle
                would only have to be performed on a small chunk of the input code block and not the whole of it.

                It is possible to implement a `syn`-based visitor that would recursively traverse the AST and for select
                non-terminals (functions, structs, traits, etc) would perform substitutions on non-recursive
                parts of the non-terminal (attributes, visibility, parameters, etc), while reparsing these parts for
                validation in one-by-one fashion, thus achieving incremental editing, and then recurse down the recursive
                part of the non-terminal (usually - the code-block) starting the process again for each encountered
                non-terminal. Those non-terminals for which recursive processing is not implemented would be treated as
                non-recursive and processed one whole (similar to how non-recursive parts of recursive AST nodes are
                processed - attributes, parameters, etc).

                The algorithm would consist of two parts:
                  A. Recursive incremental processing of a single non-recursive AST-node at token-level. The node is
                     converted to a `TokenStream`, then recursively traversed while performing substitutions.
                     `TokenStream` is a recursive data-structure since it contains nested groups (`TokenTree::Group`) -
                     hence the recursive traversal.
                  B. Recursive incremental processing of `syn`-AST nodes at AST level.

                The algorithm A. would look like this:
                  1. Take the alias-definitions and an AST-node as input.
                  2. Convert the AST-node to `TokenStream`.
                  3. Recursively walk through the `TokenStream`:
                     1. When an `Ident` token is encountered, and it matches one of the aliases - store its `Span` and
                        substitute it.
                     2. Parse the whole resulting `TokenStream` back into the original AST-node type.
                     3. If parsing fails - return an error with the stored span of the substituted `Ident`.
                        This way - we achieve precise error reporting with the reference to the exact element which
                        replacement caused the error.
                     4. If parsing succeeds - return to step 3 (continue the recursive walk).

                And the algorithm B. would look like this:
                  1. Take the alias-definitions and the code-block (`syn::Block` - essentially a sequence of statements)
                     where the substitutions are to be performed.
                  2. Recursively walk through the input code-block:
                     1. Iterate over the statements in the code-block.
                     2. For each statement - check whether it is a recursive AST-node or a non-recursive one.
                     3. If it is a non-recursive AST-node - process it with the algorithm A. Recursive iteration with
                        incremental substitution and validation - achieves incremental editing.
                     4. If it is a recursive AST-node:
                        1. Process its non-recursive sub-nodes (attributes, visibility, parameters, etc) with the
                           algorithm A.
                        2. Recurse down the recursive part of the AST-node (the code-block) and process it with the
                           algorithm B (starting with the step 2 - code block processing).

                The more AST nodes the algorithm B. recognizes as recursive - the more incremental and efficient the
                algorithm would be because it would be able to split the problem into smaller parts and decrease the
                overhead of the validation step.

                A similar approach would be to first - construct a generic tree where each node is of a single universal
                AST-node type, and allows access to raw tokens, `syn`-defined struct that represents these tokens as a
                higher-level non-terminal and also child nodes. Then this generic tree could be used to incrementally edit
                the AST via manipulating low-level tokens and then checking if they parse as high-level `syn` constructs.
                It's a bit redundant compared to the above approach, but it's a proven concept, and it's described here for
                historic purposes and for the case if it is needed in some future case.

                Pros:
                  - All the pros of the above approaches.
                  - Would be more performant than the above - because it would only reparse small chunks of the input code
                    block, and not the whole block at each substitution. The time complexity would be reduced from O(M*N)
                    full-block parses to O(M*N) of small-chunk parses, where chunks are typically 10-50 tokens vs 1000+
                    tokens for the full block.
                  - Would allow to avoid crafting an exhaustive implementation of a parser (`syn`-based or otherwise) and
                    instead would allow to just override hooks (in `syn::visit_mut::VisitMut`) for select non-terminals.
                Cons:
                  - Performance benefits would depend on granularity of processing.

   2. `syn`-based visitor with intelligent type coercion.

      The idea is to create a visitor implementation (via one of the `syn` visitor traits) that would intelligently
      convert the type of the alias depending on the context. For example, if the alias is used in a function
      definition as the name, it would try to coerce the alias to an ident. And if the alias is used in some
      other context - it would just use it as is.

      A challenge here is how to handle both cases - of substitution with intelligent coercion based on the context
      and simple substitution while handling potential substitution of idents with multiple tokens.

      One solution here is to have two-pass processing with a unique ID for each `Ident` via simple counting.

      `syn`-visitors visit idents in lexical order, it means that counter-based IDs would remain the same in
      both visitor-based traversals and sequential scans.

      This way - a visitor could record a coerced replacement into a map keyed by ident-ID. And during the second pass,
      done via a sequential scan, it could perform actual substitutions using both the map and simple replacement in
      case if the current `Ident` is not in the map.

   3. Textual substitution.

      Here the idea is to perform substitutions in the textual representation of the input `TokenStream` using the
      information from the respective `Span`s.

   4. Using `tree-sitter`.

      Pros:
        - `tree-sitter` represents AST using generic nodes that can be incrementally updated with nodes of arbitrary
          other types.
      Cons:
        - `tree-sitter` is somewhat heavy on dependencies and compile-time side. Its latest `docs.rs` build took 0.96s.
          It requires C-toolchain for the build and depends on 4 non-optional libraries.

   5. Using a parser-generator (similar to [LALRPOP][1]) to craft a custom Rust-parser that would be more suitable
      for incremental updating.

      Rust-analyzer has a formal grammar specification for Rust that could be used for this:
      https://github.com/rust-analyzer/ungrammar/blob/53bc777c2400acea2ee576123c9232d50731563c/rust.ungram
4. Function overloading.
   1. No function overloading.

      Each function would have a unique name and accept specific types. Users would need to call different functions
      for different type combinations (e.g., `concat_idents()`, `concat_str()`, `concat_tokens()`).

      Pros:
        - Simplicity of implementation - no need for complex function type resolution logic.
        - No ambiguity in function calls.
      Cons:
        - Verbose and ugly syntax.

   2. Function overloading with speculative signature resolution via cost-function optimization.

      Functions can have multiple signatures with different parameter types. The system automatically selects
      the best matching signature based on minimizing cost function that takes into account cost of coercing arguments
      and the output value. The candidate signatures are selected by resolving the subtree of the AST for multiple
      available signatures and filtering out those that don't match.

      Pros:
        - Clean, intuitive syntax - users can use the same function name regardless of types.
        - Leverages the type system and coercion rules.
        - Allows for gradual addition of new overloads.
      Cons:
        - More complex implementation - requires function resolution algorithm.

5. AST metadata storage.

   Introduction of any kind of type system would require storing metadata for particular AST nodes.

   1. In-node metadata storage.

      Embed metadata directly within AST nodes.

      Pros:
        - No introduction of new entities into the project.
        - Simple access pattern - metadata travels with the node.
        - No need for separate lookup mechanisms.
      Cons:
        - Would require making AST nodes mutable, wrapping them in `RefCell`.
        - Would require transitioning between `Rc<RefCell<T: Ast + Clone>>` and `&mut dyn Ast + Clone` to enable
          speculative type resolution (needed for 4.2). `dyn Clone` is not possible directly (since `Clone` isn't
          object-safe), so it would require Clone-Box pattern implementation.
        - Cloning a subtree would imply recursive pointer chasing and cloning - which is not efficient in terms of memory
          access patterns.

   2. Side-tables linked to the AST via Node IDs.

      Store metadata in separate maps or similar structures, linked to AST nodes via IDs unique to each node.

      Pros:
        - Much simpler way of indirection - no need for borrowing, converting to dyn-object, etc.
        - Better performance - less intermediate steps for de-referencing and subtree cloning could be achieved by cloning
          the metadata store data-structure (in the simplest implementation case) - it can be done in one shot which is more
          efficient. Another benefit is that all mutable operations are done on metadata only - non cloning or passing around
          AST data (when working with metadata).
        - Further more - opens up a possibility of using copy-on-write pattern for more efficient snapshotting.
        - AST remains immutable and read-only.
        - Arguably - more idiomatic approach:
          - Index-based indirection is used in graph-processing [3].
          - Authoritative "Crafting Interpreters" book refers to this pattern as "side tables" [2].
      Cons:
        - Requires AST nodes to have unique IDs.
        - Requires adding new entities specific to metadata management.
        - Making metadata-snapshot (in the simplest implementation case) clones all the metadata, not just the relevant parts.

6. Backwards compatibility.

   On one hand breaking backwards compatibility is not desirable, but on the other hand - silent conversion has been
   introduced at the time when the library meant to only replace idents with other idents. Now, when other types are
   going to be introduced - there is a dilemma - either to expand coercion or to completely drop it.

   1. Keep coercion of string literals to idents.

      Pros:
        - Least surprising for the user.
        - Would allow the user to mix the types without caring about explicit casting.
      Cons:
        - Contradictory to Rust's design philosophy.
        - It's not clear if there are actual practical cases for that - why would anyone want to pass a string literal
          and use it as a function name, or something like that?
        - It's not possible to keep backwards-compatibility in all cases. Previously, the library converted arguments
          of all types into idents. Now, when library became meant to operate on values of other types - ambiguous
          cases would arise, for example - what if user would want to substitute an RHS value with a string literal?
          The old behavior would be to convert it to an ident. But now it doesn't make sense - so there is inherent
          breakage of backwards compatibility anyway.

   2. Drop coercion of string literals to idents and introduce casting functions.

      Pros:
        - More explicit.
        - More consistent with Rust's design philosophy.
        - Easier to implement.
      Cons:
        - Breakage of backwards compatibility, surprising new behavior.

## Decision Outcome

1.1.2 Static typing with type inference - Accepted. Why static - for now requirements for the type system are simple
  enough for resolving types statically during parsing. Why type inference - it would allow user to not specify types,
  and reduce mental overhead, syntax clutter.
1.5.2 Strong typing with restricted coercion - Accepted. It provides the best balance between simplicity of syntax,
  mental overhead on the user's end and implementation complexity. It allows to coerce between `Ident`, `Path`, `Type`
  types that have large intersection without forcing user to explicitly specify types, which is another decision that
  was made - keep the syntax simple and mental overhead of the user minimal. On the other hand it doesn't attempt to
  coerce between all types - it saves from implementation complexity for useless cases.
1.3 Optional typing - I tentatively reject this option, but leave the door open for implementing it in the future.
2.2 Speculative parsing - Accepted. It's a bit complex approach, but it allows to keep the syntax minimal while
  parsing multiple different types separated by only a comma.
3.1.1.1 Incremental editing via `syn`-based visitor - Accepted. It's the simplest solution that allows to
        provide rich and precise error messages, and it's also faster than simple sequential scan with reparsing for
        each alias-ident pair. And it also allows to avoid implementing an exhaustive parser.
4.2 Function overloading - Accepted. It fulfills the requirement of achieving more minimalist and pleasant syntax (by avoiding having
    many different function names for different type combinations).
5.2 Side-tables linked to the AST via Node IDs - Accepted. It's a better, simpler, faster approach.
6.2 Drop coercion of string literals to idents and introduce casting functions - Accepted. Backwards compatibility would
    have been broken in any case, and explicit typing is more consistent with Rust's design philosophy.

### Consequences

1. Functional completeness.

   The library would be able to provide support for many different types (including raw tokens).
2. Simplicity of syntax.

   User would be able to work with the library without need to learn complex syntax or think about types too much.
   Function overloading further reduces cognitive load by allowing user to just call the function and letting the type system
   automatically find the suitable variant.
3. Backwards compatibility.

   Backwards compatibility would be broken in the case where string literals were expected to be coerced to idents.
4. Debuggability.

   Precise and rich error messages would be achieved through careful span tracking and the incremental editing approach.
5. Extensibility.

   Immutable AST, metadata side-tables, speculative type resolution enclosed within the resolve-pass - all these decisions
   provide universal (as opposed to ad-hoc) architecture that allows to build on top of it in the future.
6. Performance.

   Immutable AST and snapshottable side-tables are clearly the winners in terms of performance (among simple solutions).

## More Information

### Useful sources

- ["Crafting Interpreters" book by Robert Nystrom][2] - was very helpful for designing this and other features.


[1]: https://github.com/lalrpop/lalrpop
[2]: https://craftinginterpreters.com/resolving-and-binding.html
[3]: https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
