//! The form's own hint on Makepad (`targets/makepad/form_hint.rs`).

use super::*;
use crawrad_ir::{Rect, Value};

fn form(hint: &str, with_control: bool) -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .props
        .insert("hint".into(), Value::Str(hint.into()));
    if with_control {
        let mut n = r
            .instantiate("Button", "btn", Rect::new(10.0, 10.0, 80.0, 30.0))
            .unwrap();
        n.set_name("btn");
        d.add_child("form_main", n).unwrap();
    }
    d
}

fn block(d: &Document) -> Option<FormHint> {
    let r = crawrad_registry::Registry::builtin();
    let emitted: Vec<&Node> = d.nodes.values().filter(|n| n.id != d.root).collect();
    form_hint(&r, d, &emitted).unwrap()
}

#[test]
fn over_no_control_the_form_shows_its_hint() {
    let fh = block(&form("the form", true)).expect("a hint to show");
    let code = fh.after_widgets;
    assert!(
        code.contains("id!(btn)).area().rect(cx).contains(e.abs)"),
        "{code}"
    );
    assert!(
        code.contains("over = Some(CRAWRAD_FORM_SHOW_HINT && self.form.btn.show_hint && !self.form.btn.hint.is_empty());"),
        "{code}"
    );
    assert!(code.contains("if over.is_none() {"), "{code}");
    assert!(
        code.contains("show_with_options(cx, e.abs + dvec2(12.0, 20.0), \"the form\");"),
        "{code}"
    );
    assert!(fh.field.contains("crawrad_form_hint: bool"));
}

#[test]
fn onto_a_control_with_a_hint_the_tooltip_is_left_to_it() {
    let code = block(&form("the form", true)).unwrap().after_widgets;
    let keep = code
        .find("} else if over == Some(true) {")
        .expect("the control's");
    let hide = code
        .find("} else if self.crawrad_form_hint {")
        .expect("the hide");
    assert!(keep < hide, "{code}");
}

#[test]
fn a_form_with_no_hint_or_show_hint_off_has_none() {
    assert!(block(&form("", true)).is_none());
    let mut d = form("the form", true);
    d.node_mut("form_main")
        .unwrap()
        .props
        .insert("show_hint".into(), Value::Bool(false));
    assert!(block(&d).is_none());
}

#[test]
fn a_form_with_no_control_follows_the_pointer() {
    let fh = block(&form("the form", false)).unwrap();
    let code = fh.after_widgets;
    assert!(
        code.contains("if let Event::MouseMove(e) = event {"),
        "{code}"
    );
    assert!(code.contains("show_with_options("), "{code}");
    // nothing to hide for, so no flag
    assert!(!code.contains("crawrad_form_hint"), "{code}");
    assert!(fh.field.is_empty());
}

#[test]
fn tab_sheet_hint_shows_over_tab_in_form_hint() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let mut pc = r
        .instantiate("PageControl", "pages1", Rect::new(10.0, 10.0, 200.0, 150.0))
        .unwrap();
    pc.set_name("pages1");
    d.add_child("form_main", pc).unwrap();

    let mut t1 = r
        .instantiate("TabSheet", "sheet1", Rect::new(0.0, 0.0, 200.0, 150.0))
        .unwrap();
    t1.set_name("sheet1");
    t1.props
        .insert("hint".into(), Value::Str("sheet hint".into()));
    d.add_child("pages1", t1).unwrap();

    let fh = block(&d).expect("tab sheet hint generates FormHint");
    let code = fh.after_widgets;
    assert!(
        code.contains(
            "self.ui.widget(id!(pages1.tabs_header.tab_0)).area().rect(cx).contains(e.abs)"
        ),
        "{code}"
    );
    assert!(code.contains("self.form.sheet1.hint"), "{code}");
}
