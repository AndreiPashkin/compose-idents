use crate::ast::{TerminatedValue, Value};
use crate::util::terminated::Terminated;
use crate::util::token_distance::token_distance;
use proc_macro2::TokenStream;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::Token;

impl<Term: Parse> TerminatedValue<Term> {
    fn try_parse<'a, T: Parse>(input: &'a ParseStream) -> syn::Result<(usize, ParseBuffer<'a>, T)> {
        let fork = input.fork();
        let value = fork.parse::<T>()?;
        let term_fork = fork.fork();
        if !term_fork.is_empty() {
            let _ = term_fork.parse::<Term>()?;
        }
        let num_tokens = token_distance(&input.cursor(), &fork.cursor());
        Ok((num_tokens, fork, value))
    }
}

/// Parses the argument intelligently determining its type.
///
/// Tries to parse different types of expressions and chooses the one that consumes the most tokens.
/// In case of a failure falls back to raw tokens.
impl<Term: Parse> Parse for TerminatedValue<Term> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = Vec::new();
        let mut max_num_tokens = Option::<usize>::None;
        let mut value = Option::<(ParseBuffer, Value)>::None;

        match Self::try_parse::<syn::LitInt>(&input) {
            Ok((num_tokens, fork, int)) => {
                max_num_tokens = max_num_tokens
                    .map(|n| n.max(num_tokens))
                    .or(Some(num_tokens));
                value = Some((fork, Value::from_lit_int(int)));
            }
            Err(err) => errors.push(err),
        }
        match Self::try_parse::<syn::LitStr>(&input) {
            Ok((num_tokens, fork, lit_str)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_lit_str(lit_str)));
                }
            }
            Err(err) => errors.push(err),
        }
        match Self::try_parse::<Token![_]>(&input) {
            Ok((num_tokens, fork, underscore)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_ident(underscore.into())));
                }
            }
            Err(err) => errors.push(err),
        }
        match Self::try_parse::<syn::Ident>(&input) {
            Ok((num_tokens, fork, ident)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_ident(ident)));
                }
            }
            Err(err) => errors.push(err),
        }
        match Self::try_parse::<syn::Path>(&input) {
            Ok((num_tokens, fork, path)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_path(path)));
                }
            }
            Err(err) => errors.push(err),
        }
        match Self::try_parse::<syn::Type>(&input) {
            Ok((num_tokens, fork, type_)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_type(type_)));
                }
            }
            Err(err) => errors.push(err),
        }
        #[allow(unused_assignments)]
        match Self::try_parse::<syn::Expr>(&input) {
            Ok((num_tokens, fork, expr)) => {
                if max_num_tokens.is_none() || num_tokens > max_num_tokens.unwrap() {
                    max_num_tokens = Some(num_tokens);
                    value = Some((fork, Value::from_expr(expr)));
                }
            }
            Err(err) => errors.push(err),
        }

        if value.is_none() {
            // Falling back to tokens if no other types have been matched.
            let fork = input.fork();
            match fork.parse::<Terminated<TokenStream, Term>>() {
                Ok(terminated) => {
                    let tokens = terminated.into_value();
                    value = Some((fork, Value::from_tokens(tokens)));
                }
                Err(err) => errors.push(err),
            }
        }
        let Some((fork, arg)) = value else {
            unreachable!()
        };

        input.advance_to(&fork);
        Ok(Self::new(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Type;
    use crate::util::token_stream::TokenStreamExt;
    use quote::ToTokens;
    use rstest::rstest;
    use syn::parse::Parser;
    use syn::Token;

    #[rstest]
    // Ident.
    #[case::ident_simple("foo, 42", "foo", Type::Ident, ", 42")]
    #[case::ident_type("u32, 42", "u32", Type::Ident, ", 42")]
    #[case::ident_underscore("_, 42", "_", Type::Ident, ", 42")]
    #[case::ident_camel_case("FooBar, 42", "FooBar", Type::Ident, ", 42")]
    #[case::ident_with_digits("foo123, 42", "foo123", Type::Ident, ", 42")]
    #[case::ident_leading_underscore("_foo, 42", "_foo", Type::Ident, ", 42")]
    #[case::ident_double_underscore("__, 42", "__", Type::Ident, ", 42")]
    #[case::ident_snake_mixed("foo_bar_123, 42", "foo_bar_123", Type::Ident, ", 42")]
    #[case::ident_raw_keyword("r#type, 42", "r#type", Type::Ident, ", 42")]
    #[case::ident_option_plain("Option, 42", "Option", Type::Ident, ", 42")]
    // Path cases.
    #[case::path_simple("foo::bar, 42", "foo :: bar", Type::Path, ", 42")]
    #[case::path_type_generic(
        "Result<i32, String>, 42",
        "Result < i32 , String >",
        Type::Path,
        ", 42"
    )]
    #[case::path_type_generic_turbofish("Vec::<i32>, 42", "Vec :: < i32 >", Type::Path, ", 42")]
    #[case::path_type_generic_lifetime(
        "std::slice::Iter<'a, T>, 42",
        "std :: slice :: Iter < 'a , T >",
        Type::Path,
        ", 42"
    )]
    #[case::path_generic_nested(
        "a::b::C<D<E<F>>, G<H>>, 42",
        "a :: b :: C < D < E < F > > , G < H > >",
        Type::Path,
        ", 42"
    )]
    #[case::path_root_crate("crate::foo, 42", "crate :: foo", Type::Path, ", 42")]
    #[case::path_expr(
        "Option::<i32>::None, 42",
        "Option :: < i32 > :: None",
        Type::Path,
        ", 42"
    )]
    #[case::path_raw_segments(
        "r#type::r#match::r#dyn, 42",
        "r#type :: r#match :: r#dyn",
        Type::Path,
        ", 42"
    )]
    // Type cases.
    #[case::type_array("[u8; 32], 42", "[u8 ; 32]", Type::Type, ", 42")]
    #[case::type_array_const_expr("[u8; 1 + 2], 42", "[u8 ; 1 + 2]", Type::Type, ", 42")]
    #[case::type_never("!, 42", "!", Type::Type, ", 42")]
    #[case::type_tuple("(u32, u32), 42", "(u32 , u32)", Type::Type, ", 42")]
    #[case::type_unit("(), 42", "()", Type::Type, ", 42")]
    #[case::type_paren_wrapped("(T), 42", "(T)", Type::Type, ", 42")]
    #[case::type_macro_simple("ty!(u32), 42", "ty ! (u32)", Type::Type, ", 42")]
    #[case::type_macro_nested_args(
        "ty!(Result<u8, E>), 42",
        "ty ! (Result < u8 , E >)",
        Type::Type,
        ", 42"
    )]
    #[case::type_ref("&'a mut T, 42", "& 'a mut T", Type::Type, ", 42")]
    #[case::type_fn("fn(i32) -> i32, 42", "fn (i32) -> i32", Type::Type, ", 42")]
    #[case::type_fn_zero_args("fn(), 42", "fn ()", Type::Type, ", 42")]
    #[case::type_fn_hrtb(
        "for<'a> fn(&'a str) -> &'a str, 42",
        "for < 'a > fn (& 'a str) -> & 'a str",
        Type::Type,
        ", 42"
    )]
    #[case::type_fn_extern_c(
        "extern \"C\" fn(i32) -> i32, 42",
        "extern \"C\" fn (i32) -> i32",
        Type::Type,
        ", 42"
    )]
    #[case::type_dyn_simple("dyn Display, 42", "dyn Display", Type::Type, ", 42")]
    #[case::type_dyn_bounds(
        "dyn Iterator<Item = u8> + Send + 'static, 42",
        "dyn Iterator < Item = u8 > + Send + 'static",
        Type::Type,
        ", 42"
    )]
    #[case::type_dyn_hrtb_fn(
        "dyn for<'a> Fn(&'a T) + Send, 42",
        "dyn for < 'a > Fn (& 'a T) + Send",
        Type::Type,
        ", 42"
    )]
    #[case::type_impl_trait(
        "impl Iterator<Item=u8>, 42",
        "impl Iterator < Item = u8 >",
        Type::Type,
        ", 42"
    )]
    #[case::type_impl_multiple_bounds(
        "impl Read + Write + 'a, 42",
        "impl Read + Write + 'a",
        Type::Type,
        ", 42"
    )]
    #[case::type_qual_path_assoc(
        "<T as Iterator>::Item, 42",
        "< T as Iterator > :: Item",
        Type::Type,
        ", 42"
    )]
    // Expr cases.
    #[case::expr_unary_not("!flag, 42", "! flag", Type::Expr, ", 42")]
    #[case::expr_binary("2 + 2, 42", "2 + 2", Type::Expr, ", 42")]
    #[case::expr_if_else(
        "if cond { 1 } else { 0 }, 42",
        "if cond { 1 } else { 0 }",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_if_let(
        "if let Some(x) = opt { x } else { 0 }, 42",
        "if let Some (x) = opt { x } else { 0 }",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_match(
        "match x { 0 => 1, _ => 2 }, 42",
        "match x { 0 => 1 , _ => 2 }",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_block("{ let x = 1; x }, 42", "{ let x = 1 ; x }", Type::Expr, ", 42")]
    #[case::expr_async_block("async { 1 + 2 }, 42", "async { 1 + 2 }", Type::Expr, ", 42")]
    #[case::expr_unsafe_block(
        "unsafe { do_something() }, 42",
        "unsafe { do_something () }",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_call_with_args("foo(1, 2), 42", "foo (1 , 2)", Type::Expr, ", 42")]
    #[case::expr_method_call("obj.method(1, 2), 42", "obj . method (1 , 2)", Type::Expr, ", 42")]
    #[case::expr_method_call_turbofish(
        "obj.method::<T>(x), 42",
        "obj . method :: < T > (x)",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_await_call(
        "make_future().await, 42",
        "make_future () . await",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_array_simple("[1, 2, 3], 42", "[1 , 2 , 3]", Type::Expr, ", 42")]
    #[case::expr_array_repeat("[0; N], 42", "[0 ; N]", Type::Expr, ", 42")]
    #[case::expr_array_index("arr[0], 42", "arr [0]", Type::Expr, ", 42")]
    #[case::expr_tuple_struct_literal("Point(1, 2), 42", "Point (1 , 2)", Type::Expr, ", 42")]
    #[case::expr_field_access("s.field, 42", "s . field", Type::Expr, ", 42")]
    #[case::expr_closure("|| 42, 42", "| | 42", Type::Expr, ", 42")]
    #[case::expr_return("return 5, 42", "return 5", Type::Expr, ", 42")]
    #[case::expr_pathlike_1("foo::<i32>(), 42", "foo :: < i32 > ()", Type::Expr, ", 42")]
    #[case::expr_pathlike_2(
        "Option::<i32>::Some(1), 42",
        "Option :: < i32 > :: Some (1)",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_macro_chained_method(
        "vec![1, 2].len(), 42",
        "vec ! [1 , 2] . len ()",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_path_call_turbofish(
        "Option::<i32>::Some(1), 42",
        "Option :: < i32 > :: Some (1)",
        Type::Expr,
        ", 42"
    )]
    #[case::expr_trait_fn_call("Trait::method(x), 42", "Trait :: method (x)", Type::Expr, ", 42")]
    // LitInt cases.
    #[case::litint_dec("123, 42", "123", Type::LitInt, ", 42")]
    #[case::litint_underscore("1_000_000, 42", "1_000_000", Type::LitInt, ", 42")]
    #[case::litint_hex("0xFF, 42", "0xFF", Type::LitInt, ", 42")]
    #[case::litint_bin_suffix("0b1010_1010u8, 42", "0b1010_1010u8", Type::LitInt, ", 42")]
    #[case::litint_suffix_usize("0usize, 42", "0usize", Type::LitInt, ", 42")]
    // LitStr cases.
    #[case::litstr_simple("\"hello\", 42", "\"hello\"", Type::LitStr, ", 42")]
    #[case::litstr_escape("\"a \\\"quote\\\"\", 42", "\"a \\\"quote\\\"\"", Type::LitStr, ", 42")]
    #[case::litstr_unicode("\"😀\", 42", "\"😀\"", Type::LitStr, ", 42")]
    #[case::litstr_raw("r\"no escapes\", 42", "r\"no escapes\"", Type::LitStr, ", 42")]
    #[case::litstr_raw_hashes("r#\"he\"llo\"#, 42", "r#\"he\"llo\"#", Type::LitStr, ", 42")]
    // Tokens cases.
    #[case::tokens_simple(
        "pub fn foo() -> u32 { 42 }, 42",
        "pub fn foo () -> u32 { 42 }",
        Type::Tokens,
        ", 42"
    )]
    #[case::tokens_let_stmt("let x = 1;, 42", "let x = 1 ;", Type::Tokens, ", 42")]
    #[case::tokens_use_stmt(
        "use crate::path::Item;, 42",
        "use crate :: path :: Item ;",
        Type::Tokens,
        ", 42"
    )]
    #[case::tokens_keyword("fn, 42", "fn", Type::Tokens, ", 42")]
    #[case::tokens_attr_struct(
        "#[derive(Debug)] struct S;, 42",
        "# [derive (Debug)] struct S ;",
        Type::Tokens,
        ", 42"
    )]
    #[case::tokens_impl_block(
        "impl Trait for T {}, 42",
        "impl Trait for T { }",
        Type::Tokens,
        ", 42"
    )]
    #[case::tokens_where_clause("where T: Trait, 42", "where T : Trait", Type::Tokens, ", 42")]
    #[case::tokens_angle_params_one("<'a>, 42", "<'a >", Type::Tokens, ", 42")]
    fn terminated_value_parsing(
        #[case] input: &str,
        #[case] expected_value: &str,
        #[case] expected_type: Type,
        #[case] expected_rest: &str,
    ) {
        let parser =
            |input: ParseStream| -> syn::Result<(TerminatedValue<Token![,]>, TokenStream)> {
                Ok((
                    TerminatedValue::<Token![,]>::parse(input)?,
                    input.parse::<TokenStream>()?,
                ))
            };
        let result = parser.parse_str(input);
        assert!(result.is_ok(), "Failed to parse input: {:?}", result);
        let (terminated_value, actual_rest) = result.unwrap();
        let actual_value = terminated_value.into_value();

        assert_eq!(actual_value.to_token_stream().to_string(), expected_value);
        assert_eq!(actual_value.type_(), expected_type);
        assert_eq!(actual_rest.to_string(), expected_rest);
    }
}
