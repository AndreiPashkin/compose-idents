use proc_macro2::Span;

/// Combine multiple errors into a single syn::Error.
pub fn combine_errors(message: &str, span: Span, mut errors: Vec<syn::Error>) -> syn::Error {
    if errors.is_empty() {
        panic!("Empty vector");
    }

    if errors.len() == 1 {
        errors.pop().unwrap()
    } else {
        let mut error = syn::Error::new(span, message);
        errors.iter().for_each(|err| error.combine(err.clone()));
        error
    }
}
