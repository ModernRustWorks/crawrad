//! The egui target's pointer (`targets/egui/form.rs`).

use super::*;
use crawrad_ir::{Cursor, Rect, Value};

fn node(class: &str, cursor: Cursor) -> Node {
    let r = crawrad_registry::Registry::builtin();
    let mut n = r
        .instantiate(class, "ctl", Rect::new(10.0, 10.0, 120.0, 30.0))
        .unwrap();
    n.props.insert("cursor".into(), Value::Cursor(cursor));
    n
}

#[test]
fn a_control_shows_its_cursor_while_the_pointer_is_in_its_box() {
    let r = crawrad_registry::Registry::builtin();
    let code = cursor_de(&r, &node("Button", Cursor::Hand), "ctl");
    assert!(
        code.contains("self.ctl.visible && self.ctl.enabled && self.ctl.cursor != Cursor::Default && ui.rect_contains_pointer(r)"),
        "{code}"
    );
    assert!(
        code.contains("set_cursor_icon(cursor_icon_of(self.ctl.cursor))"),
        "{code}"
    );
}

#[test]
fn every_cursor_has_its_egui_pointer() {
    for (cursor, icon) in [
        ("Cursor::Hand", "PointingHand"),
        ("Cursor::Text", "Text"),
        ("Cursor::Wait", "Wait"),
        ("Cursor::Crosshair", "Crosshair"),
    ] {
        assert!(
            CURSOR_ICON_FN.contains(&format!("{cursor} => egui::CursorIcon::{icon}")),
            "{cursor}"
        );
    }
}

fn form(cursor: Cursor) -> String {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .props
        .insert("cursor".into(), Value::Cursor(cursor));
    form_cursor(d.node("form_main").unwrap())
}

#[test]
fn the_form_shows_its_cursor_where_no_control_did() {
    let code = form(Cursor::Crosshair);
    assert!(
        code.contains("o.cursor_icon) == egui::CursorIcon::Default"),
        "{code}"
    );
    assert!(
        code.contains("set_cursor_icon(egui::CursorIcon::Crosshair)"),
        "{code}"
    );
    assert!(form(Cursor::Default).is_empty());
    assert!(form(Cursor::Arrow).is_empty());
}
