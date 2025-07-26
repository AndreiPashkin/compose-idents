//! Implementation of resolve-phase logic.

use crate::ast::{AliasSpec, Ast, Scope};
use crate::error::Error;
use std::collections::hash_map::Entry;

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
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        let names = scope.names_mut();
        for item in self.items() {
            let name = item.alias().ident().to_string();
            match names.entry(name) {
                Entry::Occupied(_) => {
                    return Err(Error::RedefinedNameError(
                        item.alias().ident().to_string(),
                        item.span(),
                    ));
                }
                Entry::Vacant(entry) => {
                    entry.insert(item.clone());
                }
            }
        }
        Ok(())
    }
}
