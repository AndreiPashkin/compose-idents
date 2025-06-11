use proc_macro2::Span;
use std::convert::TryFrom;
use syn::Error as SynError;
use thiserror::Error as ThisError;

/// Project-wise error type.
#[derive(Debug, ThisError, Clone)]
pub enum Error {
    #[error("{0}")]
    TypeError(String, Span),
    #[error("{0}")]
    EvalError(String, Span),
}

impl TryFrom<Error> for SynError {
    type Error = SynError;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        match value {
            Error::TypeError(msg, span) => {
                let syn_error = SynError::new(span, msg);
                Ok(syn_error)
            }
            Error::EvalError(msg, span) => {
                let syn_error = SynError::new(span, msg);
                Ok(syn_error)
            }
        }
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
