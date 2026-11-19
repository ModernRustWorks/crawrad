//! **A class that declares content shows that content on the canvas**.
//!
//! The content of a component lives in a different property depending on the
//! class: a `Memo` keeps its lines in `text`, a `ListBox` and a `TreeView`
//! keep theirs in `items`. That asymmetry already cost a defect the owner saw
//! on screen: the canvas read `items` for the `Memo` by analogy, and a memo
//! with 12 lines came out as a blank box. It passed `cargo check`, clippy and
//! every test.
//!
//! The model keeps its shape (it is a format decision, with `.crawform` files
//! already saved). What changes is how the mistake shows up: red here, and
//! not blank on screen. The content is read from what the REGISTRY declares
//! (its multiline text and its item collections), so a new class with content
//! is measured the day it enters the catalogue.

use crawrad_ir::{Document, Item, Rect, Value};
use crawrad_registry::{PropDesc, PropKind, Registry};

use crate::MakepadRuntime;
use crate::components::desenho_de;
use crawrad_runtime::Runtime;

/// The texts a property declares as content, from its catalogue default.
fn declared_content(p: &PropDesc) -> Vec<String> {
    fn flatten(items: &[Item], out: &mut Vec<String>) {
        for i in items {
            out.push(i.text.clone());
            flatten(&i.children, out);
        }
    }
    match (&p.kind, &p.default) {
        (PropKind::Multiline, Value::Str(s)) => s.lines().map(str::to_string).collect(),
        (PropKind::Collection { .. }, v) => {
            let mut out = Vec::new();
            flatten(&Item::list_from(v), &mut out);
            out
        }
        _ => Vec::new(),
    }
}

/// The lines the canvas draws for `class`, instantiated from the catalogue.
fn drawn_lines(reg: &Registry, class: &str) -> Option<Vec<String>> {
    let mut doc = Document::new("form", "Form");
    if let Some(form) = doc.node_mut("form") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }
    let node = reg.instantiate(class, "probe", Rect::new(10.0, 10.0, 200.0, 200.0))?;
    doc.add_child("form", node).ok()?;
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).ok()?;
    let c = rt.ctl("probe")?;
    Some(
        desenho_de(class)
            .lines(c)
            .into_iter()
            .map(|l| l.text)
            .collect(),
    )
}

#[test]
fn every_class_that_draws_lines_draws_the_content_it_declares() {
    let reg = Registry::builtin();
    let mut checked = Vec::new();
    let mut blank = Vec::new();
    for desc in reg.all() {
        let content: Vec<String> = desc.props.iter().flat_map(declared_content).collect();
        if content.is_empty() {
            continue;
        }
        let Some(lines) = drawn_lines(&reg, desc.class) else {
            continue;
        };
        // A class that draws no lines at all paints its content some other
        // way (a ComboBox shows one item, a RadioGroup its buttons); what is
        // measured here is the class that DOES draw lines and draws the
        // wrong ones.
        if lines.is_empty() {
            continue;
        }
        checked.push(desc.class);
        let missing: Vec<&String> = content.iter().filter(|t| !lines.contains(t)).collect();
        if !missing.is_empty() {
            blank.push(format!("{}: {missing:?} not drawn", desc.class));
        }
    }
    for class in ["Memo", "ListBox", "TreeView"] {
        assert!(
            checked.contains(&class),
            "`{class}` drew no lines from its declared content: {checked:?}"
        );
    }
    assert!(
        blank.is_empty(),
        "declared content left off the canvas: {blank:?}"
    );
}

#[test]
fn collapsed_tree_branch_hides_its_children_in_lines() {
    let item_open = Item::new("OpenLeaf");
    let item_sub = Item::new("HiddenSub");
    let mut branch_collapsed = Item::with_children("CollapsedBranch", vec![item_sub]);
    branch_collapsed.expanded = false;

    let mut doc = Document::new("form", "Form");
    let mut node = crawrad_ir::Node::new("tv1", "TreeView");
    node.props.insert(
        "items".to_string(),
        Item::list_to(&[branch_collapsed, item_open]),
    );
    doc.add_child("form", node).unwrap();

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).unwrap();
    let c = rt.ctl("tv1").unwrap();
    let lines = desenho_de("TreeView").lines(c);

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].text, "CollapsedBranch");
    assert!(lines[0].branch);
    assert!(!lines[0].expanded);
    assert_eq!(lines[1].text, "OpenLeaf");
    assert!(!lines[1].branch);
}

#[test]
fn tree_view_canvas_lines_include_symbol_from_linked_image_list() {
    let mut doc = Document::new("form", "Form");

    let mut il = crawrad_ir::Node::new("il1", "ImageList");
    il.set_name("il1");
    let mut item0 = std::collections::BTreeMap::new();
    item0.insert("name".to_string(), Value::Str("folder".into()));
    item0.insert("symbol".to_string(), Value::Str("📁".into()));
    let mut item1 = std::collections::BTreeMap::new();
    item1.insert("name".to_string(), Value::Str("file".into()));
    item1.insert("symbol".to_string(), Value::Str("📄".into()));
    il.props.insert(
        "images_data".into(),
        Value::List(vec![Value::Map(item0), Value::Map(item1)]),
    );
    doc.add_child("form", il).unwrap();

    let mut item_with_icon = Item::new("Folder");
    item_with_icon.image_index = 0;
    let mut sub_item = Item::new("Doc");
    sub_item.image_index = 1;
    item_with_icon.children.push(sub_item);

    let mut item_no_icon = Item::new("NoIcon");
    item_no_icon.image_index = -1;

    let mut item_oob = Item::new("OutOfBounds");
    item_oob.image_index = 99;

    let mut tv = crawrad_ir::Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.props.insert(
        "items".to_string(),
        Item::list_to(&[item_with_icon, item_no_icon, item_oob]),
    );
    tv.props
        .insert("images".to_string(), Value::Ref("il1".into()));
    doc.add_child("form", tv).unwrap();

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).unwrap();
    let c = rt.ctl("tv1").unwrap();
    let lines = desenho_de("TreeView").lines(c);

    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].text, "📁 Folder");
    assert_eq!(lines[1].text, "📄 Doc");
    assert_eq!(lines[2].text, "NoIcon");
    assert_eq!(lines[3].text, "OutOfBounds");

    // Updating the ImageList and touching it re-synchronises TreeView symbols
    let il_mut = doc.node_mut("il1").unwrap();
    let mut new_item0 = std::collections::BTreeMap::new();
    new_item0.insert("name".to_string(), Value::Str("folder".into()));
    new_item0.insert("symbol".to_string(), Value::Str("🗂️".into()));
    il_mut.props.insert(
        "images_data".into(),
        Value::List(vec![Value::Map(new_item0)]),
    );

    let mut touched = std::collections::BTreeSet::new();
    touched.insert("il1".to_string());
    rt.update(&doc, &touched).unwrap();

    let c_updated = rt.ctl("tv1").unwrap();
    let lines_updated = desenho_de("TreeView").lines(c_updated);
    assert_eq!(lines_updated[0].text, "🗂️ Folder");
    assert_eq!(lines_updated[1].text, "Doc");
}
