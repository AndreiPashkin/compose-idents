use crate::ast::AliasValue;
use crate::core::State;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};

impl Eval for AliasValue {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        let Evaluated::Value(value) = self.expr().eval(state, context)?;

        // Validate that the resulting string is a valid identifier.
        if syn::parse_str::<syn::Ident>(&value).is_err() {
            return Err(Error::EvalError(
                format!("`{}` is not a valid identifier", value),
                self.span(),
            ));
        }

        Ok(Evaluated::Value(value))
    }
}
