//! **The collection declares what the items are** — the `TCollection`/`TCollectionItem` of
//! ADR 0037.
//!
//! # What existed, and what was missing
//!
//! The mechanism was already here and nobody noticed: `ActionList` declares
//! `Children::Collection` — children in the model and tree, never on screen — and
//! `Action` is a class with its own properties living as its child.
//! This **is** a `TCollection` with a `TCollectionItem`.
//!
//! What was missing was the declaration. `accepts_children()` answered yes for the
//! collection, `Document::add_child` only checks duplicate id and existing parent, and
//! nothing in the repository said which class the items had to belong to. **A
//! `Button` could be a child of an `ActionList`.**
//!
//! # Why the class lives INSIDE the variant
//!
//! `Children::Collection { item }` instead of a neighbouring field: this way there is no way
//! to declare the collection and forget the item — whoever tries does not compile. It is the same
//! reason why `PropKind::Reference` carries the class it accepts.
//!
//! In a third-party component's manifest the form is different, and on purpose:
//! `children = "collection"` with `item_class = "..."` beside it, which is the grammar
//! that file already uses for `kind`. There validation enforces it, because
//! TOML has no compiler.

use crate::{Children, Registry};

/// `ActionList` declares the class of its items, and it exists in the catalog.
///
/// A collection pointing to a non-existent class is worse than one without
/// a class: it appears declared.
#[test]
fn every_collection_points_to_a_class_in_the_catalog() {
    let r = Registry::builtin();
    let mut collections = 0usize;
    for d in r.all() {
        if let Some(item) = d.item_class() {
            collections += 1;
            assert!(
                r.get(item).is_some(),
                "`{}` says its items are `{item}`, and `{item}` is not \
                 in the catalog",
                d.class
            );
        }
    }
    assert!(
        collections > 0,
        "no collections in catalog — sweep stopped measuring"
    );
}

/// **The enforcement.** What is not of the declared class does not enter the collection.
#[test]
fn collection_rejects_what_is_not_of_its_type() {
    let r = Registry::builtin();
    let acl = r.get("ActionList").expect("`ActionList` is in the catalog");

    assert!(
        acl.accepts_child("Action"),
        "`Action` is `ActionList`'s item and must enter"
    );
    assert!(
        !acl.accepts_child("Button"),
        "a `Button` inside an `ActionList` was accepted until 18/09/2026, and is \
         exactly what the declared class now prevents"
    );
    assert!(!acl.accepts_child("ActionList"), "nor itself");
}

/// Area continues accepting any control, and that is NOT an oversight.
///
/// A `Panel` receives whatever the user wants to put in it — that is what a visual
/// container is. The class rule applies to the collection, which is a typed list,
/// not to the surface.
#[test]
fn area_continues_accepting_any_control() {
    let r = Registry::builtin();
    let panel = r.get("Panel").expect("`Panel` is in the catalog");
    assert!(matches!(panel.children, Children::Area));
    for class in ["Button", "Label", "Panel", "ListBox"] {
        assert!(
            panel.accepts_child(class),
            "`Panel` must accept `{class}` in its area"
        );
    }
}

/// What accepts no children continues accepting none.
#[test]
fn leaf_accepts_no_one() {
    let r = Registry::builtin();
    let btn = r.get("Button").expect("`Button` is in the catalog");
    assert!(matches!(btn.children, Children::NoChildren));
    assert!(!btn.accepts_child("Label"));
    assert!(!btn.accepts_child("Action"));
}

/// The Inspector row of a child collection is named after its items.
#[test]
fn the_collection_label_is_the_item_without_the_owner_words() {
    let reg = Registry::builtin();
    let label = |class: &str| reg.get(class).and_then(|d| d.collection_label());
    assert_eq!(label("Grid").as_deref(), Some("Columns"));
    assert_eq!(label("StatusBar").as_deref(), Some("Panels"));
    assert_eq!(label("ActionList").as_deref(), Some("Actions"));
    assert_eq!(label("MainMenu").as_deref(), Some("Items"));
    assert_eq!(label("ListView").as_deref(), Some("Columns"));
    assert_eq!(label("GridColumn").as_deref(), Some("StyleRules"));
    assert_eq!(label("Button"), None);
}

/// `GridColumn` accepts `StyleRule` items as its collection children.
#[test]
fn grid_column_accepts_style_rule_collection_items() {
    let r = Registry::builtin();
    let col = r.get("GridColumn").expect("GridColumn is in the catalog");
    assert_eq!(col.item_class(), Some("StyleRule"));
    assert!(col.accepts_child("StyleRule"));
    assert!(!col.accepts_child("Button"));
    assert!(!col.accepts_child("GridColumn"));
    assert_eq!(col.collection_label().as_deref(), Some("StyleRules"));
}

/// `GridColumn` has the `OnDrawCell` event for custom cell rendering.
#[test]
fn grid_column_has_on_draw_cell_event() {
    let r = Registry::builtin();
    let col = r.get("GridColumn").expect("GridColumn is in the catalog");
    let ev = col
        .events
        .iter()
        .find(|e| e.name == "on_draw_cell")
        .expect("on_draw_cell event exists on GridColumn");
    assert_eq!(ev.display, "OnDrawCell");
}
