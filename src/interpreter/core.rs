//! Implements the [`Interpreter`] type and the core logic of the library.

use crate::ast::{ComposeIdentsArgs, Value};
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use crate::resolve::{Resolve, Scope};
use crate::substitution::AliasSubstitutionVisitor;
use crate::util::deprecation::DeprecationServiceScope;
use crate::util::log::debug;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::rc::Rc;
use syn::{visit_mut::VisitMut, Ident};

/// Executes the lifecycle of the macro starting from analyzing the AST down to generating the
/// final output.
///
/// # Notes
///
/// Workflow of the interpreter consists of three main phases:
///
/// 1. Resolve – static analysis, fills the scope and catches duplicate aliases.
///     - [`Resolve::resolve`] is called on relevant AST nodes.
///     - Since the syntax is simple - there is no need for full AST traversal - instead
///       ad-hoc traversal of relevant nodes is enough.
///     - The scope is currently single and global due to the simplicity of requirements.
/// 2. Evaluate – runs functions and concatenations to obtain final identifier strings.
///    - [`Eval::eval`] is called on relevant AST nodes to retrieve execution results.
///    - Global execution context is maintained. It is singular and global due to simplicity of
///      requirements at this point.
/// 3. Code-gen – rewrites the user block with `ComposeIdentsVisitor` and returns a TokenStream.
pub struct Interpreter {
    /// Generated identifier substitutions
    substitutions: HashMap<Ident, Rc<Value>>,
    environment: Rc<Environment>,
    deprecation_service: DeprecationServiceScope,
}

impl Interpreter {
    /// Creates a new interpreter instance with the AST as an input.
    pub fn new(environment: Rc<Environment>, deprecation_service: DeprecationServiceScope) -> Self {
        Interpreter {
            environment,
            substitutions: HashMap::new(),
            deprecation_service,
        }
    }
    /// Executes the interpreter - main entry-point of the library.
    pub fn execute(mut self, mut args: ComposeIdentsArgs) -> Result<TokenStream, Error> {
        debug!("Executing interpreter with arguments: {:?}", args);

        let mut scope = Scope::default();

        args.spec()
            .resolve(self.environment.as_ref(), &mut scope, None)?;

        let mut context = Context::new(scope.metadata_rc());

        for item in args.spec().items() {
            let Evaluated::Value(value) = item.value().eval(&self.environment, &mut context)?;

            context.add_variable(item.alias().ident(), Evaluated::Value(value.clone()));

            self.substitutions
                .insert(item.alias().ident().clone(), value);
        }

        let block = args.block_mut();

        let mut visitor = AliasSubstitutionVisitor::new(self.substitutions);
        visitor.visit_block_mut(block);

        if let Some(err) = visitor.error() {
            return Err(err.clone());
        }

        self.deprecation_service.emit("compose_idents!: ", block);
        let block_content = &block.stmts;
        let expanded = quote! { #(#block_content)* };

        Ok(expanded)
    }
}
