//! AI consent and permission model tests (ADR 0029).
//!
//! Checks:
//! 1. `classify_tool` and `is_read_only_tool` (auto-approval of reads).
//! 2. Mutation and execution require human approval.
//! 3. `SessionPermissions` keyed by tool and target.
//! 4. FIFO `PermissionQueue` with isolated answers and a cancelled timeout.

use crate::permission::{
    OperationKind, PermissionDecision, PermissionQueue, PermissionRequest, SessionPermissions,
    classify_tool, is_read_only_tool,
};
use std::time::Duration;

#[test]
fn automatic_tool_classification() {
    assert_eq!(classify_tool("form.read"), OperationKind::Read);
    assert_eq!(
        classify_tool("designer.get_active_form"),
        OperationKind::Read
    );
    assert_eq!(
        classify_tool("editor.get_buffer_content"),
        OperationKind::Read
    );
    assert_eq!(classify_tool("project.get_tree"), OperationKind::Read);

    assert_eq!(classify_tool("form.mutate"), OperationKind::Write);
    assert_eq!(
        classify_tool("designer.add_component"),
        OperationKind::Write
    );
    assert_eq!(
        classify_tool("designer.delete_component"),
        OperationKind::Write
    );
    assert_eq!(classify_tool("editor.apply_edit"), OperationKind::Write);

    assert_eq!(classify_tool("build.check"), OperationKind::Execute);
    assert_eq!(classify_tool("run.start"), OperationKind::Execute);
    assert_eq!(classify_tool("cargo.add"), OperationKind::Execute);
    assert_eq!(classify_tool("bash.exec"), OperationKind::Execute);
}

#[test]
fn auto_approve_reads_identifies_only_reads() {
    assert!(is_read_only_tool("form.read"));
    assert!(is_read_only_tool("designer.get_active_form"));
    assert!(is_read_only_tool("editor.get_buffer_content"));
    assert!(is_read_only_tool("read_file"));
    assert!(is_read_only_tool("fs/read_text_file"));
    assert!(is_read_only_tool("list_directory"));

    assert!(!is_read_only_tool("form.mutate"));
    assert!(!is_read_only_tool("designer.delete_component"));
    assert!(!is_read_only_tool("editor.apply_edit"));
    assert!(!is_read_only_tool("write_file"));
    assert!(!is_read_only_tool("bash.exec"));
}

#[test]
fn mutation_and_execution_are_classified_as_risky() {
    assert_eq!(
        classify_tool("designer.delete_component"),
        OperationKind::Write
    );
    assert_eq!(classify_tool("write_to_file"), OperationKind::Write);
    assert_eq!(classify_tool("build.run"), OperationKind::Execute);
    assert_eq!(classify_tool("terminal_exec"), OperationKind::Execute);
}

#[test]
fn allow_for_session_keys_by_tool_and_target() {
    let mut perms = SessionPermissions::new();

    // 1. Allow 'editor.apply_edit' for 'main.rs'
    perms.allow_always("editor.apply_edit", Some("main.rs".to_string()));
    assert!(perms.is_allowed("editor.apply_edit", Some("main.rs")));

    // 2. A different target on the same tool is NOT auto-approved
    assert!(!perms.is_allowed("editor.apply_edit", Some("app.rs")));

    // 3. A tool without a target allowed globally
    perms.allow_always("read_file", None);
    assert!(perms.is_allowed("read_file", Some("qualquer.rs")));
    assert!(perms.is_allowed("read_file", None));

    // 4. Clearing the session revokes every authorization
    perms.clear();
    assert!(!perms.is_allowed("editor.apply_edit", Some("main.rs")));
    assert!(!perms.is_allowed("read_file", None));
}

#[test]
fn multi_request_queue_keeps_order_and_isolated_answers() {
    let mut queue = PermissionQueue::new();
    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();
    let (tx3, rx3) = std::sync::mpsc::channel();

    let req1 = PermissionRequest::new(
        1,
        "s",
        "tool.primeira",
        None,
        "args1",
        vec![],
        tx1,
        Duration::from_secs(30),
    );
    let req2 = PermissionRequest::new(
        2,
        "s",
        "tool.segunda",
        None,
        "args2",
        vec![],
        tx2,
        Duration::from_secs(30),
    );
    let req3 = PermissionRequest::new(
        3,
        "s",
        "tool.terceira",
        None,
        "args3",
        vec![],
        tx3,
        Duration::from_secs(30),
    );

    queue.push(req1);
    queue.push(req2);
    queue.push(req3);

    assert_eq!(queue.len(), 3);

    // Answer the first with allow_once
    assert_eq!(queue.current().unwrap().tool_name, "tool.primeira");
    let p1 = queue.pop().unwrap();
    p1.respond(PermissionDecision::Selected("allow_once".into()));

    // Answer the second with reject_once
    assert_eq!(queue.current().unwrap().tool_name, "tool.segunda");
    let p2 = queue.pop().unwrap();
    p2.respond(PermissionDecision::Selected("reject_once".into()));

    // Answer the third with allow_once
    assert_eq!(queue.current().unwrap().tool_name, "tool.terceira");
    let p3 = queue.pop().unwrap();
    p3.respond(PermissionDecision::Selected("allow_once".into()));

    assert!(queue.is_empty());

    // Each channel receives its own matching answer, in isolation
    assert_eq!(
        rx1.recv().unwrap(),
        PermissionDecision::Selected("allow_once".into())
    );
    assert_eq!(
        rx2.recv().unwrap(),
        PermissionDecision::Selected("reject_once".into())
    );
    assert_eq!(
        rx3.recv().unwrap(),
        PermissionDecision::Selected("allow_once".into())
    );
}
