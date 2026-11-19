//! Integration tests for dataset-aware standard actions code generation (ADR 0041).
//!
//! Verifies that standard actions configured with dataset operations (First, Prior, Next,
//! Last, Insert, Edit, Delete, Post, Cancel, Refresh) generate:
//! - Click dispatch per operation on the adapter
//! - Correct enabled expressions decided by dataset state
//! - Single synchronous dataset open across all actions
//! - Replacement of default operation when an `on_execute` handler is assigned
//! - Suppression of dataset-decided enabled when an `on_update` handler is assigned
//! - Preservation of existing code emission when the dataset reference is dangling

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::{Gerador, GeradorMakepad, GeradorRxp};

fn catalog() -> Registry {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let manifest: Manifest = serde_json::from_str(text).expect("valid manifest");
    let mut r = Registry::builtin();
    for c in manifest.to_components().expect("the manifest converts") {
        r.register_external(c).expect("the catalog takes it");
    }
    r
}

const OPERATIONS: &[(&str, &str, &str)] = &[
    ("act_first", "btn_first", "First"),
    ("act_prior", "btn_prior", "Prior"),
    ("act_next", "btn_next", "Next"),
    ("act_last", "btn_last", "Last"),
    ("act_insert", "btn_insert", "Insert"),
    ("act_edit", "btn_edit", "Edit"),
    ("act_delete", "btn_delete", "Delete"),
    ("act_post", "btn_post", "Post"),
    ("act_cancel", "btn_cancel", "Cancel"),
    ("act_refresh", "btn_refresh", "Refresh"),
];

fn dataset_actions_form(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
    }

    let mut engine = r
        .instantiate(
            "Connection",
            "conn1".to_string(),
            Rect::new(8.0, 8.0, 28.0, 28.0),
        )
        .expect("Connection instantiates");
    engine.set_name("conn1");
    engine
        .props
        .insert("url".to_string(), Value::Str("sqlite://shop.db".into()));
    d.add_child("form_main", engine).expect("add engine");

    let mut dataset = r
        .instantiate(
            "Dataset",
            "dst1".to_string(),
            Rect::new(40.0, 8.0, 28.0, 28.0),
        )
        .expect("Dataset instantiates");
    dataset.set_name("dst1");
    dataset
        .props
        .insert("connection".to_string(), Value::Ref("conn1".into()));
    dataset
        .props
        .insert("table".to_string(), Value::Str("clients".into()));
    d.add_child("form_main", dataset).expect("add dataset");

    let mut acl = r
        .instantiate(
            "ActionList",
            "acl1".to_string(),
            Rect::new(80.0, 8.0, 28.0, 28.0),
        )
        .expect("ActionList instantiates");
    acl.set_name("acl1");
    d.add_child("form_main", acl).expect("add ActionList");

    let mut y = 50.0;
    for (act_name, btn_name, op) in OPERATIONS {
        let mut act = r
            .instantiate(
                "Action",
                (*act_name).to_string(),
                Rect::new(0.0, 0.0, 28.0, 28.0),
            )
            .expect("Action instantiates");
        act.set_name(*act_name);
        act.props
            .insert("dataset".to_string(), Value::Ref("dst1".into()));
        act.props
            .insert("dataset_operation".to_string(), Value::Enum((*op).into()));
        d.add_child("acl1", act).expect("add action");

        let mut btn = r
            .instantiate(
                "Button",
                (*btn_name).to_string(),
                Rect::new(10.0, y, 100.0, 28.0),
            )
            .expect("Button instantiates");
        btn.set_name(*btn_name);
        btn.props
            .insert("action".to_string(), Value::Ref((*act_name).into()));
        d.add_child("form_main", btn).expect("add button");
        y += 32.0;
    }

    // Custom post action with OnExecute handler:
    let mut custom_post = r
        .instantiate(
            "Action",
            "act_custom_post".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("Action instantiates");
    custom_post.set_name("act_custom_post");
    custom_post
        .props
        .insert("dataset".to_string(), Value::Ref("dst1".into()));
    custom_post
        .props
        .insert("dataset_operation".to_string(), Value::Enum("Post".into()));
    custom_post
        .events
        .insert("on_execute".to_string(), "custom_post_handler".to_string());
    d.add_child("acl1", custom_post).expect("add custom_post");

    let mut btn_custom_post = r
        .instantiate(
            "Button",
            "btn_custom_post".to_string(),
            Rect::new(120.0, 50.0, 100.0, 28.0),
        )
        .expect("Button instantiates");
    btn_custom_post.set_name("btn_custom_post");
    btn_custom_post
        .props
        .insert("action".to_string(), Value::Ref("act_custom_post".into()));
    d.add_child("form_main", btn_custom_post)
        .expect("add btn_custom_post");

    // Action with OnUpdate handler:
    let mut custom_update = r
        .instantiate(
            "Action",
            "act_custom_update".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("Action instantiates");
    custom_update.set_name("act_custom_update");
    custom_update
        .props
        .insert("dataset".to_string(), Value::Ref("dst1".into()));
    custom_update
        .props
        .insert("dataset_operation".to_string(), Value::Enum("Post".into()));
    custom_update
        .events
        .insert("on_update".to_string(), "custom_update_handler".to_string());
    d.add_child("acl1", custom_update)
        .expect("add custom_update");

    let mut btn_custom_update = r
        .instantiate(
            "Button",
            "btn_custom_update".to_string(),
            Rect::new(120.0, 85.0, 100.0, 28.0),
        )
        .expect("Button instantiates");
    btn_custom_update.set_name("btn_custom_update");
    btn_custom_update
        .props
        .insert("action".to_string(), Value::Ref("act_custom_update".into()));
    d.add_child("form_main", btn_custom_update)
        .expect("add btn_custom_update");

    // An action that names the dataset but runs no operation is an ordinary action:
    let mut plain = r
        .instantiate(
            "Action",
            "act_plain".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("Action instantiates");
    plain.set_name("act_plain");
    plain
        .props
        .insert("dataset".to_string(), Value::Ref("dst1".into()));
    plain
        .props
        .insert("dataset_operation".to_string(), Value::Enum("None".into()));
    d.add_child("acl1", plain).expect("add plain");

    // Dangling action (pointing to nonexistent dataset):
    let mut dangling = r
        .instantiate(
            "Action",
            "act_dangling".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("Action instantiates");
    dangling.set_name("act_dangling");
    dangling
        .props
        .insert("dataset".to_string(), Value::Ref("missing_ds".into()));
    dangling
        .props
        .insert("dataset_operation".to_string(), Value::Enum("Post".into()));
    d.add_child("acl1", dangling).expect("add dangling");

    let mut btn_dangling = r
        .instantiate(
            "Button",
            "btn_dangling".to_string(),
            Rect::new(120.0, 120.0, 100.0, 28.0),
        )
        .expect("Button instantiates");
    btn_dangling.set_name("btn_dangling");
    btn_dangling
        .props
        .insert("action".to_string(), Value::Ref("act_dangling".into()));
    d.add_child("form_main", btn_dangling)
        .expect("add btn_dangling");

    d
}

fn generate_designer(g: &dyn Gerador, d: &Document, r: &Registry) -> String {
    g.gerar(d, "app", r)
        .expect("target generates")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("designer file")
        .conteudo
}

/// The adapter call each operation's click makes.
fn operation_call(op: &str) -> &'static str {
    match op {
        "First" => "adapter.collection().first()",
        "Prior" => "adapter.collection().previous()",
        "Next" => "adapter.collection().next()",
        "Last" => "adapter.collection().last()",
        "Insert" => "adapter.insert()",
        "Edit" => "adapter.edit()",
        "Delete" => "adapter.delete()",
        "Post" => "adapter.post()",
        "Cancel" => "adapter.cancel()",
        "Refresh" => "adapter.refresh()",
        other => panic!("unknown operation {other}"),
    }
}

/// The Makepad click block of `btn` running `act`, up to the next statement at its level.
fn makepad_click_of<'a>(code: &'a str, act: &str, btn: &str) -> &'a str {
    let head =
        format!("if self.form.{act}.enabled && self.ui.button(id!({btn})).clicked(actions) {{\n");
    let start = code
        .find(&head)
        .unwrap_or_else(|| panic!("no click of `{btn}`:\n{code}"));
    let rest = &code[start..];
    &rest[..rest.find("\n        }\n").map_or(rest.len(), |end| end)]
}

/// The egui click block of `btn` running `act`: from its enabled test to the next component.
fn egui_click_of<'a>(code: &'a str, act: &str, btn: &str) -> &'a str {
    let head = format!("let is_enabled = self.{btn}.enabled && self.{act}.enabled;");
    let start = code
        .find(&head)
        .unwrap_or_else(|| panic!("no click of `{btn}`:\n{code}"));
    let rest = &code[start..];
    &rest[..rest.find("        // ").map_or(rest.len(), |end| end)]
}

#[test]
fn makepad_dataset_actions_dispatch_and_enabled_and_single_open() {
    let r = catalog();
    let d = dataset_actions_form(&r);
    let code = generate_designer(&GeradorMakepad, &d, &r);

    // Single open of the dataset table:
    let open_call = "crawrad_binding::PortaDbBridge::open_table(&crawrad_runtime::db_url::resolve_sqlite_url(&self.form.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.form.dst1.table)";
    assert_eq!(
        code.matches(open_call).count(),
        1,
        "Makepad must open the dataset exactly once:\n{code}"
    );

    // Adapter field on Makepad App:
    assert!(
        code.contains("pub dst1_adapter: Option<DatasetAdapter>,"),
        "missing adapter in Makepad App:\n{code}"
    );

    // Click dispatch per operation, on the button linked to the action:
    for (act, btn, op) in OPERATIONS {
        let call = format!(
            "let _ = self.dst1_adapter.as_ref().map(|adapter| {});",
            operation_call(op)
        );
        let click = makepad_click_of(&code, act, btn);
        assert!(
            click.contains(&call),
            "`{btn}` must run {op} on the adapter:\n{click}"
        );
    }

    // Enabled expressions:
    assert!(
        code.contains("self.form.act_post.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().can_modify());"),
        "act_post must use can_modify():\n{code}"
    );
    assert!(
        code.contains("self.form.act_cancel.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().can_modify());"),
        "act_cancel must use can_modify():\n{code}"
    );
    assert!(
        code.contains("self.form.act_refresh.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().is_open());"),
        "act_refresh must use is_open():\n{code}"
    );
    assert!(
        code.contains("self.form.act_first.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().is_open() && adapter.collection().cursor().is_some() && adapter.collection().cursor() != Some(0));"),
        "act_first enabled condition:\n{code}"
    );

    // Handler REPLACES default operation:
    let click = makepad_click_of(&code, "act_custom_post", "btn_custom_post");
    assert!(
        click.contains("self.form.custom_post_handler();") && !click.contains("adapter.post()"),
        "custom post handler must replace the default post operation:\n{click}"
    );

    // OnUpdate suppresses dataset-decided enabled:
    assert!(
        !code.contains("self.form.act_custom_update.enabled = self.dst1_adapter"),
        "OnUpdate handler must suppress dataset-decided enabled:\n{code}"
    );
    assert!(
        code.contains("self.form.custom_update_handler();"),
        "OnUpdate handler must be called in pre_sync:\n{code}"
    );

    assert!(
        !code.contains("self.form.act_plain.enabled ="),
        "an action without an operation keeps its own enabled:\n{code}"
    );

    // Dangling dataset emits today's code (no adapter op, no enabled assignment):
    assert!(
        !code.contains("self.form.act_dangling.enabled = self.missing_ds_adapter"),
        "dangling dataset must not emit enabled assignment:\n{code}"
    );
    assert!(
        !code.contains("self.missing_ds_adapter"),
        "dangling dataset must not emit adapter field:\n{code}"
    );
}

#[test]
fn egui_dataset_actions_dispatch_and_enabled_and_single_open() {
    let r = catalog();
    let d = dataset_actions_form(&r);
    let code = generate_designer(&GeradorRxp, &d, &r);

    // Single open of the dataset table:
    let open_call = "crawrad_binding::PortaDbBridge::open_table(&crawrad_runtime::db_url::resolve_sqlite_url(&self.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.dst1.table)";
    assert_eq!(
        code.matches(open_call).count(),
        1,
        "egui must open the dataset exactly once:\n{code}"
    );

    // Adapter field on egui form:
    assert!(
        code.contains("pub dst1_adapter: Option<crawrad_binding::DatasetAdapter>,"),
        "missing adapter in egui form:\n{code}"
    );

    // Click dispatch per operation, on the button linked to the action:
    for (act, btn, op) in OPERATIONS {
        let call = format!(
            "let _ = self.dst1_adapter.as_ref().map(|adapter| {});",
            operation_call(op)
        );
        let click = egui_click_of(&code, act, btn);
        assert!(
            click.contains(&call),
            "`{btn}` must run {op} on the adapter:\n{click}"
        );
    }

    // Enabled expressions:
    assert!(
        code.contains("self.act_post.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().can_modify());"),
        "act_post must use can_modify():\n{code}"
    );
    assert!(
        code.contains("self.act_cancel.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().can_modify());"),
        "act_cancel must use can_modify():\n{code}"
    );
    assert!(
        code.contains("self.act_refresh.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().is_open());"),
        "act_refresh must use is_open():\n{code}"
    );
    assert!(
        code.contains("self.act_first.enabled = self.dst1_adapter.as_ref().is_some_and(|adapter| adapter.state().get().is_open() && adapter.collection().cursor().is_some() && adapter.collection().cursor() != Some(0));"),
        "act_first enabled condition:\n{code}"
    );

    // Handler REPLACES default operation:
    let click = egui_click_of(&code, "act_custom_post", "btn_custom_post");
    assert!(
        click.contains("self.custom_post_handler();") && !click.contains("adapter.post()"),
        "custom post handler must replace the default post operation:\n{click}"
    );

    // OnUpdate suppresses dataset-decided enabled:
    assert!(
        !code.contains("self.act_custom_update.enabled = self.dst1_adapter"),
        "OnUpdate handler must suppress dataset-decided enabled:\n{code}"
    );
    assert!(
        code.contains("self.custom_update_handler();"),
        "OnUpdate handler must be called in Action emit:\n{code}"
    );

    assert!(
        !code.contains("self.act_plain.enabled ="),
        "an action without an operation keeps its own enabled:\n{code}"
    );

    // Dangling dataset emits today's code:
    assert!(
        !code.contains("self.act_dangling.enabled = self.missing_ds_adapter"),
        "dangling dataset must not emit enabled assignment:\n{code}"
    );
    assert!(
        !code.contains("self.missing_ds_adapter"),
        "dangling dataset must not emit adapter field:\n{code}"
    );
}
