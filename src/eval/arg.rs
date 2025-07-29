use crate::ast::{Arg, ArgInner};
use crate::core::State;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};

impl Eval for Arg {
    fn eval(&self, _state: &State, context: &mut Context) -> Result<Evaluated, Error> {
        match self.inner() {
            ArgInner::Ident(ident) => {
                let value = ident.to_string();
                let res = match context.get_variable(&value) {
                    Some(Evaluated::Value(v)) => Evaluated::Value(v.clone()),
                    None => Evaluated::Value(value),
                };
                Ok(res)
            }
            ArgInner::LitStr(value) => Ok(Evaluated::Value(value.clone())),
            ArgInner::LitInt(value) => Ok(Evaluated::Value(value.to_string())),
            ArgInner::Tokens(tokens) => Ok(Evaluated::Value(tokens.to_string())),
            ArgInner::Underscore => Ok(Evaluated::Value("_".to_string())),
        }
    }
}
