//! Implementation of resolve-phase logic.

use crate::ast::{
    AliasSpec, Ast, LoopAlias, LoopSourceValue, LoopSpec, LoopSpecItem, Scope, TupleValue,
};
use crate::error::Error;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// A syntactic structure that supports static analysis.
///
/// Encapsulates the logic of the resolve pass of the interpreter.
///
/// # Notes
///
/// Right now the only job of the implementation is to publish the aliases defined by the AST node.
pub trait Resolve: Ast {
    fn resolve<'a>(&'a self, scope: &mut Scope<'a>) -> Result<(), Error>;
}

impl Resolve for AliasSpec {
    fn resolve<'a>(&'a self, scope: &mut Scope<'a>) -> Result<(), Error> {
        let names = scope.names_mut();
        for item in self.items() {
            let name = item.alias().ident().to_string();
            match names.entry(name) {
                Entry::Occupied(_) => {
                    return Err(Error::TypeError(
                        format!("Alias `{}` is already defined", item.alias().ident()),
                        item.span(),
                    ));
                }
                Entry::Vacant(entry) => {
                    entry.insert(item);
                }
            }
        }
        Ok(())
    }
}

impl Resolve for LoopSpecItem {
    fn resolve<'a>(&'a self, scope: &mut Scope<'a>) -> Result<(), Error> {
        // Verify no duplicate aliases are defined.
        let mut names: HashMap<String, &'a dyn Ast> = HashMap::new();
        match self.alias() {
            LoopAlias::Simple(alias) => {
                names.insert(alias.ident().to_string(), self);
            }
            LoopAlias::Tuple(tuple) => {
                for alias in tuple.iter_recursive() {
                    match names.entry(alias.ident().to_string()) {
                        Entry::Occupied(_) => {
                            return Err(Error::TypeError(
                                format!("Alias `{}` is already defined", alias.ident()),
                                alias.span(),
                            ));
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(self);
                        }
                    }
                }
            }
        }

        // Verify tuples' structure matches.
        for value in self.list().values() {
            match (self.alias(), value) {
                (LoopAlias::Simple(_), LoopSourceValue::Simple(_)) => {}
                (LoopAlias::Tuple(alias), LoopSourceValue::Tuple(value)) => {
                    if alias.iter_destructuring().count() != value.iter_destructuring().count() {
                        return Err(Error::TypeError(
                            "Mismatched number of elements in the tuple".to_string(),
                            value.span(),
                        ));
                    }
                    let iter = alias.iter_destructuring().zip(value.iter_destructuring());
                    for (a, b) in iter {
                        let span = b.span();
                        match (a, b) {
                            (TupleValue::Value(_), TupleValue::Value(_)) => {}
                            (TupleValue::Tuple(_), TupleValue::Tuple(_)) => {}
                            _ => {
                                return Err(Error::TypeError(
                                    "Shape of the value tuple doesn't match the shape of the alias tuple".to_string(),
                                    span,
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::TypeError(
                        "Mismatched alias and value types".to_string(),
                        self.span(),
                    ));
                }
            }
        }

        // Verify no already existing aliases are defined.
        let scope_names = scope.names_mut();
        for (name, item) in names {
            match scope_names.entry(name.clone()) {
                Entry::Occupied(_) => {
                    return Err(Error::TypeError(
                        format!("Alias `{}` is already defined", name),
                        item.span(),
                    ));
                }
                Entry::Vacant(entry) => {
                    entry.insert(item);
                }
            }
        }

        Ok(())
    }
}

impl Resolve for LoopSpec {
    fn resolve<'a>(&'a self, scope: &mut Scope<'a>) -> Result<(), Error> {
        let mut scope_temp = scope.clone();
        for item in self.loops() {
            item.resolve(&mut scope_temp)?;
        }
        scope.names_mut().extend(scope_temp.names_mut().drain());

        Ok(())
    }
}
