//! Dock layout persistence and recovery helpers for Makepad.
//!
//! Provides serialization, deserialization, and hierarchy repair for
//! Makepad's `Dock` arrangements.

use std::collections::HashMap;

use crawrad_note::note;
use makepad_widgets::makepad_micro_serde::{DeRon, SerRon};
use makepad_widgets::splitter::{SplitterAlign, SplitterAxis};
use makepad_widgets::*;

#[note("dock-file-name")]
pub const FILE_NAME: &str = "makepad-dock-layout.ron";

#[note("dock-format-version")]
pub const FORMAT_VERSION: u32 = 1;

#[note("dock-marker")]
pub fn marker() -> String {
    format!("crawrad-dock-layout {FORMAT_VERSION}")
}

#[note("dock-encode")]
pub fn encode(items: &HashMap<LiveId, DockItem>) -> String {
    format!("{}\n{}\n", marker(), items.serialize_ron())
}

#[note("dock-with-root-restored")]
pub fn with_root_restored(items: &HashMap<LiveId, DockItem>) -> Option<HashMap<LiveId, DockItem>> {
    let root = live_id!(root);
    if items.is_empty() || items.contains_key(&root) {
        return None;
    }
    let children: Vec<LiveId> = items
        .values()
        .flat_map(|item| match item {
            DockItem::Splitter { a, b, .. } => vec![*a, *b],
            _ => vec![],
        })
        .collect();
    let mut orphans = items
        .iter()
        .filter(|(id, item)| !matches!(item, DockItem::Tab { .. }) && !children.contains(id));
    let (orphan, _) = orphans.next()?;
    if orphans.next().is_some() {
        return None;
    }
    let mut repaired = items.clone();
    let item = repaired.remove(&orphan.clone())?;
    repaired.insert(root, item);
    Some(repaired)
}

#[note("dock-decode")]
pub fn decode(
    text: &str,
    is_valid_tab: impl Fn(LiveId) -> bool,
) -> Option<HashMap<LiveId, DockItem>> {
    let (head, body) = text.split_once('\n')?;
    if head.trim() != marker() {
        return None;
    }
    let items: HashMap<LiveId, DockItem> = DeRon::deserialize_ron(body).ok()?;
    if !items.contains_key(&live_id!(root)) {
        return None;
    }
    for (id, item) in &items {
        if matches!(item, DockItem::Tab { .. }) && !is_valid_tab(*id) {
            return None;
        }
    }
    Some(items)
}

#[note("dock-register-tab-names")]
pub fn register_tab_names<I, S>(names: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for name in names {
        let name_str = name.as_ref();
        assert!(
            LiveId::from_str_with_lut(name_str).is_ok(),
            "the tab id `{name_str}` collided with another already registered name"
        );
    }
}

#[cfg(test)]
#[path = "test/dock/test.rs"]
mod tests;

#[note("dock-side-by-side")]
pub fn side_by_side(a: LiveId, b: LiveId, align: SplitterAlign) -> DockItem {
    DockItem::Splitter {
        axis: SplitterAxis::Horizontal,
        align,
        a,
        b,
    }
}
