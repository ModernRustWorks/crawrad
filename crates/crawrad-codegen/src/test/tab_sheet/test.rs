//! **A page outside a page control is not generated.**
//!
//! The catalogue says it in words, in `crawrad_registry::componentes::tab_sheet`:
//! *"a page outside a page control is nothing at all"* — which is
//! why `TabSheet` is off the palette and why its rectangle is derived rather
//! than chosen.
//!
//! Generating one anyway produced a field and a type that **no line of the
//! application read**: the caption feeds the tab strip its page control draws,
//! and with no page control there is no strip. That was real `dead_code` in
//! every project that had one, and it sat quiet for as long as the designer
//! carried a `#![allow(dead_code)]` at the top.
//!
//! The drawing side already agreed: `desenha_por_si` has never given a
//! `TabSheet` a rectangle of its own. So not emitting it is the two sides
//! saying the same thing — which is the rule this house calls the golden one.

use crate::Gerador;
use crawrad_ir::{Document, Node, Rect};

/// The designer file for `doc`, from the egui target.
fn designer(doc: &Document) -> String {
    crate::GeradorRxp
        .gerar(doc, "app", &crawrad_registry::Registry::builtin())
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

fn form() -> Document {
    let mut d = Document::new("form_main", "Form");
    let root = d.node_mut("form_main").expect("the brand new root");
    root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    root.set_name("form_main");
    root.set_text("Form1");
    d
}

fn filho(d: &mut Document, pai: &str, id: &str, classe: &str, r: Rect) {
    let mut n = Node::new(id, classe);
    n.set_name(id);
    n.set_rect(r);
    d.add_child(pai, n).expect("it goes in");
}

/// A page **inside** a `PageControl` becomes a field, type, and caption on the
/// strip. That is the side that must not break when the other is cut.
#[test]
fn a_page_inside_a_page_control_is_generated() {
    let mut d = form();
    filho(
        &mut d,
        "form_main",
        "pgc",
        "PageControl",
        Rect::new(8.0, 8.0, 320.0, 200.0),
    );
    filho(
        &mut d,
        "pgc",
        "tab_um",
        "TabSheet",
        Rect::new(0.0, 24.0, 320.0, 176.0),
    );
    d.validate().expect("valid");

    let g = designer(&d);
    assert!(g.contains("pub tab_um: TabSheet,"), "missing field:\n{g}");
    assert!(g.contains("pub struct TabSheet {"), "missing type");
    assert!(
        g.contains("let captions: [(&str, &str); 1] = [(self.tab_um.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.tab_um.show_hint { self.tab_um.hint.as_str() } else { \"\" })];"),
        "the tab caption did not come from page state:\n{g}"
    );
}

/// A page **loose on the form** becomes nothing — neither field nor type.
///
/// The mutation this test prevents is removing the parent check in
/// [`crate::targets::egui::emitido_no`]: with it removed, the field returns, nobody
/// reads it, and user's `cargo` reports `dead_code` in a file rewritten by
/// the generator — meaning they cannot fix it.
#[test]
fn a_page_with_no_page_control_is_not_generated() {
    let mut d = form();
    filho(
        &mut d,
        "form_main",
        "tab_solta",
        "TabSheet",
        Rect::new(8.0, 8.0, 320.0, 176.0),
    );
    d.validate().expect("valid");

    let g = designer(&d);
    assert!(
        !g.contains("tab_solta"),
        "the loose page became code, and nothing in the application reads it:\n{g}"
    );
    assert!(
        !g.contains("pub struct TabSheet {"),
        "the page type was emitted with nothing constructing it:\n{g}"
    );
}

/// And the child of a loose page still gets emitted — it does not vanish along with it.
///
/// The page guard climbs the tree looking for `TabSheet` **inside a**
/// `PageControl`; a loose page adds no condition, so the
/// button draws directly, without referencing the non-existent field.
#[test]
fn the_child_of_a_stray_page_still_draws() {
    let mut d = form();
    filho(
        &mut d,
        "form_main",
        "tab_solta",
        "TabSheet",
        Rect::new(8.0, 8.0, 320.0, 176.0),
    );
    filho(
        &mut d,
        "tab_solta",
        "btn_ok",
        "Button",
        Rect::new(8.0, 8.0, 96.0, 32.0),
    );
    d.validate().expect("valid");

    let g = designer(&d);
    assert!(
        g.contains("pub btn_ok: Button,"),
        "the button vanished along with it:\n{g}"
    );
    assert!(
        !g.contains("tab_solta"),
        "the button was guarded by a field that does not exist:\n{g}"
    );
}
