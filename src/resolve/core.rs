use crate::ast::{Ast, AstMetadata};
use crate::core::{Environment, Type};
use crate::error::Error;
use std::cell::{RefCell, RefMut};
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
    pub fn get_name(&self, name: &str) -> Option<Rc<dyn Ast>> {
        self.aliases.get(name).cloned()
    }

    /// Returns a reference to the metadata associated with the current scope.
    pub fn metadata_rc(&self) -> Rc<RefCell<AstMetadata>> {
        self.metadata.clone()
    }

    /// Returns a mutable reference to the metadata associated with the current scope.
    pub fn metadata_mut(&self) -> RefMut<'_, AstMetadata> {
        self.metadata.borrow_mut()
    }
    pub fn deep_clone(&self) -> Self {
        let aliases = self.aliases.clone();
        let metadata = Rc::new(RefCell::new(self.metadata.borrow().clone()));
        Scope { aliases, metadata }
    }
}

/// A syntactic structure that supports static analysis performed during the resolve phase.
///
/// Right now the only job of the implementation is to publish the aliases defined by the AST node.
pub trait Resolve: Ast {
    fn resolve(
        &self,
        environment: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error>;
}
