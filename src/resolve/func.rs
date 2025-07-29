use crate::ast::{Ast, Func, FuncInner};
use crate::error::Error;
use crate::resolve::{Resolve, Scope};
use std::rc::Rc;

impl Resolve for Func {
    /// Resolves a function call by resolving its arguments and binding the call to a built-in
    /// function.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        let name = self.name().to_string();

        match name.as_str() {
            "normalize" => {}
            _ => {
                if let Some(args) = self.args() {
                    for arg in args {
                        arg.resolve(scope)?;
                    }
                }
            }
        };

        let inner = match (name.as_str(), self.args(), self.tokens()) {
            ("upper", Some(args), _) => match &args {
                [expr] => FuncInner::Upper(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "upper(arg)")),
            },
            ("lower", Some(args), _) => match &args {
                [expr] => FuncInner::Lower(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "lower(arg)")),
            },
            ("snake_case", Some(args), _) => match &args {
                [expr] => FuncInner::SnakeCase(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "snake_case(arg)")),
            },
            ("camel_case", Some(args), _) => match &args {
                [expr] => FuncInner::CamelCase(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "camel_case(arg)")),
            },
            ("pascal_case", Some(args), _) => match &args {
                [expr] => FuncInner::PascalCase(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "pascal_case(arg)")),
            },
            ("hash", Some(args), _) => match &args {
                [expr] => FuncInner::Hash(expr.clone()),
                _ => return Err(Error::make_sig_error(self, "hash(arg)")),
            },
            ("normalize", _, Some(tokens)) => FuncInner::Normalize(tokens.clone()),
            ("concat", Some(args), _) if !args.is_empty() => FuncInner::Concat(args.to_vec()),
            ("concat", _, _) => return Err(Error::make_sig_error(self, "concat(arg1, arg2, ...)")),
            _ => {
                return Err(Error::UndefinedFunctionError(
                    self.name().to_string(),
                    self.span(),
                ))
            }
        };

        // Store the resolved function inner in metadata
        scope
            .metadata()
            .borrow_mut()
            .set_func_metadata(self.id(), Rc::new(inner));

        Ok(())
    }
}
