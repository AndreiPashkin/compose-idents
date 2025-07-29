use crate::ast::{Ast, Func, FuncInner};
use crate::core::State;
use crate::error::{internal_error, Error};
use crate::eval::{Context, Eval, Evaluated};
use crate::funcs::{
    concat, hash, lower, normalize, to_camel_case, to_pascal_case, to_snake_case, upper,
};
use std::ops::Deref;

impl Eval for Func {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let Some(metadata) = context.metadata().borrow().get_func_metadata(self.id()) else {
            return Err(internal_error!(
                "Expected function metadata to be set at the eval phase for function: {}",
                self.name()
            ));
        };
        let inner = metadata.inner.clone();

        match inner.deref() {
            FuncInner::Upper(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(upper(value.as_str())))
            }
            FuncInner::Lower(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(lower(value.as_str())))
            }
            FuncInner::SnakeCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_snake_case(value.as_str())))
            }
            FuncInner::CamelCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_camel_case(value.as_str())))
            }
            FuncInner::PascalCase(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(to_pascal_case(value.as_str())))
            }
            FuncInner::Hash(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(hash(value.as_str(), state)))
            }
            FuncInner::Normalize(expr) => {
                let Evaluated::Value(value) = expr.eval(state, context)?;
                Ok(Evaluated::Value(normalize(value.as_str())))
            }
            FuncInner::Concat(exprs) => {
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
        }
    }
}
