//! The headless **ActionList Editor** and **Standard Action Classes**.
//!
//! The two windows of the classic `TActionList`: the one that lists the
//! existing actions, grouped by category, and the one offering the standard
//! classes to add.
//!
//! # Why it lives in `crawrad-actions`
//!
//! It is a headless model, and without a screen it is not `crawrad-ide` business
//! (ADR 0043): it moved out of there. The shell keeps asking by
//! the same name (`crate::action_list::model`), and the category filter and
//! the label of each action remain written only once.
//!
//! # What is NOT here
//!
//! What the standard action classes list contains: that belongs to
//! [`crawrad_registry::standard_actions_by_category`], and continues to be. This
//! file only FLATTENS it for whoever draws a flat list — Makepad's `PortalList`
//! asks for item index `i`, and someone had to have decided beforehand what
//! item `i` is.

use crate::Args;
pub use crate::grid_columns::ACTION_ADD_ALL_FIELDS;
use crawrad_ir::{Document, Value};
use crawrad_note::note;
use crawrad_registry::Registry;

#[note("action-list-action-new")]
pub const ACTION_NEW: &str = "action.new";
pub const ACTION_STANDARD: &str = "action.standard";
pub const ACTION_DELETE: &str = "edit.delete";
#[note("action-list-action-reorder")]
pub const ACTION_REORDER: &str = "component.reorder";

pub const CLASSE_ACTION_LIST: &str = "ActionList";
pub const EDITANDO: &str = "Editing";
pub const ACT_CLOSE: &str = "Close";

/// The ActionList strip's labels: new, standard, delete, up, down, fields.
/// An empty label hides that button.
pub const ACTION_LIST_BUTTONS: [&str; 6] = [
    "New Action",
    "Standard...",
    "Delete",
    "Move Up",
    "Move Down",
    "",
];
/// A typed collection's strip labels, same order as [`ACTION_LIST_BUTTONS`].
pub const COLLECTION_BUTTONS: [&str; 6] = [
    "Add",
    "",
    "Delete",
    "Move up",
    "Move down",
    "Add all fields",
];
const COLLECTION_FOOTER: &str =
    "Only the items. Click one: its properties open in the Object Inspector.";

#[note("action-list-cat-all")]
pub const CAT_ALL: &str = "(All)";
#[note("action-list-cat-none")]
pub const CAT_NONE: &str = "(No Category)";

#[note("action-list-categories")]
pub fn categories(doc: &Document, list: &str) -> Vec<String> {
    let mut out = vec![CAT_ALL.to_string(), CAT_NONE.to_string()];
    for id in children(doc, list) {
        let Some(cat) = category_of(doc, &id) else {
            continue;
        };
        if !cat.is_empty() && !out.contains(&cat) {
            out.push(cat);
        }
    }
    out
}

#[note("action-list-actionrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRow {
    /// The stable identity (ADR 0008) — addresses, does not appear.
    pub id: String,
    /// What is read: `edit_copy1  ·  &Copy`.
    pub label: String,
}

#[note("action-list-actions")]
pub fn actions(doc: &Document, list: &str, category: &str) -> Vec<ActionRow> {
    let mut out = Vec::new();
    for id in children(doc, list) {
        let cat = category_of(doc, &id).unwrap_or_default();
        let cat = if cat.is_empty() {
            CAT_NONE.to_string()
        } else {
            cat
        };
        if category != CAT_ALL && category != cat {
            continue;
        }
        let Some(node) = doc.node(&id) else { continue };
        let caption = match node.props.get("text") {
            Some(Value::Str(s)) => s.clone(),
            _ => id.clone(),
        };
        out.push(ActionRow {
            label: format!("{}  ·  {caption}", node.name()),
            id,
        });
    }
    out
}

#[note("action-list-position")]
pub fn position(doc: &Document, list: &str, action: &str) -> Option<usize> {
    children(doc, list).iter().position(|c| c == action)
}

#[note("action-list-selected")]
pub fn selected(doc: &Document, list: &str, selection: &[String]) -> Option<String> {
    let id = selection.first()?;
    children(doc, list).contains(id).then(|| id.clone())
}

/// Collection items as rows: `<index>  <title, field, text or name>`.
fn items(doc: &Document, owner: &str) -> Vec<ActionRow> {
    let prop = |n: &crawrad_ir::Node, key: &str| match n.props.get(key) {
        Some(Value::Str(s) | Value::Enum(s)) if !s.trim().is_empty() => Some(s.trim().to_string()),
        _ => None,
    };
    children(doc, owner)
        .into_iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let node = doc.node(&id)?;
            let label = prop(node, "title")
                .or_else(|| prop(node, "field"))
                .or_else(|| prop(node, "when"))
                .or_else(|| prop(node, "text"))
                .unwrap_or_else(|| node.name().to_string());
            Some(ActionRow {
                label: format!("{i}  {label}"),
                id,
            })
        })
        .collect()
}

#[note("action-list-is-flat-collection")]
pub fn is_flat_collection(registry: &Registry, class: &str) -> bool {
    let Some(item_class) = registry.get(class).and_then(|d| d.item_class()) else {
        return false;
    };
    // A tree (a MenuItem holding MenuItems) has its own designer; an item with a
    // collection of ANOTHER class (a GridColumn's StyleRules) is one level down
    // of the same dialog.
    registry.get(item_class).is_some_and(|d| {
        !d.accepts_children() || d.item_class().is_some_and(|nested| nested != item_class)
    })
}

fn is_action_list(doc: &Document, list: &str) -> bool {
    doc.node(list)
        .is_some_and(|n| n.class == CLASSE_ACTION_LIST)
}

fn can_move(doc: &Document, list: &str, selection: &[String], step: i64) -> bool {
    let count = children(doc, list).len() as i64;
    selected(doc, list, selection)
        .and_then(|s| position(doc, list, &s))
        .is_some_and(|p| (0..count).contains(&(p as i64 + step)))
}

fn title(doc: &Document, registry: &Registry, list: &str) -> String {
    let name = |id: &str| doc.node(id).map(|n| n.name().to_string());
    let own = name(list).unwrap_or_else(|| list.to_string());
    if is_action_list(doc, list) {
        let form = name(&doc.root).unwrap_or_default();
        return format!("{EDITANDO} {form}.{own}");
    }
    let coll = doc
        .node(list)
        .and_then(|n| registry.get(&n.class)?.collection_label())
        .unwrap_or_else(|| "Items".to_string());
    format!("{own}.{coll}")
}

#[note("action-list-dialogstate")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogState {
    pub title: String,
    /// Strip labels (see [`ACTION_LIST_BUTTONS`]); empty hides the button.
    pub buttons: [&'static str; 6],
    /// The categories column shows (ActionList only).
    pub categories: bool,
    pub list_label: &'static str,
    pub footer: &'static str,
    pub can_delete: bool,
    pub can_up: bool,
    pub can_down: bool,
    pub can_add_fields: bool,
    pub back: bool,
    pub breadcrumb: Vec<String>,
    /// The levels above the open one, dim before the title: `grid1.Columns › ` (empty at the top).
    pub crumb_prev: String,
}

/// The owners the dialog shows after opening `list`: one level down when `list`
/// is a child of the open owner (a GridColumn of the open Grid), else `list` alone.
pub fn owners_after_open(
    doc: &Document,
    owners: &[crawrad_ir::NodeId],
    list: &str,
) -> Vec<crawrad_ir::NodeId> {
    let mut next = match owners.last() {
        Some(open) if doc.parent_of(list) == Some(open) => owners.to_vec(),
        _ => Vec::new(),
    };
    next.push(list.to_string());
    next
}

/// `‹`: the owners one level up and the item to select there (the one the user came
/// from); `None` at the top.
pub fn owners_after_back(
    owners: &[crawrad_ir::NodeId],
) -> Option<(Vec<crawrad_ir::NodeId>, crawrad_ir::NodeId)> {
    let (came_from, up) = owners.split_last()?;
    (!up.is_empty()).then(|| (up.to_vec(), came_from.clone()))
}

/// Returns the breadcrumb trail for an owner hierarchy.
pub fn breadcrumb(
    doc: &Document,
    registry: &Registry,
    owners: &[crawrad_ir::NodeId],
) -> Vec<String> {
    owners
        .iter()
        .map(|id| {
            let name = doc.node(id).map_or_else(|| id.as_str(), |n| n.name());
            let coll = doc
                .node(id)
                .and_then(|n| registry.get(&n.class)?.collection_label())
                .unwrap_or_else(|| "Items".to_string());
            format!("{name}.{coll}")
        })
        .collect()
}

/// What the ActionList / collection dialog shows, with owner hierarchy.
pub fn state_with_owners(
    doc: &Document,
    registry: &Registry,
    owners: &[crawrad_ir::NodeId],
    selection: &[String],
    columns: Option<&[String]>,
) -> DialogState {
    let list = owners.last().map(String::as_str).unwrap_or("");
    let actions = is_action_list(doc, list);
    let mut buttons = if actions {
        ACTION_LIST_BUTTONS
    } else {
        COLLECTION_BUTTONS
    };
    if columns.is_none() {
        buttons[5] = "";
    }
    DialogState {
        title: title(doc, registry, list),
        buttons,
        categories: actions,
        list_label: if actions { "Actions:" } else { "Items:" },
        footer: if actions { "" } else { COLLECTION_FOOTER },
        can_delete: selected(doc, list, selection).is_some(),
        can_up: can_move(doc, list, selection, -1),
        can_down: can_move(doc, list, selection, 1),
        can_add_fields: columns.is_some_and(|c| !c.is_empty()),
        back: owners.len() > 1,
        crumb_prev: breadcrumb(doc, registry, &owners[..owners.len().saturating_sub(1)])
            .iter()
            .map(|c| format!("{c} › "))
            .collect(),
        breadcrumb: breadcrumb(doc, registry, owners),
    }
}

#[note("action-list-state")]
pub fn state(
    doc: &Document,
    registry: &Registry,
    list: &str,
    selection: &[String],
    columns: Option<&[String]>,
) -> DialogState {
    state_with_owners(doc, registry, &[list.to_string()], selection, columns)
}

#[note("action-list-rows")]
pub fn rows(doc: &Document, list: &str, category: &str) -> (Vec<String>, Vec<ActionRow>) {
    if is_action_list(doc, list) {
        (categories(doc, list), actions(doc, list, category))
    } else {
        (Vec::new(), items(doc, list))
    }
}

#[note("action-list-add-args")]
pub fn add_args(doc: &Document, registry: &Registry, list: &str) -> Option<(&'static str, Args)> {
    if is_action_list(doc, list) {
        return Some((ACTION_NEW, Args::none()));
    }
    let item = registry.get(&doc.node(list)?.class)?.item_class()?;
    let args = Args::text("class", item).and("parent", Value::Str(list.to_string()));
    Some(("component.add", args))
}

#[note("action-list-add-all-fields-args")]
pub fn add_all_fields_args(node: &str, fields: &[String], kinds: &[String]) -> Args {
    let list = |v: &[String]| Value::List(v.iter().cloned().map(Value::Str).collect());
    Args::text("node", node)
        .and("fields", list(fields))
        .and("kinds", list(kinds))
}

#[note("action-list-standardrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardRow {
    /// A category header — `Edit`, `File`.
    Group {
        name: &'static str,
        /// Does what is underneath it appear?
        open: bool,
    },
    /// A standard action class, with the chosen check mark.
    Class {
        /// The class name, which is what goes into the catalogue action argument.
        name: &'static str,
        /// What is read: `TEditCopy (Ctrl+C)`.
        label: String,
        /// The explanation, for the tooltip.
        hint: &'static str,
        checked: bool,
    },
}

#[note("action-list-standard-rows")]
pub fn standard_rows(
    closed: &dyn Fn(&str) -> bool,
    checked: &dyn Fn(&str) -> bool,
) -> Vec<StandardRow> {
    let mut out = Vec::new();
    for (cat, action_items) in crawrad_registry::standard_actions_by_category() {
        let open = !closed(cat);
        out.push(StandardRow::Group { name: cat, open });
        if !open {
            continue;
        }
        for a in action_items {
            let label = if a.shortcut.is_empty() {
                a.name.to_string()
            } else {
                format!("{} ({})", a.name, a.shortcut)
            };
            out.push(StandardRow::Class {
                name: a.name,
                label,
                hint: a.hint,
                checked: checked(a.name),
            });
        }
    }
    out
}

#[note("action-list-group-starts-open")]
pub fn group_starts_open(cat: &str) -> bool {
    cat == CAT_NONE || cat == "Edit"
}

fn children(doc: &Document, list: &str) -> Vec<String> {
    doc.node(list)
        .map(|n| n.children.clone())
        .unwrap_or_default()
}

fn category_of(doc: &Document, id: &str) -> Option<String> {
    match doc.node(id)?.props.get("category")? {
        Value::Str(s) => Some(s.clone()),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/action_list/test.rs"]
mod tests;
