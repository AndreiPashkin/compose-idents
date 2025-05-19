//! Implements parsing logic for different internal components.

use super::core::{AliasSpec, AliasSpecItem, Arg, ComposeIdentsArgs, Expr, Func};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::HashSet;
use std::marker::PhantomData;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{bracketed, parenthesized, Block, Ident, Token};

/// A terminator token.
pub trait Terminator {
    fn is_terminator(input: &ParseStream) -> bool;
}

impl Terminator for Token![,] {
    fn is_terminator(input: &ParseStream) -> bool {
        input.peek(Token![,])
    }
}

/// A sequence of tokens (token-trees more specifically) terminated by a terminator-token.
struct TerminatedTokens<T: Terminator> {
    tokens: TokenStream,
    token_type: PhantomData<T>,
}

impl<T: Terminator> TerminatedTokens<T> {
    fn into_token_stream(self) -> TokenStream {
        self.tokens
    }
}

impl<T: Terminator> Parse for TerminatedTokens<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = TokenStream::new();

        while !input.is_empty() && !T::is_terminator(&input) {
            let tt = input.parse::<TokenTree>()?;
            tokens.extend(tt.into_token_stream());
        }

        Ok(TerminatedTokens {
            tokens,
            token_type: PhantomData,
        })
    }
}

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
            let tokens = input.parse::<TerminatedTokens<Token![,]>>()?;
            value = tokens.into_token_stream().to_string();
        }
        Ok(Arg { value })
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let func_name = ident.to_string();
        let raw_args;
        parenthesized!(raw_args in input);
        let args_tokens =
            raw_args.parse_terminated(TerminatedTokens::<Token![,]>::parse, Token![,])?;

        let mut args: Vec<Expr> = Vec::new();
        for arg in args_tokens {
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
            ("normalize", args) => match &args {
                [expr] => Ok(Func::Normalize(expr.clone())),
                _ => Ok(Func::SignatureMismatch("normalize(tokens)".to_string())),
            },
            _ => Ok(Func::Undefined),
        }
    }
}

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

        if errors.len() == 1 {
            Err(errors.pop().unwrap())
        } else {
            let mut error = syn::Error::new(
                input.span(),
                "Expected argument or function call (see the errors below)",
            );
            errors.iter().for_each(|err| error.combine(err.clone()));
            Err(error)
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
            let expr = content.parse::<Expr>()?;
            exprs.push(expr);
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
