use crate::capability::*;
use crate::{Args, Context, Effect, ParamKind, Project, ViewAffinity};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Author;
use crawrad_extension_api::contribution::{ActionContribution, ToolContribution};
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_registry::Registry;

fn test_context<'a>(
    reg: &'a Registry,
    clipboard: &'a Clipboard,
    args: Args,
    author: Author,
) -> Context<'a> {
    Context {
        doc: None,
        registry: reg,
        clipboard,
        args,
        project: Project::default(),
        author,
        editor_dirty: false,
        text: Default::default(),
    }
}

#[test]
fn the_registry_initializes_with_all_native_actions() {
    let reg = CapabilityRegistry::with_builtin_actions().expect("loads native actions");
    assert!(
        reg.len() >= 30,
        "must contain all actions of the native catalogue, got: {}",
        reg.len()
    );
    assert!(reg.contains("file.save"));
    assert!(reg.contains("edit.copy"));
    assert!(reg.contains("project.run"));
    assert!(reg.contains("align.left"));

    let cap_save = reg.get("file.save").expect("file.save present");
    assert_eq!(cap_save.source, CapabilitySource::Native);
    assert_eq!(cap_save.group, "File");
}

#[test]
fn canonical_id_validation_rejects_invalid_formats() {
    assert!(CapabilityRegistry::validate_id("file.save").is_ok());
    assert!(CapabilityRegistry::validate_id("git.status").is_ok());
    assert!(CapabilityRegistry::validate_id("ext.command_one").is_ok());
    assert!(CapabilityRegistry::validate_id("my-ext.command-two").is_ok());

    assert!(matches!(
        CapabilityRegistry::validate_id("without_dot"),
        Err(CapabilityError::InvalidId(_))
    ));
    assert!(matches!(
        CapabilityRegistry::validate_id(".beginning"),
        Err(CapabilityError::InvalidId(_))
    ));
    assert!(matches!(
        CapabilityRegistry::validate_id("end."),
        Err(CapabilityError::InvalidId(_))
    ));
    assert!(matches!(
        CapabilityRegistry::validate_id(""),
        Err(CapabilityError::InvalidId(_))
    ));
    assert!(matches!(
        CapabilityRegistry::validate_id("id with space.cmd"),
        Err(CapabilityError::InvalidId(_))
    ));
}

#[test]
fn rejects_id_collision_strictly() {
    let mut reg = CapabilityRegistry::new();
    let cap1 = Capability::custom(
        "test.sample",
        "Sample 1",
        "First sample",
        "Test",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    );
    let cap2 = Capability::custom(
        "test.sample",
        "Sample 2",
        "Second sample",
        "Test",
        CapabilitySource::Extension {
            extension_id: "ext.sample".to_string(),
        },
        |_ctx: &Context<'_>| Ok(Effect::none()),
    );

    assert!(reg.register(cap1).is_ok());
    let err = reg.register(cap2).expect_err("must refuse ID collision");

    match err {
        CapabilityError::IdCollision {
            id,
            existing,
            attempted,
        } => {
            assert_eq!(id, "test.sample");
            assert_eq!(existing, CapabilitySource::Native);
            assert_eq!(
                attempted,
                CapabilitySource::Extension {
                    extension_id: "ext.sample".to_string()
                }
            );
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn explicit_substitution_with_replace_works() {
    let mut reg = CapabilityRegistry::new();
    let cap1 = Capability::custom(
        "test.replace",
        "V1",
        "Version 1",
        "Test",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    );
    let cap2 = Capability::custom(
        "test.replace",
        "V2",
        "Version 2",
        "Test",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    );

    assert!(reg.register(cap1).is_ok());
    let old = reg.replace(cap2).expect("replace must succeed");
    assert!(old.is_some());
    assert_eq!(old.unwrap().label, "V1");

    let current = reg.get("test.replace").unwrap();
    assert_eq!(current.label, "V2");
}

#[test]
fn dynamic_registration_of_extension_manifest() {
    let mut reg = CapabilityRegistry::new();

    let manifest = ExtensionManifest {
        id: "crawrad.git-tools".to_string(),
        name: "Git Tools".to_string(),
        version: "1.0.0".to_string(),
        publisher: "CrawRAD Team".to_string(),
        description: "Integrated version control".to_string(),
        contributions: ExtensionContributions {
            actions: vec![
                ActionContribution {
                    id: "status".to_string(),
                    label: "Git Status".to_string(),
                    description: Some("Shows file status".to_string()),
                    shortcut: Some("Ctrl+Shift+G".to_string()),
                    icon: None,
                },
                ActionContribution {
                    id: "git.commit".to_string(),
                    label: "Git Commit".to_string(),
                    description: Some("Commits changes".to_string()),
                    shortcut: None,
                    icon: None,
                },
            ],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: vec![ToolContribution {
                name: "git_diff".to_string(),
                description: "Returns repository diff".to_string(),
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    }
                }),
            }],
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    let count = reg
        .register_extension_manifest(&manifest)
        .expect("registers manifest");
    assert_eq!(count, 3);

    assert!(reg.contains("crawrad.git-tools.status"));
    assert!(reg.contains("git.commit"));
    assert!(reg.contains("crawrad.git-tools.tool.git_diff"));

    let cap_status = reg.get("crawrad.git-tools.status").unwrap();
    assert_eq!(cap_status.label, "Git Status");
    assert_eq!(cap_status.group, "Git Tools");
    assert_eq!(cap_status.shortcut.as_deref(), Some("Ctrl+Shift+G"));

    let ext_source = CapabilitySource::Extension {
        extension_id: "crawrad.git-tools".to_string(),
    };
    assert_eq!(reg.list_by_source(&ext_source).len(), 3);
}

#[test]
fn unregistering_extension_removes_its_capabilities() {
    let mut reg = CapabilityRegistry::with_builtin_actions().unwrap();
    let initial_count = reg.len();

    let manifest = ExtensionManifest {
        id: "temp.ext".to_string(),
        name: "Temporary".to_string(),
        version: "0.1.0".to_string(),
        publisher: "Dev".to_string(),
        description: "".to_string(),
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "do_something".to_string(),
                label: "Do Something".to_string(),
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
        entrypoint: None,
        activation_events: Vec::new(),
    };

    reg.register_extension_manifest(&manifest).unwrap();
    assert_eq!(reg.len(), initial_count + 1);
    assert!(reg.contains("temp.ext.do_something"));

    let removed = reg.unregister_extension("temp.ext");
    assert_eq!(removed, vec!["temp.ext.do_something".to_string()]);
    assert_eq!(reg.len(), initial_count);
    assert!(!reg.contains("temp.ext.do_something"));
}

#[test]
fn generation_of_input_schema_and_all_schemas_for_mcp() {
    let mut reg = CapabilityRegistry::new();

    let cap = Capability::custom(
        "custom.format",
        "Format Document",
        "Formats source code",
        "Editor",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    )
    .with_params(vec![
        CapabilityParam::new("file", ParamKind::String, "File path").with_required(true),
        CapabilityParam::new("indent", ParamKind::Integer, "Indentation size").with_required(false),
    ]);

    reg.register(cap).unwrap();

    let schema = reg.input_schema("custom.format").expect("schema available");
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["properties"]["file"]["type"], "string");
    assert_eq!(schema["properties"]["indent"]["type"], "integer");
    assert_eq!(schema["required"], serde_json::json!(["file"]));

    let all = reg.all_schemas();
    assert!(all.contains_key("custom.format"));
}

#[test]
fn transactional_dispatch_with_author_and_audit() {
    let mut reg = CapabilityRegistry::new();

    let cap = Capability::custom(
        "editor.test_action",
        "Test Action",
        "Executes test action",
        "Test",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none().with_status("Action executed")),
    );
    reg.register(cap).unwrap();

    let r_registry = Registry::builtin();
    let r_clipboard = Clipboard::default();

    // 1. Dispatch by User
    let ctx_user = test_context(&r_registry, &r_clipboard, Args::none(), Author::User);
    let effect_user = reg.dispatch("editor.test_action", &ctx_user, None).unwrap();
    assert_eq!(effect_user.status.as_deref(), Some("Action executed"));

    // 2. Dispatch by Agent
    let ctx_agent = test_context(
        &r_registry,
        &r_clipboard,
        Args::none(),
        Author::Agent("claude".to_string()),
    );
    let effect_agent = reg
        .dispatch("editor.test_action", &ctx_agent, None)
        .unwrap();
    assert_eq!(effect_agent.status.as_deref(), Some("Action executed"));

    // 3. Verification of audit log
    let log = reg.audit_log();
    assert_eq!(log.len(), 2);

    assert_eq!(log[0].capability_id, "editor.test_action");
    assert_eq!(log[0].author, Author::User);
    assert!(log[0].success);
    assert_eq!(log[0].effect_summary, "Action executed");
    assert!(log[0].timestamp_ms > 0);

    assert_eq!(log[1].capability_id, "editor.test_action");
    assert_eq!(log[1].author, Author::Agent("claude".to_string()));
    assert!(log[1].success);

    reg.clear_audit_log();
    assert_eq!(reg.audit_log().len(), 0);
}

#[test]
fn validation_of_view_affinity() {
    let mut reg = CapabilityRegistry::new();

    let cap = Capability::custom(
        "designer.align",
        "Align Left",
        "Aligns components to the left",
        "Design",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    )
    .with_view_affinity(ViewAffinity::Design);

    reg.register(cap).unwrap();

    let r_registry = Registry::builtin();
    let r_clipboard = Clipboard::default();
    let ctx = test_context(&r_registry, &r_clipboard, Args::none(), Author::User);

    // Attempt in Code view: must refuse with ViewMismatch
    let err = reg
        .dispatch("designer.align", &ctx, Some(ViewAffinity::Code))
        .expect_err("must reject outside Design view");

    assert!(matches!(
        err,
        CapabilityError::ViewMismatch {
            required: ViewAffinity::Design,
            active: ViewAffinity::Code,
            ..
        }
    ));

    // In correct view (Design): must succeed
    let ok = reg.dispatch("designer.align", &ctx, Some(ViewAffinity::Design));
    assert!(ok.is_ok());
}

#[test]
fn validation_of_required_parameters() {
    let mut reg = CapabilityRegistry::new();

    let cap = Capability::custom(
        "node.delete",
        "Delete Node",
        "Deletes node from form",
        "Edit",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    )
    .with_params(vec![
        CapabilityParam::new("node_id", ParamKind::ComponentId, "Component ID").with_required(true),
    ]);

    reg.register(cap).unwrap();

    let r_registry = Registry::builtin();
    let r_clipboard = Clipboard::default();

    // Without the required parameter
    let ctx_without_param = test_context(&r_registry, &r_clipboard, Args::none(), Author::User);
    let err = reg
        .dispatch("node.delete", &ctx_without_param, None)
        .expect_err("must fail due to missing parameter");

    assert!(matches!(
        err,
        CapabilityError::MissingParameter {
            param,
            ..
        } if param == "node_id"
    ));

    // With the required parameter present
    let ctx_with_param = test_context(
        &r_registry,
        &r_clipboard,
        Args::text("node_id", "btn_ok"),
        Author::User,
    );
    let ok = reg.dispatch("node.delete", &ctx_with_param, None);
    assert!(ok.is_ok());
}

#[test]
fn search_in_command_palette() {
    let mut reg = CapabilityRegistry::new();

    reg.register(Capability::custom(
        "file.save",
        "Save File",
        "Saves current file",
        "File",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    ))
    .unwrap();

    reg.register(Capability::custom(
        "file.save_as",
        "Save File As...",
        "Saves under another name",
        "File",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    ))
    .unwrap();

    reg.register(Capability::custom(
        "edit.copy",
        "Copy",
        "Copies to clipboard",
        "Edit",
        CapabilitySource::Native,
        |_ctx: &Context<'_>| Ok(Effect::none()),
    ))
    .unwrap();

    let save_results = reg.search("save");
    assert_eq!(save_results.len(), 2);
    assert_eq!(save_results[0].id, "file.save");

    let copy_results = reg.search("copy");
    assert_eq!(copy_results.len(), 1);
    assert_eq!(copy_results[0].id, "edit.copy");

    let all_results = reg.search("");
    assert_eq!(all_results.len(), 3);
}

#[test]
fn search_commands_over_the_catalog() {
    let registry = super::CapabilityRegistry::default();
    assert!(
        !registry.is_empty(),
        "the default registry holds the built-in actions"
    );

    // ">" alone lists the whole catalog
    let all = search_commands(&registry, ">");
    assert_eq!(all.len(), registry.len());

    // a term
    let search_save = search_commands(&registry, "> save");
    assert!(!search_save.is_empty());
    assert_eq!(search_save[0].id, "file.save");
    assert_eq!(search_save[0].shortcut.as_deref(), Some("Ctrl+S"));

    // no match
    let empty = search_commands(&registry, "> comando_inexistente_xyz_123");
    assert!(empty.is_empty());
}

#[test]
fn search_commands_includes_extension_capabilities() {
    use crawrad_extension_api::contribution::ActionContribution;
    use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};

    let mut registry = super::CapabilityRegistry::default();
    let manifest = ExtensionManifest {
        id: "vendor.git".into(),
        name: "Git Integration".into(),
        version: "1.0.0".into(),
        publisher: "Vendor".into(),
        description: "Git tools".into(),
        contributions: ExtensionContributions {
            actions: vec![ActionContribution {
                id: "git.commit".into(),
                label: "Git: Commit Changes".into(),
                description: Some("Commit workspace changes".into()),
                shortcut: Some("Ctrl+Shift+G".into()),
                icon: None,
            }],
            menus: Vec::new(),
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    registry
        .register_extension_manifest(&manifest)
        .expect("the extension manifest registers");

    let results = search_commands(&registry, "> commit");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "git.commit");
    assert_eq!(results[0].label, "Git: Commit Changes");
    assert_eq!(results[0].shortcut.as_deref(), Some("Ctrl+Shift+G"));
}
