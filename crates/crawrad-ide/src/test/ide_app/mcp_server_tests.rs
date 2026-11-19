//! Comprehensive test suite for in-IDE MCP Server and stdio bridge (ADR 0029).

use std::path::PathBuf;

use serde_json::{Value, json};

use crate::ide_app::IdeApp;
use crate::ide_app::mcp_host::McpServer;
use crate::ide_app::source::Center;
use crawrad_actions::Author;
use crawrad_doc::Doc;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use makepad_widgets::{Cx, LiveNew};

/// Creates a headless IdeApp with an open Form document.
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
    app.set_center(Center::Design);
    app.acp_config.diff_review_mode = crawrad_agent::diff_review::DiffReviewMode::ApplyDirectly;

    (cx, app)
}

#[test]
fn test_tools_call_adds_component_author_agent_and_undo() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    assert_eq!(app.doc().unwrap().ir().nodes.len(), 1); // only form_main

    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "component.add",
            "arguments": {
                "class": "Button",
                "parent": "form_main",
                "x": 10,
                "y": 20,
                "w": 100,
                "h": 35
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["mutated"], true);
    assert_eq!(resp["result"]["data"]["action"], "component.add");
    assert_eq!(app.doc().unwrap().ir().nodes.len(), 2); // form_main + Button1

    // Verify transaction author was signed as Author::Agent
    let audit_log = app.doc().unwrap().audit();
    let last_entry = audit_log.last().expect("audit must have entry");
    assert!(
        matches!(last_entry.author, Author::Agent(_)),
        "audit entry author must be Author::Agent, got {:?}",
        last_entry.author
    );

    // Execute Undo (Ctrl+Z)
    let undo_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "edit.undo",
            "arguments": {}
        }
    });

    let undo_resp_str = server.handle_raw_request(&mut app, &mut cx, &undo_req.to_string());
    let undo_resp: Value = serde_json::from_str(&undo_resp_str).expect("valid json");
    assert_eq!(undo_resp["result"]["isError"], Value::Null);
    assert_eq!(undo_resp["result"]["data"]["mutated"], true);
    assert_eq!(undo_resp["result"]["data"]["action"], "edit.undo");

    // Document is reverted back to initial state
    assert_eq!(app.doc().unwrap().ir().nodes.len(), 1);
}

#[test]
fn test_context_guard_rejects_disabled_action_leaving_document_identical() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    let doc_json_before = app.doc().unwrap().ir().to_json().expect("json before");

    // Call edit.undo when undo stack is empty (disabled by predicate pode_desfazer)
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "edit.undo",
            "arguments": {}
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    // Guard rejected the action
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "action-disabled");

    // Document is 100% IDENTICAL before and after
    let doc_json_after = app.doc().unwrap().ir().to_json().expect("json after");
    assert_eq!(
        doc_json_before, doc_json_after,
        "Document MUST remain 100% identical when action is rejected by context guard"
    );
}

#[test]
fn test_mutation_proof_disabled_guard_protects_document() {
    let (_, app) = create_test_ide_app();

    // Verify directly on the action predicate: if undo stack is empty, habilitada is false
    let undo_action = crawrad_actions::catalogue()
        .into_iter()
        .find(|a| a.id == "edit.undo")
        .expect("edit.undo");

    let args = crawrad_actions::Args::default();
    let registry = &crawrad_project::installed::ide_catalog().catalog;
    let ctx = crawrad_actions::Context {
        doc: app.doc(),
        registry,
        clipboard: app.area_transferencia(),
        args,
        project: app.projeto_contexto(),
        author: Author::Agent("mcp".into()),
        editor_dirty: false,
        text: Default::default(),
    };

    let is_enabled = undo_action.enabled.allows(&ctx);
    assert!(
        !is_enabled,
        "action.habilitada MUST return false when undo stack is empty"
    );
}

#[test]
fn test_context_guard_wrong_view_and_view_switch_exemption() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    // Switch view to Code
    app.set_center(Center::Code);

    // 1. Calling designer action when in Code view -> wrong-view-code
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "component.add",
            "arguments": {
                "class": "Button"
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "wrong-view-code");

    // 2. View switch tool is exempt from guard
    let switch_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "view.toggle_code",
            "arguments": {}
        }
    });

    let switch_resp_str = server.handle_raw_request(&mut app, &mut cx, &switch_req.to_string());
    let switch_resp: Value = serde_json::from_str(&switch_resp_str).expect("valid json");
    assert_eq!(switch_resp["result"]["isError"], Value::Null);
    assert_eq!(app.center(), Center::Design);

    // 3. In Design view, calling code edit tool -> wrong-view-design
    let code_req = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "code.read",
            "arguments": {
                "path": "src/form_main.rs"
            }
        }
    });

    let code_resp_str = server.handle_raw_request(&mut app, &mut cx, &code_req.to_string());
    let code_resp: Value = serde_json::from_str(&code_resp_str).expect("valid json");
    assert_eq!(code_resp["result"]["isError"], true);
    assert_eq!(code_resp["result"]["data"]["code"], "wrong-view-design");
}

#[test]
fn test_read_before_edit_and_content_hash_enforcement() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    app.set_center(Center::Code);
    let file_path = PathBuf::from("src/calc.rs");
    app.set_editor_buffer(
        file_path.clone(),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }".into(),
    );

    // 1. Edit without prior read -> rejected with no-prior-read
    let edit_req_no_read = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": "src/calc.rs",
                "base_hash": "deadbeef1234",
                "content": "new code"
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &edit_req_no_read.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "no-prior-read");

    // 2. Read file -> returns content_hash
    let read_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "code.read",
            "arguments": {
                "path": "src/calc.rs"
            }
        }
    });

    let read_resp_str = server.handle_raw_request(&mut app, &mut cx, &read_req.to_string());
    let read_resp: Value = serde_json::from_str(&read_resp_str).expect("valid json");
    assert_eq!(read_resp["result"]["isError"], Value::Null);
    let hash1 = read_resp["result"]["data"]["content_hash"]
        .as_str()
        .expect("content_hash");
    assert_eq!(hash1.len(), 64); // sha256 hex string

    // 3. Edit with stale / wrong base_hash -> rejected with hash-mismatch
    let edit_req_stale = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": "src/calc.rs",
                "base_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "content": "pub fn sub(a: i32, b: i32) -> i32 { a - b }"
            }
        }
    });

    let stale_resp_str = server.handle_raw_request(&mut app, &mut cx, &edit_req_stale.to_string());
    let stale_resp: Value = serde_json::from_str(&stale_resp_str).expect("valid json");
    assert_eq!(stale_resp["result"]["isError"], true);
    assert_eq!(stale_resp["result"]["data"]["code"], "hash-mismatch");

    // 4. Edit with correct base_hash -> succeeds and updates live buffer
    let new_content = "pub fn add(a: i32, b: i32) -> i32 { a + b /* modified */ }";
    let edit_req_ok = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": "src/calc.rs",
                "base_hash": hash1,
                "content": new_content
            }
        }
    });

    let ok_resp_str = server.handle_raw_request(&mut app, &mut cx, &edit_req_ok.to_string());
    let ok_resp: Value = serde_json::from_str(&ok_resp_str).expect("valid json");
    assert_eq!(ok_resp["result"]["isError"], Value::Null);
    let hash2 = ok_resp["result"]["data"]["content_hash"]
        .as_str()
        .expect("new content_hash");
    assert_ne!(hash1, hash2);

    // Verify buffer was updated in live IDE state
    assert_eq!(app.get_editor_buffer(&file_path).unwrap(), new_content);

    // 5. Chained edit with hash2 succeeds
    let chained_content = "pub fn add(a: i32, b: i32) -> i32 { a + b /* chained */ }";
    let edit_req_chained = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "code.edit",
            "arguments": {
                "path": "src/calc.rs",
                "base_hash": hash2,
                "content": chained_content
            }
        }
    });

    let chain_resp_str =
        server.handle_raw_request(&mut app, &mut cx, &edit_req_chained.to_string());
    let chain_resp: Value = serde_json::from_str(&chain_resp_str).expect("valid json");
    assert_eq!(chain_resp["result"]["isError"], Value::Null);
    assert_eq!(app.get_editor_buffer(&file_path).unwrap(), chained_content);
}

#[test]
fn test_audit_log_records_applied_rejected_and_error() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp_log = std::env::temp_dir().join(format!("test_audit_{}.jsonl", std::process::id()));
    if temp_log.exists() {
        let _ = std::fs::remove_file(&temp_log);
    }

    let mut server = McpServer::new("test-audit-inst").with_audit_log(temp_log.clone());

    // 1. Applied: initialize
    let init_req = json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} });
    server.handle_raw_request(&mut app, &mut cx, &init_req.to_string());

    // 2. Rejected: wrong view
    app.set_center(Center::Code);
    let add_req = json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "component.add", "arguments": { "class": "Button" } } });
    server.handle_raw_request(&mut app, &mut cx, &add_req.to_string());

    // 3. Error: unknown method
    let err_req = json!({ "jsonrpc": "2.0", "id": 3, "method": "unknown/method", "params": {} });
    server.handle_raw_request(&mut app, &mut cx, &err_req.to_string());

    // Read the audit log file
    let log_content = std::fs::read_to_string(&temp_log).expect("read audit log");
    let lines: Vec<&str> = log_content.lines().collect();
    assert_eq!(lines.len(), 3, "expected 3 audit log records");

    let r1: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(r1["outcome"], "applied");
    assert_eq!(r1["tool"], "initialize");

    let r2: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(r2["outcome"], "rejected");
    assert_eq!(r2["tool"], "component.add");

    let r3: Value = serde_json::from_str(lines[2]).unwrap();
    assert_eq!(r3["outcome"], "error");
    assert_eq!(r3["tool"], "unknown/method");

    let _ = std::fs::remove_file(&temp_log);
}

#[test]
fn test_tools_call_form_read() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-form-read");

    let req = json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "form.read",
            "arguments": {}
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    let data = &resp["result"]["data"];
    assert_eq!(data["root"]["id"], "form_main");
    assert_eq!(data["root"]["class"], "Form");
    assert!(data["nodes"].is_array());
    assert_eq!(data["center_view"], "design");
    assert_eq!(data["is_dirty"], false);
}

#[test]
fn test_tools_call_project_info() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-project-info");

    let req = json!({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "tools/call",
        "params": {
            "name": "project.info",
            "arguments": {}
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    let data = &resp["result"]["data"];
    assert_eq!(data["center_view"], "design");
    assert_eq!(data["is_dirty"], false);
    assert!(data["open_buffers"].is_array());
}

#[test]
fn test_tools_call_action_without_a_project_is_refused() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-action-no-project");
    let req = json!({
        "jsonrpc": "2.0",
        "id": 13,
        "method": "tools/call",
        "params": {
            "name": "action.execute",
            "arguments": { "name": "explorer.toggle_folder", "path": "src" }
        }
    });
    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let text = resp_str.to_string();
    assert!(
        text.contains("action-disabled") && text.contains("needs an open project"),
        "expected the no-project refusal, got: {text}"
    );
}

#[test]
fn test_tools_call_action_noop_reported() {
    let (mut cx, mut app) = create_test_ide_app();
    // The Explorer's commands need an open project: its folder is the tree's root.
    app.desktop_root = Some(std::env::temp_dir());
    let mut server = McpServer::new("test-action-noop");

    // Execute an action like explorer.toggle_folder - produces no document or view state change
    let req = json!({
        "jsonrpc": "2.0",
        "id": 12,
        "method": "tools/call",
        "params": {
            "name": "action.execute",
            "arguments": {
                "name": "explorer.toggle_folder",
                "path": "src"
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    let content = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(
        content.contains("no document or state change"),
        "expected no-op notification, got: {content}"
    );
}

#[test]
fn test_tools_call_project_create_and_open() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-project-create");

    let temp_dir =
        std::env::temp_dir().join(format!("crawrad_mcp_test_create_{}", std::process::id()));
    if temp_dir.exists() {
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
    std::fs::create_dir_all(&temp_dir).expect("create temp dir");

    let req = json!({
        "jsonrpc": "2.0",
        "id": 13,
        "method": "tools/call",
        "params": {
            "name": "project.create",
            "arguments": {
                "name": "mcp_sample_app",
                "template": "console_app",
                "path": temp_dir.to_str().unwrap(),
                "open": true
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    // `mcp_sample_app/` is the workspace, the project lives inside it
    let workspace = temp_dir.join("mcp_sample_app");
    let proj_dir = workspace.join("mcp_sample_app");
    let root = std::fs::read_to_string(workspace.join("Cargo.toml")).expect("workspace manifest");
    assert!(root.contains("[workspace]") && root.contains("\"mcp_sample_app\""));
    assert!(proj_dir.join("Cargo.toml").exists());
    assert!(proj_dir.join("src").join("main.rs").exists());
    let created = resp["result"]["data"]["created_path"]
        .as_str()
        .unwrap_or_default();
    assert_eq!(
        std::path::Path::new(created),
        proj_dir,
        "created_path is the project, inside its workspace"
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_open_cargo_toml_in_editor() {
    let (mut cx, mut app) = create_test_ide_app();
    let temp = std::env::temp_dir().join(format!("test_cargo_open_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp);
    let cargo_path = temp.join("Cargo.toml");
    std::fs::write(
        &cargo_path,
        "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    app.host
        .set_active_file(Some(temp.join("src").join("main.crawform")));
    app.abrir_no_editor(&mut cx, cargo_path.clone());
    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn test_designer_tools_lifecycle() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst-designer");

    // 1. designer.get_active_form
    let req_form = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "designer.get_active_form",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_form.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["root"]["id"], "form_main");

    // 2. designer.add_component (Button)
    let req_add = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "designer.add_component",
            "arguments": {
                "class": "Button",
                "x": 30.0,
                "y": 40.0,
                "w": 120.0,
                "h": 40.0
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_add.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    let comp_id = resp["result"]["data"]["id"].as_str().unwrap().to_string();
    assert_eq!(comp_id, "button1");

    // 3. designer.list_components
    let req_list = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "designer.list_components",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_list.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["count"], 1);
    assert_eq!(resp["result"]["data"]["components"][0]["id"], comp_id);

    // 4. designer.get_component_properties
    let req_props = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "designer.get_component_properties",
            "arguments": { "id": &comp_id }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_props.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["id"], comp_id);
    assert_eq!(resp["result"]["data"]["class"], "Button");

    // 5. designer.set_component_properties
    let req_set = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "designer.set_component_properties",
            "arguments": {
                "id": &comp_id,
                "properties": { "text": "Save Data" }
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_set.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["properties_updated"], 1);

    // 5b. A key the class does not have is REFUSED and said so — never
    // counted as updated — and the valid key in the same call still applies.
    let req_bad = json!({
        "jsonrpc": "2.0",
        "id": 55,
        "method": "tools/call",
        "params": {
            "name": "designer.set_component_properties",
            "arguments": {
                "id": &comp_id,
                "properties": { "caption": "Nope", "hint": "ok" }
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_bad.to_string()))
            .unwrap();
    let data = &resp["result"]["data"];
    assert_eq!(data["properties_updated"], 1, "{resp}");
    assert_eq!(data["applied"], json!(["hint"]), "{resp}");
    assert_eq!(data["refused"][0]["key"], "caption", "{resp}");
    assert!(
        data["refused"][0]["reason"]
            .as_str()
            .unwrap_or("")
            .contains("text"),
        "the refusal names the class's keys, so the agent can correct itself: {resp}"
    );

    // 6. designer.wire_event
    let req_wire = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "designer.wire_event",
            "arguments": {
                "id": &comp_id,
                "event": "on_click"
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_wire.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(
        resp["result"]["data"]["handler"],
        format!("{comp_id}_click")
    );

    // 7. designer.delete_component
    let req_del = json!({
        "jsonrpc": "2.0",
        "id": 7,
        "method": "tools/call",
        "params": {
            "name": "designer.delete_component",
            "arguments": { "id": &comp_id }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_del.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["deleted_id"], comp_id);

    // Error test: delete root
    let req_del_root = json!({
        "jsonrpc": "2.0",
        "id": 8,
        "method": "tools/call",
        "params": {
            "name": "designer.delete_component",
            "arguments": { "id": "form_main" }
        }
    });
    let resp: Value = serde_json::from_str(&server.handle_raw_request(
        &mut app,
        &mut cx,
        &req_del_root.to_string(),
    ))
    .unwrap();
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "cannot-delete-root");

    // Error test: component not found
    let req_not_found = json!({
        "jsonrpc": "2.0",
        "id": 9,
        "method": "tools/call",
        "params": {
            "name": "designer.get_component_properties",
            "arguments": { "id": "NonExistent" }
        }
    });
    let resp: Value = serde_json::from_str(&server.handle_raw_request(
        &mut app,
        &mut cx,
        &req_not_found.to_string(),
    ))
    .unwrap();
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "component-not-found");
}

#[test]
fn test_editor_tools_lifecycle() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst-editor");

    let file_path = PathBuf::from("src/sample.rs");
    let initial_content = "line 1\nline 2: to be edited\nline 3\n";
    app.set_editor_buffer(file_path.clone(), initial_content.to_string());
    app.host.set_active_file(Some(file_path.clone()));

    // 1. editor.get_active_file
    let req_act = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "editor.get_active_file",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_act.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(
        resp["result"]["data"]["path"]
            .as_str()
            .unwrap()
            .contains("sample.rs")
    );

    // 2. editor.get_buffer_content
    let req_buf = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "editor.get_buffer_content",
            "arguments": {
                "path": "src/sample.rs"
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_buf.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["lines_count"], 3);
    let hash1 = resp["result"]["data"]["content_hash"]
        .as_str()
        .unwrap()
        .to_string();

    // 3. editor.apply_edit (surgical line replacement of line 2)
    let req_edit = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "editor.apply_edit",
            "arguments": {
                "path": "src/sample.rs",
                "start_line": 2,
                "end_line": 2,
                "new_text": "line 2: surgically updated",
                "base_hash": hash1
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_edit.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);

    // Verify buffer was updated
    let updated = app.get_editor_buffer(&file_path).unwrap();
    assert!(updated.contains("line 2: surgically updated"));
    assert!(updated.contains("line 1"));
    assert!(updated.contains("line 3"));

    // 4. editor.list_open_files
    let req_list = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "editor.list_open_files",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_list.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(resp["result"]["data"]["count"].as_u64().unwrap() >= 1);

    // 5. editor.apply_edit with invalid line range
    let req_inv = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "editor.apply_edit",
            "arguments": {
                "path": "src/sample.rs",
                "start_line": 99,
                "end_line": 105,
                "new_text": "impossible"
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_inv.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "invalid-line-range");
}

#[test]
fn test_project_tools_lifecycle() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst-proj");

    let temp_dir = std::env::temp_dir().join(format!("test_mcp_proj_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let cargo_toml = temp_dir.join("Cargo.toml");
    std::fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0\"\n",
    )
    .unwrap();
    app.host.set_active_file(Some(cargo_toml));

    // 1. project.create_file
    let req_cf = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "project.create_file",
            "arguments": {
                "path": "test_note.txt",
                "content": "CrawRAD MCP tools are live!",
                "open_in_editor": false
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_cf.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(temp_dir.join("test_note.txt").exists());

    // 2. project.get_dependencies
    let req_deps = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "project.get_dependencies",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_deps.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(resp["result"]["data"]["dependencies"]["serde"].is_string());

    // 3. project.get_tree
    let req_tree = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "project.get_tree",
            "arguments": { "max_depth": 2 }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_tree.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(resp["result"]["data"]["tree"].as_array().unwrap().len() >= 2);

    // 4. project.create_form
    let req_cform = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "project.create_form",
            "arguments": {
                "name": "form_profile",
                "folder": "src"
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_cform.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(temp_dir.join("src").join("form_profile.crawform").exists());
    assert!(temp_dir.join("src").join("form_profile.rs").exists());

    // 5. project.delete_file
    let req_df = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "project.delete_file",
            "arguments": {
                "path": "test_note.txt"
            }
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req_df.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert!(!temp_dir.join("test_note.txt").exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_run_stop_when_idle() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst-run");

    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "run.stop",
            "arguments": {}
        }
    });
    let resp: Value =
        serde_json::from_str(&server.handle_raw_request(&mut app, &mut cx, &req.to_string()))
            .unwrap();
    assert_eq!(resp["result"]["isError"], Value::Null);
    assert_eq!(resp["result"]["data"]["status"], "not_running");
}

#[test]
fn test_permission_classification_for_all_tools() {
    use crawrad_agent::permission::{OperationKind, PermissionManager};

    // Reads
    assert_eq!(
        PermissionManager::classify_tool("designer.get_active_form"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.list_components"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.get_selection"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.get_component_properties"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.get_active_file"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.get_buffer_content"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.get_selection"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.open_file"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.list_open_files"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("project.get_tree"),
        OperationKind::Read
    );
    assert_eq!(
        PermissionManager::classify_tool("project.get_dependencies"),
        OperationKind::Read
    );

    // Writes
    assert_eq!(
        PermissionManager::classify_tool("designer.add_component"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.set_component_properties"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.delete_component"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("designer.wire_event"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("editor.apply_edit"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("project.create_file"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("project.create_form"),
        OperationKind::Write
    );
    assert_eq!(
        PermissionManager::classify_tool("project.delete_file"),
        OperationKind::Write
    );

    // Executes
    assert_eq!(
        PermissionManager::classify_tool("build.check"),
        OperationKind::Execute
    );
    assert_eq!(
        PermissionManager::classify_tool("build.build"),
        OperationKind::Execute
    );
    assert_eq!(
        PermissionManager::classify_tool("build.get_diagnostics"),
        OperationKind::Execute
    );
    assert_eq!(
        PermissionManager::classify_tool("run.start"),
        OperationKind::Execute
    );
    assert_eq!(
        PermissionManager::classify_tool("run.stop"),
        OperationKind::Execute
    );
    assert_eq!(
        PermissionManager::classify_tool("project.add_dependency"),
        OperationKind::Execute
    );
}

#[test]
fn test_tools_list_publishes_dynamic_extension_capabilities_and_schemas() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    let manifest = crawrad_extension_api::manifest::ExtensionManifest {
        id: "acme.git-tools".to_string(),
        name: "Git Tools".to_string(),
        version: "1.0.0".to_string(),
        publisher: "Acme".to_string(),
        description: "Git integration tools".to_string(),
        contributions: crawrad_extension_api::manifest::ExtensionContributions {
            actions: vec![crawrad_extension_api::contribution::ActionContribution {
                id: "status".to_string(),
                label: "Git Status".to_string(),
                description: Some("Show working tree status".to_string()),
                shortcut: Some("Ctrl+Shift+G".to_string()),
                icon: None,
            }],
            tools: vec![crawrad_extension_api::contribution::ToolContribution {
                name: "git_commit".to_string(),
                description: "Commit staged files".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Commit message"
                        }
                    },
                    "required": ["message"]
                }),
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    app.capabilities_mut()
        .register_extension_manifest(&manifest)
        .expect("register extension manifest");

    let list_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &list_req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 1);
    let tools = resp["result"]["tools"].as_array().expect("tools array");

    // 1. Native tools are present
    assert!(tools.iter().any(|t| t["name"] == "code.read"));
    assert!(
        tools
            .iter()
            .any(|t| t["name"] == "designer.get_active_form")
    );
    assert!(tools.iter().any(|t| t["name"] == "component.add"));

    // 2. Extension action capability is published
    let action_tool = tools
        .iter()
        .find(|t| t["name"] == "acme.git-tools.status")
        .expect("extension action tool found");
    assert_eq!(
        action_tool["description"].as_str().unwrap(),
        "Show working tree status"
    );

    // 3. Extension tool capability is published with exact schema
    let commit_tool = tools
        .iter()
        .find(|t| t["name"] == "acme.git-tools.tool.git_commit")
        .expect("extension tool found");
    assert_eq!(
        commit_tool["description"].as_str().unwrap(),
        "Commit staged files"
    );
    assert_eq!(commit_tool["inputSchema"]["type"], "object");
    assert_eq!(
        commit_tool["inputSchema"]["properties"]["message"]["type"],
        "string"
    );
    assert_eq!(commit_tool["inputSchema"]["required"], json!(["message"]));
}

#[test]
fn test_tools_call_dispatches_extension_tool_with_structured_result_and_audit() {
    use crawrad_extension_api::traits::{ExtensionContext, MessageLevel};

    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    let manifest = crawrad_extension_api::manifest::ExtensionManifest {
        id: "acme.linter".to_string(),
        name: "Acme Linter".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Acme".to_string(),
        description: "Static code analysis".to_string(),
        contributions: crawrad_extension_api::manifest::ExtensionContributions {
            actions: Vec::new(),
            tools: vec![crawrad_extension_api::contribution::ToolContribution {
                name: "analyze".to_string(),
                description: "Run linter analysis on active file".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "severity": { "type": "string" }
                    },
                    "required": ["severity"]
                }),
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    app.capabilities_mut()
        .register_extension_manifest(&manifest)
        .expect("register extension manifest");

    // Registers procedural handler that emits messages
    let msgs_clone = app.extensions().context.messages.clone();
    app.extensions_mut()
        .context
        .register_command(
            "acme.linter.tool.analyze",
            Box::new(move || {
                let mut guard = msgs_clone.lock().unwrap();
                guard.push((
                    MessageLevel::Info,
                    "Analysis complete: 0 warnings found".into(),
                ));
            }),
        )
        .expect("register command");

    // 1. Successful call via canonical ID
    let call_req = json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "acme.linter.tool.analyze",
            "arguments": {
                "severity": "warning"
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &call_req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 10);
    assert_ne!(resp["result"]["isError"], true);
    assert_eq!(
        resp["result"]["data"]["capability"],
        "acme.linter.tool.analyze"
    );
    assert_eq!(
        resp["result"]["data"]["status"],
        "Extension tool 'analyze' invoked"
    );
    assert_eq!(
        resp["result"]["data"]["messages"][0]["message"],
        "Analysis complete: 0 warnings found"
    );

    // Verify audit with Author::Agent in CapabilityRegistry
    let audit_entries = app.capabilities().audit_log();
    let last_entry = audit_entries.last().expect("audit entry recorded");
    assert_eq!(last_entry.capability_id, "acme.linter.tool.analyze");
    match &last_entry.author {
        Author::Agent(s) => assert!(
            s.starts_with("mcp-"),
            "author string should start with mcp-, got {s}"
        ),
        other => panic!("expected Author::Agent, got {:?}", other),
    }
    assert!(last_entry.success);

    // 2. Call via short name "analyze" (friendly resolution)
    let call_short = json!({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "tools/call",
        "params": {
            "name": "analyze",
            "arguments": {
                "severity": "error"
            }
        }
    });

    let resp_short_str = server.handle_raw_request(&mut app, &mut cx, &call_short.to_string());
    let resp_short: Value = serde_json::from_str(&resp_short_str).expect("valid json");
    assert_ne!(resp_short["result"]["isError"], true);
    assert_eq!(
        resp_short["result"]["data"]["capability"],
        "acme.linter.tool.analyze"
    );
}

#[test]
fn test_tools_call_parameter_validation_and_view_affinity_guards() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-inst");

    let manifest = crawrad_extension_api::manifest::ExtensionManifest {
        id: "acme.db".to_string(),
        name: "Acme Database".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Acme".to_string(),
        description: "DB schema tools".to_string(),
        contributions: crawrad_extension_api::manifest::ExtensionContributions {
            actions: Vec::new(),
            tools: vec![crawrad_extension_api::contribution::ToolContribution {
                name: "query_schema".to_string(),
                description: "Query DB schema".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "table": { "type": "string" }
                    },
                    "required": ["table"]
                }),
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    app.capabilities_mut()
        .register_extension_manifest(&manifest)
        .expect("register extension manifest");

    // 1. Missing required parameter "table" -> rejected with "invalid-params"
    let req_missing_param = json!({
        "jsonrpc": "2.0",
        "id": 20,
        "method": "tools/call",
        "params": {
            "name": "acme.db.tool.query_schema",
            "arguments": {}
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req_missing_param.to_string());
    let resp: Value = serde_json::from_str(&resp_str).unwrap();
    assert_eq!(resp["result"]["isError"], true);
    assert_eq!(resp["result"]["data"]["code"], "invalid-params");

    // 2. View Affinity Guard: registers capability that requires Center::Code
    let mut code_cap = crawrad_actions::capability::Capability::custom(
        "custom.code_action",
        "Code Action",
        "Requires code view",
        "Custom",
        crawrad_actions::capability::CapabilitySource::Native,
        |_| Ok(crawrad_actions::Effect::none().with_status("Code action ok")),
    );
    code_cap.view_affinity = crawrad_actions::ViewAffinity::Code;
    app.capabilities_mut().register(code_cap).unwrap();

    // Since app is in Center::Design (default of create_test_ide_app):
    assert_eq!(app.center(), Center::Design);
    let req_view_mismatch = json!({
        "jsonrpc": "2.0",
        "id": 21,
        "method": "tools/call",
        "params": {
            "name": "custom.code_action",
            "arguments": {}
        }
    });

    let resp_view_str =
        server.handle_raw_request(&mut app, &mut cx, &req_view_mismatch.to_string());
    let resp_view: Value = serde_json::from_str(&resp_view_str).unwrap();
    assert_eq!(resp_view["result"]["isError"], true);
    assert_eq!(resp_view["result"]["data"]["code"], "wrong-view-design");
}

/// A panel id is an `Enum(&[])` whose values come from a `Domain`
/// (`crawrad-actions`, `PARAM_PANEL`). The validator used to check the empty
/// literal list first and refuse every panel with `Allowed values: []`, so
/// no `layout.*` action on a panel could run over MCP.
#[test]
fn test_enum_param_with_domain_accepts_domain_values_only() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-enum-domain");
    let call = |server: &mut McpServer, app: &mut IdeApp, cx: &mut Cx, panel: &str| -> String {
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "layout.toggle_panel",
                "arguments": { "panel": panel }
            }
        });
        server.handle_raw_request(app, cx, &req.to_string())
    };

    let valid = call(&mut server, &mut app, &mut cx, "components");
    assert!(
        !valid.contains("invalid-params"),
        "a panel of the domain must pass validation, got: {valid}"
    );

    let bogus = call(&mut server, &mut app, &mut cx, "no-such-panel");
    assert!(
        bogus.contains("invalid-params") && bogus.contains("components"),
        "a value outside the domain must be refused and name the real panels, got: {bogus}"
    );
}

#[test]
fn test_chat_get_messages_returns_turns_with_meta() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-chat-mcp");

    let mut u = crawrad_agent::ChatMessage::user("u1", "Hello from MCP test");
    u.sent_at = Some("14:30".into());
    let mut a = crawrad_agent::ChatMessage::assistant("a1", "Reply from MCP test");
    a.sent_at = Some("14:31".into());
    app.chat.session.messages = vec![u, a];

    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "view.get_chat",
            "arguments": {}
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");

    assert_eq!(resp["result"]["isError"], Value::Null);
    let turns = resp["result"]["data"]["turns"]
        .as_array()
        .expect("turns array");
    assert_eq!(turns.len(), 1);
    assert_eq!(turns[0]["user"]["text"], "Hello from MCP test");
    assert_eq!(turns[0]["user"]["meta"], "14:30 ✓✓");
    assert_eq!(turns[0]["reply"]["text"], "Reply from MCP test");
    assert_eq!(turns[0]["reply"]["meta"], "14:31");
}

/// D9: a console project has no form; `project.open` leaves the workspace
/// folder as the host's file, and the Explorer, the chat and the LSP find
/// the project through the code file in front, not "No folder open".
#[test]
fn without_a_form_the_project_is_found_through_the_code_file() {
    let (mut cx, mut app) = create_test_ide_app();
    let dir = std::env::temp_dir().join(format!("d9-explorer-root-{}", std::process::id()));
    let pkg = dir.join("app");
    std::fs::create_dir_all(pkg.join("src")).expect("project folder");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace]
members = [\"app\"]
",
    )
    .expect("workspace");
    let manifest = "[package]
name = \"app\"
version = \"0.1.0\"
edition = \"2024\"
";
    std::fs::write(pkg.join("Cargo.toml"), manifest).expect("manifest");
    let main = pkg.join("src").join("main.rs");
    std::fs::write(
        &main,
        "fn main() {}
",
    )
    .expect("main.rs");
    app.host.set_active_file(Some(dir.clone()));
    let mut server = McpServer::new("test-d9");
    let req = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": { "name": "editor.open_file", "arguments": { "path": main } }
    });
    server.handle_raw_request(&mut app, &mut cx, &req.to_string());
    let same = |p: Option<PathBuf>| p.and_then(|p| p.canonicalize().ok());
    let (root, member) = (same(app.raiz()), same(app.membro()));
    let (want_root, want_member) = (same(Some(dir.clone())), same(Some(pkg)));
    let _ = std::fs::remove_dir_all(&dir);
    assert!(want_root.is_some() && want_member.is_some());
    assert_eq!(root, want_root, "the Explorer's root is the workspace");
    assert_eq!(member, want_member, "the member is the package");
}

/// A Web or Mobile template is a Makepad app with no form yet; the
/// footer's target cell said "console". A UI framework in the manifest
/// means it is not a console program.
#[test]
fn a_ui_framework_without_a_form_is_not_a_console_project() {
    let kind_of = |deps: &str| {
        let (mut cx, mut app) = create_test_ide_app();
        app.host.set_doc(None);
        let dir =
            std::env::temp_dir().join(format!("pr4-kind-{}-{}", std::process::id(), deps.len()));
        std::fs::create_dir_all(dir.join("src")).expect("project folder");
        let manifest = format!(
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{deps}"
        );
        std::fs::write(dir.join("Cargo.toml"), manifest).expect("manifest");
        let main = dir.join("src").join("main.rs");
        std::fs::write(&main, "fn main() {}\n").expect("main.rs");
        let mut server = McpServer::new("test-pr4-kind");
        let req = json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": { "name": "editor.open_file", "arguments": { "path": main } }
        });
        server.handle_raw_request(&mut app, &mut cx, &req.to_string());
        let kind = app.project_kind_now().0;
        let _ = std::fs::remove_dir_all(&dir);
        kind
    };
    assert_eq!(kind_of(""), Some("console"));
    assert_eq!(kind_of("makepad-widgets = \"1.0\"\n"), None);
}

#[test]
fn test_view_get_rail_and_toolbox() {
    let (mut cx, mut app) = create_test_ide_app();
    let mut server = McpServer::new("test-rail-toolbox");

    let req_rail = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "view.get_rail",
            "arguments": {}
        }
    });
    let resp_rail_str = server.handle_raw_request(&mut app, &mut cx, &req_rail.to_string());
    let resp_rail: Value = serde_json::from_str(&resp_rail_str).expect("valid json");
    assert_eq!(resp_rail["result"]["isError"], Value::Null);
    assert!(resp_rail["result"]["data"]["panels"].is_array());

    let req_tb = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "view.get_toolbox",
            "arguments": {}
        }
    });
    let resp_tb_str = server.handle_raw_request(&mut app, &mut cx, &req_tb.to_string());
    let resp_tb: Value = serde_json::from_str(&resp_tb_str).expect("valid json");
    assert_eq!(resp_tb["result"]["isError"], Value::Null);
    assert_eq!(resp_tb["result"]["data"]["empty_state"], Value::Null);

    // Empty state appears when project has no form and no document is open.
    app.host.set_doc(None);
    app.project_without_form = true;
    let resp_tb_str2 = server.handle_raw_request(&mut app, &mut cx, &req_tb.to_string());
    let resp_tb2: Value = serde_json::from_str(&resp_tb_str2).expect("valid json");
    assert_eq!(resp_tb2["result"]["isError"], Value::Null);
    assert_eq!(
        resp_tb2["result"]["data"]["empty_state"]["title"],
        "This project has no form"
    );
    assert_eq!(
        resp_tb2["result"]["data"]["empty_state"]["body"],
        "Components go on a form. Add one and the Toolbox and the Structure fill in."
    );
    assert_eq!(
        resp_tb2["result"]["data"]["empty_state"]["button"]["text"],
        "New Form…"
    );
}
