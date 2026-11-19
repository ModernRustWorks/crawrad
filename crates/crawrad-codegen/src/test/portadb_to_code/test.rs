//! **The `Connection` becomes CODE.**
//!
//! This file is not about a component: it is the end of the chain opened by
//! ADR 0033.
//! A component that **does not exist in this repository** — arriving as a
//! `manifest.json` from a crates.io crate — travels through the catalogue, validation,
//! the generator, and comes out as real Rust inside user code.
//!
//! # The entire path, and what each piece solved
//!
//! | step | where it was decided |
//! |---|---|
//! | the component is SOURCE, and the IDE compiles | ADR 0033 |
//! | the class `enum` no longer blocks anyone | ADR 0031, Amendment 2 |
//! | there are no privileged components | ADR 0035 |
//! | the unit of the format is the OBJECT, and the verbs are four | ADR 0036 |
//! | the PIECES are the product | ADR 0037 |
//! | the generator RECEIVES the catalogue | 18/09/2026 |
//! | the third-party type has an address: the `crate_module` | 18/09/2026 |
//!
//! # What this test proves, and what it does NOT prove
//!
//! Proves that the **code is emitted**: field with the right type, `default()`, and one
//! assignment per declared property.
//!
//! Does not prove that the generated project **compiles** — for that its `Cargo.toml`
//! needs the dependency, which is added by `cargo add` in the shell
//! (ADR 0018). That is the next half, and saying it is done because the text
//! came out right would be the lie this house spent the day rooting out.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::Gerador;

/// The catalogue with `Connection` read from the **real** `manifest.json`.
///
/// Not an `ExternalComponent` written here: it is the file in
/// `components/portadb-crawrad/`, the same one that would go to crates.io.
fn catalogo_com_portadb() -> Registry {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let manifesto: Manifest = serde_json::from_str(text).expect("manifest.json is valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto
        .to_components()
        .expect("portadb-crawrad manifest passes conversion")
    {
        r.register_external(c)
            .expect("and passes the seven gate checks");
    }
    r
}

/// A form with a `Connection` in it.
fn doc_com_portadb(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let mut n = r
        .instantiate(
            "Connection",
            "conn1".to_string(),
            Rect::new(8.0, 8.0, 28.0, 28.0),
        )
        .expect("the registry instantiates what it registered");
    n.set_name("conn1");
    n.props
        .insert("url".to_string(), Value::Str("sqlite://loja.db".into()));
    d.add_child("form_main", n).expect("added to the form");
    d
}

/// The designer emitted by the target.
fn designer(g: &dyn Gerador, d: &Document, r: &Registry) -> String {
    g.gerar(d, "app", r)
        .unwrap_or_else(|e| panic!("target `{}` rejected: {e}", g.alvo()))
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

/// **WE DID IT.** The component from the file became a type in user code.
#[test]
fn the_portadb_of_the_manifest_becomes_a_typed_field() {
    let r = catalogo_com_portadb();
    let d = doc_com_portadb(&r);
    let fonte = designer(&crate::GeradorRxp, &d, &r);

    assert!(
        fonte.contains("pub conn1: portadb_crawrad::Connection,"),
        "the `Connection` field was not emitted as typed. What was emitted:\n{}",
        trecho_com(&fonte, "conn1")
    );
    assert!(
        fonte.contains("conn1: portadb_crawrad::Connection::default(),"),
        "the `Create` from ADR 0036 did not become `default()`:\n{}",
        trecho_com(&fonte, "conn1")
    );
}

/// And the properties edited by the user reach the application.
#[test]
fn the_portadb_properties_reach_the_code() {
    let r = catalogo_com_portadb();
    let d = doc_com_portadb(&r);
    let fonte = designer(&crate::GeradorRxp, &d, &r);

    assert!(
        fonte.contains("self.conn1.url = \"sqlite://loja.db\".to_string();"),
        "the `Set` from ADR 0036 did not become an assignment:\n{}",
        trecho_com(&fonte, "conn1")
    );
}

/// **And the `TODO` is NO longer emitted.**
///
/// The mutation written as a test: as long as the generator did not know the type's
/// address, it emitted a fake `Widget` with a `// TODO` next to it. If someone
/// reverts `crate_module`, this phrase returns — and this test fails.
#[test]
fn o_terceiro_nao_cai_mais_no_todo() {
    let r = catalogo_com_portadb();
    let d = doc_com_portadb(&r);
    let fonte = designer(&crate::GeradorRxp, &d, &r);

    assert!(
        !fonte.contains("class `Connection` has no egui generation yet"),
        "`Connection` was treated as an unknown class again"
    );
    assert!(
        !fonte.contains("pub conn1: Widget,"),
        "`Connection` turned into the fake `Widget` again"
    );
}

/// Lines referencing the component, so error messages carry context.
fn trecho_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}
