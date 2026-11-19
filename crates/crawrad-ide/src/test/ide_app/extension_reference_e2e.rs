//! End-to-end integration test validating the reference example extension
//! across the full circuit: manifest loading, UI menu injection, command execution,
//! automatic MCP tool exposure and invocation, and project template gallery contribution.

use makepad_widgets::{Cx, LiveNew};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

use crawrad_actions::Author;
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Doc;
use crawrad_extension_api::manifest::ExtensionManifest;
use crawrad_extension_api::traits::ExtensionContext;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

use crate::ide_app::IdeApp;
use crate::ide_app::mcp_host::McpServer;
use crate::ide_app::menu::Menu;
use crate::ide_app::source::Center;

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

    (cx, app)
}

fn get_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/crawrad-ide -> workspace root is two levels up
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn test_reference_extension_full_circuit_e2e() {
    let ws_root = get_workspace_root();
    let manifest_path = ws_root
        .join("examples")
        .join("extensions")
        .join("formatter")
        .join("manifest.json");

    assert!(
        manifest_path.is_file(),
        "Reference extension manifest not found at {:?}",
        manifest_path
    );

    // 1. Installation and Reading of Versioned Manifest
    let manifest_content = std::fs::read_to_string(&manifest_path).expect("read manifest content");
    let manifest = ExtensionManifest::from_json_str(&manifest_content)
        .expect("parse reference extension manifest");
    manifest.validate().expect("validate manifest schema");

    assert_eq!(manifest.id, "crawrad.formatter");
    assert_eq!(manifest.name, "CrawRAD Code Formatter");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.contributions.actions.len(), 1);
    assert_eq!(
        manifest.contributions.actions[0].id,
        "crawrad_format.format_file"
    );
    assert_eq!(manifest.contributions.menus.len(), 1);
    assert_eq!(manifest.contributions.menus[0].target, "Tools");
    assert_eq!(manifest.contributions.tools.len(), 1);
    assert_eq!(
        manifest.contributions.tools[0].name,
        "crawrad_format_format_file"
    );
    assert_eq!(manifest.contributions.templates.len(), 1);
    assert_eq!(
        manifest.contributions.templates[0].id,
        "crawrad.template.formatter"
    );

    // 2. Live IDE Initialization and Extension Registration
    let (mut cx, mut app) = create_test_ide_app();
    app.extensions_mut()
        .register_manifest(manifest.clone())
        .expect("register manifest into host");
    app.capabilities_mut()
        .register_extension_manifest(&manifest)
        .expect("register extension capabilities");

    // 3. IDE Menu Contribution (Tools > Format Code)
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let ctx = crawrad_actions::Context {
        doc: app.host.doc(),
        registry: &registry,
        clipboard: &area,
        args: crawrad_actions::Args::none(),
        project: app.projeto_contexto(),
        author: crawrad_actions::Author::User,
        editor_dirty: app.editor_sujo(),
        text: Default::default(),
    };

    let contributed_menus = app
        .extensions()
        .contributed_menus()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(contributed_menus.len(), 1);
    assert_eq!(contributed_menus[0].action_id, "crawrad_format.format_file");

    let tools_rows = crate::ide_app::menu::rows_with_scheme_and_contributions(
        Menu::Tools,
        &ctx,
        app.esquema_teclado,
        Some(app.capabilities()),
        &contributed_menus,
    );

    let format_row = tools_rows
        .iter()
        .find(|r| r.action.as_deref() == Some("crawrad_format.format_file"));
    assert!(
        format_row.is_some(),
        "Tools menu should contain 'crawrad_format.format_file'"
    );
    let format_row = format_row.unwrap();
    assert_eq!(format_row.label.as_ref(), "Format Code");

    // 4. Real Command Execution in the IDE with Editor Manipulation
    let test_file = "src/main.rs";
    app.editavel = Some(PathBuf::from(test_file));
    app.texto_na_vista = "fn   main(  )   {\nprintln!(\"hello\");\n}\n".to_string();

    // Synchronize initial buffer in editor service
    let initial_text = app.texto_na_vista.clone();
    if let Some(ref mut ed) = app.extensions_mut().context.editor {
        ed.active_file = Some(PathBuf::from(test_file));
        ed.set_buffer(Path::new(test_file), initial_text);
    }

    // Register the extension's functional formatting handler
    let msgs_clone = app.extensions().context.messages.clone();
    app.extensions_mut()
        .context
        .register_command(
            "crawrad_format.format_file",
            Box::new(move || {
                let mut guard = msgs_clone.lock().unwrap();
                guard.push((
                    crawrad_extension_api::traits::MessageLevel::Info,
                    "Document formatted successfully".into(),
                ));
            }),
        )
        .expect("register format command handler");

    // Execute command via IDE host
    let cmd_res = app
        .execute_extension_command_with_args("crawrad_format.format_file", None)
        .expect("command execution should succeed");
    assert_eq!(cmd_res, None);

    let messages = app.take_extension_messages();
    assert!(
        messages
            .iter()
            .any(|(_lvl, msg)| msg == "Document formatted successfully")
    );

    // 5. Automatic Operation Exposure as an MCP Tool for AI Agents
    let mut server = McpServer::new("e2e-formatter-inst");
    let list_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });

    let list_resp_str = server.handle_raw_request(&mut app, &mut cx, &list_req.to_string());
    let list_resp: Value = serde_json::from_str(&list_resp_str).expect("parse tools/list response");

    let tools = list_resp["result"]["tools"]
        .as_array()
        .expect("tools array in result");
    let formatter_tool = tools
        .iter()
        .find(|t| t["name"] == "crawrad.formatter.tool.crawrad_format_format_file");
    assert!(
        formatter_tool.is_some(),
        "tools/list must automatically expose the extension tool"
    );
    let formatter_tool = formatter_tool.unwrap();
    assert_eq!(
        formatter_tool["description"],
        "Formats the source code of the document open in the CrawRAD editor"
    );
    assert!(formatter_tool["inputSchema"]["properties"]["path"].is_object());

    // 6. Tool Call Validation via MCP / JSON-RPC Client
    // Also register the command with the canonical tool identifier
    let msgs_clone2 = app.extensions().context.messages.clone();
    app.extensions_mut()
        .context
        .register_command(
            "crawrad.formatter.tool.crawrad_format_format_file",
            Box::new(move || {
                let mut guard = msgs_clone2.lock().unwrap();
                guard.push((
                    crawrad_extension_api::traits::MessageLevel::Info,
                    "Format tool executed via MCP".into(),
                ));
            }),
        )
        .expect("register canonical tool command");

    let call_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "crawrad.formatter.tool.crawrad_format_format_file",
            "arguments": {
                "path": "src/main.rs"
            }
        }
    });

    let call_resp_str = server.handle_raw_request(&mut app, &mut cx, &call_req.to_string());
    let call_resp: Value = serde_json::from_str(&call_resp_str).expect("parse tools/call response");

    assert_eq!(call_resp["id"], 2);
    assert_ne!(call_resp["result"]["isError"], true);
    assert_eq!(
        call_resp["result"]["data"]["capability"],
        "crawrad.formatter.tool.crawrad_format_format_file"
    );

    // Validate audit log recorded in the IDE
    let audit_log = app.capabilities().audit_log();
    let last_entry = audit_log.last().expect("audit entry recorded");
    assert_eq!(
        last_entry.capability_id,
        "crawrad.formatter.tool.crawrad_format_format_file"
    );
    match &last_entry.author {
        Author::Agent(s) => assert!(
            s.starts_with("mcp-"),
            "expected author starting with mcp-, got {s}"
        ),
        other => panic!("expected Author::Agent, got {:?}", other),
    }
    assert!(last_entry.success);

    // 7. Project Template Contribution to the New Project Gallery
    let templates = app.extensions().contributed_templates();
    assert_eq!(templates.len(), 1);
    let tpl = templates[0];
    assert_eq!(tpl.id, "crawrad.template.formatter");
    assert_eq!(tpl.name, "Extension: Code Formatter Template");
    assert_eq!(tpl.icon.as_deref(), Some("✨"));
    assert_eq!(tpl.category.as_deref(), Some("Extensions"));
}
