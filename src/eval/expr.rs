use crate::ast::{Expr, ExprKind};
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use std::ops::Deref;

impl Eval for Expr {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        match self.kind() {
            ExprKind::ValueExpr(value) => value.deref().eval(environment, context),
            ExprKind::FuncCallExpr(value) => value.eval(environment, context),
        }
    }
}
