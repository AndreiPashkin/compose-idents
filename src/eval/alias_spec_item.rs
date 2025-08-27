use crate::ast::AliasSpecItem;
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use std::collections::HashMap;

impl Eval for AliasSpecItem {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        let evaluated_value = self.value().eval(environment, context)?;

        let mut bindings = HashMap::new();
        context.add_variable(self.alias().ident(), evaluated_value.clone());
        bindings.insert(self.alias(), evaluated_value);

        Ok(Evaluated::Bindings(bindings))
    }
}
