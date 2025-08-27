//! Utilities for testing substitution.

/// Makes a substitution map.
macro_rules! make_substitutions {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            HashMap::from([
                $(($key.to_string(), Rc::new($value)))*
            ])
        }
    }
}
pub(super) use make_substitutions;
