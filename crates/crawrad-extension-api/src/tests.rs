use crate::contribution::{CardArea, MenuPosition};
use crate::error::ExtensionError;
use crate::manifest::ExtensionManifest;
use crate::services::{
    ComponentInfo, ComponentRect, DesignerService, EditorSelection, EditorService, FormInfo,
    ProjectService, TextEdit, TextRange,
};
use crate::traits::{CommandHandler, ExtensionContext, IdeExtension, MessageLevel};
use std::collections::HashMap;
use std::sync::Mutex;

struct MockEditorService {
    active: Option<String>,
    content: String,
}

impl EditorService for MockEditorService {
    fn active_file(&self) -> Option<String> {
        self.active.clone()
    }

    fn open_file(
        &mut self,
        path: &str,
        _position: Option<(usize, usize)>,
    ) -> Result<(), ExtensionError> {
        self.active = Some(path.to_string());
        Ok(())
    }

    fn get_content(&self, _path: &str) -> Result<String, ExtensionError> {
        Ok(self.content.clone())
    }

    fn get_lines(
        &self,
        _path: &str,
        start_line: usize,
        end_line: usize,
    ) -> Result<Vec<String>, ExtensionError> {
        let lines: Vec<String> = self.content.lines().map(|s| s.to_string()).collect();
        if start_line == 0 || start_line > lines.len() {
            return Ok(Vec::new());
        }
        let end = end_line.min(lines.len());
        Ok(lines[start_line - 1..end].to_vec())
    }

    fn apply_edit(&mut self, _path: &str, edit: TextEdit) -> Result<(), ExtensionError> {
        self.content = edit.new_text;
        Ok(())
    }

    fn active_selection(&self) -> Option<EditorSelection> {
        Some(EditorSelection {
            range: TextRange::new(1, 1, 1, 5),
            selected_text: "hello".into(),
        })
    }
}

struct MockProjectService {
    forms: Vec<FormInfo>,
}

impl ProjectService for MockProjectService {
    fn active_project_path(&self) -> Option<String> {
        Some("/home/dev/project".into())
    }

    fn active_project_name(&self) -> Option<String> {
        Some("MeuApp".into())
    }

    fn list_project_files(&self) -> Result<Vec<String>, ExtensionError> {
        Ok(vec!["src/main.rs".into(), "src/main_form.rs".into()])
    }

    fn list_forms(&self) -> Result<Vec<FormInfo>, ExtensionError> {
        Ok(self.forms.clone())
    }

    fn add_file(&mut self, _path: &str) -> Result<(), ExtensionError> {
        Ok(())
    }

    fn request_build(&mut self) -> Result<(), ExtensionError> {
        Ok(())
    }
}

struct MockDesignerService {
    properties: HashMap<String, String>,
    selected: Vec<String>,
}

impl DesignerService for MockDesignerService {
    fn active_form_name(&self) -> Option<String> {
        Some("FormPrincipal".into())
    }

    fn list_components(&self) -> Result<Vec<ComponentInfo>, ExtensionError> {
        Ok(vec![
            ComponentInfo {
                name: "btnSalvar".into(),
                class_name: "Button".into(),
                parent_name: None,
                rect: ComponentRect::new(10.0, 10.0, 80.0, 25.0),
                is_visual: true,
            },
            ComponentInfo {
                name: "gridDados".into(),
                class_name: "Grid".into(),
                parent_name: None,
                rect: ComponentRect::new(10.0, 50.0, 400.0, 200.0),
                is_visual: true,
            },
        ])
    }

    fn get_property(
        &self,
        component_name: &str,
        property_key: &str,
    ) -> Result<Option<String>, ExtensionError> {
        let key = format!("{component_name}.{property_key}");
        Ok(self.properties.get(&key).cloned())
    }

    fn set_property(
        &mut self,
        component_name: &str,
        property_key: &str,
        property_value: &str,
    ) -> Result<(), ExtensionError> {
        let key = format!("{component_name}.{property_key}");
        self.properties.insert(key, property_value.to_string());
        Ok(())
    }

    fn selected_components(&self) -> Vec<String> {
        self.selected.clone()
    }

    fn select_components(&mut self, names: &[&str]) -> Result<(), ExtensionError> {
        self.selected = names.iter().map(|s| s.to_string()).collect();
        Ok(())
    }

    fn delete_selected(&mut self) -> Result<(), ExtensionError> {
        self.selected.clear();
        Ok(())
    }
}

struct MockExtensionContext {
    registered_commands: HashMap<String, CommandHandler>,
    messages: Mutex<Vec<(MessageLevel, String)>>,
    workspace: Option<String>,
    editor: MockEditorService,
    project: MockProjectService,
    designer: MockDesignerService,
}

impl MockExtensionContext {
    fn new() -> Self {
        Self {
            registered_commands: HashMap::new(),
            messages: Mutex::new(Vec::new()),
            workspace: Some("/home/dev/project".into()),
            editor: MockEditorService {
                active: Some("src/main.rs".into()),
                content: "fn main() {\n    println!(\"hello\");\n}\n".into(),
            },
            project: MockProjectService {
                forms: vec![FormInfo {
                    name: "FormPrincipal".into(),
                    form_file: "/home/dev/project/src/main.crawform".into(),
                    code_file: "/home/dev/project/src/main_form.rs".into(),
                }],
            },
            designer: MockDesignerService {
                properties: {
                    let mut m = HashMap::new();
                    m.insert("btnSalvar.text".into(), "Salvar Registro".into());
                    m
                },
                selected: vec!["btnSalvar".into()],
            },
        }
    }
}

impl ExtensionContext for MockExtensionContext {
    fn register_command(
        &mut self,
        action_id: &str,
        handler: CommandHandler,
    ) -> Result<(), ExtensionError> {
        self.registered_commands
            .insert(action_id.to_string(), handler);
        Ok(())
    }

    fn show_message(&self, level: MessageLevel, text: &str) {
        if let Ok(mut msgs) = self.messages.lock() {
            msgs.push((level, text.to_string()));
        }
    }

    fn active_workspace_path(&self) -> Option<String> {
        self.workspace.clone()
    }

    fn editor(&self) -> Option<&dyn EditorService> {
        Some(&self.editor)
    }

    fn editor_mut(&mut self) -> Option<&mut dyn EditorService> {
        Some(&mut self.editor)
    }

    fn project(&self) -> Option<&dyn ProjectService> {
        Some(&self.project)
    }

    fn project_mut(&mut self) -> Option<&mut dyn ProjectService> {
        Some(&mut self.project)
    }

    fn designer(&self) -> Option<&dyn DesignerService> {
        Some(&self.designer)
    }

    fn designer_mut(&mut self) -> Option<&mut dyn DesignerService> {
        Some(&mut self.designer)
    }
}

struct SamplePlugin {
    activated: bool,
}

impl SamplePlugin {
    fn new() -> Self {
        Self { activated: false }
    }
}

impl IdeExtension for SamplePlugin {
    fn id(&self) -> &str {
        "sample.plugin"
    }

    fn activate(&mut self, ctx: &mut dyn ExtensionContext) -> Result<(), ExtensionError> {
        self.activated = true;
        ctx.register_command("sample.plugin.hello", Box::new(|| {}))?;
        ctx.show_message(MessageLevel::Info, "Sample Plugin Activated");
        Ok(())
    }

    fn deactivate(&mut self) -> Result<(), ExtensionError> {
        self.activated = false;
        Ok(())
    }
}

#[test]
fn parse_valid_extension_manifest() {
    let json = r#"{
        "id": "vendor.git_tools",
        "name": "Git Enhanced Tools",
        "version": "1.0.0",
        "publisher": "Vendor Corp",
        "description": "Integrated Git diff and log views",
        "contributions": {
            "actions": [
                {
                    "id": "vendor.git_tools.status",
                    "label": "Git Status",
                    "shortcut": "Ctrl+Shift+G"
                }
            ],
            "menus": [
                {
                    "target": "Tools",
                    "action_id": "vendor.git_tools.status",
                    "position": "bottom"
                }
            ],
            "cards": [
                {
                    "id": "vendor.git_tools.history",
                    "title": "Git History",
                    "area": "bottom",
                    "closeable": true
                }
            ]
        }
    }"#;

    let manifest = ExtensionManifest::from_json_str(json).expect("valid manifest");
    assert_eq!(manifest.id, "vendor.git_tools");
    assert_eq!(manifest.contributions.actions.len(), 1);
    assert_eq!(
        manifest.contributions.actions[0].id,
        "vendor.git_tools.status"
    );
    assert_eq!(
        manifest.contributions.menus[0].position,
        MenuPosition::Bottom
    );
    assert_eq!(manifest.contributions.cards[0].area, CardArea::Bottom);
}

#[test]
fn manifest_validation_rejects_empty_id() {
    let json = r#"{
        "id": "",
        "name": "Invalid",
        "version": "1.0.0",
        "publisher": "Vendor"
    }"#;

    let err = ExtensionManifest::from_json_str(json).unwrap_err();
    assert!(matches!(err, ExtensionError::InvalidManifest(_)));
}

#[test]
fn extension_lifecycle_and_command_registration() {
    let mut plugin = SamplePlugin::new();
    let mut ctx = MockExtensionContext::new();

    assert!(!plugin.activated);
    assert_eq!(
        ctx.active_workspace_path(),
        Some("/home/dev/project".into())
    );

    plugin.activate(&mut ctx).expect("activation succeeds");
    assert!(plugin.activated);
    assert!(ctx.registered_commands.contains_key("sample.plugin.hello"));
    assert_eq!(ctx.messages.lock().unwrap().len(), 1);

    plugin.deactivate().expect("deactivation succeeds");
    assert!(!plugin.activated);
}

#[test]
fn editor_service_interaction() {
    let mut ctx = MockExtensionContext::new();
    let editor = ctx.editor_mut().expect("editor service available");

    assert_eq!(editor.active_file(), Some("src/main.rs".into()));
    let lines = editor.get_lines("src/main.rs", 1, 2).unwrap();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "fn main() {");

    let edit = TextEdit::new(TextRange::new(1, 1, 3, 1), "// replaced content");
    editor.apply_edit("src/main.rs", edit).unwrap();
    assert_eq!(
        editor.get_content("src/main.rs").unwrap(),
        "// replaced content"
    );
}

#[test]
fn project_service_interaction() {
    let ctx = MockExtensionContext::new();
    let project = ctx.project().expect("project service available");

    assert_eq!(project.active_project_name(), Some("MeuApp".into()));
    let forms = project.list_forms().unwrap();
    assert_eq!(forms.len(), 1);
    assert_eq!(forms[0].name, "FormPrincipal");
    assert_eq!(forms[0].code_file, "/home/dev/project/src/main_form.rs");
}

#[test]
fn designer_service_interaction() {
    let mut ctx = MockExtensionContext::new();
    let designer = ctx.designer_mut().expect("designer service available");

    assert_eq!(designer.active_form_name(), Some("FormPrincipal".into()));
    let components = designer.list_components().unwrap();
    assert_eq!(components.len(), 2);
    assert_eq!(components[0].name, "btnSalvar");
    assert_eq!(components[1].class_name, "Grid");

    let val = designer.get_property("btnSalvar", "text").unwrap();
    assert_eq!(val, Some("Salvar Registro".into()));

    designer
        .set_property("btnSalvar", "text", "Confirmar")
        .unwrap();
    assert_eq!(
        designer.get_property("btnSalvar", "text").unwrap(),
        Some("Confirmar".into())
    );

    assert_eq!(designer.selected_components(), vec!["btnSalvar"]);
    designer.select_components(&["gridDados"]).unwrap();
    assert_eq!(designer.selected_components(), vec!["gridDados"]);
}

#[test]
fn test_external_manifest_properties() {
    let json = r#"{
        "id": "acme.git-tools",
        "name": "Git Tools",
        "version": "1.0.0",
        "publisher": "Acme",
        "entrypoint": "bin/git-runner.exe",
        "activation_events": ["onCommand:git.status", "onLanguage:rust"],
        "contributions": {}
    }"#;

    let manifest = ExtensionManifest::from_json_str(json).expect("valid manifest");
    assert!(manifest.is_external());
    assert_eq!(manifest.entrypoint.as_deref(), Some("bin/git-runner.exe"));
    assert!(manifest.should_activate_on("onCommand:git.status"));
    assert!(manifest.should_activate_on("onLanguage:rust"));
    assert!(!manifest.should_activate_on("onCommand:other"));

    // Manifest without entrypoint is in-process
    let inproc_json = r#"{
        "id": "acme.inproc",
        "name": "Inproc Plugin",
        "version": "0.1.0",
        "publisher": "Acme"
    }"#;
    let inproc = ExtensionManifest::from_json_str(inproc_json).expect("valid inproc");
    assert!(!inproc.is_external());
}

#[test]
fn test_jsonrpc_protocol_parsing_and_frames() {
    use crate::protocol::{
        IncomingMessage, JsonRpcRequest, JsonRpcResponse, METHOD_EXECUTE_COMMAND, parse_message,
    };

    // Request parsing
    let req_str = r#"{"jsonrpc":"2.0","id":42,"method":"extension/executeCommand","params":{"command":"test.cmd"}}"#;
    match parse_message(req_str).expect("parse request") {
        IncomingMessage::Request(req) => {
            assert_eq!(req.id, Some(42));
            assert_eq!(req.method, METHOD_EXECUTE_COMMAND);
            assert_eq!(req.params.unwrap()["command"], "test.cmd");
        }
        IncomingMessage::Response(_) => panic!("expected request"),
    }

    // Response parsing
    let resp_str = r#"{"jsonrpc":"2.0","id":42,"result":{"success":true}}"#;
    match parse_message(resp_str).expect("parse response") {
        IncomingMessage::Response(resp) => {
            assert_eq!(resp.id, Some(42));
            assert!(resp.is_success());
            assert_eq!(resp.result.unwrap()["success"], true);
        }
        IncomingMessage::Request(_) => panic!("expected response"),
    }

    // Helper constructors
    let notif = JsonRpcRequest::new_notification("window/showMessage", None);
    assert!(notif.is_notification());
    assert_eq!(notif.id, None);

    let err_resp = JsonRpcResponse::error(Some(10), -32600, "Invalid Request");
    assert!(!err_resp.is_success());
    assert_eq!(err_resp.error.unwrap().code, -32600);
}
