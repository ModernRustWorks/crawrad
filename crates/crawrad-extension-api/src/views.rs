//! **Extension views**: a tree an extension fills and the IDE draws in
//! the left rail's Extensions panel — a header, a filter box and the rows.
//! The extension owns the items; the IDE owns the drawing, the filtering and
//! the jump to an item's place, and tells the extension what was clicked.

use serde::{Deserialize, Serialize};

/// A view declared in an extension manifest.
///
/// A view the manifest declares (`contributions.views`): its tree appears in
/// the Extensions panel under `title`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ViewContribution;
///
/// let view = ViewContribution {
///     id: "my_view".to_string(),
///     title: "My View".to_string(),
///     icon: None,
/// };
/// assert_eq!(view.id, "my_view");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewContribution {
    /// Unique within the extension; `view/setTree` names it.
    pub id: String,
    /// The header over the view's tree.
    pub title: String,
    /// Optional icon name.
    #[serde(default)]
    pub icon: Option<String>,
}

/// Where an item points: a click opens `file` at `line`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ItemLocation;
///
/// let loc = ItemLocation {
///     file: "src/main.rs".to_string(),
///     line: 42,
/// };
/// assert_eq!(loc.line, 42);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemLocation {
    /// The file's path, as the extension sees it.
    pub file: String,
    /// 0-based line.
    #[serde(default)]
    pub line: u32,
}

/// One node of a view's tree.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ViewItem;
///
/// let item = ViewItem {
///     id: "item-1".to_string(),
///     label: "First".to_string(),
///     detail: None,
///     kind: None,
///     location: None,
///     children: Vec::new(),
/// };
/// assert_eq!(item.id, "item-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewItem {
    /// Unique within the view; the click sends it back.
    pub id: String,
    pub label: String,
    /// A dimmer second text after the label.
    #[serde(default)]
    pub detail: Option<String>,
    /// The colour family, as for line marks (`todo`, `fixme`, `hack`, ...).
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub location: Option<ItemLocation>,
    #[serde(default)]
    pub children: Vec<ViewItem>,
}

/// Parameters for setting an extension view tree.
///
/// `view/setTree`: the whole tree of `view_id` — it replaces the previous one
/// (an empty list clears it).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::SetTreeParams;
///
/// let params = SetTreeParams {
///     view_id: "my_view".to_string(),
///     items: Vec::new(),
/// };
/// assert!(params.items.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetTreeParams {
    pub view_id: String,
    pub items: Vec<ViewItem>,
}

/// `view/itemClicked`: the IDE tells the extension an item was clicked.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ItemClicked;
///
/// let clicked = ItemClicked {
///     view_id: "my_view".to_string(),
///     item_id: "item-1".to_string(),
/// };
/// assert_eq!(clicked.item_id, "item-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemClicked {
    pub view_id: String,
    pub item_id: String,
}
