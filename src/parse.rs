//! Implements parsing logic for different internal components.

use crate::ast::{Alias, AliasSpec, AliasSpecItem, AliasValue, Arg, ComposeIdentsArgs, Expr, Func};
use crate::error::combine_errors;
use crate::util::combined::combine;
use crate::util::deprecation::DeprecationService;
use crate::util::terminated::Terminated;
use crate::util::unique_id::next_unique_id;
use proc_macro2::TokenStream;
use std::collections::HashSet;
use std::rc::Rc;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, parenthesized, Block, Ident, Token};

/// Parses the argument until the end of the input.
impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_terminated = input.peek2(syn::parse::End);
        if input.peek(syn::Ident) && is_terminated {
            let ident = input.parse::<syn::Ident>()?;
            Ok(Arg::from_ident(ident))
        } else if input.peek(Token![_]) && is_terminated {
            input.parse::<Token![_]>()?;
            Ok(Arg::from_underscore())
        } else if input.peek(syn::LitStr) && is_terminated {
            let lit_str = input.parse::<syn::LitStr>()?;
            Ok(Arg::from_lit_str(lit_str.value()))
        } else if input.peek(syn::LitInt) && is_terminated {
            let lit_int = input.parse::<syn::LitInt>()?;
            Ok(Arg::from_lit_int(lit_int.base10_parse::<u64>()?))
        } else {
            let tokens = input.parse::<TokenStream>()?;
            Ok(Arg::from_tokens(tokens))
        }
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let name = input.parse::<Ident>()?;
        let raw_args;
        parenthesized!(raw_args in input);
        let punctuated = raw_args
            .fork()
            .parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,]);
        let args = match punctuated {
            Ok(punctuated) => Some(
                punctuated
                    .into_iter()
                    .map(|arg| Rc::new(arg.into_value()))
                    .collect::<Vec<_>>(),
            ),
            Err(_) => None,
        };
        let tokens = raw_args
            .parse::<TokenStream>()
            .ok()
            .map(|tokens| Rc::new(Expr::ArgExpr(Box::new(Arg::from_tokens(tokens)))));

        Ok(Func::new(next_unique_id(), name, args, tokens, span))
    }
}

/// Just like impl of [`Parse`] for [`Arg`] - parses the input either until the end or a comma.
impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();
        let fork = input.fork();

        match fork.parse::<Func>() {
            Ok(func) => {
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

        Ok(ComposeIdentsArgs::new(
            next_unique_id(),
            Rc::new(spec),
            block,
        ))
    }
}

impl Parse for Alias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(Alias::new(next_unique_id(), ident))
    }
}

impl Parse for AliasValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let deprecation_service = DeprecationService::scoped();
        let span = input.span();
        let expr;

        if input.peek(Bracket) {
            // Fall back to the deprecated bracket-based syntax
            let mut exprs = Vec::new();
            let content;
            bracketed!(content in input);
            let punctuated =
                content.parse_terminated(Terminated::<Expr, Token![,]>::parse, Token![,])?;
            exprs.extend(
                punctuated
                    .into_iter()
                    .map(|arg| Rc::new(arg.into_value()))
                    .collect::<Vec<_>>(),
            );
            let func = Func::new(
                next_unique_id(),
                Ident::new("concat", span),
                Some(exprs),
                None,
                span,
            );
            expr = Expr::FuncCallExpr(Box::new(func));

            deprecation_service.add_bracket_syntax_warning();
        } else {
            let terminated = input.parse::<Terminated<Expr, combine!(Token![,], Token![;])>>()?;
            expr = terminated.into_value();
        }

        Ok(AliasValue::new(next_unique_id(), Rc::new(expr), span))
    }
}

impl Parse for AliasSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let alias: Alias = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: AliasValue = input.parse()?;

        Ok(AliasSpecItem::new(
            next_unique_id(),
            Rc::new(alias),
            Rc::new(value),
        ))
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

        Ok(AliasSpec::new(
            next_unique_id(),
            items.into_iter().map(Rc::new).collect(),
            is_comma_used,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{AliasValue, Arg, ArgInner, Expr, Func};
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

        assert!(matches!(actual.inner(), ArgInner::Ident(ident) if ident == "foo"));
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

        assert!(matches!(&actual_arg.inner(), ArgInner::Ident(ident) if ident == "foo"));
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
        let actual_expr = alias_value.expr();

        assert!(
            matches!(actual_expr.as_ref(), Expr::ArgExpr(boxed_arg) if matches!(boxed_arg.inner(), ArgInner::Ident(_)))
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
        let actual_expr = alias_value.expr();

        assert!(
            matches!(actual_expr.as_ref(), Expr::ArgExpr(boxed_arg) if matches!(boxed_arg.inner(), ArgInner::Ident(_))),
        );
        assert_eq!(tokens.to_string(), ",");
    }

    /// Tests that a simple func call can be parsed as a [`Func`].
    #[test]
    fn parse_func_upper() {
        let result = syn::parse_str::<Func>("upper(foo)");
        assert!(result.is_ok(), "Expected func to be successfully parsed");
        let actual = result.unwrap();

        assert_eq!(actual.name(), "upper");
        assert!(
            matches!(actual.args(), Some(args) if args.len() == 1 && matches!(args[0].as_ref(), Expr::ArgExpr(boxed_arg) if matches!(boxed_arg.inner(), ArgInner::Ident(ident) if ident == "foo"))),
        );
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
