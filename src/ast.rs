//! Defines the AST nodes that describe the syntax of the macro.
//!
//! The convention is that AST elements are themselves immutable and their mutable metadata is
//! stored in a separate [`AstMetadata`] structure:
//!
//!   - Each AST element has a unique ID (of type [`NodeId`]).
//!   - [`AstMetadata`] allows to reference metadata by the ID of the related AST-elements.

use crate::error::Error;
use crate::util::unique_id::next_unique_id;
use proc_macro2::{Ident, Span, TokenStream};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use syn::Block;

pub type NodeId = u64;

/// An AST node that has a syntactic span.
pub trait Ast {
    /// Ast node identifier.
    fn id(&self) -> NodeId;
    /// Returns the span of the item.
    fn span(&self) -> Span;
}

/// Lexical scope.
#[derive(Default, Clone)]
pub struct Scope {
    aliases: HashMap<String, Rc<dyn Ast>>,
    metadata: Rc<RefCell<AstMetadata>>,
}

impl Scope {
    pub fn try_add_name(&mut self, name: String, item: Rc<dyn Ast>) -> Result<(), Error> {
        if self.aliases.contains_key(&name) {
            return Err(Error::RedefinedNameError(name, item.span()));
        }
        self.aliases.insert(name, item);
        Ok(())
    }

    pub fn metadata(&self) -> Rc<RefCell<AstMetadata>> {
        self.metadata.clone()
    }
}

/// Argument to the [`compose_idents`] macro.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub struct Arg {
    id: NodeId,
    inner: ArgInner,
}

#[derive(Debug, Clone)]
pub enum ArgInner {
    Ident(Ident),
    LitStr(String),
    LitInt(u64),
    Tokens(TokenStream),
    Underscore,
}

impl Arg {
    pub fn new(id: NodeId, inner: ArgInner) -> Self {
        Self { id, inner }
    }

    pub fn from_ident(ident: Ident) -> Self {
        Self::new(next_unique_id(), ArgInner::Ident(ident))
    }

    pub fn from_underscore() -> Self {
        Self::new(next_unique_id(), ArgInner::Underscore)
    }

    pub fn from_lit_str(value: String) -> Self {
        Self::new(next_unique_id(), ArgInner::LitStr(value))
    }

    pub fn from_lit_int(value: u64) -> Self {
        Self::new(next_unique_id(), ArgInner::LitInt(value))
    }

    pub fn from_tokens(tokens: TokenStream) -> Self {
        Self::new(next_unique_id(), ArgInner::Tokens(tokens))
    }

    pub fn inner(&self) -> &ArgInner {
        &self.inner
    }
}

impl Ast for Arg {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        match &self.inner {
            ArgInner::Ident(ident) => ident.span(),
            ArgInner::LitStr(_) => Span::call_site(),
            ArgInner::LitInt(_) => Span::call_site(),
            ArgInner::Tokens(_) => Span::call_site(),
            ArgInner::Underscore => Span::call_site(),
        }
    }
}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub struct Func {
    id: NodeId,
    args: Option<Vec<Rc<Expr>>>,
    tokens: Option<Rc<Expr>>,
    name: Ident,
    span: Span,
}

/// Metadata for storing resolved function information
#[derive(Debug, Clone)]
pub struct FuncMetadata {
    pub inner: Rc<FuncInner>,
}

impl Func {
    /// Creates a new [`Func`] with the given name and arguments.
    pub fn new(
        id: NodeId,
        name: Ident,
        args: Option<Vec<Rc<Expr>>>,
        tokens: Option<Rc<Expr>>,
        span: Span,
    ) -> Self {
        Self {
            id,
            name,
            args,
            tokens,
            span,
        }
    }

    /// The function's name.
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// The function's arguments.
    pub fn args(&self) -> Option<&[Rc<Expr>]> {
        self.args.as_ref().map(|x| x as _)
    }

    /// The function's arguments as raw tokens.
    pub fn tokens(&self) -> Option<Rc<Expr>> {
        self.tokens.as_ref().map(|tokens| tokens.clone())
    }

    pub fn make_sig_error(&self, sig: &str) -> Error {
        Error::SignatureError(sig.to_string(), self.to_string(), self.span)
    }
}

#[derive(Debug, Clone)]
pub enum FuncInner {
    Upper(Rc<Expr>),
    Lower(Rc<Expr>),
    SnakeCase(Rc<Expr>),
    CamelCase(Rc<Expr>),
    PascalCase(Rc<Expr>),
    Hash(Rc<Expr>),
    Normalize(Rc<Expr>),
    Concat(Vec<Rc<Expr>>),
}

impl Ast for Func {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.span
    }
}

/// Expression in form of an argument or a function call.
#[derive(Debug, Clone)]
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::ArgExpr(arg) => match arg.inner() {
                ArgInner::Ident(ident) => write!(f, "{}", ident),
                ArgInner::LitStr(value) => write!(f, "\"{}\"", value),
                ArgInner::LitInt(value) => write!(f, "{}", value),
                ArgInner::Tokens(tokens) => write!(f, "{}", tokens),
                ArgInner::Underscore => write!(f, "_"),
            },
            Expr::FuncCallExpr(func) => write!(f, "{}", func),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name(),
            match (self.args(), self.tokens()) {
                (Some(args), _) => args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                (_, Some(tokens)) => tokens.to_string(),
                _ => String::new(),
            },
        )
    }
}

impl Ast for Expr {
    fn id(&self) -> NodeId {
        match self {
            Expr::ArgExpr(arg) => arg.id(),
            Expr::FuncCallExpr(func) => func.id(),
        }
    }
    fn span(&self) -> Span {
        match self {
            Expr::ArgExpr(arg) => arg.span(),
            Expr::FuncCallExpr(func) => func.span(),
        }
    }
}

/// Alias declaration.
#[derive(Debug, Clone)]
pub struct Alias {
    id: NodeId,
    ident: Ident,
}

impl Ast for Alias {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl Alias {
    /// Creates a new [`Alias`] with the given identifier.
    pub fn new(id: NodeId, ident: Ident) -> Self {
        Self { id, ident }
    }

    /// Reads the identifier.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

/// Alias value, which is a sequence of expressions that form the value of the alias.
pub struct AliasValue {
    id: NodeId,
    span: Span,
    expr: Rc<Expr>,
}

impl AliasValue {
    /// Creates a new [`AliasValue`] with the given expressions.
    pub fn new(id: NodeId, expr: Rc<Expr>, span: Span) -> Self {
        Self { id, span, expr }
    }

    /// Reads the expressions.
    pub fn expr(&self) -> Rc<Expr> {
        self.expr.clone()
    }

    /// Reads the span of the alias value.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Ast for AliasValue {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.span
    }
}

/// A single alias specification.
pub struct AliasSpecItem {
    id: NodeId,
    alias: Rc<Alias>,
    value: Rc<AliasValue>,
}

impl Ast for AliasSpecItem {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.alias.span()
    }
}

impl AliasSpecItem {
    /// Creates a new [`AliasSpecItem`] with the given alias and expressions.
    pub fn new(id: NodeId, alias: Rc<Alias>, value: Rc<AliasValue>) -> Self {
        Self { id, alias, value }
    }

    /// Reads the alias identifier.
    pub fn alias(&self) -> Rc<Alias> {
        self.alias.clone()
    }

    /// Reads the alias value.
    pub fn value(&self) -> Rc<AliasValue> {
        self.value.clone()
    }
}

/// Specification of aliases provided to the [`compose_idents`] macro.
pub struct AliasSpec {
    id: NodeId,
    items: Vec<Rc<AliasSpecItem>>,
    is_comma_used: Option<bool>,
}

impl Ast for AliasSpec {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.items
            .first()
            .map(|item| item.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl AliasSpec {
    /// Creates a new [`AliasSpec`] with the given items and separator information.
    pub fn new(id: NodeId, items: Vec<Rc<AliasSpecItem>>, is_comma_used: Option<bool>) -> Self {
        Self {
            id,
            items,
            is_comma_used,
        }
    }

    /// Reads the individual items in the alias specification.
    pub fn items(&self) -> &[Rc<AliasSpecItem>] {
        &self.items
    }

    /// Whether a comma is used as a separator.
    pub fn is_comma_used(&self) -> Option<bool> {
        self.is_comma_used
    }
}

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    id: NodeId,
    spec: Rc<AliasSpec>,
    block: Block,
}

impl Ast for ComposeIdentsArgs {
    fn id(&self) -> NodeId {
        self.id
    }
    fn span(&self) -> Span {
        self.spec.span()
    }
}

impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(id: NodeId, spec: Rc<AliasSpec>, block: Block) -> Self {
        Self { id, spec, block }
    }

    /// Reads the alias specification.
    pub fn spec(&self) -> Rc<AliasSpec> {
        self.spec.clone()
    }

    /// Reads a mutable reference to the code block.
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}

#[derive(Debug, Clone, Default)]
pub struct AstMetadata {
    func_metadata: HashMap<NodeId, Rc<FuncMetadata>>,
}

impl AstMetadata {
    pub fn new() -> Self {
        Self {
            func_metadata: HashMap::new(),
        }
    }

    pub fn set_func_metadata(&mut self, id: NodeId, inner: Rc<FuncInner>) {
        self.func_metadata
            .insert(id, Rc::new(FuncMetadata { inner }));
    }

    pub fn get_func_metadata(&self, id: NodeId) -> Option<Rc<FuncMetadata>> {
        self.func_metadata.get(&id).cloned()
    }
}
