/// Converts the input string to snake_case.
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();

    let chars = input.chars().collect::<Vec<char>>();

    let mut chunks = chars.chunks_exact(2);
    while let Some([a, b]) = chunks.next() {
        result.push(a.to_lowercase().next().unwrap());
        if b.is_uppercase() && a.is_lowercase() || a.is_uppercase() && b.is_lowercase() {
            result.push('_');
        }
        result.push(b.to_lowercase().next().unwrap());
    }
    if chunks.remainder().len() == 1 {
        result.push(chunks.remainder()[0].to_lowercase().next().unwrap());
    }
    result
}

/// Converts the input string to camelCase.
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();

    let chars = input.chars().collect::<Vec<char>>();

    let mut should_upper = false;
    for char in chars {
        if char == '_' || char == '-' {
            should_upper = true;
        } else if should_upper {
            result.push(char.to_uppercase().next().unwrap());
            should_upper = false;
        } else {
            result.push(char.to_lowercase().next().unwrap());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("fooBar", "foo_bar")]
    #[case("foo_bar", "foo_bar")]
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
    #[case("foo__bar", "fooBar")]
    #[case("FOO_BAR", "fooBar")]
    #[case("foo-bar", "fooBar")]
    #[case("FOO-BAR", "fooBar")]
    #[case("foo", "foo")]
    #[case("FOO", "foo")]
    #[case("F", "f")]
    #[case("f", "f")]
    #[case("", "")]
    fn test_to_camel_case(#[case] input: &str, #[case] expected: &str) {
        let actual = to_camel_case(input);
        assert_eq!(actual, expected, "Input: {}", input);
    }
}
