//! Provides implementations of the functions that can be used by the user in alias specifications.
use crate::core::State;
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use std::hash::{DefaultHasher, Hash, Hasher};

/// Converts the input string to snake_case.
pub fn to_snake_case(input: &str) -> String {
    input.to_snake_case()
}

/// Converts the input string to camelCase.
pub fn to_camel_case(input: &str) -> String {
    input.to_lower_camel_case()
}

/// Converts the input string to PascalCase.
pub fn to_pascal_case(input: &str) -> String {
    input.to_pascal_case()
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
    if inserted_underscore {
        result.pop();
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
    #[case("&'static str", "static_str")]
    #[case("&'static str ", "static_str")]
    #[case("Result<T, E>", "Result_T_E")]
    #[case("Result< T, E >", "Result_T_E")]
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
