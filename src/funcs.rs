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
}
