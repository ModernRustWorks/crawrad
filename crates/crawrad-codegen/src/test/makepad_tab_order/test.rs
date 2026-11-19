//! The Makepad target's Tab call (`targets/makepad/tab_order.rs`).

use super::*;
use crawrad_ir::{Rect, Value};

/// A form with `left` (TabOrder 2) and `right` (0) side by side, and a
/// panel (1) holding `inner` (0).
fn form() -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 300.0));
    let mut add = |parent: &str, class: &str, name: &str, order: i64, x: f32| {
        let mut n = r
            .instantiate(class, name, Rect::new(x, 10.0, 120.0, 60.0))
            .unwrap();
        n.set_name(name);
        n.props.insert("tab_order".into(), Value::Int(order));
        d.add_child(parent, n).unwrap();
    };
    add("form_main", "Edit", "left", 2, 10.0);
    add("form_main", "Edit", "right", 0, 300.0);
    add("form_main", "Panel", "box", 1, 150.0);
    add("box", "Edit", "inner", 0, 5.0);
    d
}

fn call() -> String {
    let d = form();
    let r = crawrad_registry::Registry::builtin();
    let emitted: Vec<&Node> = d.nodes.values().filter(|n| n.id != d.root).collect();
    tab_key_call(&r, &d, &emitted).unwrap()
}

#[test]
fn the_stops_come_in_tab_order_container_by_container() {
    let code = call();
    let at = |name: &str| {
        code.find(&format!("id!({name})"))
            .unwrap_or_else(|| panic!("`{name}` is no stop:\n{code}"))
    };
    assert!(
        at("right") < at("inner"),
        "right (0) before the panel (1)'s edit"
    );
    assert!(at("inner") < at("left"), "the panel's edit before left (2)");
    assert!(code.contains("// TabOrder &[1, 0]"), "{code}");
}

#[test]
fn a_container_is_no_stop() {
    assert!(
        !call().contains("id!(box)"),
        "the panel's view holds no keyboard"
    );
}

#[test]
fn a_form_with_no_control_that_takes_the_focus_has_no_call() {
    let d = Document::new("form_main", "Form");
    let r = crawrad_registry::Registry::builtin();
    assert_eq!(tab_key_call(&r, &d, &[]).unwrap(), "");
}

#[test]
fn page_control_with_tab_stop_enters_tab_order_addressing_active_tab() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut edit = r
        .instantiate("Edit", "edit1", Rect::new(10.0, 10.0, 100.0, 24.0))
        .unwrap();
    edit.set_name("edit1");
    edit.props.insert("tab_order".into(), Value::Int(0));
    d.add_child("form_main", edit).unwrap();

    let mut pc = r
        .instantiate(
            "PageControl",
            "page_control1",
            Rect::new(10.0, 50.0, 320.0, 200.0),
        )
        .unwrap();
    pc.set_name("page_control1");
    pc.props.insert("tab_order".into(), Value::Int(1));
    d.add_child("form_main", pc).unwrap();

    let mut t1 = r
        .instantiate("TabSheet", "tab1", Rect::new(0.0, 0.0, 320.0, 176.0))
        .unwrap();
    t1.set_name("tab1");
    d.add_child("page_control1", t1).unwrap();

    let mut t2 = r
        .instantiate("TabSheet", "tab2", Rect::new(0.0, 0.0, 320.0, 176.0))
        .unwrap();
    t2.set_name("tab2");
    d.add_child("page_control1", t2).unwrap();

    let emitted: Vec<&Node> = d.nodes.values().filter(|n| n.id != d.root).collect();
    let code = tab_key_call(&r, &d, &emitted).unwrap();

    let at_edit = code.find("id!(edit1)").expect("edit in tab stops");
    let at_pc = code
        .find("self.form.page_control1.active_page")
        .expect("page control addresses active page");
    assert!(at_edit < at_pc, "edit before page control");
    assert!(
        code.contains("page_control1.tabs_header.tab_0"),
        "references tab_0"
    );
    assert!(
        code.contains("page_control1.tabs_header.tab_1"),
        "references tab_1"
    );
}

#[test]
fn page_control_with_tab_stop_false_has_no_stop() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut pc = r
        .instantiate(
            "PageControl",
            "page_control1",
            Rect::new(10.0, 50.0, 320.0, 200.0),
        )
        .unwrap();
    pc.set_name("page_control1");
    pc.props.insert("tab_order".into(), Value::Int(0));
    pc.props.insert("tab_stop".into(), Value::Bool(false));
    d.add_child("form_main", pc).unwrap();

    let mut t1 = r
        .instantiate("TabSheet", "tab1", Rect::new(0.0, 0.0, 320.0, 176.0))
        .unwrap();
    t1.set_name("tab1");
    d.add_child("page_control1", t1).unwrap();

    let emitted: Vec<&Node> = d.nodes.values().filter(|n| n.id != d.root).collect();
    let code = tab_key_call(&r, &d, &emitted).unwrap();
    assert!(
        !code.contains("page_control1"),
        "no stop when tab_stop is false"
    );
}
