use crate::ast::{Ast, Value, ValueKind};
use crate::core::Environment;
use crate::error::{internal_error, Error};
use crate::eval::{Context, Eval, Evaluated};
use std::rc::Rc;

impl Eval for Value {
    fn eval(&self, _: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        let metadata = context.metadata();
        let value = match self.kind() {
            ValueKind::Ident(ident) => match context.get_variable(ident) {
                Some(Evaluated::Value(value)) => value.clone(),
                None => Rc::new(self.clone()),
            },
            _ => Rc::new(self.clone()),
        };
        let Some(metadata) = metadata.get_value_metadata(self.id()) else {
            return Err(internal_error!(
                "Value metadata is expected to be set after the resolve phase"
            ));
        };
        let value = match value.try_cast(&metadata.target_type) {
            Ok(value) => value,
            Err(err) => panic!("Unexpected coercion error: {:?}", err),
        };

        Ok(Evaluated::Value(Rc::new(value)))
    }
}
