use super::*;
use crate::permission::PermissionOption;
use serde_json::json;
use std::sync::mpsc::channel;
use std::time::Duration;

fn edit_call(input: serde_json::Value) -> serde_json::Value {
    json!({ "toolCallId": "t1", "kind": "edit", "rawInput": input })
}

#[test]
fn an_edit_replaces_the_first_match_only() {
    let edit = ProposedEdit::from_tool_call(&edit_call(json!({
        "file_path": "C:/p/a.rs", "old_string": "x", "new_string": "y"
    })))
    .unwrap();
    assert_eq!(edit.path, PathBuf::from("C:/p/a.rs"));
    assert_eq!(edit.apply("x x").unwrap(), "y x");
}

#[test]
fn replace_all_replaces_every_match() {
    let edit = ProposedEdit::from_tool_call(&edit_call(json!({
        "file_path": "a.rs", "old_string": "x", "new_string": "y", "replace_all": true
    })))
    .unwrap();
    assert_eq!(edit.apply("x x").unwrap(), "y y");
}

#[test]
fn a_write_proposes_the_whole_file() {
    let edit = ProposedEdit::from_tool_call(&edit_call(json!({
        "file_path": "a.rs", "content": "fn main() {}\n"
    })))
    .unwrap();
    assert_eq!(edit.apply("old").unwrap(), "fn main() {}\n");
}

#[test]
fn a_multi_edit_applies_in_order() {
    let edit = ProposedEdit::from_tool_call(&edit_call(json!({
        "file_path": "a.rs",
        "edits": [
            { "old_string": "a", "new_string": "b" },
            { "old_string": "b", "new_string": "c", "replace_all": true }
        ]
    })))
    .unwrap();
    assert_eq!(edit.apply("a b").unwrap(), "c c");
}

#[test]
fn a_missing_text_refuses_to_guess() {
    let edit = ProposedEdit::from_tool_call(&edit_call(json!({
        "file_path": "a.rs", "old_string": "gone", "new_string": "y"
    })))
    .unwrap();
    assert!(edit.apply("here").is_err());
}

#[test]
fn a_call_without_a_file_is_not_an_edit() {
    assert!(ProposedEdit::from_tool_call(&json!({ "rawInput": { "command": "ls" } })).is_none());
    assert!(ProposedEdit::from_tool_call(&json!({ "title": "Edit" })).is_none());
}

fn ask(
    options: Vec<PermissionOption>,
) -> (
    Arc<PermissionRequest>,
    std::sync::mpsc::Receiver<PermissionDecision>,
) {
    let (tx, rx) = channel();
    let req = PermissionRequest::new(
        1,
        "s",
        "Edit",
        None,
        "",
        options,
        tx,
        Duration::from_secs(9),
    );
    (Arc::new(req), rx)
}

#[test]
fn accepting_the_diff_answers_the_agents_allow_option() {
    let (req, rx) = ask(vec![
        PermissionOption::new("yes", "Yes", PermissionOptionKind::AllowOnce),
        PermissionOption::new("no", "No", PermissionOptionKind::RejectOnce),
    ]);
    let mut asks = AgentEditAsks::default();
    asks.hold(PathBuf::from("a.rs"), req);
    asks.decide(Path::new("a.rs"), true);
    assert_eq!(
        rx.try_recv().unwrap(),
        PermissionDecision::Selected("yes".into())
    );
}

#[test]
fn rejecting_the_diff_answers_reject() {
    let (req, rx) = ask(Vec::new());
    let mut asks = AgentEditAsks::default();
    asks.hold(PathBuf::from("a.rs"), req);
    asks.decide(Path::new("a.rs"), false);
    assert_eq!(
        rx.try_recv().unwrap(),
        PermissionDecision::Selected("reject_once".into())
    );
}

#[test]
fn a_newer_ask_on_the_same_file_rejects_the_older() {
    let (old, old_rx) = ask(Vec::new());
    let (new, _new_rx) = ask(Vec::new());
    let mut asks = AgentEditAsks::default();
    asks.hold(PathBuf::from("a.rs"), old);
    asks.hold(PathBuf::from("a.rs"), new);
    assert_eq!(
        old_rx.try_recv().unwrap(),
        PermissionDecision::Selected("reject_once".into())
    );
}
