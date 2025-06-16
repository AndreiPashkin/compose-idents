//! Implements parsing logic for different internal components.

use crate::ast::{Alias, AliasSpec, AliasSpecItem, AliasValue, Arg, ComposeIdentsArgs, Expr, Func};
use crate::error::combine_errors;
use crate::util::combined::combine;
use crate::util::deprecation::DeprecationService;
use crate::util::terminated::Terminated;
use proc_macro2::TokenStream;
use std::collections::HashSet;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, parenthesized, Block, Ident, Token};

/// Parses the argument until the end of the input.
impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_terminated = input.peek2(syn::parse::End);
        let value: Arg;
        if input.peek(syn::Ident) && is_terminated {
            let ident = input.parse::<syn::Ident>()?;
            value = Arg::Ident(ident);
        } else if input.peek(Token![_]) && is_terminated {
            input.parse::<Token![_]>()?;
            value = Arg::Underscore;
        } else if input.peek(syn::LitStr) && is_terminated {
            let lit_str = input.parse::<syn::LitStr>()?;
            value = Arg::LitStr(lit_str.value());
        } else if input.peek(syn::LitInt) && is_terminated {
            let lit_int = input.parse::<syn::LitInt>()?;
            value = Arg::LitInt(lit_int.base10_parse::<u64>()?);
        } else {
            let tokens = input.parse::<TokenStream>()?;
            value = Arg::Tokens(tokens);
        }
        Ok(value)
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let func_name = ident.to_string();
        let raw_args;
        parenthesized!(raw_args in input);
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
            .map(|tokens| Expr::ArgExpr(Box::new(Arg::Tokens(tokens))));

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
            input.span(),
            errors,
        ))
    }
}

// Note: the parsing code handles both commas or semicolons as argument separators
// this is done for backwards-compatibility with <= 0.0.4 versions.
const MIXING_SEP_ERROR: &str = r#"Mixing "," and ";" as separators is not allowed"#;

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec: AliasSpec = input.parse()?;
        let block: Block = input.parse()?;
        let deprecation_service = DeprecationService::scoped();

        if spec.is_comma_used().is_some_and(|value| !value) {
            deprecation_service.add_semicolon_separator_warning();
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

        if let (Some(is_comma_current_sep), Some(is_comma_used)) =
            (is_comma_current_sep, spec.is_comma_used())
        {
            if is_comma_current_sep ^ is_comma_used {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

        Ok(ComposeIdentsArgs::new(spec, block))
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

#[cfg(test)]
mod tests {
    use crate::ast::{AliasValue, Arg, Expr, Func};
    use crate::parse::Terminated;
    use crate::util::combined::combine;
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

    /// Tests usage of [`crate::parse::Combined`] parser-combinator and [`crate::parse::combine`]
    /// macro helper.
    #[test]
    fn combine() {
        type CombinedType = combine!(syn::Ident, syn::LitStr, syn::LitInt);

        let parser = |input: ParseStream| {
            let item: CombinedType = input.parse()?;
            let rest: TokenStream = input.parse()?;
            Ok((item, rest))
        };

        let result = parser.parse_str("foo");
        assert!(result.is_ok(), "Failed to parse the expression");

        let (actual_combined, actual_tail) = result.unwrap();
        assert_eq!(actual_tail.to_string(), "");

        assert!(matches!(
            actual_combined,
            CombinedType::A(ident) if ident == "foo"
        ));
        assert_eq!(actual_tail.to_string(), "");
    }
}
