use std::collections::HashMap;

use makepad_widgets::splitter::{SplitterAlign, SplitterAxis};
use makepad_widgets::*;

use crate::dock::*;

fn splitter(a: LiveId, b: LiveId) -> DockItem {
    DockItem::Splitter {
        axis: SplitterAxis::Horizontal,
        align: SplitterAlign::Weighted(0.5),
        a,
        b,
    }
}

fn tab_group(tab: LiveId) -> DockItem {
    DockItem::Tabs {
        tabs: vec![tab],
        selected: 0,
        closable: true,
    }
}

fn tab_item(name: &str) -> DockItem {
    DockItem::Tab {
        name: name.into(),
        template: live_id!(PermanentTab),
        kind: live_id!(PanelComponents),
    }
}

fn minimal_arrangement(tab: LiveId) -> HashMap<LiveId, DockItem> {
    HashMap::from([
        (
            live_id!(root),
            DockItem::Tabs {
                tabs: vec![tab],
                selected: 0,
                closable: true,
            },
        ),
        (tab, tab_item("x")),
    ])
}

#[test]
fn saved_arrangement_roundtrips() {
    let original = minimal_arrangement(live_id!(tab_components));
    let decoded = decode(&encode(&original), |_| true).expect("arrangement did not decode");
    assert_eq!(original.len(), decoded.len());
    assert!(decoded.contains_key(&live_id!(root)));
    assert!(decoded.contains_key(&live_id!(tab_components)));
}

#[test]
fn file_carries_format_version() {
    let text = encode(&minimal_arrangement(live_id!(tab_design)));
    assert!(text.starts_with(&format!("crawrad-dock-layout {FORMAT_VERSION}")));
}

#[test]
fn unknown_version_is_discarded() {
    let text = encode(&minimal_arrangement(live_id!(tab_design)));
    let other_version = text.replacen("crawrad-dock-layout 1", "crawrad-dock-layout 99", 1);
    assert!(decode(&other_version, |_| true).is_none());
    assert!(decode("invalid marker\n{}", |_| true).is_none());
    assert!(decode("", |_| true).is_none());
}

#[test]
fn unknown_tab_discards_arrangement() {
    let text = encode(&minimal_arrangement(live_id!(tab_extinct)));
    assert!(decode(&text, |id| id != live_id!(tab_extinct)).is_none());
}

#[test]
fn arrangement_without_root_is_discarded() {
    let mut no_root = minimal_arrangement(live_id!(tab_chat));
    no_root.remove(&live_id!(root));
    assert!(decode(&encode(&no_root), |_| true).is_none());
}

#[test]
fn tab_names_registration_succeeds() {
    register_tab_names(["tab_test_a", "tab_test_b"]);
}

#[test]
fn with_root_restored_gives_root_to_surviving_sibling() {
    let center_group = live_id!(dock_center);
    let right_group = live_id!(dock_right);
    let tab_design = live_id!(tab_design);
    let tab_chat = live_id!(tab_chat);

    let without_left = HashMap::from([
        (live_id!(dock_rest), splitter(center_group, right_group)),
        (center_group, tab_group(tab_design)),
        (right_group, tab_group(tab_chat)),
        (tab_design, tab_item("design")),
        (tab_chat, tab_item("chat")),
    ]);

    let repaired =
        with_root_restored(&without_left).expect("the arrangement lost root and must get it back");
    match repaired.get(&live_id!(root)) {
        Some(DockItem::Splitter { a, b, .. }) => {
            assert_eq!((*a, *b), (center_group, right_group));
        }
        other => panic!("expected root splitter, got {other:?}"),
    }
    assert!(!repaired.contains_key(&live_id!(dock_rest)));
    assert_eq!(repaired.len(), without_left.len());
}

#[test]
fn with_root_restored_single_tabs_becomes_root() {
    let center_group = live_id!(dock_center);
    let tab_design = live_id!(tab_design);
    let center_only = HashMap::from([
        (center_group, tab_group(tab_design)),
        (tab_design, tab_item("design")),
    ]);
    let repaired = with_root_restored(&center_only).expect("root is restored");
    assert!(matches!(
        repaired.get(&live_id!(root)),
        Some(DockItem::Tabs { .. })
    ));
}

#[test]
fn with_root_restored_intact_arrangement_untouched() {
    let intact = minimal_arrangement(live_id!(tab_design));
    assert!(with_root_restored(&intact).is_none());
    assert!(with_root_restored(&HashMap::new()).is_none());
}

#[test]
fn with_root_restored_ambiguous_two_orphans_returns_none() {
    let center_group = live_id!(dock_center);
    let right_group = live_id!(dock_right);
    let ambiguous = HashMap::from([
        (live_id!(dock_rest), splitter(center_group, right_group)),
        (live_id!(dock_left), tab_group(live_id!(tab_components))),
    ]);
    assert!(with_root_restored(&ambiguous).is_none());
}
