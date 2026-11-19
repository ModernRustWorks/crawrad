//! **Line marks**: what an extension paints on a line of the code
//! editor — a pill over a span of the line, with its label, and an icon in
//! the gutter. A TODO, a lint, a coverage gap: the extension decides where;
//! the IDE draws it the way it draws a sidecar note, and tells the extension
//! when the icon is clicked.

use serde::{Deserialize, Serialize};

/// One mark on a line.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::marks::LineMark;
///
/// let mark = LineMark {
///     id: "mark-1".to_string(),
///     line: 10,
///     start: 0,
///     end: 5,
///     label: "TODO".to_string(),
///     kind: "todo".to_string(),
/// };
/// assert_eq!(mark.line, 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineMark {
    /// Unique among this extension's marks; the click sends it back.
    pub id: String,
    /// 0-based line.
    pub line: u32,
    /// The span the pill covers, in bytes of the line (UTF-8).
    pub start: u32,
    pub end: u32,
    /// What the pill says.
    pub label: String,
    /// The colour family: `todo`, `fixme`, `hack`, `info`, `warning`.
    #[serde(default)]
    pub kind: String,
}

/// Parameters for setting line marks on a file.
///
/// `editor/setLineMarks`: every mark of this extension on `file` — the
/// previous ones of that file are replaced (an empty list clears them).
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::marks::SetLineMarksParams;
///
/// let params = SetLineMarksParams {
///     file: "src/main.rs".to_string(),
///     marks: Vec::new(),
/// };
/// assert!(params.marks.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetLineMarksParams {
    /// The file's path, as the extension sees it.
    pub file: String,
    pub marks: Vec<LineMark>,
}

/// Notification that a line mark's icon was clicked.
///
/// `editor/lineMarkClicked`: the IDE tells the extension its mark's icon
/// was clicked.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::marks::LineMarkClicked;
///
/// let clicked = LineMarkClicked {
///     id: "mark-1".to_string(),
///     file: "src/main.rs".to_string(),
/// };
/// assert_eq!(clicked.id, "mark-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineMarkClicked {
    pub id: String,
    pub file: String,
}
