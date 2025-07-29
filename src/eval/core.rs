use crate::ast::{Ast, AstMetadata};
use crate::core::State;
use crate::error::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Result of evaluating a statement.
pub enum Evaluated {
    /// A singular value
    Value(String),
}

/// Runtime context of evaluation.
#[derive(Default)]
pub struct Context {
    context: HashMap<String, Evaluated>,
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
    pub fn add_variable(&mut self, name: String, value: Evaluated) {
        self.context.insert(name, value);
    }

    /// Gets a variable reference from the evaluation context.
    pub fn get_variable(&mut self, name: &str) -> Option<&Evaluated> {
        self.context.get(name)
    }

    /// Gets a reference to the AST metadata.
    pub fn metadata(&self) -> Rc<RefCell<AstMetadata>> {
        self.metadata.clone()
    }
}

/// A syntactic structure that can be evaluated.
///
/// For example, it could be a function call passed by a user to the macro as an argument.
pub trait Eval: Ast {
    fn eval(&self, state: &State, context: &mut Context) -> Result<Evaluated, Error>;
}
