//! The guard the Makepad dispatch puts around a control's events
//! (`targets/makepad/actions.rs`): one `if` takes it into its
//! condition, so the user's project has no `clippy::collapsible_if`.

use super::*;

const GUARD: &str = "self.form.x.enabled";

#[test]
fn a_plain_if_takes_the_guard() {
    let block = "        if a.clicked(actions) {\n            go();\n        }\n";
    assert_eq!(
        guarded(GUARD, block),
        "        if self.form.x.enabled && a.clicked(actions) {\n            go();\n        }\n"
    );
}

#[test]
fn an_if_let_becomes_a_let_chain() {
    let block = "        if let Some(v) = a.changed(actions) {\n            take(v);\n        }\n";
    assert_eq!(
        guarded(GUARD, block),
        "        if self.form.x.enabled && let Some(v) = a.changed(actions) {\n            take(v);\n        }\n"
    );
}

#[test]
fn an_or_is_kept_apart_from_the_guard() {
    let block = "        if a || b {\n            go();\n        }\n";
    assert!(guarded(GUARD, block).starts_with("        if self.form.x.enabled && (a || b) {\n"));
}

#[test]
fn more_than_one_statement_keeps_the_guard_around_them() {
    let block = "        if a {\n            go();\n        }\n        if b {\n            stop();\n        }\n";
    let out = guarded(GUARD, block);
    assert!(
        out.starts_with("        if self.form.x.enabled {\n            if a {\n"),
        "{out}"
    );
    assert!(out.ends_with("        }\n"), "{out}");
}

#[test]
fn an_else_keeps_the_guard_around_it() {
    let block =
        "        if a {\n            go();\n        } else {\n            stop();\n        }\n";
    assert!(guarded(GUARD, block).starts_with("        if self.form.x.enabled {\n"));
}
