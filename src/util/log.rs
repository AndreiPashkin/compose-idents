//! A small logging utility. Only for development use.

/// Emits a debug message if the `_debug` feature is enabled.
macro_rules! debug {
    ($($args:tt)+) => {
        #[cfg(feature="_debug")]
        eprintln!("[compose-idents] | {} | DEBUG: {}", module_path!(), format!($($args)+))
    };
}
pub(crate) use debug;
