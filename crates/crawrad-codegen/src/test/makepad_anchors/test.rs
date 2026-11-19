//! The Makepad target's anchors call: which controls follow the window,
//! with which rect and which anchors.

use super::anchors_call;
use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;
use std::collections::BTreeMap;

fn form() -> Document {
    let mut d = Document::new("form_main", "Form");
    let root = d.node_mut("form_main").unwrap();
    root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    root.set_name("form_main");
    d
}

fn add(d: &mut Document, parent: &str, class: &str, name: &str, props: &[(&str, Value)]) {
    let r = Registry::builtin();
    let mut n = r
        .instantiate(class, name, Rect::new(20.0, 30.0, 100.0, 24.0))
        .unwrap();
    n.set_name(name);
    for (k, v) in props {
        n.props.insert((*k).into(), v.clone());
    }
    d.add_child(parent, n).unwrap();
}

fn anchors(left: bool, top: bool, right: bool, bottom: bool) -> Value {
    let mut m = BTreeMap::new();
    for (k, v) in [
        ("left", left),
        ("top", top),
        ("right", right),
        ("bottom", bottom),
    ] {
        m.insert(k.to_string(), Value::Bool(v));
    }
    Value::Map(m)
}

fn call(d: &Document) -> String {
    let r = Registry::builtin();
    let emitted: Vec<_> = d.walk().into_iter().filter(|n| n.id != d.root).collect();
    anchors_call(&r, d, &emitted).unwrap()
}

#[test]
fn an_anchored_control_is_placed_on_every_geometry_change() {
    let mut d = form();
    add(
        &mut d,
        "form_main",
        "Edit",
        "edt",
        &[("anchors", anchors(true, true, true, true))],
    );
    let c = call(&d);
    assert!(
        c.contains("if let Event::WindowGeomChange(e) = event"),
        "{c}"
    );
    assert!(c.contains("dvec2(640.0, 420.0)"), "the design size:\n{c}");
    assert!(
        c.contains("place(cx, &self.ui.widget(id!(edt)), (20.0, 30.0, 100.0, 24.0), crawrad_runtime_makepad::anchors::Anchors { left: true, top: true, right: true, bottom: true }, grown)"),
        "{c}"
    );
}

#[test]
fn an_aligned_control_follows_by_the_anchors_its_alignment_implies() {
    let mut d = form();
    add(
        &mut d,
        "form_main",
        "Panel",
        "pnl",
        &[("align", Value::Enum("Bottom".into()))],
    );
    let c = call(&d);
    assert!(
        c.contains("Anchors { left: true, top: false, right: true, bottom: true }"),
        "{c}"
    );
}

#[test]
fn a_form_with_still_controls_carries_no_call() {
    let mut d = form();
    add(&mut d, "form_main", "Edit", "edt", &[]);
    add(
        &mut d,
        "form_main",
        "Timer",
        "tmr",
        &[("anchors", anchors(true, true, true, true))],
    );
    assert_eq!(call(&d), "");
}

#[test]
fn a_control_inside_a_container_goes_with_the_container() {
    let mut d = form();
    add(&mut d, "form_main", "Panel", "pnl", &[]);
    add(
        &mut d,
        "pnl",
        "Edit",
        "edt",
        &[("anchors", anchors(false, false, true, true))],
    );
    assert_eq!(call(&d), "");
}

/// **`place` reaches the widget that carries the rect**, for every class the
/// form draws: the instance named after the control is the one whose own
/// block holds `margin`, `width` and `height` (a `Label` is a `View` around
/// the label, and the `View` has the name). Moving any other widget would
/// leave the control where it was designed.
#[test]
fn every_drawn_class_carries_its_rect_on_the_widget_named_after_it() {
    let r = Registry::builtin();
    let mut checked = 0;
    for desc in r.all().filter(|d| d.emits_code) {
        let class = desc.class;
        if !desc.visual || desc.derived_geometry || class == "Form" {
            continue;
        }
        let mut d = form();
        let Some(mut n) = r.instantiate(class, "probe1", Rect::new(20.0, 30.0, 100.0, 24.0)) else {
            continue;
        };
        n.set_name("probe1");
        if d.add_child("form_main", n).is_err() {
            continue;
        }
        let node = d.node("probe1").unwrap();
        let dsl =
            crate::targets::makepad::components::emit_dsl(&d, &r, node, 20.0, 30.0, 100.0, 24.0)
                .unwrap();
        let start = dsl
            .find("probe1 = <")
            .unwrap_or_else(|| panic!("{class}: no widget named after the control:\n{dsl}"));
        // the widget's own block, up to its first nested widget
        let own = &dsl[start..];
        let own = &own[own.find('{').unwrap() + 1..];
        let own = own.split(" = <").next().unwrap();
        for key in ["margin: {left: 20", "width: 100", "height: 24"] {
            assert!(
                own.contains(key),
                "{class}: `{key}` is not on the named widget:\n{dsl}"
            );
        }
        checked += 1;
    }
    assert!(checked > 30, "only {checked} classes measured");
}
