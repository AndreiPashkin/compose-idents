use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

/// Argument in form of an identifier, underscore or a string literal.
#[derive(Debug)]
pub struct Arg {
    pub(super) value: String,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value: String;
        if input.peek(syn::Ident) && !input.peek2(syn::token::Paren) {
            let ident = input.parse::<syn::Ident>()?;
            value = ident.to_string();
        } else if input.parse::<syn::Token![_]>().is_ok() {
            value = "_".to_string();
        } else if let Ok(lit_str) = input.parse::<syn::LitStr>() {
            value = lit_str.value();
        } else {
            return Err(input.error("Expected identifier or _"));
        }
        Ok(Arg { value })
    }
}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug)]
pub enum Func {
    Upper(Box<Expr>),
    Lower(Box<Expr>),
    SnakeCase(Box<Expr>),
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let call = input.parse::<syn::ExprCall>()?;
        let func_name = call.func.to_token_stream().to_string();
        match func_name.as_str() {
            "upper" | "lower" | "snake_case" => {
                let args = call.args;
                if args.len() != 1 {
                    return Err(input.error("Expected 1 argument"));
                }
                let arg = syn::parse2::<Expr>(args.into_token_stream())?;
                match func_name.as_str() {
                    "upper" => Ok(Func::Upper(Box::new(arg))),
                    "lower" => Ok(Func::Lower(Box::new(arg))),
                    "snake_case" => Ok(Func::SnakeCase(Box::new(arg))),
                    _ => unreachable!(),
                }
            }
            _ => Err(input.error(r#"Expected "upper()" or "lower()""#)),
        }
    }
}

/// Expression in form of an argument or a function call.
#[derive(Debug)]
pub(super) enum Expr {
    ArgExpr { value: Box<Arg> },
    FuncCallExpr { value: Box<Func> },
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
