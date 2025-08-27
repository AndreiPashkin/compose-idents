use crate::ast::{Ast, NodeId};
use crate::core::Type;
use crate::error::Error;
use crate::util::unique_id::next_unique_id;
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::fmt::Display;
use std::marker::PhantomData;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::LitStr;

/// Argument to the [`compose_idents`] macro.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub struct Value {
    id: NodeId,
    kind: ValueKind,
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    Ident(Ident),
    Path(syn::Path),
    Type(syn::Type),
    Expr(syn::Expr),
    LitStr(syn::LitStr),
    LitInt(syn::LitInt),
    Tokens(TokenStream),
    Raw(TokenStream),
}

impl Value {
    pub fn new(id: NodeId, kind: ValueKind) -> Self {
        Self { id, kind }
    }
    pub fn from_ident(ident: Ident) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Ident(ident))
    }
    pub fn from_path(path: syn::Path) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Path(path))
    }
    pub fn from_type(type_: syn::Type) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Type(type_))
    }
    pub fn from_expr(expr: syn::Expr) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Expr(expr))
    }
    pub fn from_lit_str(lit_str: syn::LitStr) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::LitStr(lit_str))
    }
    pub fn from_lit_int(lit_int: syn::LitInt) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::LitInt(lit_int))
    }
    pub fn from_tokens(tokens: TokenStream) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Tokens(tokens))
    }
    pub fn from_raw(tokens: TokenStream) -> Self {
        Self::new(next_unique_id() as NodeId, ValueKind::Raw(tokens))
    }
    pub fn kind(&self) -> &ValueKind {
        &self.kind
    }
    pub fn type_(&self) -> Type {
        match self.kind() {
            ValueKind::Ident(_) => Type::Ident,
            ValueKind::Path(_) => Type::Path,
            ValueKind::Type(_) => Type::Type,
            ValueKind::Expr(_) => Type::Expr,
            ValueKind::LitStr(_) => Type::LitStr,
            ValueKind::LitInt(_) => Type::LitInt,
            ValueKind::Tokens(_) => Type::Tokens,
            ValueKind::Raw(_) => Type::Raw,
        }
    }
    fn make_cast_error(
        error: Option<syn::Error>,
        error_str: Option<&str>,
        from_type: Type,
        to_type: Type,
    ) -> Error {
        if let Some(error) = error {
            Error::TypeError(
                format!("Unable to cast {:?} to {:?}: {}", from_type, to_type, error),
                error.span(),
            )
        } else if let Some(error_str) = error_str {
            Error::TypeError(
                format!(
                    "Unable to cast {:?} to {:?}: {}",
                    from_type, to_type, error_str
                ),
                error_str.span(),
            )
        } else {
            panic!("make_error called without an error provided");
        }
    }

    fn from_lit_str_as_ident(lit_str: LitStr) -> Result<Value, Error> {
        match syn::parse_str::<Ident>(lit_str.value().as_str()) {
            Err(error) => Err(Self::make_cast_error(
                Some(error),
                None,
                Type::LitStr,
                Type::Ident,
            )),
            Ok(ident) => Ok(Value::from_ident(ident)),
        }
    }

    /// Tries to cast the value to the specified type.
    pub fn try_cast(&self, type_: &Type) -> Result<Value, Error> {
        match (&self.type_(), type_) {
            (from_type, to_type) if from_type == to_type => Ok(self.clone()),
            (Type::Ident, Type::Path) => {
                let ValueKind::Ident(ident) = self.kind.clone() else {
                    unreachable!()
                };
                let path = syn::Path::from(ident.clone());
                Ok(Value::from_path(path))
            }
            (Type::Ident, Type::Type) => {
                let ValueKind::Ident(ident) = self.kind.clone() else {
                    unreachable!()
                };
                match syn::parse2::<syn::Type>(ident.clone().to_token_stream()) {
                    Err(error) => Err(Self::make_cast_error(
                        Some(error),
                        None,
                        Type::Ident,
                        Type::Type,
                    )),
                    Ok(type_) => Ok(Value::from_type(type_)),
                }
            }
            (Type::Ident, Type::Expr) => {
                let ValueKind::Ident(ref ident) = self.kind else {
                    unreachable!()
                };
                match syn::parse2::<syn::Expr>(ident.clone().to_token_stream()) {
                    Err(error) => Err(Self::make_cast_error(
                        Some(error),
                        None,
                        Type::Ident,
                        Type::Expr,
                    )),
                    Ok(expr) => Ok(Value::from_expr(expr)),
                }
            }
            (Type::Ident, Type::LitStr) => {
                let ValueKind::Ident(ref ident) = self.kind else {
                    unreachable!()
                };
                match syn::parse_str::<syn::LitStr>(format!("\"{}\"", ident).as_str()) {
                    Err(error) => Err(Self::make_cast_error(
                        Some(error),
                        None,
                        Type::Ident,
                        Type::LitStr,
                    )),
                    Ok(mut lit_str) => {
                        lit_str.set_span(Ast::span(self));
                        Ok(Value::from_lit_str(lit_str))
                    }
                }
            }
            (Type::LitStr, Type::Ident) => {
                let ValueKind::LitStr(lit_str) = self.kind.clone() else {
                    unreachable!()
                };

                Self::from_lit_str_as_ident(lit_str)
            }
            (from_type, Type::Ident) => {
                let tokens = self.to_token_stream();
                if let Ok(lit_str) = syn::parse2::<LitStr>(tokens) {
                    return Self::from_lit_str_as_ident(lit_str);
                };

                match syn::parse2::<Ident>(self.to_token_stream()) {
                    Err(error) => Err(Self::make_cast_error(
                        Some(error),
                        None,
                        from_type.clone(),
                        Type::Ident,
                    )),
                    Ok(ident) => Ok(Value::from_ident(ident)),
                }
            }
            (from_type, Type::Path) => match syn::parse2::<syn::Path>(self.to_token_stream()) {
                Err(error) => Err(Self::make_cast_error(
                    Some(error),
                    None,
                    from_type.clone(),
                    Type::Path,
                )),
                Ok(path) => Ok(Value::from_path(path)),
            },
            (from_type, Type::Type) => match syn::parse2::<syn::Type>(self.to_token_stream()) {
                Err(error) => Err(Self::make_cast_error(
                    Some(error),
                    None,
                    from_type.clone(),
                    Type::Type,
                )),
                Ok(type_) => Ok(Value::from_type(type_)),
            },
            (from_type, Type::Expr) => match syn::parse2::<syn::Expr>(self.to_token_stream()) {
                Err(error) => Err(Self::make_cast_error(
                    Some(error),
                    None,
                    from_type.clone(),
                    Type::Expr,
                )),
                Ok(expr) => Ok(Value::from_expr(expr)),
            },
            (from_type, Type::LitStr) => {
                if let Ok(ident) = syn::parse2::<Ident>(self.to_token_stream()) {
                    let lit_str = LitStr::new(&ident.to_string(), ident.span());
                    return Ok(Value::from_lit_str(lit_str));
                }
                match syn::parse2::<syn::LitStr>(self.to_token_stream()) {
                    Err(error) => Err(Self::make_cast_error(
                        Some(error),
                        None,
                        from_type.clone(),
                        Type::LitStr,
                    )),
                    Ok(lit_str) => Ok(Value::from_lit_str(lit_str)),
                }
            }
            (from_type, Type::LitInt) => match syn::parse2::<syn::LitInt>(self.to_token_stream()) {
                Err(error) => Err(Self::make_cast_error(
                    Some(error),
                    None,
                    from_type.clone(),
                    Type::LitInt,
                )),
                Ok(lit_int) => Ok(Value::from_lit_int(lit_int)),
            },
            (_, Type::Tokens) => Ok(Value::from_tokens(self.to_token_stream())),
            (_, Type::Raw) => Ok(Value::from_raw(self.to_token_stream())),
            (from_type, to_type) => Err(Self::make_cast_error(
                None,
                Some("impossible cast"),
                from_type.clone(),
                to_type.clone(),
            )),
        }
    }
}

impl Ast for Value {
    fn id(&self) -> NodeId {
        self.id
    }

    fn span(&self) -> Span {
        match &self.kind() {
            ValueKind::Ident(ident) => ident.span(),
            ValueKind::Path(path) => path.span(),
            ValueKind::Type(type_) => type_.span(),
            ValueKind::Expr(expr) => expr.span(),
            ValueKind::LitStr(lit_str) => lit_str.span(),
            ValueKind::LitInt(lit_int) => lit_int.span(),
            ValueKind::Tokens(tokens) => tokens.span(),
            ValueKind::Raw(tokens) => tokens.span(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            ValueKind::Ident(ident) => write!(f, "{}", ident),
            ValueKind::Path(path) => write!(f, "{}", path.to_token_stream()),
            ValueKind::Type(type_) => write!(f, "{}", type_.to_token_stream()),
            ValueKind::Expr(expr) => write!(f, "{}", expr.to_token_stream()),
            ValueKind::LitStr(value) => write!(f, "\"{}\"", value.value()),
            ValueKind::LitInt(value) => write!(f, "{}", value.base10_digits()),
            ValueKind::Tokens(tokens) => write!(f, "{}", tokens),
            ValueKind::Raw(tokens) => write!(f, "{}", tokens),
        }
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.kind {
            ValueKind::Ident(ident) => tokens.extend(ident.to_token_stream()),
            ValueKind::Path(path) => tokens.extend(path.to_token_stream()),
            ValueKind::Type(type_) => tokens.extend(type_.to_token_stream()),
            ValueKind::Expr(expr) => tokens.extend(expr.to_token_stream()),
            ValueKind::LitStr(lit_str) => tokens.extend(lit_str.to_token_stream()),
            ValueKind::LitInt(lit_int) => tokens.extend(lit_int.to_token_stream()),
            ValueKind::Tokens(tokens_) => tokens.extend(tokens_.clone()),
            ValueKind::Raw(tokens_) => tokens.extend(tokens_.clone()),
        }
    }
}

/// Metadata for [`Value`] AST elements.
#[derive(Debug, Clone)]
pub struct ValueMetadata {
    /// The target coercion type.
    pub target_type: Type,
    /// The cost of coercing to the target type.
    pub coercion_cost: u32,
}

/// Auxiliary type that represents an [`Value`] terminated by a generic terminator-token.
#[derive(Debug, Clone)]
pub struct TerminatedValue<Term>
where
    Term: Parse,
{
    value: Value,
    terminator_type: PhantomData<Term>,
}

impl<Term: Parse> TerminatedValue<Term> {
    pub fn new(arg: Value) -> Self {
        Self {
            value: arg,
            terminator_type: PhantomData,
        }
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

impl<Term: Parse> Ast for TerminatedValue<Term> {
    fn id(&self) -> NodeId {
        self.value.id()
    }
    fn span(&self) -> Span {
        Ast::span(&self.value)
    }
}
