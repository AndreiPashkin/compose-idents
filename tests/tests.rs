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

/// Tests for [`compose_idents::compose!`] macro.
#[test]
fn compose() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/compose.rs");
}

/// Tests for [`compose_idents::compose_item!`] macro.
#[test]
fn compose_item() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/compose_item/basic.rs");
    t.compile_fail("tests/compile/compose_item/trailing_comma_after_loops.rs");
    t.pass("tests/compile/compose_item/trailing_comma_after_aliases.rs");
}

/// Tests for [`compose_idents::compose_idents!`] macro.
#[test]
fn compose_idents() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/concat_idents/basic.rs");
    t.compile_fail("tests/compile/concat_idents/warning.rs");
}
