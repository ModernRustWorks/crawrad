use super::*;

#[test]
fn an_edit_kind_is_never_a_read() {
    assert!(!kind_is_read(Some("edit")));
    assert!(!kind_is_read(Some("delete")));
    assert!(!kind_is_read(Some("execute")));
    assert!(kind_is_read(Some("read")));
    assert!(kind_is_read(None));
}

#[test]
fn the_meta_hooks_every_edit_tool_with_an_ask() {
    let meta = ask_before_edits_meta();
    let hook = &meta["claudeCode"]["options"]["settings"]["hooks"]["PreToolUse"][0];
    assert_eq!(hook["matcher"], "Edit|Write|MultiEdit|NotebookEdit");
    assert!(
        hook["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .contains("'ask'")
    );
}
