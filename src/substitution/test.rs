//! Utilities for testing substitution.

/// Makes a substitution map.
macro_rules! make_substitutions {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            HashMap::from([
                $((Ident::new($key, Span::call_site()), Rc::new($value)))*
            ])
        }
    }
}
pub(super) use make_substitutions;
