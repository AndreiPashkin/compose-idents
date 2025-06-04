//! Provides implementations of the functions that can be used by the user in alias specifications.
use crate::core::State;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Converts the input string to snake_case.
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();

    let chars = input.chars().collect::<Vec<char>>();
    if chars.len() < 2 {
        result.extend(chars.iter().flat_map(|c| c.to_lowercase()));
        return result;
    }

    let mut skip = false;

    for (i, window) in chars.windows(2).enumerate() {
        let a = window[0];
        let b = window[1];
        if i == 0 {
            result.extend(a.to_lowercase());
            if b.is_lowercase() {
                skip = true;
            }
        }
        if (b.is_uppercase() && a.is_lowercase() || a.is_uppercase() && b.is_lowercase()) && !skip {
            result.push('_');
            skip = true;
        } else if skip {
            skip = false;
        }
        result.extend(b.to_lowercase());
    }
    result
}

/// Checks if the character is a punctuation character.
fn is_punct(c: char) -> bool {
    c == '_' || c == '-'
}

/// Converts the input string to camelCase.
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();

    let mut should_upper = false;
    let mut prev_char: Option<char> = None;
    let all_upper = input
        .chars()
        .all(|c| c.is_uppercase() || !c.is_alphabetic());
    let mut is_consecutive_punct = false;

    for (i, char) in input.chars().enumerate() {
        let is_first = i == 0;
        let is_second = i == 1;
        let is_last = i == input.len() - 1;

        if !is_punct(char) && is_consecutive_punct {
            is_consecutive_punct = false;
        }

        #[allow(clippy::if_same_then_else)]
        if is_punct(char) {
            if let Some(prev_char) = prev_char {
                if is_punct(prev_char) && !is_consecutive_punct {
                    is_consecutive_punct = true;
                    if !is_second {
                        // Exceptional case when the first character is a punctuation
                        // and was already pushed due to being such.
                        result.push(prev_char);
                    }
                }
            }
            if is_first || is_last || is_consecutive_punct {
                result.push(char);
            } else {
                should_upper = true;
            }
        } else if char.is_numeric() {
            should_upper = true;
            result.push(char);
        } else if prev_char.is_some_and(|c| c.is_lowercase() || !c.is_alphabetic())
            && char.is_uppercase()
        {
            result.push(char);
            should_upper = false;
        } else if prev_char.is_some_and(|c| c.is_uppercase()) && char.is_uppercase() && !all_upper {
            result.push(char);
            should_upper = false;
        } else if should_upper {
            result.extend(char.to_uppercase());
            should_upper = false;
        } else {
            result.extend(char.to_lowercase());
        }
        prev_char = Some(char);
    }
    result
}

/// Converts the input string to PascalCase.
pub fn to_pascal_case(input: &str) -> String {
    let camel_case = to_camel_case(input);

    if camel_case.is_empty() {
        return camel_case;
    }

    let mut chars: Vec<char> = camel_case.chars().collect();

    let first_alpha_idx = chars.iter().position(|c| c.is_alphabetic());

    match first_alpha_idx {
        Some(idx) => {
            chars.splice(idx..=idx, chars[idx].to_uppercase());
            chars.iter().collect()
        }
        None => chars.iter().collect(),
    }
}

/// Generates an identifier from a provided seed deterministically within a single macro invocation.
///
/// `hash(1)` called within a single macro invocation will always return the same
/// value but different in another macro invocation.
pub fn hash(input: &str, state: &State) -> String {
    let mut hasher = DefaultHasher::new();
    state.seed().hash(&mut hasher);
    input.hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let result = format!("__{}", hash);
    result
}

/// Normalizes a string to be a valid Rust identifier.
///
/// - Replaces all characters not valid for identifier with underscores.
/// - Ensures there are no consecutive underscores when generated from invalid characters.
/// - Any consecutive underscores already in the input are preserved.
/// - Redundant leading and trailing underscores (generated or original) are stripped.
pub fn normalize(input: &str) -> String {
    let mut result = String::new();
    let mut inserted_underscore = false;

    let num_chars = input.chars().count();

    for (i, char) in input.chars().enumerate() {
        let is_first = result.is_empty();
        let is_last = i == num_chars - 1;
        let should_strip = is_first || is_last;

        if char.is_alphanumeric() || char == '_' {
            if i == 0 && char.is_numeric() && !inserted_underscore {
                result.push('_');
            } else if char == '_' && should_strip {
                continue;
            }
            result.push(char);
            inserted_underscore = false;
        } else if !inserted_underscore && !should_strip {
            result.push('_');
            inserted_underscore = true;
        }
    }
    if result.is_empty() {
        result.push('_');
    }

    result
}

/// Concatenates multiple string inputs.
pub fn concat(inputs: &[&str]) -> String {
    inputs.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use syn::Ident;

    #[rstest]
    #[case("FooBar", "foo_bar")]
    #[case("fooBar", "foo_bar")]
    #[case("foBar", "fo_bar")]
    #[case("fBar", "f_bar")]
    #[case("fooBAR", "foo_bar")]
    #[case("foo_bar", "foo_bar")]
    #[case("fo_bar", "fo_bar")]
    #[case("f_bar", "f_bar")]
    #[case("_foobar", "_foobar")]
    #[case("foo_baR", "foo_ba_r")]
    #[case("FOO_BAR", "foo_bar")]
    #[case("foo", "foo")]
    #[case("FOO", "foo")]
    #[case("F", "f")]
    #[case("f", "f")]
    #[case("", "")]
    fn test_to_snake_case(#[case] input: &str, #[case] expected: &str) {
        let actual = to_snake_case(input);
        assert_eq!(actual, expected, "Input: {}", input);
    }

    #[rstest]
    #[case("foo_bar", "fooBar")]
    #[case("foo__bar", "foo__Bar")]
    #[case("FOO_BAR", "fooBar")]
    #[case("foo-bar", "fooBar")]
    #[case("FOO-BAR", "fooBar")]
    #[case("fo-bar", "foBar")]
    #[case("Foo_bar", "fooBar")]
    #[case("Foo_baR", "fooBaR")]
    #[case("foo", "foo")]
    #[case("FOO", "foo")]
    #[case("F", "f")]
    #[case("f", "f")]
    #[case("", "")]
    #[case("_foo", "_foo")]
    #[case("foo_", "foo_")]
    #[case("foo_123bar", "foo123Bar")]
    #[case("fooBAR", "fooBAR")]
    #[case("fooBar", "fooBar")]
    #[case("foo-_bar", "foo-_Bar")]
    #[case("_", "_")]
    #[case("__foo", "__foo")]
    #[case("snake_case_with_numbers_123", "snakeCaseWithNumbers123")]
    #[case("CamelCase_to_camelCase", "camelCaseToCamelCase")]
    fn test_to_camel_case(#[case] input: &str, #[case] expected: &str) {
        let actual = to_camel_case(input);
        assert_eq!(actual, expected, "Input: {}", input);
    }

    #[rstest]
    #[case("foo_bar", "FooBar")]
    #[case("foo__bar", "Foo__Bar")]
    #[case("FOO_BAR", "FooBar")]
    #[case("foo-bar", "FooBar")]
    #[case("FOO-BAR", "FooBar")]
    #[case("fo-bar", "FoBar")]
    #[case("Foo_bar", "FooBar")]
    #[case("Foo_baR", "FooBaR")]
    #[case("foo_BAR", "FooBAR")]
    #[case("foo", "Foo")]
    #[case("FOO", "Foo")]
    #[case("F", "F")]
    #[case("f", "F")]
    #[case("", "")]
    #[case("_foo", "_Foo")]
    #[case("foo_", "Foo_")]
    #[case("foo_123bar", "Foo123Bar")]
    #[case("fooBAR", "FooBAR")]
    #[case("fooBar", "FooBar")]
    #[case("foo-_bar", "Foo-_Bar")]
    #[case("_", "_")]
    #[case("__foo", "__Foo")]
    #[case("snake_case_with_numbers_123", "SnakeCaseWithNumbers123")]
    #[case("CamelCase_to_camelCase", "CamelCaseToCamelCase")]
    fn test_to_pascal_case(#[case] input: &str, #[case] expected: &str) {
        let actual = to_pascal_case(input);
        assert_eq!(actual, expected, "Input: {}", input);
    }

    #[rstest]
    fn test_random_valid_ident() {
        let state = State::new();
        let actual = hash("1", &state);
        let ident_result = syn::parse_str::<Ident>(actual.as_str());

        assert!(
            ident_result.is_ok(),
            "Result: {},\nError: {}",
            actual,
            ident_result.unwrap_err(),
        );
    }

    #[rstest]
    fn test_random_determinism() {
        let state = State::new();
        let expected = hash("1", &state);
        let actual = hash("1", &state);

        assert_eq!(actual, expected);
        assert_ne!(hash("2", &state), expected);
    }

    #[rstest]
    #[case("hello_world", "hello_world")]
    #[case("$hello_world", "hello_world")]
    #[case("hello_world$", "hello_world")]
    #[case("hello world", "hello_world")]
    #[case("hello__world", "hello__world")]
    #[case("hello-world", "hello_world")]
    #[case("hello.world", "hello_world")]
    #[case("hello...world", "hello_world")]
    #[case("hello-_-world", "hello___world")]
    #[case("123hello", "_123hello")]
    #[case("#$%^&*", "_")]
    #[case("", "_")]
    #[case("a__b___c", "a__b___c")]
    #[case("a b c", "a_b_c")]
    #[case("a.b.c", "a_b_c")]
    #[case("a!@#b$%^c", "a_b_c")]
    #[case("a_!@#_b", "a___b")]
    fn test_normalize(#[case] input: &str, #[case] expected: &str) {
        let actual = normalize(input);
        assert_eq!(actual, expected, "Input: {}", input);
    }

    #[rstest]
    #[case(&[], "")]
    #[case(&["hello"], "hello")]
    #[case(&["hello", "world"], "helloworld")]
    #[case(&["foo", "_", "bar"], "foo_bar")]
    #[case(&["a", "b", "c", "d"], "abcd")]
    #[case(&["", "hello", "", "world", ""], "helloworld")]
    fn test_concat(#[case] inputs: &[&str], #[case] expected: &str) {
        let actual = concat(inputs);
        assert_eq!(actual, expected, "Inputs: {:?}", inputs);
    }
}
