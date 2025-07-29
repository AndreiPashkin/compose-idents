use crate::ast::Expr;
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for Expr {
    /// Resolves an expression by delegating the resolution further in cases when the expression
    /// is a function call.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        match self {
            Expr::FuncCallExpr(boxed_func) => boxed_func.resolve(scope),
            _ => Ok(()),
        }
    }
}
