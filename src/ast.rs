//! Defines the AST nodes that describe the syntax of the macro.

use proc_macro2::{Ident, Span, TokenStream};
use std::collections::HashMap;
use syn::Block;

/// An AST node that has a syntactic span.
pub trait Ast {
    /// Returns the span of the item.
    fn span(&self) -> Span;
}

/// Lexical scope.
#[derive(Default, Clone)]
pub struct Scope<'a> {
    aliases: HashMap<String, &'a dyn Ast>,
}

impl<'a> Scope<'a> {
    pub fn names_mut(&mut self) -> &mut HashMap<String, &'a dyn Ast> {
        &mut self.aliases
    }
}

/// Argument to the [`compose_idents`] macro.
///
/// Its [`Parse`] impl parses the input entirely, until the end.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub enum Arg {
    Ident(Ident),
    LitStr(String),
    LitInt(u64),
    Tokens(TokenStream),
    Underscore,
}

impl Ast for Arg {
    fn span(&self) -> Span {
        match self {
            Arg::Ident(ident) => ident.span(),
            Arg::LitStr(_) => Span::call_site(),
            Arg::LitInt(_) => Span::call_site(),
            Arg::Tokens(_) => Span::call_site(),
            Arg::Underscore => Span::call_site(),
        }
    }
}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub enum Func {
    Upper(Expr),
    Lower(Expr),
    SnakeCase(Expr),
    CamelCase(Expr),
    PascalCase(Expr),
    Hash(Expr),
    Normalize(Expr),
    Concat(Vec<Expr>),
    SignatureMismatch(String),
    Undefined,
}

impl Ast for Func {
    fn span(&self) -> Span {
        match self {
            Func::Upper(expr) => expr.span(),
            Func::Lower(expr) => expr.span(),
            Func::SnakeCase(expr) => expr.span(),
            Func::CamelCase(expr) => expr.span(),
            Func::PascalCase(expr) => expr.span(),
            Func::Hash(expr) => expr.span(),
            Func::Normalize(expr) => expr.span(),
            Func::Concat(exprs) => exprs
                .first()
                .map(|e| e.span())
                .unwrap_or_else(Span::call_site),
            Func::SignatureMismatch(_) => Span::call_site(),
            Func::Undefined => Span::call_site(),
        }
    }
}

/// Expression in form of an argument or a function call.
///
/// Just like [`Arg`] - parses the input entirely, until the end.
#[derive(Debug, Clone)]
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

impl Ast for Expr {
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
    ident: Ident,
}

impl Ast for Alias {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl Alias {
    /// Creates a new [`Alias`] with the given identifier.
    pub fn new(ident: Ident) -> Self {
        Self { ident }
    }

    /// Reads the identifier.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

/// Alias value, which is a sequence of expressions that form the value of the alias.
pub struct AliasValue {
    span: Span,
    exprs: Vec<Expr>,
}

impl AliasValue {
    /// Creates a new [`AliasValue`] with the given expressions.
    pub fn new(exprs: Vec<Expr>, span: Span) -> Self {
        Self { span, exprs }
    }

    /// Reads the expressions.
    pub fn exprs(&self) -> &[Expr] {
        self.exprs.as_slice()
    }

    /// Reads the span of the alias value.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Ast for AliasValue {
    fn span(&self) -> Span {
        self.span
    }
}

/// A single alias specification.
pub struct AliasSpecItem {
    alias: Alias,
    value: AliasValue,
}

impl Ast for AliasSpecItem {
    fn span(&self) -> Span {
        self.alias.span()
    }
}

impl AliasSpecItem {
    /// Creates a new [`AliasSpecItem`] with the given alias and expressions.
    pub fn new(alias: Alias, value: AliasValue) -> Self {
        Self { alias, value }
    }

    /// Reads the alias identifier.
    pub fn alias(&self) -> &Alias {
        &self.alias
    }

    /// Reads the alias value.
    pub fn value(&self) -> &AliasValue {
        &self.value
    }
}

/// Specification of aliases provided to the [`compose_idents`] macro.
pub struct AliasSpec {
    items: Vec<AliasSpecItem>,
    is_comma_used: Option<bool>,
}

impl Ast for AliasSpec {
    fn span(&self) -> Span {
        self.items
            .first()
            .map(|item| item.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl AliasSpec {
    /// Creates a new [`AliasSpec`] with the given items and separator information.
    pub fn new(items: Vec<AliasSpecItem>, is_comma_used: Option<bool>) -> Self {
        Self {
            items,
            is_comma_used,
        }
    }

    /// Reads the individual items in the alias specification.
    pub fn items(&self) -> &[AliasSpecItem] {
        &self.items
    }

    /// Whether a comma is used as a separator.
    pub fn is_comma_used(&self) -> Option<bool> {
        self.is_comma_used
    }
}

/// Arguments to the [`compose_idents`] macro.
pub struct ComposeIdentsArgs {
    spec: AliasSpec,
    block: Block,
}

impl Ast for ComposeIdentsArgs {
    fn span(&self) -> Span {
        self.spec.span()
    }
}

impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(spec: AliasSpec, block: Block) -> Self {
        Self { spec, block }
    }

    /// Reads the alias specification.
    pub fn spec(&self) -> &AliasSpec {
        &self.spec
    }

    /// Reads a mutable reference to the code block.
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}
