//! **Line marks from an extension**: a pill over a span of a line, its
//! label in it, and an icon in the glyph margin a click reports. A sidecar
//! note's anchor wins its line (it takes no other mark).

/// One mark as the editor draws it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineMark {
    /// The extension that painted it, and its id there: the click names both.
    pub owner: String,
    pub id: String,
    pub line: usize,
    /// The span the pill covers, bytes of the line.
    pub start: usize,
    pub end: usize,
    pub label: String,
    /// The colour family (`todo`, `fixme`, `hack`, `info`, `warning`).
    pub kind: String,
}

/// The colour of a kind, as (r, g, b): TODO blue, FIXME Rust orange, HACK
/// violet, a warning amber, anything else grey-blue.
pub fn kind_rgb(kind: &str) -> (f32, f32, f32) {
    match kind {
        "todo" => (0.31, 0.76, 1.0),
        "fixme" => (0.94, 0.48, 0.27),
        "hack" => (0.77, 0.53, 0.75),
        "warning" => (0.84, 0.73, 0.49),
        _ => (0.61, 0.86, 1.0),
    }
}

/// The marks of `line`, in the order they came.
pub fn on_line(marks: &[LineMark], line: usize) -> impl Iterator<Item = &LineMark> {
    marks.iter().filter(move |m| m.line == line)
}
