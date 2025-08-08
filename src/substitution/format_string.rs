//! Provides [`format_string`] function that substitutes `% alias %`-style placeholders in strings.
use proc_macro2::{Ident, Span};
use std::collections::HashMap;

/// Substitutes `% alias %`-style placeholders in a string.
pub fn format_string(value: &str, substitutions: &HashMap<Ident, Ident>) -> String {
    let mut formatted = String::new();

    let mut placeholder = String::new();
    let mut placeholder_text = String::new();
    let mut in_placeholder = false;

    let make_ident = |placeholder: &str| Ident::new(placeholder, Span::call_site());

    for c in value.chars() {
        match (c, in_placeholder) {
            ('%', true) => match substitutions.get(&make_ident(placeholder.as_str())) {
                Some(sub) => {
                    formatted.push_str(sub.to_string().as_str());

                    in_placeholder = false;
                    placeholder.clear();
                    placeholder_text.clear();
                }
                None => {
                    formatted.push_str(placeholder_text.as_str());
                    formatted.push('%');

                    in_placeholder = false;
                    placeholder.clear();
                    placeholder_text.clear();
                }
            },
            ('%', false) => {
                in_placeholder = true;
                placeholder_text.push(c);
            },
            (c, true) if c.is_whitespace() => {
                placeholder_text.push(c);
            },
            (_, true) => {
                placeholder.push(c);
                placeholder_text.push(c);
            }
            (_, false) => formatted.push(c),
        };
    }

    if in_placeholder {
        formatted.push('%');
        formatted.push_str(&placeholder);
    }

    formatted
}
