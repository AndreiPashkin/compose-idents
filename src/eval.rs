//! Implementation of eval-phase logic.

use crate::ast::{
    Alias, AliasValue, Arg, Ast, Expr, Func, LoopAlias, LoopSourceValue, LoopSourceValueList,
    LoopSpec, LoopSpecItem, Spanned,
};
use crate::core::State;
use crate::error::Error;
use crate::funcs::{concat, hash, normalize, to_camel_case, to_pascal_case, to_snake_case};
use proc_macro2::Span;
use std::collections::HashMap;

static NON_SINGULAR_VALUE_ERROR: &str = "Expected a singular value";

/// Result of evaluating a statement.
#[derive(Debug, Clone)]
pub enum Evaluated {
    /// A singular value
    Value(Span, String),
    Bindings(Span, HashMap<Alias, Evaluated>),
    List(Span, Vec<Evaluated>),
}

impl Spanned for Evaluated {
    fn span(&self) -> Span {
        match self {
            Evaluated::Value(span, _) => *span,
            Evaluated::Bindings(span, _) => *span,
            Evaluated::List(span, _) => *span,
        }
    }
}

/// Runtime context of evaluation.
#[derive(Debug, Default, Clone)]
pub struct Context {
    context: HashMap<Alias, Evaluated>,
}

impl Context {
    pub fn context_mut(&mut self) -> &mut HashMap<Alias, Evaluated> {
        &mut self.context
    }
}

fn validate_alias_value(ident: &Evaluated) -> Result<(), Error> {
    let span = ident.span();
    let Evaluated::Value(_, value) = ident else {
        panic!("{}", NON_SINGULAR_VALUE_ERROR)
    };
    if syn::parse_str::<syn::Ident>(value.as_str()).is_err() {
        return Err(Error::EvalError(
            format!("Invalid alias value: {}", value),
            span,
        ));
    }
    Ok(())
}

fn validate_alias_value_str(ident: &str, span: &Span) -> Result<(), Error> {
    if syn::parse_str::<syn::Ident>(ident).is_err() {
        return Err(Error::EvalError(
            format!("Invalid alias value: {}", ident),
            *span,
        ));
    }
    Ok(())
}

/// A syntactic structure that can be evaluated.
///
/// For example, it could be a function call passed by a user to the macro as an argument.
pub trait Eval: Ast {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error>;
}

impl Eval for Arg {
    fn eval(&self, _: &State, context: &mut Context) -> Result<Evaluated, Error> {
        eprintln!(
            "Evaluating argument: {:?}, with context: {:?}",
            self, context
        );
        match self {
            Arg::Ident(ident) => {
                let alias = Alias::new(ident.clone());
                let context_ = context.context_mut();
                let res = match context_.get(&alias) {
                    Some(Evaluated::Value(span, v)) => {
                        eprintln!("Reusing the alias {} with value: {}", alias, v);
                        Evaluated::Value(*span, v.clone())
                    }
                    None => {
                        eprintln!("Reusing alias failed, keeping the value: {}", ident);
                        Evaluated::Value(ident.span(), ident.to_string())
                    }
                    _ => panic!("{}", NON_SINGULAR_VALUE_ERROR),
                };
                Ok(res)
            }
            Arg::LitStr(_, value) => Ok(Evaluated::Value(self.span(), value.clone())),
            Arg::LitInt(_, value) => Ok(Evaluated::Value(self.span(), value.to_string())),
            Arg::Tokens(_, tokens) => Ok(Evaluated::Value(self.span(), tokens.to_string())),
            Arg::Underscore(_) => Ok(Evaluated::Value(self.span(), "_".to_string())),
        }
    }
}

impl Eval for Func {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Func::Upper(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("Expected a singular value");
                };
                Ok(Evaluated::Value(self.span(), value.to_uppercase()))
            }
            Func::Lower(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(self.span(), value.to_lowercase()))
            }
            Func::SnakeCase(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(self.span(), to_snake_case(value.as_str())))
            }
            Func::CamelCase(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(self.span(), to_camel_case(value.as_str())))
            }
            Func::PascalCase(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(
                    self.span(),
                    to_pascal_case(value.as_str()),
                ))
            }
            Func::Hash(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(self.span(), hash(value.as_str(), state)))
            }
            Func::Normalize(expr) => {
                let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                    panic!("{}", NON_SINGULAR_VALUE_ERROR);
                };
                Ok(Evaluated::Value(self.span(), normalize(value.as_str())))
            }
            Func::Concat(exprs) => {
                let values: Result<Vec<String>, Error> = exprs
                    .iter()
                    .map(|expr| {
                        let Evaluated::Value(_, value) = expr.eval(state, context)? else {
                            panic!("{}", NON_SINGULAR_VALUE_ERROR);
                        };
                        Ok(value)
                    })
                    .collect();
                let values = values?;
                let string_refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
                Ok(Evaluated::Value(self.span(), concat(&string_refs)))
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
        eprintln!("Evaluating alias value: {:?}", self);
        let ident = self.exprs().iter().try_fold("".to_string(), |acc, item| {
            let Evaluated::Value(_, arg) = item.eval(state, context)? else {
                panic!("{}", NON_SINGULAR_VALUE_ERROR);
            };
            eprintln!("Evaluated item: {:?}, value: {:?}", item, arg);
            Ok::<String, Error>(format!("{}{}", acc, arg))
        })?;

        // Validate that the resulting string is a valid identifier.
        validate_alias_value_str(ident.as_str(), &self.span())?;

        Ok(Evaluated::Value(self.span(), ident))
    }
}

impl Eval for LoopSourceValue {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            LoopSourceValue::Simple(value) => {
                let evaluated = value.eval(state, context)?;
                validate_alias_value(&evaluated)?;
                Ok(evaluated)
            }
            LoopSourceValue::Tuple(tuple) => {
                let mut values = Vec::new();
                for value in tuple.iter_recursive() {
                    let evaluated = value.eval(state, context)?;
                    validate_alias_value(&evaluated)?;
                    values.push(evaluated);
                }
                Ok(Evaluated::List(self.span(), values))
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

        Ok(Evaluated::List(self.span(), values))
    }
}

impl Eval for LoopSpecItem {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        eprintln!(
            "Evaluating loop spec item: {:?}, with context: {:?}",
            self, context
        );
        let aliases = match self.alias() {
            LoopAlias::Simple(alias) => {
                vec![alias.clone()]
            }
            LoopAlias::Tuple(tuple) => tuple.iter_recursive().collect(),
        };
        let Evaluated::List(_, list) = self.list().eval(state, context)? else {
            unreachable!()
        };
        eprintln!("List of evaluated values: {:?}", list);
        let bindings: Vec<Evaluated> = list
            .iter()
            .map(|evaluated| match evaluated {
                // Simple value
                Evaluated::Value(span, value) => {
                    vec![Evaluated::Value(*span, value.clone())]
                }
                // Tuple
                Evaluated::List(_, values) => values.clone(),
                _ => unreachable!(),
            })
            .map(|values| {
                Evaluated::Bindings(
                    self.span(),
                    aliases.clone().into_iter().zip(values).collect(),
                )
            })
            .collect();

        Ok(Evaluated::List(self.span(), bindings))
    }
}

// Helper function that recursively processes the loops.
fn eval_loop_spec(
    loops: &[LoopSpecItem],
    idx: usize,
    state: &State,
    // Current immutable context available to the RHS of the remaining loops.
    context: &mut Context,
    // Where to push the final combined bindings.
    out: &mut Vec<Evaluated>,
    // Span of the parent LoopSpec for positioning of errors / results.
    span: Span,
) -> Result<(), Error> {
    if idx == loops.len() {
        // All loops processed – store the combined bindings.
        out.push(Evaluated::Bindings(span, context.context_mut().clone()));
        return Ok(());
    }

    let loop_spec_item = &loops[idx];

    // Build a mutable context that contains the outer context + already accumulated bindings.
    let mut local_context = context.clone();

    // Evaluate the current loop once with the current bindings.
    let evaluated_list = loop_spec_item.eval(state, &mut local_context)?;
    let Evaluated::List(_, items) = evaluated_list else {
        unreachable!()
    };

    for evaluated in items {
        let Evaluated::Bindings(_, ref bindings_map) = evaluated else {
            unreachable!()
        };

        // Update context for inner loops: outer context + current bindings.
        let mut next_context = local_context.clone();
        next_context.context_mut().extend(bindings_map.clone());

        eval_loop_spec(loops, idx + 1, state, &mut next_context, out, span)?;
    }

    Ok(())
}

impl Eval for LoopSpec {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        // Accumulated results of cartesian product – each entry is a combined map of
        // alias → evaluated value for one concrete iteration of all the loops.
        let mut results: Vec<Evaluated> = Vec::new();

        // Kick-off recursion.
        let span = self.span();
        eval_loop_spec(self.loops(), 0, state, context, &mut results, span)?;

        Ok(Evaluated::List(span, results))
    }
}
