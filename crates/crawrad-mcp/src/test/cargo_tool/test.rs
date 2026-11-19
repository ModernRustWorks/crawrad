//! `project.build`'s subcommand, and the required parameters of a tool.

use super::{compiler_messages, dependencies, form_unit, required, subcommand, summary};
use serde_json::json;

#[test]
fn only_build_and_test_change_the_check() {
    assert_eq!(subcommand(&json!({})), "check");
    assert_eq!(subcommand(&json!({ "command": "build" })), "build");
    assert_eq!(subcommand(&json!({ "command": "test" })), "test");
    assert_eq!(subcommand(&json!({ "command": "rm -rf" })), "check");
}

#[test]
fn the_first_missing_parameter_is_named() {
    let args = json!({ "path": "a.rs", "content": "x" });
    assert_eq!(
        required(&args, "code.edit", &["path", "base_hash", "content"]),
        Err("Missing required parameter 'base_hash' for 'code.edit'".to_string())
    );
    assert_eq!(
        required(&args, "code.edit", &["path", "content"]),
        Ok(vec!["a.rs", "x"])
    );
}

/// The compiler messages of `--message-format=json`, and one in short.
#[test]
fn compiler_messages_are_read_and_summed_up() {
    let stdout = br#"{"reason":"compiler-artifact"}
{"reason":"compiler-message","message":{"level":"error","message":"boom","spans":[{"file_name":"src/a.rs","line_start":3,"column_start":5}],"rendered":"r"}}
not json"#;
    let msgs = compiler_messages(stdout);
    assert_eq!(msgs.len(), 1);
    let s = summary(&msgs[0]);
    assert_eq!(s["level"], "error");
    assert_eq!(s["file"], "src/a.rs");
    assert_eq!(s["line"], 3);
    assert_eq!(s["column"], 5);
}

/// Only `[dependencies]`, comments and other tables left out.
#[test]
fn the_dependencies_section_only() {
    let toml = "[package]
name = \"x\"
[dependencies]
serde = \"1\"
# c
rand = { version = \"0.8\" }
[dev-dependencies]
z = \"1\"
";
    let deps = dependencies(toml);
    assert_eq!(deps.len(), 2);
    assert_eq!(deps["serde"], "\"1\"");
    assert!(deps["rand"].starts_with('{'));
}

#[test]
fn the_form_unit_names_its_struct() {
    let unit = form_unit("FormA");
    assert!(unit.contains("pub struct FormA {"));
    assert!(unit.contains("impl FormA {"));
}
