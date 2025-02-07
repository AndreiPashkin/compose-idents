use crate::core::{Arg, Expr, Func};

/// A syntactic structure that can be evaluated.
pub trait Eval {
    fn eval(&self) -> String;
}

impl Eval for Arg {
    fn eval(&self) -> String {
        self.value.clone()
    }
}

impl Eval for Func {
    fn eval(&self) -> String {
        match self {
            Func::Upper(expr) => expr.eval().to_uppercase(),
            Func::Lower(expr) => expr.eval().to_lowercase(),
        }
    }
}

impl Eval for Expr {
    fn eval(&self) -> String {
        match self {
            Expr::ArgExpr { value } => value.eval(),
            Expr::FuncCallExpr { value } => value.eval(),
        }
    }
}
