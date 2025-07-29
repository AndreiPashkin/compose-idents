use crate::ast::Expr;
use crate::core::State;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};

impl Eval for Expr {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self {
            Expr::ArgExpr(value) => value.eval(state, context),
            Expr::FuncCallExpr(value) => value.eval(state, context),
        }
    }
}
