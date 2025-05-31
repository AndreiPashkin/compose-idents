//! Provides the [`Eval`] trait and its implementations for evaluating expressions.
use crate::core::{Arg, Expr, Func, State};
use crate::funcs::{hash, normalize, to_camel_case, to_pascal_case, to_snake_case};

/// A syntactic structure that can be evaluated.
///
/// For example, it could be a function call passed by a user to the macro as an argument.
pub trait Eval {
    fn eval(&self, state: &State) -> String;
}

impl Eval for Arg {
    fn eval(&self, _: &State) -> String {
        self.value().to_string()
    }
}

impl Eval for Func {
    fn eval(&self, state: &State) -> String {
        match self {
            Func::Upper(expr) => expr.eval(state).to_uppercase(),
            Func::Lower(expr) => expr.eval(state).to_lowercase(),
            Func::SnakeCase(expr) => to_snake_case(expr.eval(state).as_str()),
            Func::CamelCase(expr) => to_camel_case(expr.eval(state).as_str()),
            Func::PascalCase(expr) => to_pascal_case(expr.eval(state).as_str()),
            Func::Hash(expr) => hash(expr.eval(state).as_str(), state),
            Func::Normalize(expr) => normalize(expr.eval(state).as_str()),
            Func::Undefined => panic!("Attempt to evaluate an undefined function"),
            Func::SignatureMismatch(_) => {
                panic!("Attempt to evaluate a function with a mismatched signature")
            }
        }
    }
}

impl Eval for Expr {
    fn eval(&self, state: &State) -> String {
        match self {
            Expr::ArgExpr(value) => value.eval(state),
            Expr::FuncCallExpr(value) => value.eval(state),
        }
    }
}
