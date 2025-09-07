//! Implements the [`Interpreter`] type and the core logic of the library.

use crate::ast::RawAST;
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
    /// Executes the interpreter - main entry-point of the library.
    pub fn execute(self, args: RawAST) -> Result<TokenStream, Error> {
        debug!("Executing interpreter with arguments: {:?}", args);

        // Expand loops into multiple invocations
        let expanded = args.expand()?;

        let mut out = vec![];
        for inv in expanded.invocations {
            // Resolve per-invocation alias spec
            let mut scope = Scope::default();
            inv.spec()
                .resolve(self.environment.as_ref(), &mut scope, None)?;

            // Evaluate per-invocation alias spec to get bindings
            let mut context = Context::new(scope.metadata_rc());
            let evaluated = inv.spec().eval(&self.environment, &mut context)?;
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

            let mut block = inv.block().clone();
            let mut visitor = AliasSubstitutionVisitor::new(substitutions);
            visitor.visit_block_mut(&mut block);
            if let Some(err) = visitor.error() {
                return Err(err.clone());
            }

            self.deprecation_service.emit(&mut block);
            let content = &block.stmts;
            out.push(quote! { #(#content)* });
        }

        Ok(quote! { #(#out)* })
    }
}
