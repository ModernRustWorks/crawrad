//! `form.read`'s tree: the root apart, every child with its parent.

use super::{component, components, form, selection};
use crawrad_ir::{Document, Node, Rect};

#[test]
fn the_tree_names_every_node_and_its_parent() {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut b = Node::new("Button1", "Button");
    b.set_rect(Rect::new(10.0, 20.0, 96.0, 32.0));
    b.props
        .insert("caption".into(), crawrad_ir::Value::Str("OK".into()));
    d.add_child("form_main", b).expect("a child");
    let (json, count) = form(&crawrad_doc::Doc::new(d));

    assert_eq!(count, 1, "the root is not counted among the nodes");
    assert_eq!(json["revision"], 0);
    assert_eq!(json["root"]["id"], "form_main");
    assert_eq!(json["root"]["children"][0], "Button1");
    let button = &json["nodes"][0];
    assert_eq!(button["class"], "Button");
    assert_eq!(button["parent"], "form_main");
    assert_eq!(button["rect"]["x"], 10.0);
    assert_eq!(button["rect"]["w"], 96.0);
    assert!(button["properties"].get("caption").is_some());
}

/// The designer's reads: the components with their parent, the selection,
/// one component by id or by name.
#[test]
fn the_designer_reads() {
    let mut d = Document::new("form_main", "Form");
    let mut b = Node::new("Button1", "Button");
    b.set_rect(Rect::new(1.0, 2.0, 3.0, 4.0));
    d.add_child("form_main", b).expect("a child");
    let mut doc = crawrad_doc::Doc::new(d);
    let list = components(&doc);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["parent"], "form_main");
    assert_eq!(list[0]["rect"]["h"], 4.0);
    assert_eq!(selection(&doc)["count"], 0);
    doc.select_one("Button1");
    assert_eq!(selection(&doc)["selected_components"][0]["class"], "Button");
    let (found, name) = component(&doc, "Button1").expect("by id");
    assert_eq!(found["class"], "Button");
    assert_eq!(name, "Button1");
    assert!(component(&doc, "Nope").is_none());
}

/// A bound property's value in `properties` is not what the app shows: the
/// answer carries the binding, so an agent reads the same thing the Inspector
/// says.
#[test]
fn a_component_answer_carries_its_bindings() {
    let mut d = Document::new("form_main", "Form");
    let mut e = Node::new("Edit1", "Edit");
    e.set_binding(
        "text",
        crawrad_ir::BindingDef::new("dst1", crawrad_ir::BindingMode::OneWay).with_field("name"),
    );
    d.add_child("form_main", e).expect("a child");
    d.add_child("form_main", Node::new("Edit2", "Edit"))
        .expect("a child");
    let doc = crawrad_doc::Doc::new(d);

    let (bound, _) = component(&doc, "Edit1").expect("by id");
    assert_eq!(bound["bindings"]["text"]["source"], "dst1");
    assert_eq!(bound["bindings"]["text"]["field"], "name");
    assert_eq!(bound["bindings"]["text"]["mode"], "one_way");
    let (free, _) = component(&doc, "Edit2").expect("by id");
    assert_eq!(free["bindings"], serde_json::json!({}));
}
