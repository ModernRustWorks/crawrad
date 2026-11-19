//! **The controls' pointer events come out of the TABLE, not of a wrapper per
//! class** — one router, and a dispatch arm per wired handler.

use crate::targets::makepad::control_events::control_events;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// A form holding one `class` named `ctl`, with `events` wired as
/// `(event, handler)`.
fn form_with(class: &str, events: &[(&str, &str)]) -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let mut n = r
        .instantiate(class, "ctl".to_string(), Rect::new(8.0, 8.0, 120.0, 32.0))
        .expect("the catalogue instantiates what it lists");
    n.set_name("ctl");
    for (e, h) in events {
        n.events.insert((*e).into(), (*h).into());
    }
    d.add_child("form_main", n).expect("into the form");
    d
}

/// What the router adds to the `App`'s `handle_event`, if anything.
fn handle(class: &str, events: &[(&str, &str)]) -> Option<String> {
    let r = Registry::builtin();
    let d = form_with(class, events);
    let node = d.node("ctl").expect("the control");
    control_events(&r, &d, &[node])
        .expect("emitting")
        .map(|c| c.app.handle)
}

#[test]
fn a_pointer_handler_gets_the_local_position_through_the_router() {
    let h = handle(
        "Button",
        &[("on_mouse_down", "btn_down"), ("on_mouse_enter", "btn_in")],
    )
    .expect("a wired pointer event adds the router");
    assert!(
        h.contains("self.crawrad_pointer.route(event, &rects)"),
        "{h}"
    );
    assert!(h.contains("(0, CrawradPointer::Down)"), "{h}");
    assert!(
        h.contains("self.form.btn_down(MouseEvent { button: b, x: hit.x, y: hit.y"),
        "{h}"
    );
    assert!(h.contains("(0, CrawradPointer::Enter)"), "{h}");
    assert!(h.contains("self.form.btn_in()"), "{h}");
    assert!(
        h.contains("if self.form.ctl.enabled"),
        "a disabled control gets nothing: {h}"
    );
}

#[test]
fn the_click_comes_from_the_router_only_where_the_widget_has_none() {
    let shape = handle("Shape", &[("on_click", "shp_click")]).expect("a Shape reports no click");
    assert!(shape.contains("(0, CrawradPointer::Click)"), "{shape}");
    assert!(shape.contains("self.form.shp_click()"), "{shape}");
    // a Button says `clicked` itself: a second call would fire it twice (it is
    // in the router only for its Hint)
    let button = handle("Button", &[("on_click", "btn_click")]).unwrap_or_default();
    assert!(
        !button.contains("CrawradPointer::Click") && !button.contains("btn_click"),
        "the Button's own click must not be routed again: {button}"
    );
}

/// A control with no handler is in the router only for its Hint — the
/// tooltip shows on Enter and hides on Leave or a press; no handler is called.
#[test]
fn a_control_with_no_handler_gets_only_its_hint() {
    let h = handle("Button", &[]).expect("a Button has a Hint");
    assert!(
        h.contains("self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, hit.at"),
        "{h}"
    );
    assert!(
        h.contains("self.form.ctl.show_hint && !self.form.ctl.hint.is_empty()"),
        "{h}"
    );
    assert!(
        h.contains("self.ui.tooltip(id!(crawrad_tooltip)).hide(cx)"),
        "{h}"
    );
    assert!(
        !h.contains("self.form.ctl_") && !h.contains("crawrad_focus"),
        "no handler, no focus: {h}"
    );
}

#[test]
fn the_keyboard_goes_to_the_control_that_has_the_focus() {
    let h = handle(
        "Button",
        &[
            ("on_enter", "btn_in"),
            ("on_exit", "btn_out"),
            ("on_key_down", "btn_key"),
            ("on_key_press", "btn_char"),
        ],
    )
    .expect("a wired focus event adds the focus router");
    assert!(
        h.contains("self.crawrad_focus.route(event, &hits, &[0], crawrad_key_focus)")
            && h.contains("(0usize, self.ui.widget(id!(")
            && h.contains(".find(|(_, w)| w.key_focus(cx))"),
        "the Button can take the focus: {h}"
    );
    assert!(
        h.contains("(0, CrawradFocus::Enter)") && h.contains("self.form.btn_in()"),
        "{h}"
    );
    assert!(
        h.contains("(0, CrawradFocus::Exit)") && h.contains("self.form.btn_out()"),
        "{h}"
    );
    assert!(
        h.contains("self.form.btn_key(KeyEvent { key: crawrad_key(k)"),
        "{h}"
    );
    assert!(h.contains("self.form.btn_char(ch)"), "{h}");
}

#[test]
fn a_shape_cannot_take_the_focus() {
    // a Shape declares no OnEnter: a press on it is not a focus change
    let h = handle("Shape", &[("on_click", "shp_click")]).expect("routed click");
    assert!(!h.contains("crawrad_focus"), "{h}");
}
