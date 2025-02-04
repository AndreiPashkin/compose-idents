#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/simple_compose.rs");
    t.pass("tests/compile/nested_compose.rs");
    t.pass("tests/compile/nested_type_token_compose.rs");
    t.pass("tests/compile/multi_compose.rs");
    t.pass("tests/compile/const_var_compose.rs");
    t.pass("tests/compile/generic_param_compose.rs");
}
