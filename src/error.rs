use proc_macro2::Span;
use std::convert::TryFrom;
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
}

impl Error {
    pub fn span(&self) -> Span {
        match self {
            Error::TypeError(_, span) => *span,
            Error::EvalError(_, span) => *span,
            Error::RedefinedNameError(_, span) => *span,
        }
    }
}

impl TryFrom<Error> for SynError {
    type Error = SynError;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        let message = value.to_string();
        Ok(SynError::new(value.span(), message))
    }
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
