//! "Add all fields": one column per new field, typed defaults, one undo.

use super::*;
use crate::{Args, Author, Context, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Doc;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

fn form() -> Doc {
    let mut d = Document::new("form_main", "Form");
    let mut grid = Node::new("grid1", "Grid");
    grid.set_rect(Rect::new(0.0, 0.0, 300.0, 200.0));
    d.add_child("form_main", grid).unwrap();
    let mut button = Node::new("button1", "Button");
    button.set_rect(Rect::new(0.0, 210.0, 80.0, 24.0));
    d.add_child("form_main", button).unwrap();
    Doc::new(d)
}

fn ctx<'a>(doc: &'a Doc, reg: &'a Registry, clipboard: &'a Clipboard) -> Context<'a> {
    Context {
        doc: Some(doc),
        registry: reg,
        clipboard,
        args: Args::none(),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    }
}

fn columns(d: &Doc) -> Vec<&Node> {
    let ir = d.ir();
    ir.node("grid1")
        .unwrap()
        .children
        .iter()
        .filter_map(|id| ir.node(id))
        .collect()
}

#[test]
fn one_column_per_field_with_the_defaults_of_its_type() {
    let (mut d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let e = add_all_fields(
        &ctx(&d, &r, &a),
        "grid1",
        &["id", "balance", "customer_name"],
        &["INTEGER", "REAL", "TEXT"],
    );
    d.apply(e.transaction.expect("a transaction")).unwrap();
    let cols = columns(&d);
    assert_eq!(cols.len(), 3);
    assert!(cols.iter().all(|c| c.class == GRID_COLUMN_CLASS));
    let prop = |i: usize, k: &str| cols[i].props.get(k).cloned();
    assert_eq!(prop(0, "field"), Some(Value::Str("id".into())));
    assert_eq!(
        prop(0, "alignment"),
        Some(Value::Enum("taRightJustify".into()))
    );
    assert_eq!(prop(0, "width"), Some(Value::Int(70)));
    assert_eq!(
        prop(1, "display_format"),
        Some(Value::Str("#,##0.00".into()))
    );
    assert_eq!(
        prop(1, "alignment"),
        Some(Value::Enum("taRightJustify".into()))
    );
    assert_eq!(prop(2, "title"), Some(Value::Str("Customer name".into())));
    assert_eq!(
        prop(2, "alignment"),
        Some(Value::Enum("taLeftJustify".into()))
    );
    assert_eq!(prop(2, "width"), Some(Value::Int(0)));
    assert_eq!(prop(0, "field_type"), Some(Value::Str("INTEGER".into())));
    assert_eq!(prop(1, "field_type"), Some(Value::Str("REAL".into())));
    assert_eq!(prop(2, "field_type"), Some(Value::Str("TEXT".into())));
    // three distinct names
    let mut ids: Vec<_> = cols.iter().map(|c| c.id.clone()).collect();
    ids.dedup();
    assert_eq!(ids.len(), 3);
}

#[test]
fn boolean_field_gets_checkbox_renderer() {
    let (mut d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let e = add_all_fields(&ctx(&d, &r, &a), "grid1", &["is_active"], &["BOOLEAN"]);
    d.apply(e.transaction.expect("a transaction")).unwrap();
    let cols = columns(&d);
    assert_eq!(cols.len(), 1);
    assert_eq!(
        cols[0].props.get("renderer").cloned(),
        Some(Value::Enum("crCheckBox".into()))
    );
}

#[test]
fn pressing_it_again_adds_only_the_new_fields() {
    let (mut d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let e = add_all_fields(&ctx(&d, &r, &a), "grid1", &["id", "name"], &[]);
    d.apply(e.transaction.unwrap()).unwrap();
    let again = add_all_fields(&ctx(&d, &r, &a), "grid1", &["id", "name"], &[]);
    assert!(again.transaction.is_none(), "nothing new to add");
    let more = add_all_fields(&ctx(&d, &r, &a), "grid1", &["id", "name", "city"], &[]);
    d.apply(more.transaction.unwrap()).unwrap();
    assert_eq!(columns(&d).len(), 3);
}

#[test]
fn one_undo_takes_every_column_back() {
    let (mut d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let e = add_all_fields(&ctx(&d, &r, &a), "grid1", &["id", "name", "city"], &[]);
    d.apply(e.transaction.unwrap()).unwrap();
    assert_eq!(columns(&d).len(), 3);
    d.undo().unwrap();
    assert!(columns(&d).is_empty());
}

#[test]
fn a_component_without_grid_columns_is_refused() {
    let (d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let e = add_all_fields(&ctx(&d, &r, &a), "button1", &["id"], &[]);
    assert!(e.transaction.is_none());
    let e = add_all_fields(&ctx(&d, &r, &a), "grid1", &[], &[]);
    assert!(e.transaction.is_none());
}

/// `inspector.open_collection` opens the Grid's columns and refuses a Button.
#[test]
fn open_collection_takes_a_grid_and_refuses_a_button() {
    let (d, r, a) = (form(), Registry::builtin(), Clipboard::default());
    let run = |node: &str| {
        let action = crate::catalogue()
            .into_iter()
            .find(|x| x.id == "inspector.open_collection")
            .expect("the action is in the catalogue");
        let mut c = ctx(&d, &r, &a);
        c.args = Args::text("node", node.to_string());
        (action.execute)(&c)
    };
    assert_eq!(
        run("grid1").inspector,
        Some(crate::InspectorCommand::OpenCollection {
            node: "grid1".into()
        })
    );
    let refused = run("button1");
    assert!(refused.inspector.is_none());
    assert!(
        refused
            .status
            .is_some_and(|s| s.contains("no item collection"))
    );
}
