use crate::ast::{Ast, Call};
use crate::core::{Func, Type};
use proc_macro2::Span;
use syn::Error as SynError;
use thiserror::Error as ThisError;

/// Project-wise error type.
#[derive(Debug, ThisError, Clone)]
pub enum Error {
    #[error("TypeError: {0}")]
    TypeError(String, Span),
    #[error("EvalError: {0}")]
    EvalError(String, Span),
    #[error("RedefinedNameError: name {0} has already been defined")]
    RedefinedNameError(String, Span),
    #[error("SignatureError: function {0} has been called with incompatible arguments: {1}")]
    SignatureError(String, String, Span),
    #[error(r#"UndefinedFunctionError: function "{0}(...)" is undefined"#)]
    UndefinedFunctionError(String, Span),
    #[error("SubstitutionError: failed to substitute:\n\n  {0}\n\nwith:\n\n  {1}\n\nEncountered an error:\n\n  {2}")]
    SubstitutionError(String, String, #[source] syn::Error, Span),
    #[error("InternalError: {0}")]
    InternalError(String),
}

impl Error {
    pub fn span(&self) -> Span {
        match self {
            Error::TypeError(_, span) => *span,
            Error::EvalError(_, span) => *span,
            Error::RedefinedNameError(_, span) => *span,
            Error::SignatureError(_, _, span) => *span,
            Error::UndefinedFunctionError(_, span) => *span,
            Error::SubstitutionError(_, _, _, span) => *span,
            Error::InternalError(_) => Span::call_site(),
        }
    }

    pub fn make_sig_error(func: &Func, call: &Call) -> Error {
        Error::SignatureError(func.signature(), call.to_string(), call.span())
    }

    pub fn make_coercion_error(from: &Type, to: &Type) -> Error {
        Error::TypeError(
            format!("impossible to coerce from {} to {}", from, to),
            Span::call_site(),
        )
    }

    pub fn make_internal_error(message: String) -> Error {
        Error::InternalError(message)
    }

    pub fn type_(&self) -> ErrorType {
        match self {
            Error::TypeError(_, _) => ErrorType::TypeError,
            Error::EvalError(_, _) => ErrorType::EvalError,
            Error::RedefinedNameError(_, _) => ErrorType::RedefinedNameError,
            Error::SignatureError(_, _, _) => ErrorType::SignatureError,
            Error::UndefinedFunctionError(_, _) => ErrorType::UndefinedFunctionError,
            Error::SubstitutionError(_, _, _, _) => ErrorType::SubstitutionError,
            Error::InternalError(_) => ErrorType::InternalError,
        }
    }
}

macro_rules! internal_error {
    ($($arg:tt)*) => {
        $crate::error::Error::make_internal_error(format!($($arg)*))
    };
}
pub(crate) use internal_error;

impl From<Error> for SynError {
    fn from(value: Error) -> Self {
        let message = value.to_string();
        SynError::new(value.span(), message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    TypeError,
    EvalError,
    RedefinedNameError,
    SignatureError,
    UndefinedFunctionError,
    SubstitutionError,
    InternalError,
}

/// Combine multiple errors into a single syn::Error.
pub fn combine_errors(message: &str, span: Span, mut errors: Vec<syn::Error>) -> syn::Error {
    if errors.is_empty() {
        panic!("Empty errors vector");
    }

    if errors.len() == 1 {
        errors.pop().unwrap()
    } else {
        let mut error = syn::Error::new(span, message);
        errors.iter().for_each(|err| error.combine(err.clone()));
        error
    }
}
