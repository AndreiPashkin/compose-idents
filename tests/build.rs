//! Tests whether the crate correctly builds when used as a dependency of another crate.

use std::fs;
use std::process::Command;

fn main() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create a minimal Cargo.toml that depends on our crate
    let cargo_toml = format!(
        r#"
[package]
name = "feature_deps_test"
version = "0.1.0"
edition = "2021"

[dependencies]
# Path to the current crate - this will use our local version
compose-idents = {{ path = "{}" }}

# We explicitly don't include any other dependencies that might
# bring in syn with the full feature
"#,
        std::env::current_dir().unwrap().display()
    );

    fs::write(temp_path.join("Cargo.toml"), cargo_toml).unwrap();

    // Create a src directory and minimal code that uses our macro
    fs::create_dir_all(temp_path.join("src")).unwrap();

    let test_code = r#"
use compose_idents::compose_idents;

compose_idents!(test_fn = [test], {
    fn test_fn() -> &'static str {
        "success"
    }
});

fn main() {
    assert_eq!(test(), "success");
}
    "#;

    fs::write(temp_path.join("src/main.rs"), test_code).unwrap();

    // Now try to build the test crate
    let output = Command::new("cargo")
        .current_dir(temp_path)
        .args(["build", "--verbose"])
        .output()
        .expect("Failed to execute cargo build");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "{}", stderr);
}
