- Contextual Inference
  - Pros:
    - Relives the user of the burden of thinking about types.
    - Simplifies the syntax.
    - Naturally - in the context of library's use-cases the user cares what to substitute and
      where and he can specify it by placing an alias in the right place in right context.
      That alone provides enough information to perform substitution - adding types to aliases won't
      add anything, because the type of a substituted token is already known from the context.
    - `syn` crate has `VisitMut` trait that allows to traverse syntax tree and manipulate it
      in hygienic manner - using it it's possible to traverse the syntax tree and, compare tokens
      by their stringified value with the substitution, and then attempt to substitute it with the alias
      definition and if this syntactic element rejects the substitution - it would be possible to raise a compile
      error and point the user to the exact place in his code where the substitution was used in wrong context.
  - Cons:
    - It would imply backwards-incompatible changes to the library. Currently, it's possible to specify
      aliases as `"my_function"` - string literals. If we switch to contextual inference, it would mean that
      literal strings would have to be treated as literal strings.
      I'm not sure - but maybe it'd be possible to contextually recognize where to convert a string literal
      to an identifier - and where to leave it as a string literal...
    - How am I supposed to parse tuples `alias = (a, b)`? I have plans to add tuples into the future for-loop
      functionality: `for (a, b) in [(1, 2), (3, 4)] ...`. But what if user would want to actually use whole tuple
      for substitution?
- Optional typing
  - Pros:
    - Allows to specify types for aliases, which can be useful in some cases.
    - Can be used to provide additional information about the alias, such as its purpose or usage.
  - Cons:
    - Adds complexity to the syntax.
    - Requires the user to think about types, which may not be necessary in many cases.
    - May lead to confusion if the user specifies a type that is not compatible with the alias.
