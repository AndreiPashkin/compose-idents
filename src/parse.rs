//! Implements parsing logic for different internal components.

use crate::ast::{
    Alias, AliasSpec, AliasSpecItem, AliasValue, Arg, ComposeIdentsArgs, Expr, Func, LoopAlias,
    LoopSourceValue, LoopSourceValueList, LoopSpec, LoopSpecItem, Tuple, TupleValue,
};
use crate::deprecation::DeprecationService;
use crate::error::combine_errors;
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::{Bracket, Paren};
use syn::{bracketed, parenthesized, Block, Ident, Token};

/// Wraps the token-type `T` and parses it by consuming the input until the terminator `Term` or
/// the end if the input.
struct Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    value: T,
    terminator_type: PhantomData<Term>,
}

impl<T, Term> Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    fn into_value(self) -> T {
        self.value
    }
}

impl<T, Term> Parse for Terminated<T, Term>
where
    T: Parse,
    Term: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = TokenStream::new();
        while !input.is_empty() {
            let fork = input.fork();
            let is_terminator = fork.parse::<Term>().is_ok();
            if is_terminator {
                break;
            }
            let tt = input.parse::<TokenTree>()?;
            tokens.extend(tt.into_token_stream());
        }

        let value = syn::parse2::<T>(tokens)?;

        Ok(Terminated {
            value,
            terminator_type: PhantomData,
        })
    }
}

/// Combines two syntactic elements into a single one and enables parsing them speculatively.
///
/// Useful for parsing multiple alternative kinds of terminators in one go.
pub enum Combined<A, B>
where
    A: Parse,
    B: Parse,
{
    A(A),
    B(B),
}

impl<A, B> Parse for Combined<A, B>
where
    A: Parse,
    B: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        eprintln!("Parsing Arg from input: {:?}", input);
        let span = input.span();
        let mut errors = Vec::new();
        let fork = input.fork();
        match fork.parse::<A>() {
            Ok(a) => return Ok(Self::A(a)),
            Err(err) => errors.push(err),
        }

        let fork = input.fork();
        match fork.parse::<B>() {
            Ok(b) => return Ok(Self::B(b)),
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Unable to parse any of the combined tokens (see the errors below)",
            span,
            errors,
        ))
    }
}

/// Combines tokens into a single one that has a speculative [`Parse`] implemented for it.
macro_rules! combine {
    ($A:ty, $B:ty) => {
        Combined::<$A, $B>
    };
    ($A:ty, $B:ty $(,$($T:ty),*)?) => {
        combine!(Combined::<$A, $B>, $($T)*)
    };
}
pub(crate) use combine;

/// Parses the argument until the end of the input.
impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_terminated = input.peek2(syn::parse::End);
        let span = input.span();
        let value: Arg;
        if input.peek(syn::Ident) && is_terminated {
            let ident = input.parse::<syn::Ident>()?;
            value = Arg::Ident(ident);
        } else if input.peek(Token![_]) && is_terminated {
            input.parse::<Token![_]>()?;
            value = Arg::Underscore(span);
        } else if input.peek(syn::LitStr) && is_terminated {
            let lit_str = input.parse::<syn::LitStr>()?;
            value = Arg::LitStr(span, lit_str.value());
        } else if input.peek(syn::LitInt) && is_terminated {
            let lit_int = input.parse::<syn::LitInt>()?;
            value = Arg::LitInt(span, lit_int.base10_parse::<u64>()?);
        } else {
            let tokens = input.parse::<TokenStream>()?;
            value = Arg::Tokens(span, tokens);
        }
        eprintln!("Parsed Arg: {:?}", value);
        Ok(value)
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let func_name = ident.to_string();
        let raw_args;
        parenthesized!(raw_args in input);
        let args_span = raw_args.span();
        let punctuated = raw_args
            .fork()
            .parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,]);
        let args = match punctuated {
            Ok(punctuated) => Some(
                punctuated
                    .into_iter()
                    .map(|arg| arg.into_value())
                    .collect::<Vec<_>>(),
            ),
            Err(_) => None,
        };
        let tokens = raw_args
            .parse::<TokenStream>()
            .ok()
            .map(|tokens| Expr::ArgExpr(Box::new(Arg::Tokens(args_span, tokens))));

        match (func_name.as_str(), args.as_deref(), tokens) {
            ("upper", Some(args), _) => match &args {
                [expr] => Ok(Func::Upper(expr.clone())),
                _ => Ok(Func::SignatureMismatch("upper(arg)".to_string())),
            },
            ("lower", Some(args), _) => match &args {
                [expr] => Ok(Func::Lower(expr.clone())),
                _ => Ok(Func::SignatureMismatch("lower(arg)".to_string())),
            },
            ("snake_case", Some(args), _) => match &args {
                [expr] => Ok(Func::SnakeCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("snake_case(arg)".to_string())),
            },
            ("camel_case", Some(args), _) => match &args {
                [expr] => Ok(Func::CamelCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("camel_case(arg)".to_string())),
            },
            ("pascal_case", Some(args), _) => match &args {
                [expr] => Ok(Func::PascalCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("pascal_case(arg)".to_string())),
            },
            ("hash", Some(args), _) => match &args {
                [expr] => Ok(Func::Hash(expr.clone())),
                _ => Ok(Func::SignatureMismatch("hash(arg)".to_string())),
            },
            ("normalize", _, Some(tokens)) => Ok(Func::Normalize(tokens.clone())),
            ("concat", Some(args), _) if !args.is_empty() => Ok(Func::Concat(args.to_vec())),
            ("concat", _, _) => Ok(Func::SignatureMismatch(
                "concat(arg1, arg2, ...)".to_string(),
            )),
            _ => Ok(Func::Undefined),
        }
    }
}

/// Just like impl of [`Parse`] for [`Arg`] - parses the input either until the end or a comma.
impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();
        let fork = input.fork();
        let span = input.span();

        match fork.parse::<Func>() {
            Ok(func) => {
                match func {
                    Func::Undefined => {
                        return Err(syn::Error::new(
                            span,
                            "Matching function has not been found",
                        ))
                    }
                    Func::SignatureMismatch(err) => {
                        return Err(syn::Error::new(
                            span,
                            format!(
                                r#"Type constraints for function "{}" are not satisfied"#,
                                err,
                            ),
                        ));
                    }
                    _ => {}
                }
                input.advance_to(&fork);
                return Ok(Expr::FuncCallExpr(Box::new(func)));
            }
            Err(err) => errors.push(err),
        }

        match input.parse::<Arg>() {
            Ok(arg) => return Ok(Expr::ArgExpr(Box::new(arg))),
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Expected argument or function call (see the errors below)",
            span,
            errors,
        ))
    }
}

// Note: the parsing code handles both commas or semicolons as argument separators
// this is done for backwards-compatibility with <= 0.0.4 versions.
const MIXING_SEP_ERROR: &str = r#"Mixing "," and ";" as separators is not allowed"#;

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let loops = if input.peek(Token![for]) {
            Some(input.parse::<LoopSpec>()?)
        } else {
            None
        };
        let spec = if input.peek(Ident) {
            Some(input.parse::<AliasSpec>()?)
        } else {
            None
        };
        let block: Block = input.parse()?;
        let deprecation_service = DeprecationService::scoped();

        match &spec {
            Some(spec) if spec.is_comma_used().is_some_and(|v| !v) => {
                deprecation_service.add_semicolon_separator_warning();
            }
            _ => {}
        }

        let is_comma_current_sep = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(true)
        } else if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            Some(false)
        } else {
            None
        };

        if let (Some(is_comma_current_sep), Some(spec)) = (is_comma_current_sep, &spec) {
            if is_comma_current_sep ^ spec.is_comma_used().is_some_and(|value| value) {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

        Ok(ComposeIdentsArgs::new(loops, spec, block))
    }
}

impl Parse for Alias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(Alias::new(ident))
    }
}

impl Parse for AliasValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let deprecation_service = DeprecationService::scoped();
        let span = input.span();
        let mut exprs = Vec::new();

        if input.peek(Bracket) {
            // Fall back to the deprecated bracket-based syntax
            let content;
            bracketed!(content in input);
            // TODO: this thing won't work with multi-argument function because it would consume up
            //       until the first comma and then feed the result to `Expr::parse`.
            let punctuated =
                content.parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,])?;
            exprs.extend(
                punctuated
                    .into_iter()
                    .map(|arg| arg.into_value())
                    .collect::<Vec<_>>(),
            );

            deprecation_service.add_bracket_syntax_warning();
        } else {
            let expr = input.parse::<Terminated<Expr, combine!(Token![,], Token![;])>>()?;
            exprs.push(expr.into_value());
        }

        Ok(AliasValue::new(exprs, span))
    }
}

impl Parse for AliasSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let alias: Alias = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: AliasValue = input.parse()?;

        Ok(AliasSpecItem::new(alias, value))
    }
}

impl Parse for AliasSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut seen_aliases = HashSet::new();
        let mut items = Vec::new();
        let mut is_comma_used = None;

        loop {
            let spec_item: AliasSpecItem = input.parse()?;
            let alias_name = spec_item.alias().ident().to_string();
            if seen_aliases.contains(&alias_name) {
                return Err(input.error(format!(r#"Alias "{}" is already defined"#, alias_name)));
            }
            seen_aliases.insert(spec_item.alias().ident().to_string());
            items.push(spec_item);

            let is_comma_current_sep = if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                true
            } else if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                false
            } else {
                return Err(input.error(r#"Expected "," or ";""#));
            };

            if let Some(is_comma_used) = is_comma_used {
                if is_comma_used != is_comma_current_sep {
                    return Err(input.error(MIXING_SEP_ERROR));
                }
            } else {
                is_comma_used = Some(is_comma_current_sep);
            }

            if !input.peek(Ident) {
                break;
            }
        }

        Ok(AliasSpec::new(items, is_comma_used))
    }
}

impl<V> Parse for Tuple<V>
where
    V: Parse + Clone + Debug,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let content;

        parenthesized!(content in input);

        let punctuated =
            content.parse_terminated(Terminated::<TupleValue<V>, Token![,]>::parse, Token![,])?;
        let values: Vec<TupleValue<V>> = punctuated
            .into_iter()
            .map(|item| item.into_value())
            .collect();

        Ok(Tuple::<V>::new(values, span))
    }
}

impl<V> Parse for TupleValue<V>
where
    V: Parse + Clone + Debug,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let tuple = input.parse::<Tuple<V>>()?;
            Ok(TupleValue::Tuple(tuple))
        } else {
            let value = input.parse::<V>()?;
            Ok(TupleValue::Value(value))
        }
    }
}

impl Parse for LoopAlias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();

        let fork = input.fork();
        match fork.parse::<Tuple<Alias>>() {
            Ok(tuple) => {
                input.advance_to(&fork);
                return Ok(LoopAlias::Tuple(tuple));
            }
            Err(err) => errors.push(err),
        }

        let fork = input.fork();
        match fork.parse::<Alias>() {
            Ok(alias) => {
                input.advance_to(&fork);
                return Ok(LoopAlias::Simple(alias));
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Failed to parse loop alias (see errors below)",
            input.span(),
            errors,
        ))
    }
}

impl Parse for LoopSourceValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();

        let fork = input.fork();
        match fork.parse::<Tuple<Expr>>() {
            Ok(tuple) => {
                input.advance_to(&fork);
                return Ok(LoopSourceValue::Tuple(tuple));
            }
            Err(err) => errors.push(err),
        }

        let fork = input.fork();
        match fork.parse::<Terminated<Expr, Token![,]>>() {
            Ok(expr) => {
                input.advance_to(&fork);
                return Ok(LoopSourceValue::Simple(expr.into_value()));
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Failed to parse loop source value (see errors below)",
            input.span(),
            errors,
        ))
    }
}

impl Parse for LoopSourceValueList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let content;
        bracketed!(content in input);

        let punctuated = content.parse_terminated(LoopSourceValue::parse, Token![,])?;
        let source_values: Vec<LoopSourceValue> = punctuated.into_iter().collect();

        Ok(LoopSourceValueList::new(source_values, span))
    }
}

impl Parse for LoopSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        input.parse::<Token![for]>()?;

        let alias = input.parse::<LoopAlias>()?;

        input.parse::<Token![in]>()?;

        let list = input.parse::<LoopSourceValueList>()?;

        Ok(LoopSpecItem::new(alias, list, span))
    }
}

impl Parse for LoopSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut loops: Vec<LoopSpecItem> = Vec::new();

        while input.peek(Token![for]) {
            let loop_spec: LoopSpecItem = input.parse()?;
            loops.push(loop_spec);
        }
        if loops.is_empty() {
            return Err(input.error("Failed to parse any loops"));
        }
        Ok(LoopSpec::new(loops))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{AliasValue, Arg, Expr, Func};
    use crate::parse::Terminated;
    use proc_macro2::TokenStream;
    use syn::parse::{ParseStream, Parser};
    use syn::Token;

    /// Tests a simple case where [`Arg`] is parsed as an identifier.
    #[test]
    fn parse_arg_ident() {
        let result = syn::parse_str::<Arg>("foo");
        assert!(
            result.is_ok(),
            "Expected identifier to be successfully parsed"
        );
        let actual = result.unwrap();

        assert!(matches!(actual, Arg::Ident(ident) if ident == "foo"));
    }

    /// Tests that it should be possible to parse terminated [`Arg`] with [`Terminated`] helper.
    #[test]
    fn parse_arg_ident_terminated() {
        let parser = |input: ParseStream| {
            let item: Terminated<Arg, Token![,]> = input.parse()?;
            let rest: TokenStream = input.parse()?;
            Ok((item, rest))
        };

        let result = parser.parse_str("foo,");
        assert!(
            result.is_ok(),
            "Expected identifier terminated to be successfully parsed"
        );

        let (terminated_arg, actual_tail) = result.unwrap();
        let actual_arg = terminated_arg.into_value();

        assert!(matches!(&actual_arg, Arg::Ident(ident) if ident == "foo"));
        assert_eq!(actual_tail.to_string(), ",");
    }

    /// Tests that a simple identifier can be parsed as an [`AliasValue`].
    #[test]
    fn parse_alias_value_ident() {
        let result = syn::parse_str::<AliasValue>("foo");
        assert!(
            result.is_ok(),
            "Expected identifier to be successfully parsed"
        );

        let alias_value = result.unwrap();
        let actual_exprs = alias_value.exprs();
        assert_eq!(
            actual_exprs.len(),
            1,
            "Expected one expression in AliasValue"
        );

        let actual_expr = &actual_exprs[0];

        assert!(
            matches!(actual_expr, Expr::ArgExpr(boxed_arg) if matches!(boxed_arg.as_ref(), Arg::Ident(_)))
        );
    }

    /// Tests that a simple identifier can be parsed as an [`AliasValue`].
    #[test]
    fn parse_alias_value_ident_terminated() {
        let parser = |input: ParseStream| {
            let item: AliasValue = input.parse()?;
            let rest: TokenStream = input.parse()?;
            Ok((item, rest))
        };
        let result = parser.parse_str("foo,");
        assert!(
            result.is_ok(),
            "Expected identifier to be successfully parsed"
        );

        let (alias_value, tokens) = result.unwrap();
        let actual_exprs = alias_value.exprs();
        assert_eq!(
            actual_exprs.len(),
            1,
            "Expected one expression in AliasValue"
        );

        let actual_expr = &actual_exprs[0];

        assert!(
            matches!(actual_expr, Expr::ArgExpr(boxed_arg) if matches!(boxed_arg.as_ref(), Arg::Ident(_))),
        );
        assert_eq!(tokens.to_string(), ",");
    }

    /// Tests that a simple func call can be parsed as a [`Func`].
    #[test]
    fn parse_func_upper() {
        let result = syn::parse_str::<Func>("upper(foo)");
        assert!(result.is_ok(), "Expected func to be successfully parsed");
        let actual = result.unwrap();

        assert!(matches!(
            actual,
            Func::Upper(Expr::ArgExpr(boxed_arg))
            if matches!(boxed_arg.as_ref(), Arg::Ident(ident) if ident == "foo")
        ));
    }
}
