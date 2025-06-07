//! Implementation of eval-phase logic.

use crate::ast::{
    AliasValue, Arg, Ast, Expr, Func, LoopAlias, LoopSourceValue, LoopSourceValueList, LoopSpec,
    LoopSpecItem,
};
use crate::core::State;
use crate::error::Error;
use crate::funcs::{concat, hash, normalize, to_camel_case, to_pascal_case, to_snake_case};
use crate::product::product;
use std::collections::HashMap;
use std::fmt::Debug;
use std::vec::IntoIter;

static NON_SINGULAR_VALUE_ERROR: &str = "Expected a singular value";

/// Result of evaluating a statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evaluated {
    /// A singular value
    Value(String),
    Bindings(HashMap<String, Evaluated>),
    List(Vec<Evaluated>),
}

/// Runtime context of evaluation.
#[derive(Default, Clone)]
pub struct Context {
    context: HashMap<String, Evaluated>,
}

impl Context {
    pub fn context_mut(&mut self) -> &mut HashMap<String, Evaluated> {
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
                let value = ident.to_string();
                let context_ = context.context_mut();
                let res = match context_.get(&value) {
                    Some(Evaluated::Value(v)) => Evaluated::Value(v.clone()),
                    None => Evaluated::Value(value),
                    _ => panic!("{}", NON_SINGULAR_VALUE_ERROR),
                };
                Ok(res)
            }
            Arg::LitStr(value) => Ok(Evaluated::Value(value.clone())),
            Arg::LitInt(value) => Ok(Evaluated::Value(value.to_string())),
            Arg::Tokens(tokens) => Ok(Evaluated::Value(tokens.to_string())),
            Arg::Underscore => Ok(Evaluated::Value("_".to_string())),
        }
    }
}

impl Eval for Func {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Func::Upper(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("Expected a singular value");
                };
                Ok(Evaluated::Value(value.to_uppercase()))
            }
            Func::Lower(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(value.to_lowercase()))
            }
            Func::SnakeCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(to_snake_case(value.as_str())))
            }
            Func::CamelCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(to_camel_case(value.as_str())))
            }
            Func::PascalCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(to_pascal_case(value.as_str())))
            }
            Func::Hash(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(hash(value.as_str(), state)))
            }
            Func::Normalize(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(normalize(value.as_str())))
            }
            Func::Concat(exprs) => {
                let values: Result<Vec<String>, Error> = exprs
                    .iter()
                    .map(|expr| {
                        let Evaluated::Value(value) = expr.eval(state, context)? else {
                            panic!("{}", NON_SINGULAR_VALUE_ERROR);
                        };
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
            let Evaluated::Value(arg) = item.eval(state, context)? else {
                panic!("{}", NON_SINGULAR_VALUE_ERROR);
            };
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

impl Eval for LoopSourceValue {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            LoopSourceValue::Simple(value) => value.eval(state, context),
            LoopSourceValue::Tuple(tuple) => {
                let mut values = Vec::new();
                for value in tuple.iter_recursive() {
                    let evaluated = value.eval(state, context)?;
                    values.push(evaluated);
                }
                Ok(Evaluated::List(values))
            }
        }
    }
}

impl Eval for LoopSourceValueList {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let values = self
            .values()
            .iter()
            .map(|value| value.eval(state, context))
            .collect::<Result<Vec<Evaluated>, Error>>()?;

        Ok(Evaluated::List(values))
    }
}

impl Eval for LoopSpecItem {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let aliases = match self.alias() {
            LoopAlias::Simple(alias) => {
                vec![alias.ident().to_string().clone()]
            }
            LoopAlias::Tuple(tuple) => tuple
                .iter_recursive()
                .map(|alias| alias.ident().to_string())
                .collect(),
        };
        let Evaluated::List(list) = self.list().eval(state, context)? else {
            unreachable!()
        };
        let bindings: Vec<Evaluated> = list
            .iter()
            .map(|evaluated| match evaluated {
                Evaluated::Value(value) => vec![Evaluated::Value(value.clone())],
                Evaluated::List(values) => values.clone(),
                _ => unreachable!(),
            })
            .map(|values| Evaluated::Bindings(aliases.clone().into_iter().zip(values).collect()))
            .collect();

        Ok(Evaluated::List(bindings))
    }
}

impl Eval for LoopSpec {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let iters = self
            .loops()
            .iter()
            .map::<Result<_, Error>, _>(|loop_item| match loop_item.eval(state, context) {
                Ok(Evaluated::List(bindings)) => Ok(bindings.into_iter()),
                Err(err) => Err(err),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<IntoIter<Evaluated>>, Error>>()?;

        let values: Vec<_> = product(iters.as_ref())
            .map(|bindings| {
                let mut combined = HashMap::new();
                for item in bindings {
                    let Evaluated::Bindings(item) = item else {
                        unreachable!();
                    };
                    combined.extend(item);
                }
                Evaluated::Bindings(combined)
            })
            .collect();

        Ok(Evaluated::List(values))
    }
}
