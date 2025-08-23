use crate::ast::{Expr, ExprInner};
use crate::core::{Environment, Type};
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for Expr {
    /// Resolves an expression by delegating the resolution further in cases when the expression
    /// is a function call.
    fn resolve(
        &self,
        environment: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error> {
        match self.inner() {
            ExprInner::ValueExpr(value) => value.resolve(environment, scope, expected_type),
            ExprInner::FuncCallExpr(call) => call.resolve(environment, scope, expected_type),
        }
    }
}
