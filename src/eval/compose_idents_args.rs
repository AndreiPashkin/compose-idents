use crate::ast::ComposeIdentsArgs;
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};

impl Eval for ComposeIdentsArgs {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        self.spec().eval(environment, context)
    }
}
