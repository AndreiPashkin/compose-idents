//! Provides [`format_string`] function that substitutes `%alias%`-style placeholders in strings.

use crate::ast::{Value, ValueKind};
use quote::ToTokens;
use std::collections::HashMap;
use std::rc::Rc;

/// Formats a [`Value`] instance into a string representation.
fn format_value(value: &Value) -> String {
    match &value.kind() {
        ValueKind::Ident(ident) => ident.to_string(),
        ValueKind::Path(path) => path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        ValueKind::Type(type_) => type_.to_token_stream().to_string(),
        ValueKind::Expr(expr) => expr.to_token_stream().to_string(),
        ValueKind::LitStr(lit_str) => lit_str.value(),
        ValueKind::LitInt(lit_int) => lit_int.to_string(),
        ValueKind::Tokens(tokens) => tokens.to_string(),
        ValueKind::Raw(tokens) => tokens.to_string(),
    }
}

/// Substitutes `% alias %`-style placeholders in a string.
pub fn format_string(value: &str, substitutions: &HashMap<String, Rc<Value>>) -> String {
    let mut formatted = String::new();

    let mut placeholder = String::new();
    let mut placeholder_text = String::new();
    let mut in_placeholder = false;
    let mut prev_c = Option::<char>::None;

    for c in value.chars() {
        match (c, prev_c, in_placeholder) {
            ('%', Some('%'), _) => {
                formatted.push('%');
                in_placeholder = false;
                placeholder.clear();
                placeholder_text.clear();
            }
            ('%', _, true) => match substitutions.get(placeholder.as_str()) {
                Some(sub) => {
                    formatted.push_str(format_value(sub.as_ref()).as_str());

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
            ('%', _, false) => {
                in_placeholder = true;
                placeholder_text.push(c);
            }
            (c, _, true) if c.is_whitespace() => {
                placeholder_text.push(c);
            }
            (_, _, true) => {
                placeholder.push(c);
                placeholder_text.push(c);
            }
            (_, _, false) => formatted.push(c),
        };
        prev_c = Some(c);
    }

    if in_placeholder {
        formatted.push_str(&placeholder_text);
    }

    formatted
}
