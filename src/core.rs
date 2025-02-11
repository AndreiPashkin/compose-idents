use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

/// State of a particular macro invocation.
#[derive(Debug)]
pub struct State {
    pub(super) seed: u64,
}

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
        } else if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
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

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug)]
pub enum Func {
    Upper(Box<Expr>),
    Lower(Box<Expr>),
    SnakeCase(Box<Expr>),
    CamelCase(Box<Expr>),
    Hash(Box<Expr>),
}

impl Func {
    fn parse_func(input: ParseStream) -> syn::Result<(String, Vec<Expr>)> {
        let call = input.parse::<syn::ExprCall>()?;
        let func_name = call.func.to_token_stream().to_string();
        let raw_args = call.args;
        let mut args = Vec::new();
        for arg in raw_args {
            args.push(syn::parse2::<Expr>(arg.into_token_stream())?);
        }
        Ok((func_name, args))
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (func, mut args) = Func::parse_func(input)?;
        match (func.as_str(), args.len()) {
            ("upper", 1) => Ok(Func::Upper(Box::new(args.drain(..).next().unwrap()))),
            ("lower", 1) => Ok(Func::Lower(Box::new(args.drain(..).next().unwrap()))),
            ("snake_case", 1) => Ok(Func::SnakeCase(Box::new(args.drain(..).next().unwrap()))),
            ("camel_case", 1) => Ok(Func::CamelCase(Box::new(args.drain(..).next().unwrap()))),
            ("hash", 1) => Ok(Func::Hash(Box::new(args.drain(..).next().unwrap()))),
            _ => Err(input.error(
                r#"Expected "upper()", "lower()", "snake_case()",
                    "camel_case()" or "hash()"."#,
            )),
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
