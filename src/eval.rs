use crate::core::{Arg, Expr, Func, State};
use crate::funcs::{hash, to_camel_case, to_snake_case};

/// A syntactic structure that can be evaluated.
pub trait Eval {
    fn eval(&self, state: &State) -> String;
}

impl Eval for Arg {
    fn eval(&self, _: &State) -> String {
        self.value.clone()
    }
}

impl Eval for Func {
    fn eval(&self, state: &State) -> String {
        match self {
            Func::Upper(expr) => expr.eval(state).to_uppercase(),
            Func::Lower(expr) => expr.eval(state).to_lowercase(),
            Func::SnakeCase(expr) => to_snake_case(expr.eval(state).as_str()),
            Func::CamelCase(expr) => to_camel_case(expr.eval(state).as_str()),
            Func::Hash(expr) => hash(expr.eval(state).as_str(), state),
        }
    }
}

impl Eval for Expr {
    fn eval(&self, state: &State) -> String {
        match self {
            Expr::ArgExpr { value } => value.eval(state),
            Expr::FuncCallExpr { value } => value.eval(state),
        }
    }
}
