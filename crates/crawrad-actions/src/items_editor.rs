//! The headless **Items Editor** — the collection state, the path to an item,
//! and the five operations triggered by the middle buttons.
//!
//! # Why it lives in `crawrad-actions`
//!
//! It is a headless model, and without a screen it is not `crawrad-ide` business
//! (ADR 0043): it moved out of there, like the ActionList one.
//! The shell keeps asking by the same name (`crate::items_editor::model`), and
//! `new_child`, `delete`, and `move_item` remain written only once.
//!
//! **The `New SubItem` rule already diverged once**, and not through
//! architectural carelessness: it was written as `node.class == "ComboBox"`
//! inside the drawing, `ListBox` arrived later, was not added to the literal,
//! and the result on the owner's screen was a `ListBox` opening the TREE
//! editor. Today the catalogue answers (`PropDesc::tree_collection`), and the
//! question is asked **once**, here.
//!
//! # What is NOT here
//!
//! Everything needing a screen: the three-panel frame, the click target, the
//! tooltip for each button. That stays in the shell, which draws its framework's
//! way and queries this file.

use crawrad_ir::Item;
use crawrad_note::note;

pub const ITEMS_NOTA: &str = "Selected Index and Enabled arrive with the resource phase \
                          — no runtime honours them yet.";
pub const ITEMS_KEY: &str = "items";
pub const ITEMS_OK: &str = "OK";
pub const ITEMS_CANCEL: &str = "Cancel";
pub const ITEMS_APPLY: &str = "Apply";

#[note("items-editor-itempath")]
pub type ItemPath = Vec<usize>;

#[note("items-editor-itemrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemRow {
    /// The item text, with no indentation — indentation belongs to the drawing.
    pub text: String,
    /// How many levels below the root: zero for a top-level item.
    pub level: usize,
    /// Where it is, so that a click knows what was selected.
    pub path: ItemPath,
}

#[note("items-editor-rows")]
pub fn rows(items: &[Item]) -> Vec<ItemRow> {
    fn walk(out: &mut Vec<ItemRow>, items: &[Item], prefix: &[usize]) {
        for (i, item) in items.iter().enumerate() {
            let mut path = prefix.to_vec();
            path.push(i);
            out.push(ItemRow {
                text: item.text.clone(),
                level: path.len() - 1,
                path: path.clone(),
            });
            walk(out, &item.children, &path);
        }
    }
    let mut out = Vec::new();
    walk(&mut out, items, &[]);
    out
}

#[derive(Debug, Clone)]
pub struct ItemsEditorState {
    /// The stable identity of the node (ADR 0008) — addresses, does not appear.
    pub node_id: String,
    /// The component class. It provides the window title — see [`editor_title`].
    pub node_class: String,
    /// The label appearing in the content header.
    pub node_name: String,
    pub items: Vec<Item>,
    pub selected: Option<ItemPath>,
    /// The collection has **children**: a tree, not a flat list.
    ///
    /// Comes from the catalogue (`PropDesc::tree_collection`) and not from a
    /// class name written in here — see the module doc for the bug that this
    /// cost. The name is the SAME on both sides on purpose: a `flat` here and a
    /// `is_tree` there is a negation in the middle of the way, and negation in
    /// the middle is where this kind of bug lives.
    pub is_tree: bool,
    /// The class declares `images`, so each item can pick an icon by index.
    pub has_images: bool,
}

impl ItemsEditorState {
    #[note("items-editor-itemseditorstate-from-node")]
    pub fn from_node(
        node_id: &str,
        node: &crawrad_ir::Node,
        registry: &crawrad_registry::Registry,
    ) -> Self {
        let items = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
        };
        let desc = registry.get(&node.class);
        let is_tree = desc
            .and_then(|d| d.prop("items"))
            .is_some_and(|p| p.tree_collection);
        let has_images = desc.and_then(|d| d.prop("images")).is_some();
        Self {
            node_id: node_id.to_string(),
            node_class: node.class.clone(),
            node_name: node.name().to_string(),
            selected: (!items.is_empty()).then(|| vec![0]),
            items,
            is_tree,
            has_images,
        }
    }
}

#[note("items-editor-editor-title")]
pub fn editor_title(class_name: &str) -> String {
    format!("{class_name} Items Editor")
}

#[note("items-editor-can-create-sub-item")]
pub fn can_create_sub_item(state: &ItemsEditorState, has_selection: bool) -> bool {
    has_selection && state.is_tree
}

// ----------------------------------------------------- navigation by path

pub fn find<'a>(items: &'a [Item], path: &[usize]) -> Option<&'a Item> {
    let (&i, rest) = path.split_first()?;
    let item = items.get(i)?;
    if rest.is_empty() {
        Some(item)
    } else {
        find(&item.children, rest)
    }
}

pub fn find_mut<'a>(items: &'a mut [Item], path: &[usize]) -> Option<&'a mut Item> {
    let (&i, rest) = path.split_first()?;
    let item = items.get_mut(i)?;
    if rest.is_empty() {
        Some(item)
    } else {
        find_mut(&mut item.children, rest)
    }
}

#[note("items-editor-owner-list")]
pub fn owner_list<'a>(
    items: &'a mut Vec<Item>,
    path: &[usize],
) -> Option<(&'a mut Vec<Item>, usize)> {
    let (&last, before) = path.split_last()?;
    if before.is_empty() {
        return (last < items.len()).then_some((items, last));
    }
    let parent = find_mut(items, before)?;
    (last < parent.children.len()).then_some((&mut parent.children, last))
}

// ------------------------------------------------------------------ operations

pub fn free_name(state: &ItemsEditorState) -> String {
    // `Item1`, `Item2`… as the classic RAD IDE numbers what is created without a name.
    let mut n = Item::total(&state.items) + 1;
    loop {
        let attempt = format!("Item{n}");
        if !contains_text(&state.items, &attempt) {
            return attempt;
        }
        n += 1;
    }
}

fn contains_text(items: &[Item], text: &str) -> bool {
    items
        .iter()
        .any(|i| i.text == text || contains_text(&i.children, text))
}

pub fn new_sibling(state: &mut ItemsEditorState) {
    let new_item = Item::new(free_name(state));
    match state.selected.clone() {
        Some(path) => {
            let Some((list, pos)) = owner_list(&mut state.items, &path) else {
                return;
            };
            list.insert(pos + 1, new_item);
            let mut c = path;
            *c.last_mut().unwrap() = pos + 1;
            state.selected = Some(c);
        }
        None => {
            state.items.push(new_item);
            state.selected = Some(vec![state.items.len() - 1]);
        }
    }
}

pub fn new_child(state: &mut ItemsEditorState) {
    let new_item = Item::new(free_name(state));
    let Some(path) = state.selected.clone() else {
        return;
    };
    let Some(item) = find_mut(&mut state.items, &path) else {
        return;
    };
    item.children.push(new_item);
    let pos = item.children.len() - 1;
    let mut c = path;
    c.push(pos);
    state.selected = Some(c);
}

pub fn delete(state: &mut ItemsEditorState) {
    let Some(path) = state.selected.clone() else {
        return;
    };
    let Some((list, pos)) = owner_list(&mut state.items, &path) else {
        return;
    };
    list.remove(pos);
    // Selection goes to the neighbour, and does not vanish: deleting three in
    // sequence must be three clicks in the same place.
    let remaining = list.len();
    let mut c = path;
    if remaining == 0 {
        c.pop();
        state.selected = (!c.is_empty()).then_some(c);
    } else {
        *c.last_mut().unwrap() = pos.min(remaining - 1);
        state.selected = Some(c);
    }
}

pub fn move_item(state: &mut ItemsEditorState, step: i32) {
    let Some(path) = state.selected.clone() else {
        return;
    };
    let Some((list, pos)) = owner_list(&mut state.items, &path) else {
        return;
    };
    let target = pos as i32 + step;
    if target < 0 || target as usize >= list.len() {
        return;
    }
    let target = target as usize;
    list.swap(pos, target);
    let mut c = path;
    *c.last_mut().unwrap() = target;
    state.selected = Some(c);
}

#[note("items-editor-itemscommand")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemsCommand {
    NewItem,
    NewSubItem,
    Delete,
    MoveUp,
    MoveDown,
}

pub fn run(state: &mut ItemsEditorState, command: ItemsCommand) {
    match command {
        ItemsCommand::NewItem => new_sibling(state),
        ItemsCommand::NewSubItem => new_child(state),
        ItemsCommand::Delete => delete(state),
        ItemsCommand::MoveUp => move_item(state, -1),
        ItemsCommand::MoveDown => move_item(state, 1),
    }
}

#[note("items-editor-selected-mut")]
fn selected_mut(state: &mut ItemsEditorState) -> Option<&mut Item> {
    let path = state.selected.clone()?;
    find_mut(&mut state.items, &path)
}

pub fn set_selected_text(state: &mut ItemsEditorState, text: String) {
    if let Some(item) = selected_mut(state) {
        item.text = text;
    }
}

#[note("items-editor-selectedfields")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedFields {
    /// `New SubItem` applies: an item is selected and the collection is a tree.
    pub can_add_child: bool,
    pub text: Option<String>,
    pub expanded: Option<bool>,
    pub image_index: Option<i64>,
}

pub fn selected_fields(state: &ItemsEditorState) -> SelectedFields {
    let item = state.selected.as_ref().and_then(|p| find(&state.items, p));
    SelectedFields {
        can_add_child: can_create_sub_item(state, item.is_some()),
        text: item.map(|i| i.text.clone()),
        expanded: item.filter(|i| !i.children.is_empty()).map(|i| i.expanded),
        image_index: item.filter(|_| state.has_images).map(|i| i.image_index),
    }
}

impl SelectedFields {
    pub fn has_selection(&self) -> bool {
        self.text.is_some()
    }

    #[note("items-editor-selectedfields-image-text")]
    pub fn image_text(&self) -> String {
        self.image_index.map(|i| i.to_string()).unwrap_or_default()
    }
}

#[note("items-editor-selected-expanded")]
pub fn selected_expanded(state: &ItemsEditorState) -> Option<bool> {
    selected_fields(state).expanded
}

#[note("items-editor-set-selected-expanded")]
pub fn set_selected_expanded(state: &mut ItemsEditorState, expanded: bool) {
    if let Some(item) = selected_mut(state).filter(|i| !i.children.is_empty()) {
        item.expanded = expanded;
    }
}

#[note("items-editor-itemedit")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemEdit {
    Text(String),
    Expanded(bool),
    /// As typed: empty means no icon; text that is not an index yet (the user
    /// is mid-typing) leaves the item alone.
    ImageIndex(String),
}

pub fn edit_selected(state: &mut ItemsEditorState, edit: ItemEdit) {
    match edit {
        ItemEdit::Text(text) => set_selected_text(state, text),
        ItemEdit::Expanded(expanded) => set_selected_expanded(state, expanded),
        ItemEdit::ImageIndex(typed) => {
            let index = match typed.trim() {
                "" => Some(-1),
                t => t.parse::<i64>().ok().filter(|n| *n >= -1),
            };
            if let Some(index) = index.filter(|_| state.has_images)
                && let Some(item) = selected_mut(state)
            {
                item.image_index = index;
            }
        }
    }
}

#[cfg(test)]
#[path = "test/items_editor/test.rs"]
mod tests;
