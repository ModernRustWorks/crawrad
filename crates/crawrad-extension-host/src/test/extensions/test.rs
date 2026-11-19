use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crawrad_extension_api::contribution::{
    ActionContribution, CardArea, CardContribution, MenuContribution,
};
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_api::services::{
    ComponentInfo, ComponentRect, DesignerService, EditorService, ProjectService, TextEdit,
    TextRange,
};
use crawrad_extension_api::traits::{ExtensionContext, IdeExtension, MessageLevel};

use crate::extensions::host::ShellExtensionHost;
use crate::extensions::services::{
    HostDesignerService, HostEditorService, HostProjectService, apply_text_edit,
};

#[test]
fn test_apply_text_edit_single_line() {
    let initial = "fn main() {\n    println!(\"hello\");\n}\n";
    let edit = TextEdit::new(TextRange::new(2, 15, 2, 20), "world");
    let updated = apply_text_edit(initial, &edit).expect("edit should succeed");
    assert_eq!(updated, "fn main() {\n    println!(\"world\");\n}\n");
}

#[test]
fn test_apply_text_edit_multiline() {
    let initial = "line 1\nline 2\nline 3\nline 4\n";
    let edit = TextEdit::new(TextRange::new(2, 1, 3, 7), "replaced content");
    let updated = apply_text_edit(initial, &edit).expect("multiline edit should succeed");
    assert_eq!(updated, "line 1\nreplaced content\nline 4\n");
}

#[test]
fn test_apply_text_edit_bounds_check() {
    let initial = "abc\ndef\n";
    // 0-based should fail
    let edit_zero = TextEdit::new(TextRange::new(0, 1, 1, 2), "x");
    assert!(apply_text_edit(initial, &edit_zero).is_err());

    // Inverted range should fail
    let edit_inverted = TextEdit::new(TextRange::new(2, 1, 1, 1), "x");
    assert!(apply_text_edit(initial, &edit_inverted).is_err());

    // Line out of range should fail
    let edit_out_line = TextEdit::new(TextRange::new(10, 1, 10, 2), "x");
    assert!(apply_text_edit(initial, &edit_out_line).is_err());
}

#[test]
fn test_host_editor_service() {
    let mut editor = HostEditorService::new();
    let path = "test_file.rs";
    editor.set_buffer(path, "line one\nline two\nline three\n".into());

    assert_eq!(
        editor.get_content(path).unwrap(),
        "line one\nline two\nline three\n"
    );

    let lines = editor.get_lines(path, 1, 2).unwrap();
    assert_eq!(lines, vec!["line one", "line two"]);

    editor.open_file(path, Some((2, 5))).unwrap();
    assert_eq!(editor.active_file(), Some(path.to_string()));
    assert_eq!(editor.open_requests.len(), 1);
    assert_eq!(editor.open_requests[0], (PathBuf::from(path), Some((2, 5))));

    editor
        .apply_edit(path, TextEdit::new(TextRange::new(1, 6, 1, 9), "1"))
        .unwrap();
    assert_eq!(
        editor.get_content(path).unwrap(),
        "line 1\nline two\nline three\n"
    );
}

#[test]
fn test_host_project_service_with_tempdir() {
    let temp_dir =
        std::env::temp_dir().join(format!("crawrad_ext_proj_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("src")).unwrap();

    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"meu_projeto\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(temp_dir.join("src").join("main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        temp_dir.join("src").join("main_form.crawform"),
        "object MainForm: TForm end\n",
    )
    .unwrap();
    fs::write(
        temp_dir.join("src").join("main_form.rs"),
        "pub struct MainForm;\n",
    )
    .unwrap();

    let mut project = HostProjectService::new(Some(temp_dir.clone()));
    assert_eq!(project.active_project_name(), Some("meu_projeto".into()));

    let files = project.list_project_files().unwrap();
    assert!(files.iter().any(|f| f.ends_with("main.rs")));
    assert!(files.iter().any(|f| f.ends_with("main_form.crawform")));

    let forms = project.list_forms().unwrap();
    assert_eq!(forms.len(), 1);
    assert_eq!(forms[0].name, "main_form");
    assert!(forms[0].form_file.ends_with("main_form.crawform"));
    assert!(forms[0].code_file.ends_with("main_form.rs"));

    assert!(!project.build_requested);
    project.request_build().unwrap();
    assert!(project.build_requested);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_host_designer_service() {
    let mut designer = HostDesignerService::new();
    designer.active_form_name = Some("Form1".into());

    designer.set_component(ComponentInfo {
        name: "btnSalvar".into(),
        class_name: "Button".into(),
        parent_name: None,
        rect: ComponentRect::new(10.0, 20.0, 80.0, 30.0),
        is_visual: true,
    });

    assert_eq!(designer.active_form_name(), Some("Form1".into()));
    let comps = designer.list_components().unwrap();
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0].name, "btnSalvar");

    designer
        .set_property("btnSalvar", "caption", "Gravar Dados")
        .unwrap();
    assert_eq!(
        designer.get_property("btnSalvar", "caption").unwrap(),
        Some("Gravar Dados".into())
    );

    designer.select_components(&["btnSalvar"]).unwrap();
    assert_eq!(designer.selected_components(), vec!["btnSalvar"]);

    designer.delete_selected().unwrap();
    assert!(designer.list_components().unwrap().is_empty());
    assert_eq!(designer.deleted_components, vec!["btnSalvar"]);
}

struct DummyExtension {
    triggered: Arc<AtomicBool>,
}

impl IdeExtension for DummyExtension {
    fn id(&self) -> &str {
        "test.dummy"
    }

    fn activate(
        &mut self,
        ctx: &mut dyn ExtensionContext,
    ) -> Result<(), crawrad_extension_api::error::ExtensionError> {
        let flag = self.triggered.clone();
        ctx.register_command(
            "test.action",
            Box::new(move || {
                flag.store(true, Ordering::SeqCst);
            }),
        )?;
        ctx.show_message(MessageLevel::Info, "Dummy activated!");
        Ok(())
    }
}

#[test]
fn test_shell_extension_host_lifecycle() {
    let mut host = ShellExtensionHost::new(None);
    let triggered = Arc::new(AtomicBool::new(false));

    let manifest = ExtensionManifest {
        id: "test.dummy".into(),
        name: "Dummy Extension".into(),
        version: "1.0.0".into(),
        publisher: "CrawRAD".into(),
        description: "Test extension".into(),
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "test.action".into(),
                label: "Run Test Action".into(),
                description: Some("Performs dummy test".into()),
                shortcut: Some("Ctrl+Shift+T".into()),
                icon: None,
            }],
            menus: vec![MenuContribution {
                target: "Tools".into(),
                action_id: "test.action".into(),
                position: Default::default(),
                group: None,
            }],
            cards: vec![CardContribution {
                id: "test.card".into(),
                title: "Test Panel".into(),
                area: CardArea::Bottom,
                icon: None,
                closeable: true,
            }],
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    host.register_manifest(manifest)
        .expect("manifest should register");
    assert_eq!(host.contributed_actions().len(), 1);
    assert_eq!(host.contributed_menus().len(), 1);
    assert_eq!(host.contributed_cards().len(), 1);

    let ext = Box::new(DummyExtension {
        triggered: triggered.clone(),
    });
    host.register_extension(ext)
        .expect("extension should activate");

    let messages = host.context.take_messages();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].1, "Dummy activated!");

    assert!(!triggered.load(Ordering::SeqCst));
    host.execute_command("test.action")
        .expect("command execution should succeed");
    assert!(triggered.load(Ordering::SeqCst));
}

#[test]
fn test_scan_workspace_extensions() {
    let temp_dir =
        std::env::temp_dir().join(format!("crawrad_ext_scan_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    let ext_dir = temp_dir
        .join(".crawrad")
        .join("extensions")
        .join("my_git_ext");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = r#"{
            "id": "my.git.ext",
            "name": "Git Integration",
            "version": "0.1.0",
            "publisher": "CrawRAD Team",
            "contributions": {
                "actions": [
                    { "id": "git.status", "label": "Git Status" }
                ]
            }
        }"#;
    fs::write(ext_dir.join("manifest.json"), json).unwrap();

    let host = ShellExtensionHost::new(Some(temp_dir.clone()));
    assert_eq!(host.manifests.len(), 1);
    assert_eq!(host.manifests[0].id, "my.git.ext");
    assert_eq!(host.contributed_actions().len(), 1);
    assert_eq!(host.contributed_actions()[0].id, "git.status");

    let _ = fs::remove_dir_all(&temp_dir);
}

pub(crate) fn get_mock_extension_binary() -> PathBuf {
    // The test runs from `<target>/debug/deps/<test>.exe`, and cargo puts the
    // binaries one level up, in `<target>/debug/`. Deriving the folder from the
    // test's own executable is what makes this work with any `CARGO_TARGET_DIR`
    // (a fixed `target/debug` only worked where a stale binary happened to be).
    let exe = std::env::current_exe().unwrap();
    let debug_dir = exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("the test runs from <target>/debug/deps")
        .to_path_buf();
    let binary = debug_dir.join(if cfg!(windows) {
        "mock_extension.exe"
    } else {
        "mock_extension"
    });
    // ALWAYS built (a no-op when fresh), into this very target: a binary that
    // merely exists may be older than the mock's source — a new method the
    // test drives would then never be answered.
    let target = debug_dir.parent().expect("<target>/debug").to_path_buf();
    let status = std::process::Command::new("cargo")
        .env("CARGO_TARGET_DIR", &target)
        .args([
            "build",
            "-p",
            "crawrad-extension-host",
            "--bin",
            "mock_extension",
        ])
        .status()
        .expect("cargo build mock_extension");
    assert!(status.success(), "building mock_extension failed");
    assert!(
        binary.exists(),
        "mock_extension was built but is not at {}",
        binary.display()
    );
    binary
}

#[test]
fn test_process_extension_lifecycle_and_handshake() {
    use crate::extensions::process::{ProcessExtension, ProcessStatus};

    let bin_path = get_mock_extension_binary();
    let manifest = ExtensionManifest {
        id: "acme.mock".to_string(),
        name: "Mock Ext".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Acme".to_string(),
        description: "Mock test extension".to_string(),
        entrypoint: Some("mock_extension".to_string()),
        activation_events: vec!["*".to_string()],
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "mock.action".to_string(),
                label: "Mock Action".to_string(),
                description: None,
                shortcut: None,
                icon: None,
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
    };

    let mut ext = ProcessExtension::new(manifest, bin_path, None);
    assert_eq!(ext.status, ProcessStatus::Unstarted);

    // 1. Start the process and run the extension/initialize handshake
    ext.start().expect("start extension process");
    assert!(ext.is_alive());
    assert_eq!(ext.status, ProcessStatus::Running);

    // 2. Heartbeat ping / pong
    assert!(ext.ping().expect("ping should succeed"));

    // 3. Run a command over stdio IPC
    let res = ext
        .execute_command("mock.action", None)
        .expect("command execution succeeded")
        .expect("structured result returned");
    assert_eq!(res["executed"], "mock.action");
    assert_eq!(res["status"], "ok");

    // 4. Receive asynchronous window/showMessage messages
    let msgs = ext.take_messages();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].0, MessageLevel::Info);
    assert_eq!(msgs[0].1, "Mock executed mock.action");

    // 5. Graceful shutdown
    ext.shutdown().expect("graceful shutdown");
    assert!(!ext.is_alive());
    assert_eq!(ext.status, ProcessStatus::Stopped);
}

#[test]
fn test_process_extension_fault_isolation_on_crash() {
    use crate::extensions::process::{ProcessExtension, ProcessStatus};
    use crawrad_extension_api::error::ExtensionError;

    let bin_path = get_mock_extension_binary();
    let manifest = ExtensionManifest {
        id: "acme.crasher".to_string(),
        name: "Crasher Ext".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Acme".to_string(),
        description: "Fault isolation test extension".to_string(),
        entrypoint: Some("mock_extension".to_string()),
        activation_events: vec!["onCommand:mock.crash".to_string()],
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "mock.crash".to_string(),
                label: "Crash Subprocess".to_string(),
                description: None,
                shortcut: None,
                icon: None,
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
    };

    let mut ext = ProcessExtension::new(manifest, bin_path, None);
    ext.start().expect("start extension process");
    assert!(ext.is_alive());

    // Run a command that makes the child process exit(101)
    let err = ext
        .execute_command("mock.crash", None)
        .expect_err("should return error on child crash");

    // The IDE neither panics nor dies: it receives ProcessCrashed
    match err {
        ExtensionError::ProcessCrashed {
            extension_id,
            exit_code,
        } => {
            assert_eq!(extension_id, "acme.crasher");
            // On Windows / Unix the exit code is propagated
            if let Some(code) = exit_code {
                assert_eq!(code, 101);
            }
        }
        other => panic!("expected ProcessCrashed, got {other:?}"),
    }

    assert!(!ext.is_alive());
    match ext.status {
        ProcessStatus::Crashed(Some(code)) => assert_eq!(code, 101),
        ProcessStatus::Crashed(None) => {}
        other => panic!("expected Crashed status, got {other:?}"),
    }
}

#[test]
fn test_shell_extension_host_with_external_process() {
    let bin_path = get_mock_extension_binary();
    let manifest = ExtensionManifest {
        id: "acme.external-tools".to_string(),
        name: "External Tools".to_string(),
        version: "1.0.0".to_string(),
        publisher: "Acme".to_string(),
        description: "External tool host test".to_string(),
        entrypoint: Some(bin_path.to_string_lossy().to_string()),
        activation_events: vec!["onCommand:mock.action".to_string()],
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "mock.action".to_string(),
                label: "Mock Action".to_string(),
                description: None,
                shortcut: None,
                icon: None,
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
    };

    let mut host = ShellExtensionHost::new(None);
    host.register_manifest(manifest).expect("register manifest");

    // 1. Run a command with on-demand activation
    host.execute_command("mock.action")
        .expect("host executes external command");

    // 2. Messages from the child process are aggregated transparently in the host
    let msgs = host.take_messages();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].1, "Mock executed mock.action");

    // 3. Run with structured arguments
    let res = host
        .execute_command_with_args("mock.action", Some(serde_json::json!({ "flag": true })))
        .expect("exec with args")
        .expect("structured result");
    assert_eq!(res["status"], "ok");

    // 4. Graceful shutdown
    host.shutdown();
}

#[test]
fn test_bidirectional_ui_card_flow_with_external_process() {
    use crate::extensions::ExtensionCardManager;
    use crawrad_extension_api::ui::{UiElementKind, UiEventKind};
    use std::time::{Duration, Instant};

    let bin_path = get_mock_extension_binary();
    let manifest = ExtensionManifest {
        id: "acme.ui-extension".to_string(),
        name: "UI Extension".to_string(),
        version: "1.0.0".to_string(),
        publisher: "Acme".to_string(),
        description: "Interactive UI card test".to_string(),
        entrypoint: Some(bin_path.to_string_lossy().to_string()),
        activation_events: vec!["onCommand:mock.showCard".to_string()],
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "mock.showCard".to_string(),
                label: "Show Card".to_string(),
                description: None,
                shortcut: None,
                icon: None,
            }],
            menus: Vec::new(),
            cards: vec![CardContribution {
                id: "mock.panel".to_string(),
                title: "Mock Panel".to_string(),
                area: CardArea::Right,
                icon: None,
                closeable: true,
            }],
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
    };

    let mut host = ShellExtensionHost::new(None);
    host.register_manifest(manifest).expect("register manifest");

    // 1. Trigger command that asks subprocess to render card
    host.execute_command("mock.showCard")
        .expect("showCard command");

    // Wait for card to be rendered via background reader thread
    let start = Instant::now();
    let card_mgr = host.card_manager();
    let mut rendered = false;
    while start.elapsed() < Duration::from_secs(3) {
        if let Ok(mgr) = card_mgr.lock()
            && let Some(card) = mgr.get_card("mock.panel")
            && card.open
            && card.root.is_some()
        {
            rendered = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
    assert!(
        rendered,
        "Card should have been rendered by extension subprocess"
    );

    // Verify initial rendered DOM
    {
        let mgr = card_mgr.lock().unwrap();
        let card = mgr.get_card("mock.panel").unwrap();
        let root = card.root.as_ref().unwrap();
        let btn = ExtensionCardManager::find_node(root, "btn_count").expect("btn_count exists");
        match &btn.kind {
            UiElementKind::Button { text, .. } => assert_eq!(text, "Click me: 0"),
            _ => panic!("Expected Button"),
        }
    }

    // 2. Dispatch UI event from host IDE to extension subprocess (Click on btn_count)
    host.dispatch_ui_event("mock.panel", "btn_count", UiEventKind::Click)
        .expect("dispatch click event");

    // Wait for patch to be applied by subprocess
    let start = Instant::now();
    let mut patched = false;
    while start.elapsed() < Duration::from_secs(3) {
        if let Ok(mgr) = card_mgr.lock()
            && let Some(card) = mgr.get_card("mock.panel")
            && let Some(root) = &card.root
            && let Some(btn) = ExtensionCardManager::find_node(root, "btn_count")
            && let UiElementKind::Button { text, .. } = &btn.kind
            && text == "Clicked 1 time"
        {
            patched = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
    assert!(
        patched,
        "Button should have been patched with 'Clicked 1 time'"
    );

    // 3. Dispatch close event from host IDE to extension subprocess
    host.dispatch_ui_event("mock.panel", "btn_close", UiEventKind::Click)
        .expect("dispatch close event");

    let start = Instant::now();
    let mut closed = false;
    while start.elapsed() < Duration::from_secs(3) {
        if let Ok(mgr) = card_mgr.lock()
            && let Some(card) = mgr.get_card("mock.panel")
            && !card.open
        {
            closed = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
    assert!(closed, "Card should have been closed");

    // 4. Shutdown subprocess
    host.shutdown();
}

/// A subprocess fills its declared view over `view/setTree`, and a
/// click goes back to it as `view/itemClicked`.
#[test]
fn a_subprocess_fills_its_view_and_hears_the_click() {
    use crate::extensions::process::ProcessExtensionHost;
    use crawrad_extension_api::views::ViewContribution;

    let bin_path = get_mock_extension_binary();
    let dir = bin_path.parent().expect("bin dir").to_path_buf();
    let manifest = ExtensionManifest {
        id: "acme.views".to_string(),
        name: "Views".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Acme".to_string(),
        description: String::new(),
        entrypoint: bin_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned()),
        activation_events: vec!["*".to_string()],
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "mock.tree".to_string(),
                label: "Tree".to_string(),
                description: None,
                shortcut: None,
                icon: None,
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: vec![ViewContribution {
                id: "todos".to_string(),
                title: "TODOs".to_string(),
                icon: None,
            }],
        },
    };
    let mut host = ProcessExtensionHost::new(Some(dir), Arc::default());
    assert!(host.register_manifest(manifest).expect("registered"));
    let args = serde_json::json!({
        "view_id": "todos",
        "items": [{ "id": "a", "label": "a.rs", "children": [
            { "id": "1", "label": "TODO one", "kind": "todo",
              "location": { "file": "src/a.rs", "line": 2 } }
        ]}]
    });
    host.execute_command("mock.tree", Some(args))
        .expect("mock.tree ran");
    let views = host.views.lock().expect("views").views();
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].title, "TODOs");
    assert_eq!(views[0].items[0].children[0].label, "TODO one");

    let ext = host.extensions.get_mut("acme.views").expect("running");
    let _ = ext.take_messages();
    host.view_item_clicked("acme.views", "todos", "1")
        .expect("click sent");
    let ext = host.extensions.get_mut("acme.views").expect("running");
    let mut heard = Vec::new();
    for _ in 0..50 {
        heard.extend(ext.take_messages());
        if !heard.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert_eq!(
        heard.first().map(|m| m.1.as_str()),
        Some("Mock item clicked: 1")
    );
    let _ = ext.shutdown();
}

#[test]
fn the_shell_syncs_the_editor_and_designer_state_into_the_host() {
    let mut host = ShellExtensionHost::new(None);
    let p = PathBuf::from("src/form.rs");
    host.sync_editor_state(Some(p.clone()), "fn test() {}");
    assert_eq!(
        host.context
            .editor
            .as_ref()
            .and_then(|e| e.get_content("src/form.rs").ok()),
        Some("fn test() {}".to_string())
    );

    host.sync_designer_state(Some("MyForm".into()), vec!["btn1".into()]);
    let dsg = host.context.designer.as_ref().expect("a designer service");
    assert_eq!(dsg.active_form_name, Some("MyForm".to_string()));
    assert_eq!(dsg.selected, vec!["btn1".to_string()]);
}
