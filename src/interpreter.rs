//! Implements the [`Interpreter`] type and the core logic of the library.

use crate::ast::{Alias, ComposeIdentsArgs, Scope};
use crate::core::{ComposeIdentsVisitor, State};
use crate::deprecation::DeprecationServiceScope;
use crate::error::Error;
use crate::eval::{Context, Eval, Evaluated};
use crate::resolve::Resolve;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::io;
use std::io::Write;
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
    /// Generated identifier replacements
    deprecation_service: DeprecationServiceScope,
}

impl Interpreter {
    /// Creates a new interpreter instance with the AST as an input.
    pub fn new(args: ComposeIdentsArgs, deprecation_service: DeprecationServiceScope) -> Self {
        Interpreter {
            args,
            state: State::new(),
            deprecation_service,
        }
    }

    /// Executes the loops and returns resulting bindings.
    fn make_loop_bindings<'a>(
        &'a self,
        scope: &mut Scope<'a>,
        context: &mut Context,
    ) -> Result<Vec<HashMap<Alias, Evaluated>>, Error> {
        let bindings = match self.args.loops() {
            Some(loops) => {
                loops.resolve(scope)?;
                let Evaluated::List(_, bindings) = loops.eval(&self.state, context)? else {
                    unreachable!()
                };
                bindings
                    .iter()
                    .map(|item| {
                        let Evaluated::Bindings(_, item) = item else {
                            unreachable!()
                        };
                        item.clone()
                    })
                    .collect::<Vec<_>>()
            }
            None => vec![HashMap::new()],
        };
        Ok(bindings)
    }

    /// Creates initial version of identifier replacement map.
    fn make_replacements(bindings: &HashMap<Alias, Evaluated>) -> HashMap<Ident, Ident> {
        bindings
            .iter()
            .map(|(alias, value)| {
                let Evaluated::Value(span, value) = value else {
                    unreachable!()
                };
                (alias.ident().clone(), Ident::new(value.as_str(), *span))
            })
            .collect()
    }

    /// Executes the interpreter - main entry-point of the library.
    pub fn execute(mut self) -> Result<TokenStream, Error> {
        let mut scope = Scope::default();
        if let Some(spec) = self.args.spec() {
            spec.resolve(&mut scope)?;
        }
        let mut loop_context = Context::default();
        eprintln!("Executing macro with loops: {:?}", self.args.loops());
        let bindings = self.make_loop_bindings(&mut scope, &mut loop_context)?;

        let mut blocks = Vec::new();

        // if self.args.loops().is_none() {
        //     panic!("No loops have been specified..");
        // }

        for bindings_item in bindings {
            let mut replacements = Self::make_replacements(&bindings_item);
            let mut context = Context::default();
            eprintln!("Loop bindings: {:?}", bindings_item);
            context.context_mut().extend(bindings_item);

            let spec_items = match self.args.spec() {
                Some(spec) => spec.items(),
                None => &[],
            };

            for item in spec_items {
                eprintln!("Evaluating alias: {}", item.alias());
                eprintln!("Context: {:?}", context);
                io::stderr().flush().unwrap();
                let Evaluated::Value(span, value_str) =
                    item.value().eval(&self.state, &mut context)?
                else {
                    unreachable!()
                };

                context.context_mut().insert(
                    item.alias().clone(),
                    Evaluated::Value(span, value_str.clone()),
                );

                let replacement_ident: Ident = syn::parse_str(&value_str).expect("Invalid ident");
                replacements.insert(item.alias().ident().clone(), replacement_ident);
            }
            let mut block = self.args.block_mut().clone();

            let mut visitor = ComposeIdentsVisitor::new(replacements);
            visitor.visit_block_mut(&mut block);

            self.deprecation_service
                .emit("compose_idents!: ", &mut block);
            let block_content = &block.stmts;

            blocks.push(quote! { #(#block_content)* });
        }

        let expanded = quote! { #(#blocks)* };

        eprintln!("Expanded code:\n{}", expanded);

        Ok(expanded)
    }
}
