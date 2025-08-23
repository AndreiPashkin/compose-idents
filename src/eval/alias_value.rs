use crate::ast::AliasValue;
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};

impl Eval for AliasValue {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        self.expr().eval(environment, context)
    }
}
