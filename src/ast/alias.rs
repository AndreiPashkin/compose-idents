use crate::ast::{Ast, Expr, Spanned};
use proc_macro2::{Ident, Span};
use std::fmt::Display;
use std::hash::Hash;

/// Alias declaration.
#[derive(Debug, Clone)]
pub struct Alias {
    ident: Ident,
}

impl PartialEq for Alias {
    fn eq(&self, other: &Self) -> bool {
        self.ident.to_string() == other.ident.to_string()
    }
}

impl Eq for Alias {}

impl Hash for Alias {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.to_string().hash(state);
    }
}

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl Spanned for Alias {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl Ast for Alias {}

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
#[derive(Debug)]
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
}

impl Spanned for AliasValue {
    fn span(&self) -> Span {
        self.span
    }
}

impl Ast for AliasValue {}

/// A single alias specification.
pub struct AliasSpecItem {
    alias: Alias,
    value: AliasValue,
}

impl Spanned for AliasSpecItem {
    fn span(&self) -> Span {
        self.alias.span()
    }
}

impl Ast for AliasSpecItem {}

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

impl Spanned for AliasSpec {
    fn span(&self) -> Span {
        self.items
            .first()
            .map(|item| item.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl Ast for AliasSpec {}

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
