use crate::ast::{Ast, AstMetadata};
use crate::error::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Lexical scope used during the resolve phase.
#[derive(Default, Clone)]
pub struct Scope {
    aliases: HashMap<String, Rc<dyn Ast>>,
    metadata: Rc<RefCell<AstMetadata>>,
}

impl Scope {
    /// Tries to add a new alias into the current scope, returning an error on re-definition.
    pub fn try_add_name(&mut self, name: String, item: Rc<dyn Ast>) -> Result<(), Error> {
        if self.aliases.contains_key(&name) {
            return Err(Error::RedefinedNameError(name, item.span()));
        }
        self.aliases.insert(name, item);
        Ok(())
    }

    /// Returns a reference to the metadata associated with the current scope.
    pub fn metadata(&self) -> Rc<RefCell<AstMetadata>> {
        self.metadata.clone()
    }
}

/// A syntactic structure that supports static analysis performed during the resolve phase.
///
/// Right now the only job of the implementation is to publish the aliases defined by the AST node.
pub trait Resolve: Ast {
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error>;
}
