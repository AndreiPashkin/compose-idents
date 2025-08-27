use crate::ast::AliasSpec;
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use std::collections::HashMap;

impl Eval for AliasSpec {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        let mut bindings = HashMap::new();

        for item in self.items() {
            let Evaluated::Bindings(evaluated_value) = item.eval(environment, context)? else {
                unreachable!();
            };
            bindings.extend(evaluated_value.into_iter());
        }

        Ok(Evaluated::Bindings(bindings))
    }
}
