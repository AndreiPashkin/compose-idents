//! Implementation of resolve-phase logic.
//!
//! Resolve-phase is supposed to be executed after the parse-phase and before the eval-phase. Its
//! general responsibility is to perform static analysis (which is pretty minimal at this point) and
//! binding function calls.

use crate::ast::{AliasSpec, AliasSpecItem, Ast, Expr, Func, FuncInner, Scope};
use crate::error::Error;
use std::rc::Rc;

/// A syntactic structure that supports static analysis.
///
/// Encapsulates the logic of the resolve pass of the interpreter.
///
/// # Notes
///
/// Right now the only job of the implementation is to publish the aliases defined by the AST node.
pub trait Resolve: Ast {
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error>;
}

impl Resolve for AliasSpec {
    /// Resolves [`AliasSpec`] by delegating resolution process further down to each of the
    /// items it contains.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        for item in self.items() {
            item.resolve(scope)?;
        }
        Ok(())
    }
}

impl Resolve for AliasSpecItem {
    /// Resolves an [`AliasSpecItem`] by adding its alias to the global scope and checking for
    /// redefinition of aliases.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        let name = self.alias().ident().to_string();
        self.value().expr().resolve(scope)?;
        scope.try_add_name(name, self.value())?;
        Ok(())
    }
}

impl Resolve for Expr {
    /// Resolves an expression by delegating the resolution further in cases if the expression
    /// is a function.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        match self {
            Expr::FuncCallExpr(boxed_func) => boxed_func.resolve(scope),
            _ => Ok(()),
        }
    }
}

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
                _ => return Err(self.make_sig_error("upper(arg)")),
            },
            ("lower", Some(args), _) => match &args {
                [expr] => FuncInner::Lower(expr.clone()),
                _ => return Err(self.make_sig_error("lower(arg)")),
            },
            ("snake_case", Some(args), _) => match &args {
                [expr] => FuncInner::SnakeCase(expr.clone()),
                _ => return Err(self.make_sig_error("snake_case(arg)")),
            },
            ("camel_case", Some(args), _) => match &args {
                [expr] => FuncInner::CamelCase(expr.clone()),
                _ => return Err(self.make_sig_error("camel_case(arg)")),
            },
            ("pascal_case", Some(args), _) => match &args {
                [expr] => FuncInner::PascalCase(expr.clone()),
                _ => return Err(self.make_sig_error("pascal_case(arg)")),
            },
            ("hash", Some(args), _) => match &args {
                [expr] => FuncInner::Hash(expr.clone()),
                _ => return Err(self.make_sig_error("hash(arg)")),
            },
            ("normalize", _, Some(tokens)) => FuncInner::Normalize(tokens.clone()),
            ("concat", Some(args), _) if !args.is_empty() => FuncInner::Concat(args.to_vec()),
            ("concat", _, _) => return Err(self.make_sig_error("concat(arg1, arg2, ...)")),
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
