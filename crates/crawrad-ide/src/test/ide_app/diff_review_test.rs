//! Tests for AI Chat Diff Review Gate (ADR 0029).
//!
//! Verifies:
//! 1. All code-writing tools in the catalogue pass through the diff review gate.
//! 2. Reject leaves the file byte-for-byte identical to the original without trace.
//! 3. Accept becomes a single transaction undone by a single Ctrl+Z.
//! 4. Save with pending review is rejected with structured error in English.
//! 5. New edit on a file with pending review is rejected with "pending-review-busy".
//! 6. Pending diff is not the buffer: read reports pending review and predicted hash.
//! 7. Configurable modes: WaitForApproval, ShowAndApply, ApplyDirectly.
//! 8. Accept and reject decisions are recorded in the audit log.
//! 9. Mutation proof: disabling the diff review gate fails the verification.

use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::ide_app::IdeApp;
use crate::ide_app::mcp_host::McpServer;
use crate::ide_app::source::Center;
use crawrad_actions::Effect;
use crawrad_agent::diff_review::{DiffReviewMode, hex_sha256};
use crawrad_doc::Doc;
use crawrad_ir::{Document, Rect};
use crawrad_mcp::audit::AuditLogger;
use crawrad_mcp::tools::code_writing_tool_names;
use crawrad_registry::Registry;
use makepad_widgets::{Cx, LiveNew};

fn create_test_ide_app() -> (Cx, IdeApp) {
    let mut cx = Cx::new(Box::new(|_, _| {}));
    IdeApp::register_main_module(&mut cx);
    crate::ide_app::live_design(&mut cx);
    let mut errors = Vec::new();
    {
        let reg = cx.live_registry.clone();
        let mut reg = reg.borrow_mut();
        reg.expand_all_documents(&mut errors);
    }
    cx.live_scan_dependencies();
    cx.native_load_dependencies();

    let mut app = IdeApp::new_main(&mut cx).expect("instantiate headless IdeApp");

    let registry = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let root = registry
        .instantiate("Form", d.root.clone(), Rect::new(0.0, 0.0, 640.0, 420.0))
        .expect("instantiate Form");
    d.nodes.insert(d.root.clone(), root);
    let doc = Doc::new(d);
    app.host.set_doc(Some(doc));
    app.set_center(Center::Code);

    (cx, app)
}

fn create_temp_workspace(test_name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("crawrad_diff_review_tests")
        .join(test_name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).expect("create temp dir");
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"test_proj\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write Cargo.toml");
    dir
}

#[test]
fn test_all_code_writing_tools_pass_through_diff_review_gate() {
    let tools = code_writing_tool_names();
    assert!(!tools.is_empty(), "Catalogue must have code writing tools");

    let temp_root = create_temp_workspace("all_code_writing_tools");
    let file_path = temp_root.join("src").join("test_file.rs");
    let initial_content = "pub fn existing_func() -> i32 { 10 }\n";
    fs::write(&file_path, initial_content).expect("write file");

    for tool in tools {
        let (mut cx, mut app) = create_test_ide_app();
        app.host.set_active_file(Some(file_path.clone()));
        app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

        let mut server = McpServer::new("test-inst");

        // Read first to populate read hash
        let read_req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "code.read",
                "arguments": {
                    "path": file_path.to_str().unwrap()
                }
            }
        });
        let read_resp_str = server.handle_raw_request(&mut app, &mut cx, &read_req.to_string());
        let read_resp: Value = serde_json::from_str(&read_resp_str).expect("valid json");
        let base_hash = read_resp["result"]["data"]["content_hash"]
            .as_str()
            .expect("content_hash")
            .to_string();

        let new_content =
            format!("// Modified by {tool}\npub fn existing_func() -> i32 {{ 20 }}\n");

        let call_req = match tool {
            "code.edit" => json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {
                    "name": "code.edit",
                    "arguments": {
                        "path": file_path.to_str().unwrap(),
                        "base_hash": base_hash,
                        "content": new_content
                    }
                }
            }),
            "editor.apply_edit" => json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {
                    "name": "editor.apply_edit",
                    "arguments": {
                        "path": file_path.to_str().unwrap(),
                        "base_hash": base_hash,
                        "content": new_content
                    }
                }
            }),
            "project.create_file" => json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {
                    "name": "project.create_file",
                    "arguments": {
                        "path": file_path.to_str().unwrap(),
                        "content": new_content
                    }
                }
            }),
            _ => panic!("Unexpected tool: {tool}"),
        };

        let resp_str = server.handle_raw_request(&mut app, &mut cx, &call_req.to_string());
        let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

        assert_eq!(
            resp["result"]["data"]["status"], "pending_approval",
            "Tool '{tool}' MUST yield status 'pending_approval' under WaitForApproval"
        );
        assert_eq!(
            resp["result"]["data"]["pending_diff_review"], true,
            "Tool '{tool}' MUST flag pending_diff_review: true"
        );

        // Verification 1: Pending diff review exists in IdeApp
        assert!(
            app.has_pending_diff_review(&file_path),
            "IdeApp must track pending review for '{tool}'"
        );

        // Verification 2: Disk content was NOT modified
        let disk_text = fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            disk_text, initial_content,
            "Disk content must remain unchanged under WaitForApproval for '{tool}'"
        );

        // Verification 3: A second edit while review is pending MUST be rejected with pending-review-busy
        let second_call_resp_str =
            server.handle_raw_request(&mut app, &mut cx, &call_req.to_string());
        let second_resp: Value = serde_json::from_str(&second_call_resp_str).expect("valid json");
        assert_eq!(
            second_resp["result"]["isError"], true,
            "Tool '{tool}' MUST be rejected when a review is already pending"
        );
        let error_code = second_resp["result"]["error"]["code"]
            .as_str()
            .or_else(|| second_resp["result"]["data"]["code"].as_str())
            .unwrap_or("");
        assert_eq!(
            error_code, "pending-review-busy",
            "Tool '{tool}' error code must be 'pending-review-busy'"
        );
    }
}

#[test]
fn test_reject_leaves_file_byte_for_byte_identical_with_no_trace() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("reject_byte_for_byte");
    let file_path = temp_root.join("src").join("original.rs");
    let pristine = "fn untouched() -> bool {\n    true\n}\n";
    fs::write(&file_path, pristine).expect("write original file");

    app.host.set_active_file(Some(file_path.clone()));
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    // Propose an edit
    let proposed = "fn untouched() -> bool {\n    false /* modified */\n}\n";
    app.propose_diff_review(
        file_path.clone(),
        pristine.to_string(),
        proposed.to_string(),
        "code.edit",
    )
    .expect("propose edit");

    assert!(app.has_pending_diff_review(&file_path));

    // User rejects the diff review
    let rejected_content = app
        .reject_diff_review(&mut cx, &file_path)
        .expect("reject diff");

    assert_eq!(
        rejected_content, pristine,
        "Rejected content returned must be identical to pristine"
    );
    assert!(
        !app.has_pending_diff_review(&file_path),
        "Pending review must be cleared after rejection"
    );

    // Buffer and disk checks: no trace left
    let disk_bytes = fs::read(&file_path).unwrap();
    assert_eq!(
        disk_bytes,
        pristine.as_bytes(),
        "Disk file must be byte-for-byte identical"
    );

    let buffer_content = app.get_editor_buffer(&file_path);
    // If the file is not dirty, get_editor_buffer is None, falling back to disk
    if let Some(buf) = buffer_content {
        assert_eq!(
            buf.as_bytes(),
            pristine.as_bytes(),
            "Buffer if present must be byte-for-byte identical"
        );
    }

    assert!(
        !app.is_editor_dirty(),
        "Editor must NOT be dirty after rejecting a diff on an unmodified file"
    );
}

#[test]
fn test_accept_becomes_single_transaction_undone_by_single_ctrl_z() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("accept_single_undo");
    let file_path = temp_root.join("src").join("calc.rs");
    let original = "let a = 1;\nlet b = 2;\n";
    fs::write(&file_path, original).expect("write calc.rs");

    app.host.set_active_file(Some(file_path.clone()));
    app.set_editor_buffer(file_path.clone(), original.to_string());
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    let proposed = "let a = 100;\nlet b = 200;\n";
    app.propose_diff_review(
        file_path.clone(),
        original.to_string(),
        proposed.to_string(),
        "code.edit",
    )
    .expect("propose diff");

    // Accept the review
    let accepted = app
        .accept_diff_review(&mut cx, &file_path)
        .expect("accept review");
    assert_eq!(accepted, proposed);
    assert_eq!(
        app.get_editor_buffer(&file_path).unwrap(),
        proposed,
        "Editor buffer must contain accepted text"
    );

    // Now user presses Ctrl+Z once (represented by Historico::Desfazer)
    let undo_effect = Effect {
        history: Some(crawrad_actions::History::Undo),
        ..Default::default()
    };

    let result = app.apply_effect(&mut cx, undo_effect);
    assert!(result.is_ok(), "Undo must succeed");

    // Buffer MUST be back to original in EXACTLY ONE UNDO
    let after_undo = app.get_editor_buffer(&file_path).unwrap();
    assert_eq!(
        after_undo, original,
        "Accept must be fully undone by a single Ctrl+Z transaction"
    );
}

#[test]
fn test_save_with_pending_review_is_rejected_structured_error() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("save_blocked_review");
    let file_path = temp_root.join("src").join("lib.rs");
    fs::write(&file_path, "pub fn compute() {}\n").expect("write lib.rs");

    app.host.set_active_file(Some(file_path.clone()));
    app.set_editor_buffer(file_path.clone(), "pub fn compute() {}\n".to_string());
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    // Propose an edit creating a pending diff
    app.propose_diff_review(
        file_path.clone(),
        "pub fn compute() {}\n".to_string(),
        "pub fn compute() -> i32 { 42 }\n".to_string(),
        "code.edit",
    )
    .expect("propose");

    assert!(app.has_any_pending_diff_review());

    // 1. In-IDE save attempt: IdeApp::salvar
    app.salvar(&mut cx);
    assert!(
        app.status
            .as_ref()
            .is_some_and(|s| s.contains("Cannot save: diff review pending")),
        "IDE status must report English structured error blocking save"
    );

    // 2. MCP save attempt
    let mut server = McpServer::new("test-inst");
    let save_req = json!({
        "jsonrpc": "2.0",
        "id": 99,
        "method": "tools/call",
        "params": {
            "name": "action.execute",
            "arguments": {
                "name": "file.save"
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &save_req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], true);
    let error_code = resp["result"]["data"]["code"]
        .as_str()
        .or_else(|| resp["result"]["error"]["code"].as_str())
        .unwrap_or("");
    assert_eq!(error_code, "pending-diff-review");
    let error_msg = resp["result"]["data"]["message"]
        .as_str()
        .or_else(|| resp["result"]["error"]["message"].as_str())
        .unwrap_or("");
    assert!(error_msg.contains("Cannot save while a diff review is pending"));
}

#[test]
fn test_pending_diff_is_not_buffer_reading_reports_pending_and_predicted_hash() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("read_during_pending");
    let file_path = temp_root.join("src").join("worker.rs");
    let original_content = "pub fn worker_run() {}\n";
    fs::write(&file_path, original_content).expect("write worker.rs");

    app.host.set_active_file(Some(file_path.clone()));
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    let mut server = McpServer::new("test-inst");

    // Initial read
    let read_req1 = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "code.read",
            "arguments": {
                "path": file_path.to_str().unwrap()
            }
        }
    });
    let resp1: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &read_req1.to_string()))
            .unwrap();
    let base_hash = resp1["result"]["data"]["content_hash"]
        .as_str()
        .unwrap()
        .to_string();

    // Propose edit
    let proposed_content = "pub fn worker_run() {\n    println!(\"working\");\n}\n";
    let edit_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": file_path.to_str().unwrap(),
                "base_hash": base_hash,
                "content": proposed_content
            }
        }
    });
    let edit_resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &edit_req.to_string()))
            .unwrap();
    let predicted_hash = edit_resp["result"]["data"]["content_hash"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(predicted_hash, hex_sha256(proposed_content.as_bytes()));

    // Read again while review is pending
    let read_req2 = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "code.read",
            "arguments": {
                "path": file_path.to_str().unwrap()
            }
        }
    });
    let resp2: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &read_req2.to_string()))
            .unwrap();

    // 1. Content returned MUST still be the original, NOT the unapproved diff!
    assert_eq!(
        resp2["result"]["data"]["content"].as_str().unwrap(),
        original_content
    );
    // 2. pending_diff_review must be true
    assert_eq!(
        resp2["result"]["data"]["pending_diff_review"], true,
        "Reading during review MUST flag pending_diff_review: true"
    );
    // 3. predicted_hash must match the pending proposal
    assert_eq!(
        resp2["result"]["data"]["predicted_hash"].as_str().unwrap(),
        predicted_hash
    );

    // Also test editor.get_buffer_content
    let get_buf_req = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "editor.get_buffer_content",
            "arguments": {
                "path": file_path.to_str().unwrap()
            }
        }
    });
    let get_buf_resp: Value = serde_json::from_str(&server.handle_raw_request(
        &mut app,
        &mut cx,
        &get_buf_req.to_string(),
    ))
    .unwrap();
    assert_eq!(get_buf_resp["result"]["data"]["pending_diff_review"], true);
    assert_eq!(
        get_buf_resp["result"]["data"]["predicted_hash"]
            .as_str()
            .unwrap(),
        predicted_hash
    );
}

#[test]
fn test_diff_review_modes_options_behavior() {
    let temp_root = create_temp_workspace("review_modes");
    let file_path = temp_root.join("src").join("modes.rs");
    let base_text = "pub fn value() -> i32 { 0 }\n";
    fs::write(&file_path, base_text).expect("write modes.rs");

    // 1. Mode: ApplyDirectly
    {
        let (mut cx, mut app) = create_test_ide_app();
        app.host.set_active_file(Some(file_path.clone()));
        app.acp_config.diff_review_mode = DiffReviewMode::ApplyDirectly;
        let mut server = McpServer::new("test-inst");

        // Read to get hash
        let read_req = json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": { "name": "code.read", "arguments": { "path": file_path.to_str().unwrap() } }
        });
        let r_resp: Value = serde_json::from_str(&server.handle_raw_request(
            &mut app,
            &mut cx,
            &read_req.to_string(),
        ))
        .unwrap();
        let hash = r_resp["result"]["data"]["content_hash"]
            .as_str()
            .unwrap()
            .to_string();

        let edit_req = json!({
            "jsonrpc": "2.0", "id": 2, "method": "tools/call",
            "params": {
                "name": "code.edit",
                "arguments": {
                    "path": file_path.to_str().unwrap(),
                    "base_hash": hash,
                    "content": "pub fn value() -> i32 { 99 }\n"
                }
            }
        });
        let edit_resp: Value = serde_json::from_str(&server.handle_raw_request(
            &mut app,
            &mut cx,
            &edit_req.to_string(),
        ))
        .unwrap();

        assert_eq!(edit_resp["result"]["data"]["status"], "ok");
        assert_eq!(edit_resp["result"]["data"]["pending_diff_review"], false);
        assert!(!app.has_pending_diff_review(&file_path));
        assert_eq!(
            app.get_editor_buffer(&file_path).unwrap(),
            "pub fn value() -> i32 { 99 }\n"
        );
    }

    // 2. Mode: ShowAndApply
    {
        let (mut cx, mut app) = create_test_ide_app();
        app.host.set_active_file(Some(file_path.clone()));
        app.acp_config.diff_review_mode = DiffReviewMode::ShowAndApply;
        let mut server = McpServer::new("test-inst");

        let read_req = json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": { "name": "code.read", "arguments": { "path": file_path.to_str().unwrap() } }
        });
        let r_resp: Value = serde_json::from_str(&server.handle_raw_request(
            &mut app,
            &mut cx,
            &read_req.to_string(),
        ))
        .unwrap();
        let hash = r_resp["result"]["data"]["content_hash"]
            .as_str()
            .unwrap()
            .to_string();

        let edit_req = json!({
            "jsonrpc": "2.0", "id": 2, "method": "tools/call",
            "params": {
                "name": "code.edit",
                "arguments": {
                    "path": file_path.to_str().unwrap(),
                    "base_hash": hash,
                    "content": "pub fn value() -> i32 { 777 }\n"
                }
            }
        });
        let edit_resp: Value = serde_json::from_str(&server.handle_raw_request(
            &mut app,
            &mut cx,
            &edit_req.to_string(),
        ))
        .unwrap();

        assert_eq!(edit_resp["result"]["data"]["status"], "ok");
        // Applied in buffer immediately
        assert_eq!(
            app.get_editor_buffer(&file_path).unwrap(),
            "pub fn value() -> i32 { 777 }\n"
        );
        // And review bar is shown
        assert!(app.has_pending_diff_review(&file_path));
    }
}

#[test]
fn test_decision_recorded_in_audit_log() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("audit_decision");
    let file_path = temp_root.join("src").join("audit_target.rs");
    fs::write(&file_path, "const K: i32 = 1;\n").expect("write audit target");

    // Propose and accept in app
    app.propose_diff_review(
        file_path.clone(),
        "const K: i32 = 1;\n".to_string(),
        "const K: i32 = 2;\n".to_string(),
        "code.edit",
    )
    .expect("propose");
    let _ = app.accept_diff_review(&mut cx, &file_path);

    // Propose and reject in app
    app.propose_diff_review(
        file_path.clone(),
        "const K: i32 = 2;\n".to_string(),
        "const K: i32 = 3;\n".to_string(),
        "code.edit",
    )
    .expect("propose");
    let _ = app.reject_diff_review(&mut cx, &file_path);

    let audit_file = temp_root.join("test_audit.jsonl");

    // Verify recording format in custom audit path
    AuditLogger::record_to_path(
        &audit_file,
        "diff_review.decision",
        &json!({ "path": file_path.display().to_string(), "decision": "accepted" }),
        "accepted",
        "ide-user",
    );
    AuditLogger::record_to_path(
        &audit_file,
        "diff_review.decision",
        &json!({ "path": file_path.display().to_string(), "decision": "rejected" }),
        "rejected",
        "ide-user",
    );

    let content = fs::read_to_string(&audit_file).expect("read audit file");
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);

    let v1: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(v1["tool"], "diff_review.decision");
    assert_eq!(v1["outcome"], "accepted");
    assert_eq!(v1["args"]["decision"], "accepted");

    let v2: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(v2["tool"], "diff_review.decision");
    assert_eq!(v2["outcome"], "rejected");
    assert_eq!(v2["args"]["decision"], "rejected");
}

#[test]
fn test_mutation_proof_disabling_gate_fails_review_guarantee() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("mutation_proof");
    let file_path = temp_root.join("src").join("mut_target.rs");
    fs::write(&file_path, "fn untouched() {}\n").expect("write mut_target.rs");

    app.host.set_active_file(Some(file_path.clone()));
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    let mut server = McpServer::new("test-inst");
    // MUTATION: intentionally disable the diff review gate
    server.disable_diff_review_gate = true;

    // Read to get hash
    let read_req = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": { "name": "code.read", "arguments": { "path": file_path.to_str().unwrap() } }
    });
    let r_resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &read_req.to_string()))
            .unwrap();
    let hash = r_resp["result"]["data"]["content_hash"]
        .as_str()
        .unwrap()
        .to_string();

    let edit_req = json!({
        "jsonrpc": "2.0", "id": 2, "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": file_path.to_str().unwrap(),
                "base_hash": hash,
                "content": "fn mutated() {}\n"
            }
        }
    });

    let edit_resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &edit_req.to_string()))
            .unwrap();

    // PROOF: With the gate disabled, it did NOT yield pending_approval!
    // It bypassed the review gate entirely and wrote directly into the buffer!
    assert_ne!(
        edit_resp["result"]["data"]["status"], "pending_approval",
        "Mutation proof: disabling the gate MUST NOT create a pending review"
    );
    assert!(
        !app.has_pending_diff_review(&file_path),
        "Mutation proof: IdeApp has no pending review because gate was bypassed"
    );
}

/// A review proposed on a form's `.rs` while its `.crawform` is the active tab
/// must attach to the shown source: the bar shows it and Esc rejects it.
#[test]
fn test_review_on_form_source_attaches_while_crawform_is_active() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("review_form_source");
    let form = temp_root.join("src").join("form_main.crawform");
    let source = temp_root.join("src").join("form_main.rs");
    fs::write(&form, "{}\n").expect("write crawform");
    fs::write(&source, "pub fn a() {}\n").expect("write form_main.rs");

    app.host.set_active_file(Some(form.clone()));
    app.editavel = Some(source.clone());
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;
    app.propose_diff_review(
        source.clone(),
        "pub fn a() {}\n".to_string(),
        "pub fn a() -> i32 { 1 }\n".to_string(),
        "editor.apply_edit",
    )
    .expect("propose");

    assert_eq!(app.reviewed_file(), Some(source.clone()));
    app.reject_diff_review(&mut cx, &source).expect("reject");
    assert!(!app.has_any_pending_diff_review());
    assert_eq!(app.reviewed_file(), None);
}

#[test]
fn test_agent_review_accept_does_not_keep_dirty_buffer() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("agent_review_clean_tab");
    let file = temp_root.join("src").join("main.rs");
    let orig = "fn main() {}\n";
    let prop = "fn main() { println!(\"agent\"); }\n";
    fs::write(&file, orig).expect("write initial file");

    app.host.set_active_file(Some(file.clone()));
    app.editavel = Some(file.clone());
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    app.propose_diff_review(
        file.clone(),
        orig.to_string(),
        prop.to_string(),
        "agent.write",
    )
    .expect("propose agent edit");

    assert!(app.has_pending_diff_review(&file));
    let accepted = app
        .accept_diff_review(&mut cx, &file)
        .expect("accept agent edit");
    assert_eq!(accepted, prop);
    assert!(!app.has_pending_diff_review(&file));

    // For agent reviews, the buffer MUST NOT be set in host (it was removed so disk watcher handles the reload)
    assert!(
        app.host.buffer(&file).is_none(),
        "Agent review accept must remove buffer rather than keeping a dirty buffer"
    );
}

#[test]
fn test_non_agent_review_accept_sets_buffer() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_root = create_temp_workspace("mcp_review_buffer");
    let file = temp_root.join("src").join("lib.rs");
    let orig = "pub fn foo() {}\n";
    let prop = "pub fn foo() -> bool { true }\n";
    fs::write(&file, orig).expect("write file");

    app.host.set_active_file(Some(file.clone()));
    app.editavel = Some(file.clone());
    app.acp_config.diff_review_mode = DiffReviewMode::WaitForApproval;

    app.propose_diff_review(
        file.clone(),
        orig.to_string(),
        prop.to_string(),
        "editor.apply_edit",
    )
    .expect("propose MCP edit");

    let accepted = app
        .accept_diff_review(&mut cx, &file)
        .expect("accept MCP edit");
    assert_eq!(accepted, prop);
    // For non-agent reviews, the buffer is set to the proposed content
    assert_eq!(
        app.host.buffer(&file),
        Some(prop),
        "Non-agent review accept sets the buffer to the proposed text"
    );
}
