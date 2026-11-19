//! The tests of the Inspector model: which rows a selection shows.

use super::*;
use crawrad_ir::{Node, Rect};

#[test]
fn a_target_prefixed_row_shows_only_for_its_target() {
    let mut reg = Registry::builtin();
    let mut desc = reg.get("Button").expect("Button is built in").clone();
    let mut shadow_prop = desc
        .props
        .iter()
        .find(|p| p.key == "enabled")
        .expect("Button has enabled prop")
        .clone();
    shadow_prop.key = "makepad.shadow";
    shadow_prop.display = "Shadow";
    desc.props.push(shadow_prop);
    reg.register(desc);

    let mut d = Document::new("form_main", "Form");
    let mut btn = Node::new("Button1", "Button");
    btn.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    d.add_child("form_main", btn).unwrap();

    let none = std::collections::BTreeSet::new();
    let sel = ["Button1".to_string()];

    let rows_makepad = rows(
        &reg,
        Some(&d),
        &sel,
        "makepad",
        Tab::Properties,
        &none,
        &none,
    );
    let rows_egui = rows(&reg, Some(&d), &sel, "egui", Tab::Properties, &none, &none);

    let has_prop = |rows: &[Row], key: &str| {
        rows.iter()
            .any(|r| matches!(r, Row::Prop(p) if p.key == key))
    };

    assert!(
        has_prop(&rows_makepad, "makepad.shadow"),
        "makepad.shadow must be present for makepad"
    );
    assert!(
        !has_prop(&rows_egui, "makepad.shadow"),
        "makepad.shadow must be absent for egui"
    );
    assert!(
        has_prop(&rows_makepad, "text"),
        "text must be present for makepad"
    );
    assert!(
        has_prop(&rows_egui, "text"),
        "text must be present for egui"
    );
}

/// A flat child collection shows `Columns (N) …` in Identity, and the `…`
/// runs the action that opens its dialog.
#[test]
fn a_grid_shows_its_columns_row_and_the_dots_open_the_dialog() {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let mut grid = Node::new("Grid1", "Grid");
    grid.set_rect(Rect::new(0.0, 0.0, 300.0, 200.0));
    d.add_child("form_main", grid).unwrap();
    for id in ["Col1", "Col2"] {
        d.add_child("Grid1", Node::new(id, "GridColumn")).unwrap();
    }
    let none = std::collections::BTreeSet::new();
    let sel = ["Grid1".to_string()];
    let out = rows(
        &reg,
        Some(&d),
        &sel,
        "makepad",
        Tab::Properties,
        &none,
        &none,
    );
    let Some(Row::Prop(row)) = out
        .iter()
        .find(|r| matches!(r, Row::Prop(p) if p.key == CHILD_COLLECTION_KEY))
    else {
        panic!("the Grid must show its Columns row");
    };
    assert_eq!(row.label, "Columns");
    assert!(
        row.editor
            .cells()
            .iter()
            .any(|c| matches!(c, Cell::Caption { text, .. } if text == "(2)"))
    );
    assert!(matches!(
        row.editor.gesture(Slot::Button, Gesture::Pressed),
        Outcome::Run(ACTION_OPEN_COLLECTION, _)
    ));
}

/// A menu's items nest, so the tree editor stays theirs: no flat row.
#[test]
fn a_menu_has_no_flat_collection_row() {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("MainMenu1", "MainMenu"))
        .unwrap();
    let none = std::collections::BTreeSet::new();
    let sel = ["MainMenu1".to_string()];
    let out = rows(
        &reg,
        Some(&d),
        &sel,
        "makepad",
        Tab::Properties,
        &none,
        &none,
    );
    assert!(
        !out.iter()
            .any(|r| matches!(r, Row::Prop(p) if p.key == CHILD_COLLECTION_KEY))
    );
}

/// A Grid bound to a Dataset hides `TableName` (the dataset brings its own
/// query); bound to a Connection it keeps it. Decided by the capability the
/// referenced component answers to, never by its class name.
#[test]
fn table_name_hides_while_the_grid_points_at_a_dataset() {
    let mut reg = Registry::builtin();
    let base = reg.get("Timer").expect("Timer is built in").clone();
    for (class, provides) in [
        ("Dataset", &["DbDataset", "DbRowSource"][..]),
        ("Connection", &["DbConnection", "DbRowSource"][..]),
    ] {
        let mut desc = base.clone();
        desc.class = class;
        desc.provides = provides;
        reg.register(desc);
    }
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();
    d.add_child("form_main", Node::new("conn1", "Connection"))
        .unwrap();
    let mut grid = Node::new("Grid1", "Grid");
    grid.set_rect(Rect::new(0.0, 0.0, 300.0, 200.0));
    d.add_child("form_main", grid).unwrap();
    let none = std::collections::BTreeSet::new();
    let sel = ["Grid1".to_string()];
    let shows_table = |d: &Document| {
        rows(
            &reg,
            Some(d),
            &sel,
            "makepad",
            Tab::Properties,
            &none,
            &none,
        )
        .iter()
        .any(|r| matches!(r, Row::Prop(p) if p.key == "table_name"))
    };
    assert!(shows_table(&d), "an unbound grid shows TableName");
    // the reference itself accepts both by capability, and refuses a Timer
    d.add_child("form_main", Node::new("tmr1", "Timer"))
        .unwrap();
    let source = reg
        .get("Grid")
        .unwrap()
        .prop("data_source")
        .unwrap()
        .clone();
    for (id, ok) in [("dst1", true), ("conn1", true), ("tmr1", false)] {
        let got = crate::coerce::coerce_to_property(&Value::Str(id.into()), &source, &d, &reg);
        assert_eq!(got.is_ok(), ok, "{id}: {got:?}");
    }
    d.node_mut("Grid1")
        .unwrap()
        .props
        .insert("data_source".into(), Value::Ref("conn1".into()));
    assert!(shows_table(&d), "a grid on a Connection shows TableName");
    d.node_mut("Grid1")
        .unwrap()
        .props
        .insert("data_source".into(), Value::Ref("dst1".into()));
    assert!(!shows_table(&d), "a grid on a Dataset hides TableName");
}

/// A Button linked to an Action shows the ACTION's caption on the canvas and
/// in the app, so its Caption row shows that same text, read-only.
#[test]
fn a_button_with_an_action_shows_the_action_caption() {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.add_child(
        "form_main",
        Node::new("act1", "Action").with_prop("text", Value::Str("Save".into())),
    )
    .unwrap();
    let mut btn = Node::new("Button1", "Button")
        .with_prop("text", Value::Str("button1".into()))
        .with_prop("action", Value::Ref("act1".into()));
    btn.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    d.add_child("form_main", btn).unwrap();
    let none = std::collections::BTreeSet::new();
    let out = rows(
        &reg,
        Some(&d),
        &["Button1".to_string()],
        "makepad",
        Tab::Properties,
        &none,
        &none,
    );
    let caption = out
        .iter()
        .find_map(|r| match r {
            Row::Prop(p) if p.key == "text" => Some(p),
            _ => None,
        })
        .expect("Caption row");
    assert_eq!(caption.editor.template(), "RowMute");
    let cells = caption.editor.cells();
    assert!(cells.contains(&Cell::Caption {
        slot: Slot::Value,
        text: "Save".into()
    }));
    assert!(cells.contains(&Cell::Caption {
        slot: Slot::Note,
        text: LINKED_TO_ACTION.into()
    }));
    // the canvas resolves it the same way
    assert_eq!(d.linked_action_text("Button1"), Some("Save"));
}

/// A non-visual component (an ActionList) shows no `rect` and no Layout header:
/// its rect only places the icon. A visual one (a Button) keeps both.
#[test]
fn a_non_visual_component_shows_no_rect() {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("action_list1", "ActionList"))
        .unwrap();
    d.add_child("form_main", Node::new("button1", "Button"))
        .unwrap();
    let none = std::collections::BTreeSet::new();
    let shows = |id: &str| {
        let sel = [id.to_string()];
        let out = rows(
            &reg,
            Some(&d),
            &sel,
            "makepad",
            Tab::Properties,
            &none,
            &none,
        );
        let rect = out
            .iter()
            .any(|r| matches!(r, Row::Prop(p) if p.key == "rect"));
        let layout = out.iter().any(
            |r| matches!(r, Row::Category { label, .. } if *label == Category::Layout.label()),
        );
        (rect, layout)
    };
    assert_eq!(shows("action_list1"), (false, false));
    assert_eq!(shows("button1"), (true, true));
}
