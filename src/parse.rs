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

        match (func_name.as_str(), args.len()) {
            ("upper", 1) => Ok(Func::Upper(Box::new(args.drain(..).next().unwrap()))),
            ("lower", 1) => Ok(Func::Lower(Box::new(args.drain(..).next().unwrap()))),
            ("snake_case", 1) => Ok(Func::SnakeCase(Box::new(args.drain(..).next().unwrap()))),
            ("camel_case", 1) => Ok(Func::CamelCase(Box::new(args.drain(..).next().unwrap()))),
            ("pascal_case", 1) => Ok(Func::PascalCase(Box::new(args.drain(..).next().unwrap()))),
            ("hash", 1) => Ok(Func::Hash(Box::new(args.drain(..).next().unwrap()))),
            _ => Err(input.error(
                r#"Expected "upper()", "lower()", "snake_case()",
                    "camel_case()", "pascal_case()" or "hash()"."#,
            )),
        }
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(func) = fork.parse::<Func>() {
            input.advance_to(&fork);
            Ok(Expr::FuncCallExpr {
                value: Box::new(func),
            })
        } else if let Ok(arg) = input.parse::<Arg>() {
            Ok(Expr::ArgExpr {
                value: Box::new(arg),
            })
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
