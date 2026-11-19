//! Integration tests for scalar data binding code generation (ADR 0041).
//!
//! Verifies that declarative bindings on `Edit.text` and `CheckBox.checked`
//! targeting a `Dataset` component generate:
//! - An adapter field on the form/app struct
//! - A single synchronous table open on first sync
//! - Two-way and one-way synchronization loops without feedback oscillation
//! - Clean elimination when sources are missing or invalid

use crawrad_ir::binding::{BindingDef, BindingMode};
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::{Gerador, GeradorMakepad, GeradorRxp};

/// Builds the component registry containing builtins and the `portadb-crawrad` package.
fn catalog() -> Registry {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let manifest: Manifest = serde_json::from_str(text).expect("valid manifest");
    let mut r = Registry::builtin();
    for c in manifest.to_components().expect("the manifest converts") {
        r.register_external(c).expect("the catalog takes it");
    }
    r
}

/// A form with `Connection`, `Dataset`, `Edit` (TwoWay to `name`), and `CheckBox` (OneWay to `active`).
fn bound_form(r: &Registry) -> Document {
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

    let mut edit = r
        .instantiate(
            "Edit",
            "edit_name".to_string(),
            Rect::new(10.0, 50.0, 150.0, 24.0),
        )
        .expect("Edit instantiates");
    edit.set_name("edit_name");
    edit.set_binding(
        "text",
        BindingDef::new("dst1", BindingMode::TwoWay).with_field("name"),
    );
    d.add_child("form_main", edit).expect("add Edit");

    let mut chk = r
        .instantiate(
            "CheckBox",
            "chk_active".to_string(),
            Rect::new(10.0, 90.0, 100.0, 24.0),
        )
        .expect("CheckBox instantiates");
    chk.set_name("chk_active");
    chk.set_binding(
        "checked",
        BindingDef::new("dst1", BindingMode::OneWay).with_field("active"),
    );
    d.add_child("form_main", chk).expect("add CheckBox");

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

#[test]
fn makepad_generates_adapter_and_sync_and_writeback() {
    let r = catalog();
    let d = bound_form(&r);
    let code = generate_designer(&GeradorMakepad, &d, &r);

    // 1. Adapter and tracking fields in App
    assert!(
        code.contains("pub dst1_adapter: Option<DatasetAdapter>,"),
        "missing adapter in Makepad App:\n{code}"
    );
    assert!(
        code.contains("pub dst1_opened: bool,"),
        "missing opened flag in Makepad App:\n{code}"
    );
    assert!(
        code.contains("pub dst1_error: Option<String>,"),
        "missing error field in Makepad App:\n{code}"
    );
    assert!(
        code.contains("pub edit_name_text_bound: String,"),
        "missing edit bound tracker in Makepad App:\n{code}"
    );
    assert!(
        code.contains("pub chk_active_checked_bound: String,"),
        "missing checkbox bound tracker in Makepad App:\n{code}"
    );

    // 2. Single open on first sync with exact expression
    assert!(
        code.contains(
            "crawrad_binding::PortaDbBridge::open_table(&crawrad_runtime::db_url::resolve_sqlite_url(&self.form.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.form.dst1.table)"
        ),
        "missing exact open_table expression in Makepad sync:\n{code}"
    );

    // 3. Per-binding sync lines (model -> widget)
    assert!(
        code.contains("self.ui.text_input(id!(edit_name)).set_text(cx, &val);"),
        "missing text_input set_text in Makepad sync:\n{code}"
    );
    assert!(
        code.contains("inner.animator_play(cx, state);"),
        "missing checkbox animator_play in Makepad sync:\n{code}"
    );

    // 4. TwoWay write-back on Edit and NO write-back on OneWay CheckBox
    assert!(
        code.contains("let current = self.form.edit_name.text.clone();")
            && code.contains("adapter.field_or_create(\"name\").set(current.clone());"),
        "missing TwoWay edit writeback in Makepad actions:\n{code}"
    );
    assert!(
        !code.contains("self.form.chk_active.checked.to_string()"),
        "OneWay CheckBox must not generate writeback:\n{code}"
    );
}

#[test]
fn egui_generates_adapter_and_sync_and_writeback() {
    let r = catalog();
    let d = bound_form(&r);
    let code = generate_designer(&GeradorRxp, &d, &r);

    // 1. Adapter and tracking fields in Form struct
    assert!(
        code.contains("pub dst1_adapter: Option<crawrad_binding::DatasetAdapter>,"),
        "missing adapter field in egui struct:\n{code}"
    );
    assert!(
        code.contains("pub dst1_opened: bool,"),
        "missing opened flag in egui struct:\n{code}"
    );
    assert!(
        code.contains("pub dst1_error: Option<String>,"),
        "missing error field in egui struct:\n{code}"
    );
    assert!(
        code.contains("pub edit_name_text_bound: String,"),
        "missing edit tracker in egui struct:\n{code}"
    );
    assert!(
        code.contains("pub chk_active_checked_bound: String,"),
        "missing checkbox tracker in egui struct:\n{code}"
    );

    // 2. Single open on first sync with exact expression
    assert!(
        code.contains("crawrad_binding::PortaDbBridge::open_table(&crawrad_runtime::db_url::resolve_sqlite_url(&self.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.dst1.table)"),
        "missing exact open_table expression in egui show:\n{code}"
    );

    // 3. Per-binding sync lines (model -> widget)
    assert!(
        code.contains("self.edit_name.text = val.clone();"),
        "missing edit model->widget sync in egui:\n{code}"
    );
    assert!(
        code.contains("self.chk_active.checked = is_checked;"),
        "missing checkbox model->widget sync in egui:\n{code}"
    );

    // 4. TwoWay write-back on Edit and NO write-back on OneWay CheckBox
    assert!(
        code.contains("let current = self.edit_name.text.clone();")
            && code.contains("adapter.field_or_create(\"name\").set(current.clone());"),
        "missing TwoWay edit writeback in egui:\n{code}"
    );
    assert!(
        !code.contains("self.chk_active.checked.to_string()"),
        "OneWay CheckBox must not generate writeback:\n{code}"
    );
}

#[test]
fn dangling_or_invalid_binding_source_emits_nothing() {
    let r = catalog();
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
    d.add_child("form_main", engine).expect("add engine");

    // Edit bound to a dangling dataset id that does not exist in the document
    let mut edit = r
        .instantiate(
            "Edit",
            "edit_dangling".to_string(),
            Rect::new(10.0, 10.0, 150.0, 24.0),
        )
        .expect("Edit instantiates");
    edit.set_name("edit_dangling");
    edit.set_binding(
        "text",
        BindingDef::new("non_existent_dataset", BindingMode::TwoWay).with_field("col"),
    );
    d.add_child("form_main", edit).expect("add edit");

    // CheckBox bound to a Connection instead of a Dataset (invalid source class)
    let mut chk = r
        .instantiate(
            "CheckBox",
            "chk_bad_class".to_string(),
            Rect::new(10.0, 40.0, 100.0, 24.0),
        )
        .expect("CheckBox instantiates");
    chk.set_name("chk_bad_class");
    chk.set_binding(
        "checked",
        BindingDef::new("conn1", BindingMode::TwoWay).with_field("col"),
    );
    d.add_child("form_main", chk).expect("add chk");

    let makepad_code = generate_designer(&GeradorMakepad, &d, &r);
    assert!(
        !makepad_code.contains("_adapter"),
        "Makepad must not emit adapter for invalid bindings:\n{makepad_code}"
    );
    assert!(
        !makepad_code.contains("open_table"),
        "Makepad must not emit open_table for invalid bindings:\n{makepad_code}"
    );
    assert!(
        !makepad_code.contains("_bound"),
        "Makepad must not emit bound tracker for invalid bindings:\n{makepad_code}"
    );

    let egui_code = generate_designer(&GeradorRxp, &d, &r);
    assert!(
        !egui_code.contains("_adapter"),
        "egui must not emit adapter for invalid bindings:\n{egui_code}"
    );
    assert!(
        !egui_code.contains("open_table"),
        "egui must not emit open_table for invalid bindings:\n{egui_code}"
    );
    assert!(
        !egui_code.contains("_bound"),
        "egui must not emit bound tracker for invalid bindings:\n{egui_code}"
    );
}

#[test]
fn two_bindings_on_same_dataset_open_it_once() {
    let r = catalog();
    let d = bound_form(&r);

    let makepad_code = generate_designer(&GeradorMakepad, &d, &r);
    assert_eq!(
        makepad_code.matches("open_table").count(),
        1,
        "Makepad must open dataset table exactly once:\n{makepad_code}"
    );
    assert_eq!(
        makepad_code.matches("dst1_adapter: Option<").count(),
        1,
        "Makepad must declare dst1_adapter exactly once:\n{makepad_code}"
    );

    let egui_code = generate_designer(&GeradorRxp, &d, &r);
    assert_eq!(
        egui_code.matches("open_table").count(),
        1,
        "egui must open dataset table exactly once:\n{egui_code}"
    );
    assert_eq!(
        egui_code.matches("dst1_adapter: Option<").count(),
        1,
        "egui must declare dst1_adapter exactly once:\n{egui_code}"
    );
}

/// A check box compares in ONE spelling: a "1" from the database is shown as
/// checked and NOT written back as "true" the moment it is shown.
#[test]
fn a_check_box_normalizes_the_cell_before_comparing() {
    let r = catalog();
    let d = bound_form(&r);
    for code in [
        generate_designer(&GeradorMakepad, &d, &r),
        generate_designer(&GeradorRxp, &d, &r),
    ] {
        let at = code
            .find("adapter.field_or_create(\"active\").get();")
            .expect("the check box reads its cell");
        let after = &code[at..];
        let normalize = after
            .lines()
            .position(|l| {
                l.trim_start()
                    == "let val = (val.eq_ignore_ascii_case(\"true\") || val == \"1\").to_string();"
            })
            .expect("the cell is normalized, on a line of its own");
        let compare = after
            .lines()
            .position(|l| l.trim_start() == "if val != self.chk_active_checked_bound {")
            .expect("then compared");
        assert!(
            normalize < compare,
            "normalized BEFORE the comparison:\n{after}"
        );
    }
}

/// OneTime reads only on the open; the other modes read on every sync.
#[test]
fn one_time_reads_only_on_the_open() {
    let r = catalog();
    let mut d = bound_form(&r);
    d.node_mut("chk_active")
        .expect("the check box")
        .set_binding(
            "checked",
            BindingDef::new("dst1", BindingMode::OneTime).with_field("active"),
        );
    for code in [
        generate_designer(&GeradorMakepad, &d, &r),
        generate_designer(&GeradorRxp, &d, &r),
    ] {
        assert_eq!(
            code.matches("self.dst1_adapter.as_ref().filter(|_| dst1_just_opened)")
                .count(),
            1,
            "only the OneTime binding waits for the open:\n{code}"
        );
        assert!(code.contains("if let Some(adapter) = self.dst1_adapter.as_ref() {"));
    }
}

/// The column is a Rust string literal in the app, escaped like every other.
#[test]
fn the_column_is_an_escaped_literal() {
    let r = catalog();
    let mut d = bound_form(&r);
    d.node_mut("edit_name").expect("the edit").set_binding(
        "text",
        BindingDef::new("dst1", BindingMode::TwoWay).with_field("na\"me"),
    );
    let code = generate_designer(&GeradorRxp, &d, &r);
    assert_eq!(
        code.matches(r#"adapter.field_or_create("na\"me")"#).count(),
        2,
        "escaped where it is read AND where it is written back:
{code}"
    );
}

/// A form with bound controls asks for `crawrad-binding` with its SQLite driver.
#[test]
fn bound_controls_bring_the_binding_crate_with_its_driver() {
    let r = catalog();
    let d = bound_form(&r);
    for target in [crate::AlvoGerador::Makepad, crate::AlvoGerador::Egui] {
        let deps = crate::component_dependencies(&d, target);
        let binding = deps
            .iter()
            .find(|dep| dep.nome() == "crawrad-binding")
            .expect("crawrad-binding is listed");
        let args = binding.args_cargo_add();
        assert_eq!(args[args.len() - 2..], ["--features", "rusqlite"]);
    }
    let unbound = Document::new("form_main", "Form");
    assert!(
        crate::component_dependencies(&unbound, crate::AlvoGerador::Makepad)
            .iter()
            .all(|dep| dep.nome() != "crawrad-binding")
    );
}

/// A source is a `Dataset` by its CLASS, and its connection is a `Connection`
/// by its class -- a node that merely carries a `connection` property is not.
///
/// The catalog refuses an undeclared property before generating, so this asks
/// the resolver itself: a third-party class may well declare a `connection`.
#[test]
fn only_a_dataset_over_a_data_engine_is_a_source() {
    use crate::targets::shared::bound_dataset_connection;
    let r = catalog();
    let mut d = bound_form(&r);
    let resolved = bound_dataset_connection(&d, "dst1").expect("resolves");
    assert_eq!(
        resolved.map(|c| (c.dataset_name, c.engine_name)),
        Some(("dst1".to_string(), "conn1".to_string()))
    );

    for (id, class) in [("lbl1", "Label"), ("lbl2", "Label")] {
        let mut label = r
            .instantiate(class, id.to_string(), Rect::new(0.0, 0.0, 10.0, 10.0))
            .expect("Label instantiates");
        label.set_name(id);
        d.add_child("form_main", label).expect("add label");
    }
    // a Label carrying the dataset's connection is still a Label
    d.node_mut("lbl1")
        .expect("lbl1")
        .props
        .insert("connection".to_string(), Value::Ref("conn1".into()));
    assert_eq!(
        bound_dataset_connection(&d, "lbl1").expect("resolves"),
        None
    );

    // a dataset whose connection is not a Connection opens nothing
    d.node_mut("dst1")
        .expect("dst1")
        .props
        .insert("connection".to_string(), Value::Ref("lbl2".into()));
    assert_eq!(
        bound_dataset_connection(&d, "dst1").expect("resolves"),
        None
    );
}
