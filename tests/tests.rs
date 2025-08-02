/// Tests for general features of the project.
#[test]
fn general_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/simple_compose.rs");
    t.pass("tests/compile/trailing_comma.rs");
    t.pass("tests/compile/nested_compose.rs");
    t.pass("tests/compile/nested_type_token_compose.rs");
    t.pass("tests/compile/multi_compose.rs");
    t.pass("tests/compile/const_var_compose.rs");
    t.pass("tests/compile/generic_param_compose.rs");
    t.pass("tests/compile/num_compose.rs");
    t.pass("tests/compile/format_lit_str.rs");
    t.pass("tests/compile/alias_reuse.rs");
    t.pass("tests/compile/bare_arg.rs");
    t.compile_fail("tests/compile/dup_alias.rs");
}

/// Tests calls to functions.
#[test]
fn func_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/funcs/upper.rs");
    t.pass("tests/compile/funcs/lower.rs");
    t.pass("tests/compile/funcs/nested.rs");
    t.pass("tests/compile/funcs/different_arg_types.rs");
    t.pass("tests/compile/funcs/snake_case.rs");
    t.pass("tests/compile/funcs/camel_case.rs");
    t.pass("tests/compile/funcs/pascal_case.rs");
    t.pass("tests/compile/funcs/hash.rs");
    t.pass("tests/compile/funcs/normalize/general.rs");
    t.pass("tests/compile/funcs/normalize/enum_variant.rs");
    t.pass("tests/compile/funcs/concat.rs");
    t.compile_fail("tests/compile/funcs/func_wrong_arg_num.rs");
    t.compile_fail("tests/compile/funcs/undefined_func.rs");
}

/// Tests semicolon backwards-compatibility support.
#[test]
fn semicolon_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/semicolon/semicolon_syntax.rs");
    t.compile_fail("tests/compile/semicolon/mixed_separators.rs");
    t.compile_fail("tests/compile/semicolon/semicolon_syntax_warning.rs");
}

/// Tests bracket-based alias definition syntax backwards-compatibility support.
#[test]
fn bracket_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/brackets/bracket_syntax.rs");
    t.pass("tests/compile/brackets/bracket_syntax_mixed.rs");
    t.compile_fail("tests/compile/brackets/bracket_syntax_warning.rs");
}

#[test]
fn test_format_doc_attr() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&[
            "expand",
            "--features",
            "_format-doc-attr-test",
            "--test",
            "format_doc_attr",
        ])
        .output()
        .expect("Failed to execute cargo expand");

    let expanded = String::from_utf8(output.stdout).unwrap();

    let expected = r#"
#![feature(prelude_import)]
//! It should be possible to use string-formatting to format doc-attributes.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use compose_idents::compose_idents;
#[allow(dead_code)]
///My doc comment for foo_baz
fn foo_baz() -> u32 {
    42
}
fn main() {}
    "#;

    assert_eq!(expanded.trim(), expected.trim());
}
