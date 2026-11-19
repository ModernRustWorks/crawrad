//! The form's own hint on egui (`targets/egui/form.rs`).

use super::*;
use crawrad_ir::{Rect, Value};

fn form(hint: &str) -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .props
        .insert("hint".into(), Value::Str(hint.into()));
    let mut n = r
        .instantiate("Button", "btn", Rect::new(10.0, 10.0, 80.0, 30.0))
        .unwrap();
    n.set_name("btn");
    d.add_child("form_main", n).unwrap();
    d
}

fn over(d: &Document) -> String {
    let r = crawrad_registry::Registry::builtin();
    let n = d.nodes.values().find(|n| n.id != d.root).unwrap();
    over_de(&r, d, n, "btn")
}

#[test]
fn a_control_marks_the_pointer_over_it_only_when_the_form_has_a_hint() {
    let code = over(&form("the form"));
    assert!(
        code.contains("crawrad_over_control |= self.btn.visible && ui.rect_contains_pointer(r);"),
        "{code}"
    );
    assert!(over(&form("")).is_empty());
}

#[test]
fn over_no_control_the_form_shows_its_hint() {
    let d = form("the form");
    let body = over(&d);
    let (open, close) = form_hint(d.node(&d.root).unwrap(), &body);
    assert!(
        open.contains("let mut crawrad_over_control = false;"),
        "{open}"
    );
    assert!(
        close.contains("if !crawrad_over_control && ui.rect_contains_pointer(ui.max_rect())"),
        "{close}"
    );
    assert!(close.contains("ui.label(\"the form\")"), "{close}");
}

#[test]
fn with_no_control_to_cover_it_there_is_no_flag() {
    let d = form("the form");
    let (open, close) = form_hint(d.node(&d.root).unwrap(), "");
    assert!(open.is_empty());
    assert!(
        close.contains("if ui.rect_contains_pointer(ui.max_rect())"),
        "{close}"
    );
}
