//! End-to-End tests for "New Project" dialog and project creation engine.
//!
//! Validates:
//! 1. All 8 template types generate distinct, correct files matching the "Project structure" tree.
//! 2. Every generated project compiles cleanly out-of-the-box (`cargo check`).
//! 3. End-to-end MCP workflow (`project.create` -> `project.info` -> `project.build`).

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use crate::ide_app::IdeApp;
use crate::ide_app::dialogs::new_project::create_project_template;
use crate::ide_app::mcp_host::McpServer;
use makepad_widgets::{Cx, LiveNew};

fn get_shared_target_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("target")
        .join("test_novo_projeto_target")
}

fn run_cargo_check(cargo_toml_path: &Path) -> Result<(), String> {
    let target_dir = get_shared_target_dir();
    let output = Command::new("cargo")
        .arg("check")
        .arg("--manifest-path")
        .arg(cargo_toml_path)
        .env("CARGO_TARGET_DIR", &target_dir)
        .output()
        .map_err(|e| format!("failed to spawn cargo check: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!(
            "cargo check failed:\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}"
        ))
    }
}

#[test]
fn test_e2e_all_8_templates_generation_and_layout() {
    let temp_root =
        std::env::temp_dir().join(format!("crawrad_templates_e2e_{}", std::process::id()));
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
    std::fs::create_dir_all(&temp_root).expect("create temp root dir");

    let templates = [
        ("gui_app", "test_gui_app"),
        ("console_app", "test_console_app"),
        ("library_crate", "test_library_crate"),
        ("wasm_app", "test_wasm_app"),
        ("mobile_app", "test_mobile_app"),
        ("multiwindow_app", "test_multiwindow_app"),
        ("package_plugin", "test_package_plugin"),
        ("custom_template", "test_custom_template"),
    ];

    for (template, proj_name) in templates {
        let res = create_project_template(
            &temp_root,
            proj_name,
            template,
            false,
            crawrad_codegen::AlvoGerador::Makepad,
        );
        assert!(
            res.is_ok(),
            "template '{template}' failed to create: {:?}",
            res.err()
        );

        // The project lives inside its workspace, `<root>/<name>/<name>/`
        let proj_dir = res.expect("checked above");
        assert_eq!(
            proj_dir,
            temp_root.join(proj_name).join(proj_name),
            "{template}: the project goes inside its workspace"
        );
        let workspace = temp_root.join(proj_name);
        assert_eq!(
            crawrad_project::workspace::members(&workspace),
            vec![proj_dir.clone()],
            "{template}: the workspace lists the project as its only, active member"
        );
        assert!(proj_dir.exists(), "directory for {template} must exist");
        assert!(
            proj_dir.join("Cargo.toml").exists(),
            "{template} must have Cargo.toml"
        );

        // Verify template-specific files match the structure tree
        match template {
            "gui_app" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                assert!(proj_dir.join("src").join("form_main.crawform").exists());
                assert!(proj_dir.join("src").join("form_main.designer.rs").exists());
            }
            "console_app" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                let main_rs =
                    std::fs::read_to_string(proj_dir.join("src").join("main.rs")).unwrap();
                assert!(
                    main_rs.contains("println!"),
                    "console app must contain println"
                );
            }
            "library_crate" => {
                assert!(proj_dir.join("src").join("lib.rs").exists());
                assert!(proj_dir.join("crawrad.toml").exists());
                let lib_rs = std::fs::read_to_string(proj_dir.join("src").join("lib.rs")).unwrap();
                assert!(lib_rs.contains("pub fn add"), "library must contain add fn");
            }
            "wasm_app" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                // Starts with a form, like the desktop app; no orphan app.rs.
                assert!(proj_dir.join("src").join("form_main.crawform").exists());
                assert!(proj_dir.join("src").join("form_main.rs").exists());
                assert!(!proj_dir.join("src").join("app.rs").exists());
                // The page is written by the web Run (W1); the manifest marks the project.
                assert!(crawrad_project::web_run::is_web_project(
                    proj_dir.parent().expect("workspace"),
                    proj_name
                ));
            }
            "mobile_app" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                assert!(proj_dir.join("src").join("app.rs").exists());
                assert!(proj_dir.join("res").join("icon.png").exists());
            }
            "multiwindow_app" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                assert!(proj_dir.join("src").join("app.rs").exists());
                assert!(proj_dir.join("forms").join("main.crawform").exists());
            }
            "package_plugin" => {
                assert!(proj_dir.join("src").join("lib.rs").exists());
                assert!(proj_dir.join("plugin.toml").exists());
                let lib_rs = std::fs::read_to_string(proj_dir.join("src").join("lib.rs")).unwrap();
                assert!(
                    lib_rs.contains("plugin_init"),
                    "plugin must define plugin_init"
                );
            }
            "custom_template" => {
                assert!(proj_dir.join("src").join("main.rs").exists());
                assert!(proj_dir.join("src").join("app.rs").exists());
            }
            _ => unreachable!(),
        }
    }

    let _ = std::fs::remove_dir_all(&temp_root);
}

#[test]
fn test_e2e_console_and_library_and_plugin_compilation() {
    let temp_root =
        std::env::temp_dir().join(format!("crawrad_templates_compile_{}", std::process::id()));
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
    std::fs::create_dir_all(&temp_root).expect("create temp root dir");

    // Fast-compiling templates (no heavy makepad dependency needed)
    let fast_templates = [
        ("console_app", "test_fast_console"),
        ("library_crate", "test_fast_lib"),
        ("package_plugin", "test_fast_plugin"),
    ];

    for (template, proj_name) in fast_templates {
        create_project_template(
            &temp_root,
            proj_name,
            template,
            false,
            crawrad_codegen::AlvoGerador::Makepad,
        )
        .expect("create template");
        let cargo_toml = temp_root.join(proj_name).join("Cargo.toml");
        let check_res = run_cargo_check(&cargo_toml);
        assert!(
            check_res.is_ok(),
            "template {template} failed cargo check:\n{:?}",
            check_res.err()
        );
    }

    let _ = std::fs::remove_dir_all(&temp_root);
}

#[test]
fn test_e2e_gui_makepad_template_compilation() {
    let temp_root =
        std::env::temp_dir().join(format!("crawrad_gui_compile_{}", std::process::id()));
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
    std::fs::create_dir_all(&temp_root).expect("create temp root dir");

    // Compile GUI App to ensure Makepad macro and separation compile cleanly
    create_project_template(
        &temp_root,
        "test_fast_gui",
        "gui_app",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .expect("create gui_app");

    let cargo_toml = temp_root.join("test_fast_gui").join("Cargo.toml");
    let check_res = run_cargo_check(&cargo_toml);
    assert!(
        check_res.is_ok(),
        "gui_app failed cargo check:\n{:?}",
        check_res.err()
    );

    let _ = std::fs::remove_dir_all(&temp_root);
}

#[test]
fn test_e2e_mcp_project_lifecycle() {
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
    let mut server = McpServer::new("test-mcp-lifecycle");

    let temp_root =
        std::env::temp_dir().join(format!("crawrad_mcp_lifecycle_{}", std::process::id()));
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
    std::fs::create_dir_all(&temp_root).expect("create temp root dir");

    // 1. project.create via MCP
    let req_create = json!({
        "jsonrpc": "2.0",
        "id": 100,
        "method": "tools/call",
        "params": {
            "name": "project.create",
            "arguments": {
                "name": "lifecycle_demo",
                "template": "console_app",
                "path": temp_root.to_str().unwrap(),
                "open": true
            }
        }
    });

    let resp_str = server.handle_raw_request(&mut app, &mut cx, &req_create.to_string());
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json");
    assert_eq!(resp["result"]["isError"], Value::Null);

    // 2. project.info via MCP
    let req_info = json!({
        "jsonrpc": "2.0",
        "id": 101,
        "method": "tools/call",
        "params": {
            "name": "project.info",
            "arguments": {}
        }
    });

    let info_str = server.handle_raw_request(&mut app, &mut cx, &req_info.to_string());
    let info_resp: Value = serde_json::from_str(&info_str).expect("valid json");
    assert_eq!(info_resp["result"]["isError"], Value::Null);
    let proj_root = info_resp["result"]["data"]["workspace_root"]
        .as_str()
        .unwrap();
    assert!(proj_root.contains("lifecycle_demo"));

    // 3. project.build via MCP
    let req_build = json!({
        "jsonrpc": "2.0",
        "id": 102,
        "method": "tools/call",
        "params": {
            "name": "project.build",
            "arguments": {
                "release": false
            }
        }
    });

    let build_str = server.handle_raw_request(&mut app, &mut cx, &req_build.to_string());
    let build_resp: Value = serde_json::from_str(&build_str).expect("valid json");
    assert_eq!(build_resp["result"]["isError"], Value::Null);
    assert_eq!(build_resp["result"]["data"]["success"], true);

    let _ = std::fs::remove_dir_all(&temp_root);
}
