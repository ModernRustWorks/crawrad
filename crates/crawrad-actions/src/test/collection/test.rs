//! **The collection refuses what is not its own, and provides a way to receive what is.**
//!
//! Two halves of a single defect, measured on 18/09/2026 and described in
//! ADR 0037.
//!
//! # The first half: the rule had no consumer
//!
//! `Filhos::NaColecao { item }` declared the class of the items, and
//! `ComponentDesc::aceita_filho` answered — but **nobody asked**.
//! `component.add` checked that the class exists and that the parent
//! exists, and nothing else; `Document::add_child` looks at duplicate id and existing
//! parent. A `Button` entered an `ActionList` and the document accepted it.
//!
//! A rule without a consumer is a guard that never measures, and the place to ask is this:
//! the single mutation entry point of ADR 0005.
//!
//! # The second half: there was no way to CREATE an item
//!
//! `Action` has `palette: ""` on purpose — *"actions are born inside a
//! list, not loose on the form"* — and the only path that created a node was
//! dropping from the palette. Combining both: **the class existed, the collection existed, and
//! the user had no way to place an item inside.**
//!
//! `component.add_item` is the gesture, and it knows the name of no class:
//! it asks the parent's descriptor for the class of the items. A third-party
//! grid declaring `item_class = "Column"` gains this without a single line
//! written for it.

use crawrad_doc::{Author, Command, Doc};
use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;

use crate::{Args, Clipboard, Context, Effect, Project, catalogue};

/// A document with an `ActionList` and a `Button` on the form.
fn scenario() -> (Doc, Registry, Clipboard) {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
    }
    let acl = r
        .instantiate(
            "ActionList",
            "acl1".to_string(),
            Rect::new(8.0, 8.0, 28.0, 28.0),
        )
        .expect("catalogue instantiates what it lists");
    d.add_child("form_main", acl).expect("enters the form");
    let btn = r
        .instantiate(
            "Button",
            "btn1".to_string(),
            Rect::new(8.0, 50.0, 80.0, 24.0),
        )
        .expect("instantiate");
    d.add_child("form_main", btn).expect("enters the form");
    (Doc::new(d), r, Clipboard::default())
}

fn execute_action(id: &str, args: Args) -> Effect {
    let (doc, reg, clipboard) = scenario();
    let action = catalogue()
        .into_iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("action `{id}` is in the catalogue"));
    let ctx = Context {
        doc: Some(&doc),
        registry: &reg,
        clipboard: &clipboard,
        args,
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    (action.execute)(&ctx)
}

/// **The refusal.** A `Button` does not enter an `ActionList`.
#[test]
fn the_collection_refuses_the_wrong_class() {
    let e = execute_action(
        "component.add",
        Args::one("class", Value::Str("Button".into())).and("parent", Value::Str("acl1".into())),
    );
    assert!(
        e.transaction.is_none(),
        "the `Button` entered the `ActionList` — the rule of ADR 0037 has no \
         consumer in `component.add`"
    );
    let status = e.status.unwrap_or_default();
    assert!(
        status.contains("Action"),
        "the refusal must state WHAT the collection accepts, and stated: {status:?}"
    );
}

/// The declared class enters.
#[test]
fn the_collection_accepts_the_class_it_declares() {
    let e = execute_action(
        "component.add",
        Args::one("class", Value::Str("Action".into())).and("parent", Value::Str("acl1".into())),
    );
    assert!(
        e.transaction.is_some(),
        "`Action` is the item of `ActionList` and must enter: {:?}",
        e.status
    );
}

/// **The gesture.** The item is created without anyone writing the class name.
#[test]
fn add_item_derives_the_class_from_the_collection() {
    let e = execute_action(
        "component.add_item",
        Args::one("parent", Value::Str("acl1".into())),
    );
    let tx = e
        .transaction
        .clone()
        .unwrap_or_else(|| panic!("`add_item` should have created the item: {:?}", e.status));
    let created = tx.commands.iter().any(|c| {
        matches!(c, Command::AddNode { node, parent, .. }
            if node.class == "Action" && parent == "acl1")
    });
    assert!(
        created,
        "`add_item` must create an `Action` inside `acl1`, and created: {:?}",
        tx.commands
    );
}

/// Non-collections refuse the gesture, with the explanation provided.
#[test]
fn add_item_refuses_what_is_not_a_collection() {
    let e = execute_action(
        "component.add_item",
        Args::one("parent", Value::Str("btn1".into())),
    );
    assert!(e.transaction.is_none(), "a `Button` has no collection");
    assert!(
        e.status.unwrap_or_default().contains("collection"),
        "the refusal must state why"
    );
}
