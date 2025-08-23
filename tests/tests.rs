/// Tests for general features of the project.
#[test]
fn general_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/trailing_comma.rs");
    t.pass("tests/compile/nested_compose.rs");
    t.pass("tests/compile/nested_type_token_compose.rs");
}

/// Tests semicolon backwards-compatibility support.
#[test]
fn semicolon_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/semicolon/semicolon_syntax.rs");
    t.compile_fail("tests/compile/semicolon/mixed_separators.rs");
    t.compile_fail("tests/compile/semicolon/semicolon_syntax_warning.rs");
}

/// Tests error reporting.
#[test]
fn error_reporting() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/error_reporting.rs");
}
