//! Implements the [`Interpreter`] type and the core logic of the library.

use crate::ast::{BlockRewrite, RawAST, Value};
use crate::core::Environment;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use crate::expand::Expand;
use crate::resolve::{Resolve, Scope};
use crate::substitution::AliasSubstitutionVisitor;
use crate::util::deprecation::DeprecationServiceScope;
use crate::util::log::debug;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::rc::Rc;
use syn::visit_mut::VisitMut;

/// Executes the lifecycle of the macro starting from analyzing the AST down to generating the
/// final output.
///
/// # Notes
///
/// Workflow of the interpreter consists of three main phases:
///
/// 1. Expand – desugaring, conversion of higher-level AST elements to more primitive ones.
///     - [`Expand::expand`] is called on relevant AST nodes to obtain the new expanded form,
///       represented as new AST elements, chosen by the implementation.
/// 2. Resolve – static analysis, fills the scope and catches duplicate aliases.
///     - [`Resolve::resolve`] is called on relevant AST nodes.
///     - Since the syntax is simple - there is no need for full AST traversal - instead
///       ad-hoc traversal of relevant nodes is enough.
///     - Simplified bidirectional typing is used at this phase.
///     - The scope is currently single and global due to the simplicity of requirements.
/// 3. Evaluate – runs functions and concatenations to obtain final identifier strings.
///    - [`Eval::eval`] is called on relevant AST nodes to retrieve execution results.
///    - Global execution context is maintained. It is singular and global due to simplicity of
///      requirements at this point.
/// 4. Code-gen – rewrites the user block with `ComposeIdentsVisitor` and returns a TokenStream.
pub struct Interpreter {
    /// Global execution environment.
    environment: Rc<Environment>,
    deprecation_service: DeprecationServiceScope,
}

impl Interpreter {
    /// Creates a new interpreter instance with the AST as an input.
    pub fn new(environment: Rc<Environment>, deprecation_service: DeprecationServiceScope) -> Self {
        Interpreter {
            environment,
            deprecation_service,
        }
    }
    /// Takes a [`BlockRewrite`] and turns it into a substitutions-map after fully evaluating the
    /// block-rewrite AST node.
    pub fn make_substitutions(
        &self,
        block_rewrite: &BlockRewrite,
    ) -> Result<HashMap<String, Rc<Value>>, Error> {
        let mut scope = Scope::default();
        block_rewrite
            .spec()
            .resolve(self.environment.as_ref(), &mut scope, None)?;

        let mut context = Context::new(scope.metadata_rc());
        let evaluated = block_rewrite.spec().eval(&self.environment, &mut context)?;
        let Evaluated::Bindings(bindings_map) = evaluated else {
            unreachable!()
        };

        let mut substitutions = HashMap::new();
        for (alias, value) in bindings_map.iter() {
            let Evaluated::Value(value) = value else {
                unreachable!()
            };
            substitutions.insert(alias.ident().to_string(), value.clone());
        }
        Ok(substitutions)
    }
    /// Performs alias substitutions in the given block.
    pub fn substitute(
        &self,
        block: &mut syn::Block,
        substitutions: HashMap<String, Rc<Value>>,
    ) -> Result<(), Error> {
        let mut visitor = AliasSubstitutionVisitor::new(substitutions);
        visitor.visit_block_mut(block);
        if let Some(err) = visitor.error() {
            return Err(err.clone());
        }

        self.deprecation_service.emit(block);
        Ok(())
    }
    /// Executes the interpreter within the context of a single block-rewrite AST node.
    pub fn execute_block_rewrite(
        &self,
        block_rewrite: &BlockRewrite,
    ) -> Result<TokenStream, Error> {
        let substitutions = self.make_substitutions(block_rewrite)?;

        let mut block = block_rewrite.block().clone();
        self.substitute(&mut block, substitutions)?;

        let content = &block.stmts;
        Ok(quote! { #(#content)* })
    }
    /// Executes the interpreter - main entry-point of the library.
    pub fn execute(self, args: RawAST) -> Result<TokenStream, Error> {
        debug!("Executing interpreter with arguments: {:?}", args);

        let expanded = args.expand()?;

        let mut result = vec![];

        for block_rewrite in expanded.block_rewrite_items() {
            let stream = self.execute_block_rewrite(block_rewrite)?;
            result.push(stream);
        }

        Ok(quote! { #(#result)* })
    }
}
