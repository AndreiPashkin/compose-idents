use crate::ast::{Ast, Expr};
use proc_macro2::{Ident, Span};

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
