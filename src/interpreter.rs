//! Implements the [`Interpreter`] type and the core logic of the library.

use crate::ast::ComposeIdentsArgs;
use crate::core::State;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use crate::resolve::{Resolve, Scope};
use crate::substitution::AliasSubstitutionVisitor;
use crate::util::deprecation::DeprecationServiceScope;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
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
    /// Immutable AST tree
    args: ComposeIdentsArgs,
    /// Interpreter state
    state: State,
    /// Runtime context
    context: Context,
    /// Generated identifier replacements
    replacements: HashMap<Ident, Ident>,
    deprecation_service: DeprecationServiceScope,
}

impl Interpreter {
    /// Creates a new interpreter instance with the AST as an input.
    pub fn new(args: ComposeIdentsArgs, deprecation_service: DeprecationServiceScope) -> Self {
        Interpreter {
            args,
            state: State::new(),
            context: Context::default(),
            replacements: HashMap::new(),
            deprecation_service,
        }
    }

    /// Executes the interpreter - main entry-point of the library.
    pub fn execute(mut self) -> Result<TokenStream, Error> {
        let mut scope = Scope::default();
        self.args.spec().resolve(&mut scope)?;

        // Create context with metadata from scope
        self.context = Context::new(scope.metadata().clone());

        for item in self.args.spec().items() {
            let Evaluated::Value(value_str) = item.value().eval(&self.state, &mut self.context)?;

            self.context.add_variable(
                item.alias().ident().to_string(),
                Evaluated::Value(value_str.clone()),
            );

            let replacement_ident: Ident = syn::parse_str(&value_str).expect("Invalid ident");
            self.replacements
                .insert(item.alias().ident().clone(), replacement_ident);
        }

        let block = self.args.block_mut();

        let mut visitor = AliasSubstitutionVisitor::new(self.replacements);
        visitor.visit_block_mut(block);

        self.deprecation_service.emit("compose_idents!: ", block);
        let block_content = &block.stmts;
        let expanded = quote! { #(#block_content)* };

        Ok(expanded)
    }
}
