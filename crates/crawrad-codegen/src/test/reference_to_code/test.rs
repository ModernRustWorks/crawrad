//! **THE REFERENCE PASSES THROUGH** — what the user wired in the Inspector reaches
//! the application.
//!
//! # The gap this closes, and it was silent
//!
//! The vocabulary always emitted the `Set` of a reference —
//! [`crate::emission`] even orders the `Create` of the target beforehand, precisely so
//! that whatever reads a `Value::Ref` finds the target created.
//!
//! **And both targets discarded it.** Neither of the two knew how to write a
//! `Value::Ref`, and the field came out as the crate's `Default`.
//!
//! The effect only became visible with the connection design of ADR 0040: the user
//! drops a `SqliteProvider` onto the form, points the connection to it in the Object
//! Inspector, and the application started **without knowing which provider**. The choice died
//! between the click and `cargo build`.
//!
//! # Both targets in the same file, and for the same usual reason
//!
//! The defect was the same and had two copies. One file per target would make the
//! second half easy to forget.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::Gerador;

/// The catalogue with the connection package of THIS repository, read from file.
fn catalogo() -> Registry {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let m: Manifest = serde_json::from_str(text).expect("the `manifest.json` is valid JSON");
    let mut r = Registry::builtin();
    for c in m.to_components().expect("converts") {
        r.register_external(c).expect("registers");
    }
    r
}

/// A form with connection and provider, wired — or not.
fn formulario(reg: &Registry, aponta_para: Option<&str>) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    for (classe, nome) in [("SqliteProvider", "sqlt1"), ("Connection", "conn1")] {
        let mut n = reg
            .instantiate(classe, nome.to_string(), Rect::new(8.0, 8.0, 28.0, 28.0))
            .unwrap_or_else(|| panic!("the catalogue instantiates `{classe}`"));
        n.set_name(nome);
        d.add_child("form_main", n).expect("added to the form");
    }
    if let Some(alvo) = aponta_para {
        d.node_mut("conn1")
            .expect("the connection is in the form")
            .props
            .insert("provider".to_string(), Value::Ref(alvo.to_string()));
    }
    d
}

fn designer(g: &dyn Gerador, reg: &Registry, d: &Document) -> String {
    g.gerar(d, "app", reg)
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

fn linhas_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}

/// **The NAME of the referenced provider reaches code — on both targets.**
///
/// The document stores the **id**, which is an address and never reaches the interface
/// (ADR 0008). What the application receives is the NAME, which is the same as the user
/// reads in the Inspector and writes in their handler.
#[test]
fn o_nome_do_alvo_chega_ao_codigo_nos_dois_alvos() {
    let reg = catalogo();
    let doc = formulario(&reg, Some("sqlt1"));

    for (alvo, gerador) in [
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
        ("egui", &crate::GeradorRxp as &dyn Gerador),
    ] {
        let fonte = designer(gerador, &reg, &doc);
        assert!(
            fonte.contains("\"sqlt1\".to_string()"),
            "[{alvo}] Inspector choice did not reach the code:\n{}",
            linhas_com(&fonte, "conn1")
        );
    }
}

/// **No provider chosen turns into empty, not a fabricated name.**
///
/// This is the state the connection is born in, and it must pass through identically: an
/// application born mentioning a component nobody chose would be
/// worse than one saying "none".
#[test]
fn a_referencia_vazia_atravessa_como_vazia() {
    let reg = catalogo();
    let doc = formulario(&reg, None);

    for (alvo, gerador) in [
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
        ("egui", &crate::GeradorRxp as &dyn Gerador),
    ] {
        let fonte = designer(gerador, &reg, &doc);
        assert!(
            !fonte.contains("\"sqlt1\".to_string()"),
            "[{alvo}] nothing was chosen and the code chose on its own:\n{}",
            linhas_com(&fonte, "conn1")
        );
        assert!(
            fonte.contains("provider: String::new()")
                || fonte.contains(".provider = String::new()"),
            "[{alvo}] the field should have come out empty, and explicitly:\n{}",
            linhas_com(&fonte, "provider")
        );
    }
}

/// **Targeting a deleted component becomes empty.**
///
/// Deleting the provider and leaving the connection pointing to it is a state that the
/// document allows. The application must start saying "none" instead of
/// citing a component that no longer exists — and, worse, failing to compile.
#[test]
fn a_referencia_orfa_vira_vazia() {
    let reg = catalogo();
    let doc = formulario(&reg, Some("um_provider_que_foi_apagado"));

    for (alvo, gerador) in [
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
        ("egui", &crate::GeradorRxp as &dyn Gerador),
    ] {
        let fonte = designer(gerador, &reg, &doc);
        assert!(
            !fonte.contains("um_provider_que_foi_apagado"),
            "[{alvo}] deleted node id leaked into user code:\n{}",
            linhas_com(&fonte, "provider")
        );
        assert!(
            fonte.contains("provider: String::new()")
                || fonte.contains(".provider = String::new()"),
            "[{alvo}] orphan reference should have become empty:\n{}",
            linhas_com(&fonte, "provider")
        );
    }
}

/// **ID never leaks into user code.**
///
/// ADR 0008 is explicit: the id is an address, not a label. Here it has a
/// second reason not to leak — renaming the provider in the designer must change
/// what the application reads, and an id written into code would survive the rename and
/// lie.
#[test]
fn o_id_nao_vaza_para_o_codigo() {
    let reg = catalogo();
    let mut doc = formulario(&reg, Some("sqlt1"));
    // The name changes; the id does not. Code must follow the NAME.
    doc.node_mut("sqlt1")
        .expect("provider is in the form")
        .set_name("banco_local");

    for (alvo, gerador) in [
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
        ("egui", &crate::GeradorRxp as &dyn Gerador),
    ] {
        let fonte = designer(gerador, &reg, &doc);
        assert!(
            fonte.contains("\"banco_local\".to_string()"),
            "[{alvo}] rename did not reach the code:\n{}",
            linhas_com(&fonte, "provider")
        );
    }
}
