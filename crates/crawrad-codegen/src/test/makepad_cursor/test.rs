//! The Makepad target's pointer (`targets/makepad/cursor.rs`).

use super::*;
use crawrad_ir::{Cursor, Rect, Value};

/// A form whose own `Cursor` is `form`, holding an edit and a label, each
/// with its own `Cursor`.
fn form(form: Cursor, edit: Cursor, label: Cursor) -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let root = d.node_mut("form_main").unwrap();
    root.set_rect(Rect::new(0.0, 0.0, 640.0, 300.0));
    root.props.insert("cursor".into(), Value::Cursor(form));
    for (class, name, cursor, y) in [("Edit", "edt", edit, 10.0), ("Label", "lbl", label, 60.0)] {
        let mut n = r
            .instantiate(class, name, Rect::new(10.0, y, 120.0, 30.0))
            .unwrap();
        n.set_name(name);
        n.props.insert("cursor".into(), Value::Cursor(cursor));
        d.add_child("form_main", n).unwrap();
    }
    d
}

fn block(d: &Document) -> String {
    let r = crawrad_registry::Registry::builtin();
    let emitted: Vec<&Node> = d.nodes.values().filter(|n| n.id != d.root).collect();
    after_widgets(&r, d, &emitted).unwrap()
}

#[test]
fn default_is_the_controls_own_pointer() {
    let code = block(&form(Cursor::Default, Cursor::Default, Cursor::Default));
    assert!(
        code.contains("self.form.edt.cursor { Cursor::Default => \"text\""),
        "an edit's Default is the text beam:\n{code}"
    );
    assert!(
        code.contains("self.form.lbl.cursor { Cursor::Default => \"arrow\""),
        "a label's Default is the arrow:\n{code}"
    );
}

#[test]
fn over_no_control_the_form_answers() {
    let code = block(&form(Cursor::Crosshair, Cursor::Default, Cursor::Default));
    assert!(
        code.contains("cursor.unwrap_or(\"crosshair\")"),
        "the form's Cursor over no control:\n{code}"
    );
    let code = block(&form(Cursor::Default, Cursor::Default, Cursor::Default));
    assert!(
        code.contains("cursor.unwrap_or(\"arrow\")"),
        "the arrow when the form has none:\n{code}"
    );
}

#[test]
fn the_control_on_top_answers_first() {
    let code = block(&form(Cursor::Default, Cursor::Hand, Cursor::Wait));
    let at = |name: &str| code.find(&format!("id!({name})")).unwrap();
    assert!(
        at("lbl") < at("edt"),
        "the last drawn is tried first:\n{code}"
    );
    assert!(
        code.contains("self.form.edt.visible && self.form.edt.enabled"),
        "{code}"
    );
}

#[test]
fn a_form_with_no_control_shows_only_its_own() {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .props
        .insert("cursor".into(), Value::Cursor(Cursor::Hand));
    let code = block(&d);
    assert!(code.contains("cursor::show(cx, \"hand\")"), "{code}");
    let plain = Document::new("form_main", "Form");
    assert!(block(&plain).is_empty(), "nothing to show");
}
