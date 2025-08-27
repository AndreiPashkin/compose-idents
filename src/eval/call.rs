use crate::ast::{Ast, Call};
use crate::core::Environment;
use crate::error::{internal_error, Error};
use crate::eval::{Context, Eval, Evaluated};
use std::rc::Rc;

impl Eval for Call {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error> {
        let metadata_rc = context.metadata_rc();
        let metadata = metadata_rc.borrow();
        let Some(call_metadata) = metadata.get_call_metadata(self.id()) else {
            return Err(internal_error!(
                "Call metadata is expected to be set after the resolve phase"
            ));
        };

        let target_type = call_metadata.target_type.clone();

        let args = call_metadata
            .args
            .iter()
            .map(|expr| {
                let Evaluated::Value(arg) = expr.eval(environment, context)? else {
                    panic!("Function arguments should always be singular values")
                };
                Ok(arg)
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let result = call_metadata
            .func
            .call(args.as_slice(), environment, Some(self.span()))?;
        let result = result.try_cast(&target_type)?;

        Ok(Evaluated::Value(Rc::new(result)))
    }
}
