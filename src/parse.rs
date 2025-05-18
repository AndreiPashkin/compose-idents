//! Implements parsing logic for different internal components.

use super::core::{AliasSpec, AliasSpecItem, Arg, ComposeIdentsArgs, Expr, Func};
use quote::ToTokens;
use std::collections::HashSet;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{bracketed, Block, Ident, Token};

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value: String;
        if input.peek(syn::Ident) && !input.peek2(syn::token::Paren) {
            let ident = input.parse::<syn::Ident>()?;
            value = ident.to_string();
        } else if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
            value = "_".to_string();
        } else if input.peek(syn::LitStr) {
            let lit_str = input.parse::<syn::LitStr>()?;
            value = lit_str.value();
        } else if input.peek(syn::LitInt) {
            let lit_int = input.parse::<syn::LitInt>()?;
            value = lit_int.base10_digits().to_string();
        } else {
            return Err(input.error("Expected identifier or _"));
        }
        Ok(Arg { value })
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let call = input.parse::<syn::ExprCall>()?;
        let func_name = call.func.to_token_stream().to_string();
        let raw_args = call.args;
        let mut args = Vec::new();
        for arg in raw_args {
            args.push(syn::parse2::<Expr>(arg.into_token_stream())?);
        }
        match (func_name.as_str(), args.as_slice()) {
            ("upper", args) => match &args {
                [expr] => Ok(Func::Upper(expr.clone())),
                _ => Ok(Func::SignatureMismatch("upper(arg)".to_string())),
            },
            ("lower", args) => match &args {
                [expr] => Ok(Func::Lower(expr.clone())),
                _ => Ok(Func::SignatureMismatch("lower(arg)".to_string())),
            },
            ("snake_case", args) => match &args {
                [expr] => Ok(Func::SnakeCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("snake_case(arg)".to_string())),
            },
            ("camel_case", args) => match &args {
                [expr] => Ok(Func::CamelCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("camel_case(arg)".to_string())),
            },
            ("pascal_case", args) => match &args {
                [expr] => Ok(Func::PascalCase(expr.clone())),
                _ => Ok(Func::SignatureMismatch("pascal_case(arg)".to_string())),
            },
            ("hash", args) => match &args {
                [expr] => Ok(Func::Hash(expr.clone())),
                _ => Ok(Func::SignatureMismatch("hash(arg)".to_string())),
            },
            _ => Ok(Func::Undefined),
        }
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let span = input.span();
        if let Ok(func) = fork.parse::<Func>() {
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
            Ok(Expr::FuncCallExpr(Box::new(func)))
        } else if let Ok(arg) = input.parse::<Arg>() {
            Ok(Expr::ArgExpr(Box::new(arg)))
        } else {
            Err(input.error("Expected argument or function call"))
        }
    }
}

// Note: the parsing code handles both commas or semicolons as argument separators
// this is done for backwards-compatibility with <= 0.0.4 versions.
const MIXING_SEP_ERROR: &str = r#"Mixing "," and ";" as separators is not allowed"#;

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec: AliasSpec = input.parse()?;
        let block: Block = input.parse()?;

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
            (is_comma_current_sep, spec.is_comma_used)
        {
            if is_comma_current_sep ^ is_comma_used {
                return Err(input.error(MIXING_SEP_ERROR));
            }
        }

        Ok(ComposeIdentsArgs { spec, block })
    }
}

impl Parse for AliasSpecItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let alias: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let content;
        bracketed!(content in input);
        let mut exprs = Vec::new();
        loop {
            match content.parse::<Expr>() {
                Ok(expr) => exprs.push(expr),
                Err(err) => return Err(err),
            }
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }
        Ok(AliasSpecItem { alias, exprs })
    }
}

impl Parse for AliasSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut seen_aliases = HashSet::new();
        let mut items = Vec::new();
        let mut is_comma_used = None;

        loop {
            let spec_item: AliasSpecItem = input.parse()?;
            let alias_name = spec_item.alias.to_string();
            if seen_aliases.contains(&alias_name) {
                return Err(input.error(format!(r#"Alias "{}" is already defined"#, alias_name)));
            }
            seen_aliases.insert(spec_item.alias.to_string());
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

        Ok(AliasSpec {
            items,
            is_comma_used,
        })
    }
}
