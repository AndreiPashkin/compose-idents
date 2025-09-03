use crate::ast::{Alias, Ast, AstMetadata, Value};
use crate::core::Environment;
use crate::error::Error;
use proc_macro2::Ident;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

/// Result of evaluating a statement.
#[derive(Clone)]
pub enum Evaluated {
    /// A singular value
    Value(Rc<Value>),
    /// A set of variable bindings
    Bindings(HashMap<Rc<Alias>, Evaluated>),
}

/// Runtime context of evaluation.
#[derive(Default, Clone)]
pub struct Context {
    context: HashMap<Ident, Evaluated>,
    metadata: Rc<RefCell<AstMetadata>>,
}

impl Context {
    /// Creates a new `Context` with the given metadata.
    pub fn new(metadata: Rc<RefCell<AstMetadata>>) -> Self {
        Self {
            context: HashMap::new(),
            metadata,
        }
    }

    /// Adds a variable to the evaluation context.
    pub fn add_variable(&mut self, name: &Ident, value: Evaluated) {
        if !matches!(&value, Evaluated::Value(_)) {
            panic!("Only Value can be added to the context");
        }
        self.context.insert(name.clone(), value);
    }

    /// Gets a variable reference from the evaluation context.
    pub fn get_variable(&self, name: &Ident) -> Option<&Evaluated> {
        self.context.get(name)
    }

    /// Returns a reference to the metadata associated with the current scope.
    pub fn metadata_rc(&self) -> Rc<RefCell<AstMetadata>> {
        self.metadata.clone()
    }

    /// Returns a reference to the metadata associated with the current scope.
    pub fn metadata(&self) -> Ref<'_, AstMetadata> {
        self.metadata.borrow()
    }
}

/// A syntactic structure that can be evaluated.
///
/// For example, it could be a function call passed by a user to the macro as an argument.
pub trait Eval: Ast {
    fn eval(&self, environment: &Environment, context: &mut Context) -> Result<Evaluated, Error>;
}
