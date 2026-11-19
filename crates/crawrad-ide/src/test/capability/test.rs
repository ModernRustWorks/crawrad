//! **CAPABILITY, measured where it appears: the Object Inspector list.**
//!
//! # The design, in the owner's words (2026-09-18)
//!
//! > *"Our abstraction will always need a property to receive a
//! > native component. Therefore, the native component will have to exist."*
//!
//! It is the connection plus the *driver link* of a classic RAD IDE: two components on
//! screen, wired through a property.
//!
//! # What was at stake, and why simply listing classes was not enough
//!
//! The reference property declares **one** name. As long as that name had to
//! be a class, the abstraction could only accept one provider — or carry all five
//! names internally, meaning the **sixth** provider would require editing the abstraction
//! to exist. Which is the opposite of what abstraction means.
//!
//! With capability, the name is `DbProvider`: providers offer it, the abstraction
//! requests it, and neither side knows the other. See ADR 0040.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::ide_app::inspector::editor::Cell;
use crate::ide_app::inspector::editors::reference::NONE;
use crate::ide_app::inspector::{self, Row};

/// The catalog with the connection package from THIS repository, read from file.
fn catalogo() -> Registry {
    let mut r = Registry::builtin();
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let m: Manifest = serde_json::from_str(text).expect("valid JSON");
    for c in m.to_components().expect("convert") {
        r.register_external(c).expect("register");
    }
    r
}

fn com(reg: &Registry, d: &mut Document, classe: &str, nome: &str, pai: &str) {
    let mut n = reg
        .instantiate(classe, nome.to_string(), Rect::new(8.0, 8.0, 28.0, 28.0))
        .unwrap_or_else(|| panic!("catalog instantiates `{classe}`"));
    n.set_name(nome);
    d.add_child(pai, n).expect("enters form");
}

fn vazio() -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    d
}

/// A form with the abstraction, three native providers, and one that is not a provider.
fn formulario(reg: &Registry) -> Document {
    let mut d = vazio();
    for (classe, nome) in [
        ("Connection", "conn1"),
        ("SqliteProvider", "sqlt1"),
        ("PostgresProvider", "spg1"),
        ("MemoryProvider", "mem1"),
        // And one that is NOT a provider, so the list has something to leave out.
        ("Button", "btn1"),
    ] {
        com(reg, &mut d, classe, nome, "form_main");
    }
    d
}

/// The choices offered by a property row, excluding `(none)`.
///
/// Comes from `cells()`, which is the editor's public surface — the test measures what
/// the screen receives, not a private field.
fn escolhas(reg: &Registry, doc: &Document, no: &str, chave: &str) -> Vec<String> {
    let linhas = inspector::rows(
        reg,
        Some(doc),
        &[no.to_string()],
        crawrad_runtime_makepad::ALVO,
        inspector::Tab::Properties,
        &Default::default(),
        &Default::default(),
    );
    let linha = linhas
        .iter()
        .find_map(|l| match l {
            Row::Prop(p) if p.key == chave => Some(p),
            _ => None,
        })
        .unwrap_or_else(|| panic!("row `{chave}` is not in the Inspector of `{no}`"));
    linha
        .editor
        .cells()
        .into_iter()
        .find_map(|c| match c {
            Cell::Choice { options, .. } => Some(options),
            _ => None,
        })
        .unwrap_or_else(|| panic!("row `{chave}` is not a list"))
        .into_iter()
        .filter(|o| o != NONE)
        .collect()
}

/// **The abstraction offers the THREE providers on the form, and only them.**
///
/// This is the entire proof: none of the three is the class the property names — it
/// names `DbProvider`, which is nobody's class. The three appear because
/// they declared offering it, and `Button` is excluded because it did not declare it.
#[test]
fn a_abstracao_oferece_todo_provider_e_so_provider() {
    let reg = catalogo();
    let doc = formulario(&reg);
    let lista = escolhas(&reg, &doc, "conn1", "provider");

    assert_eq!(
        lista.len(),
        3,
        "expected the three providers from the form, got: {lista:?}"
    );
    for nome in ["sqlt1", "spg1", "mem1"] {
        assert!(
            lista.iter().any(|e| e == nome),
            "`{nome}` provides `DbProvider` and did not appear: {lista:?}"
        );
    }
    assert!(
        !lista.iter().any(|e| e == "btn1"),
        "`Button` does not provide `DbProvider` and entered the list: {lista:?}"
    );
}

/// **The class continues to answer by its own name.**
///
/// `Button.action` requests `"Action"`, and `Action` declares no
/// `provides` — it answers because a component's class is its first
/// capability. Without this, the change would have broken every existing
/// reference to support the new one.
#[test]
fn a_classe_continua_respondendo_pelo_proprio_nome() {
    let reg = Registry::builtin();
    let mut d = vazio();
    com(&reg, &mut d, "ActionList", "acl1", "form_main");
    com(&reg, &mut d, "Button", "btn1", "form_main");
    com(&reg, &mut d, "Action", "act1", "acl1");

    let lista = escolhas(&reg, &d, "btn1", "action");
    assert!(
        lista.iter().any(|e| e == "act1"),
        "`Action` stopped responding by its own name: {lista:?}"
    );
}

/// **A NEW provider enters the list without touching the abstraction.**
///
/// This test measures the rationale behind the decision. It registers a class
/// not present in this repository, declaring the same capability, and it appears — without
/// `Connection`, its `manifest.json`, or this file knowing it
/// exists.
///
/// If someone ever replaces capability with a list of classes inside the
/// abstraction, this is where it turns red.
#[test]
fn um_provider_novo_entra_sem_tocar_na_abstracao() {
    let mut reg = catalogo();
    let mut inventado = crawrad_registry::external::ExternalComponent::from(
        reg.get("MemoryProvider")
            .expect("MemoryProvider is present"),
    );
    inventado.class = "MongoProvider".to_string();
    inventado.display = "Mongo Provider".to_string();
    inventado.prefix = "mgo".to_string();
    reg.register_external(inventado)
        .expect("external provider passes the same validations");

    let mut doc = formulario(&reg);
    com(&reg, &mut doc, "MongoProvider", "mgo1", "form_main");

    let lista = escolhas(&reg, &doc, "conn1", "provider");
    assert!(
        lista.iter().any(|e| e == "mgo1"),
        "new provider did not appear, which is exactly what capability \
         exists for: {lista:?}"
    );
}

/// **The property's VALUE still comes from the document.**
///
/// The list changed; reading it did not. Without this, a fix in the list could hide
/// a read error — the row would show the right options with the wrong one selected.
#[test]
fn o_valor_da_referencia_continua_vindo_do_documento() {
    let reg = catalogo();
    let mut doc = formulario(&reg);
    doc.node_mut("conn1")
        .expect("abstraction is in the form")
        .props
        .insert("provider".to_string(), Value::Ref("sqlt1".to_string()));

    let linhas = inspector::rows(
        &reg,
        Some(&doc),
        &["conn1".to_string()],
        crawrad_runtime_makepad::ALVO,
        inspector::Tab::Properties,
        &Default::default(),
        &Default::default(),
    );
    let linha = linhas
        .iter()
        .find_map(|l| match l {
            Row::Prop(p) if p.key == "provider" => Some(p),
            _ => None,
        })
        .expect("Provider row");

    let Some(Cell::Choice {
        options, current, ..
    }) = linha.editor.cells().into_iter().next()
    else {
        panic!("Provider row is not a list");
    };
    let i = current.expect("document points to a provider in the list");
    assert_eq!(
        options[i], "sqlt1",
        "row selected another provider: {options:?}"
    );
}

/// **A `Grid` binds to the form's `Connection`.**
///
/// `DataSource` asks for the `Connection` class itself, not the `DbProvider`
/// capability: the grid reads through the abstraction, so the native providers
/// are not offered. `TableName` is a plain text row beside it.
#[test]
fn a_grid_points_at_the_data_engine_and_names_a_table() {
    let reg = catalogo();
    let mut doc = formulario(&reg);
    com(&reg, &mut doc, "Grid", "grid1", "form_main");
    assert_eq!(escolhas(&reg, &doc, "grid1", "data_source"), ["conn1"]);

    let rows = inspector::rows(
        &reg,
        Some(&doc),
        &["grid1".to_string()],
        crawrad_runtime_makepad::ALVO,
        inspector::Tab::Properties,
        &Default::default(),
        &Default::default(),
    );
    assert!(
        rows.iter()
            .any(|l| matches!(l, Row::Prop(p) if p.key == "table_name")),
        "`table_name` is not in the Grid's Inspector"
    );
}
