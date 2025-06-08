//! Implementation of eval-phase logic.

use crate::ast::{Alias, AliasValue, Arg, Ast, Expr, Func};
use crate::core::State;
use crate::error::Error;
use crate::funcs::{concat, hash, normalize, to_camel_case, to_pascal_case, to_snake_case};
use std::collections::HashMap;

/// Result of evaluating a statement.
pub enum Evaluated {
    /// A singular value
    Value(String),
}

/// Runtime context of evaluation.
#[derive(Default)]
pub struct Context {
    context: HashMap<Alias, Evaluated>,
}

impl Context {
    pub fn context_mut(&mut self) -> &mut HashMap<Alias, Evaluated> {
        &mut self.context
    }
}

/// A syntactic structure that can be evaluated.
///
/// For example, it could be a function call passed by a user to the macro as an argument.
pub trait Eval: Ast {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error>;
}

impl Eval for Arg {
    fn eval(&self, _: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Arg::Ident(ident) => {
                let alias = Alias::new(ident.clone());
                let context_ = context.context_mut();
                let res = match context_.get(&alias) {
                    Some(Evaluated::Value(v)) => Evaluated::Value(v.clone()),
                    None => Evaluated::Value(ident.to_string()),
                };
                Ok(res)
            }
            Arg::LitStr(_, value) => Ok(Evaluated::Value(value.clone())),
            Arg::LitInt(_, value) => Ok(Evaluated::Value(value.to_string())),
            Arg::Tokens(_, tokens) => Ok(Evaluated::Value(tokens.to_string())),
            Arg::Underscore(_) => Ok(Evaluated::Value("_".to_string())),
        }
    }
}

impl Eval for Func {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Func::Upper(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(value.to_uppercase()))
            }
            Func::Lower(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(value.to_lowercase()))
            }
            Func::SnakeCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_snake_case(value.as_str())))
            }
            Func::CamelCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_camel_case(value.as_str())))
            }
            Func::PascalCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_pascal_case(value.as_str())))
            }
            Func::Hash(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(hash(value.as_str(), state)))
            }
            Func::Normalize(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(normalize(value.as_str())))
            }
            Func::Concat(exprs) => {
                let values: Result<Vec<String>, Error> = exprs
                    .iter()
                    .map(|expr| {
                        let Evaluated::Value(value) = expr.eval(state, context)?;
                        Ok(value)
                    })
                    .collect();
                let values = values?;
                let string_refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
                Ok(Evaluated::Value(concat(&string_refs)))
            }
            Func::Undefined => panic!("Attempt to evaluate an undefined function"),
            Func::SignatureMismatch(_) => {
                panic!("Attempt to evaluate a function with a mismatched signature")
            }
        }
    }
}

impl Eval for Expr {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Expr::ArgExpr(value) => value.eval(state, context),
            Expr::FuncCallExpr(value) => value.eval(state, context),
        }
    }
}

impl Eval for AliasValue {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let ident = self.exprs().iter().try_fold("".to_string(), |acc, item| {
            let Evaluated::Value(arg) = item.eval(state, context)?;
            Ok::<String, Error>(format!("{}{}", acc, arg))
        })?;

        // Validate that the resulting string is a valid identifier.
        if syn::parse_str::<syn::Ident>(&ident).is_err() {
            return Err(Error::EvalError(
                format!("`{}` is not a valid identifier", ident),
                self.span(),
            ));
        }

        Ok(Evaluated::Value(ident))
    }
}
