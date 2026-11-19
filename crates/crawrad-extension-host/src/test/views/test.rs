//! The views store and the panel's rows: declared views only, trees
//! replaced and forgotten, the filter keeping the way to what matches.

use super::{ExtensionView, Installed, ItemPick, ViewPanel, ViewRow, ViewStore};
use crawrad_extension_api::views::{ItemLocation, SetTreeParams, ViewContribution, ViewItem};

fn leaf(id: &str, label: &str, kind: &str, line: u32) -> ViewItem {
    ViewItem {
        id: id.into(),
        label: label.into(),
        detail: None,
        kind: Some(kind.into()),
        location: Some(ItemLocation {
            file: "src/a.rs".into(),
            line,
        }),
        children: Vec::new(),
    }
}

fn branch(id: &str, label: &str, children: Vec<ViewItem>) -> ViewItem {
    ViewItem {
        id: id.into(),
        label: label.into(),
        detail: None,
        kind: None,
        location: None,
        children,
    }
}

fn todo_view() -> ExtensionView {
    ExtensionView {
        extension: "todo".into(),
        id: "todos".into(),
        title: "TODOs".into(),
        items: vec![
            branch(
                "a",
                "a.rs",
                vec![
                    leaf("1", "TODO one", "todo", 2),
                    leaf("2", "FIXME two", "fixme", 5),
                ],
            ),
            branch("b", "b.rs", vec![leaf("3", "HACK three", "hack", 1)]),
        ],
    }
}

fn labels(rows: &[ViewRow]) -> Vec<String> {
    rows.iter()
        .map(|r| match r {
            ViewRow::Header { title, count, .. } => format!("{title} ({count})"),
            ViewRow::Item { label, depth, .. } => format!("{}{label}", "  ".repeat(*depth)),
            ViewRow::Section { title, count } => format!("[{title} ({})]", count.unwrap_or(0)),
            ViewRow::Installed { name, version, .. } => format!("+ {name} {version}"),
        })
        .collect()
}

fn declared() -> Vec<ViewContribution> {
    vec![ViewContribution {
        id: "todos".into(),
        title: "TODOs".into(),
        icon: None,
    }]
}

#[test]
fn only_a_declared_view_takes_a_tree_and_forget_takes_it_away() {
    let mut s = ViewStore::default();
    let tree = SetTreeParams {
        view_id: "todos".into(),
        items: todo_view().items,
    };
    assert!(!s.set("todo", tree.clone()), "not declared yet");
    s.declare("todo", &declared());
    let r = s.revision();
    assert!(s.set("todo", tree));
    assert!(s.revision() > r);
    assert_eq!(s.views(), vec![todo_view()]);
    assert!(!s.set(
        "other",
        SetTreeParams {
            view_id: "todos".into(),
            items: Vec::new()
        }
    ));
    s.forget("todo");
    assert!(s.views().is_empty());
}

#[test]
fn no_filter_shows_everything_with_the_leaf_count() {
    let rows = ViewPanel::default().rows(&[todo_view()]);
    assert_eq!(
        labels(&rows),
        [
            "TODOs (3)",
            "a.rs",
            "  TODO one",
            "  FIXME two",
            "b.rs",
            "  HACK three"
        ]
    );
    assert_eq!(rows[0].template(), "ViewHeader");
    assert_eq!(rows[2].template(), "ViewItem");
}

#[test]
fn the_filter_keeps_the_way_to_a_match_and_a_matching_branch_brings_all() {
    let mut p = ViewPanel {
        filter: "  FixMe ".into(),
        ..ViewPanel::default()
    };
    assert_eq!(
        labels(&p.rows(&[todo_view()])),
        ["TODOs (1)", "a.rs", "  FIXME two"]
    );
    p.filter = "a.rs".into();
    assert_eq!(
        labels(&p.rows(&[todo_view()])),
        ["TODOs (2)", "a.rs", "  TODO one", "  FIXME two"]
    );
    p.filter = "nothing".into();
    assert_eq!(labels(&p.rows(&[todo_view()])), ["TODOs (0)"]);
}

#[test]
fn closing_hides_the_children_and_filtering_opens_them_again() {
    let view = todo_view();
    let mut p = ViewPanel::default();
    p.toggle_item(&view, "a");
    let rows = p.rows(std::slice::from_ref(&view));
    assert_eq!(labels(&rows), ["TODOs (3)", "a.rs", "b.rs", "  HACK three"]);
    assert!(matches!(
        &rows[1],
        ViewRow::Item {
            branch: Some(false),
            ..
        }
    ));
    p.filter = "todo".into();
    assert_eq!(
        labels(&p.rows(std::slice::from_ref(&view))),
        ["TODOs (1)", "a.rs", "  TODO one"]
    );
    p.filter.clear();
    p.toggle_view(&view);
    assert_eq!(labels(&p.rows(std::slice::from_ref(&view))), ["TODOs (3)"]);
    p.toggle_view(&view);
    p.toggle_item(&view, "a");
    assert_eq!(labels(&p.rows(&[view])).len(), 6);
}

#[test]
fn the_panel_follows_the_store_and_a_click_toggles_or_picks() {
    let mut store = ViewStore::default();
    store.declare("todo", &declared());
    let mut p = ViewPanel::default();
    assert!(p.follow(&store));
    assert!(!p.follow(&store), "same revision: nothing to draw again");
    store.set(
        "todo",
        SetTreeParams {
            view_id: "todos".into(),
            items: todo_view().items,
        },
    );
    assert!(p.follow(&store));
    assert_eq!(p.shown().len(), 6);
    // the branch: closes, and is still told to its extension
    let pick = p.click(1).expect("a branch is an item");
    assert_eq!((pick.item.as_str(), pick.location), ("a", None));
    assert_eq!(p.shown().len(), 4);
    // a leaf: picked with its place
    let leaf = p.click(3).expect("HACK three");
    assert_eq!(
        leaf,
        ItemPick {
            extension: "todo".into(),
            view: "todos".into(),
            item: "3".into(),
            location: Some(ItemLocation {
                file: "src/a.rs".into(),
                line: 1
            }),
        }
    );
    // the header: closes the view, picks nothing
    assert_eq!(p.click(0), None);
    assert_eq!(p.shown().len(), 1);
    assert_eq!(p.click(9), None);
}

#[test]
fn the_installed_section_closes_the_list_and_names_what_to_uninstall() {
    let mut p = ViewPanel::default();
    assert_eq!(p.shown().len(), 1, "nothing installed: only the hint");
    let todo = Installed {
        id: "acme.todo".into(),
        name: "TODO".into(),
        version: "0.1.0".into(),
    };
    assert!(p.set_installed(vec![todo.clone()]));
    assert!(
        !p.set_installed(vec![todo]),
        "the same list changes nothing"
    );
    p.views = vec![todo_view()];
    let rows = p.shown();
    assert_eq!(&labels(&rows)[6..], ["[Installed (1)]", "+ TODO 0.1.0"]);
    assert_eq!(rows[7].template(), "ViewInstalled");
    assert_eq!(p.uninstall_at(7).as_deref(), Some("acme.todo"));
    assert_eq!(p.uninstall_at(2), None);
    assert_eq!(p.click(6), None, "the section title is not a view");
    assert_eq!(p.click(7), None, "only the button uninstalls");
    assert!(p.set_installed(Vec::new()));
    assert_eq!(p.shown().len(), 6);
}

#[test]
fn an_empty_panel_says_how_to_install_and_a_filter_hides_that_hint() {
    let mut p = ViewPanel::default();
    assert_eq!(
        labels(&p.rows(&[])),
        ["[No extensions yet. Click + to add one (0)]"]
    );
    p.filter = "x".into();
    assert!(p.rows(&[]).is_empty());
}
