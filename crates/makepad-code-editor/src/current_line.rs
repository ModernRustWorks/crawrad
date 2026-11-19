//! Pure logic for current-line highlighting in the code editor.
//!
//! Computes which lines get the background highlight band and brightened/bold
//! line number in the gutter based on caret positions and active selections.

use crate::selection::Selection;

/// Computes the sorted, deduplicated zero-based line indices of carets to highlight.
///
/// Each selection tuple is `(start_line, end_line, caret_line)`.
/// A line is highlighted when highlighting is `enabled` and the selection for that
/// caret does not span across multiple lines (i.e. `start_line == end_line`).
/// Multi-line selections produce no line highlight.
pub fn highlighted_lines(selections: &[(usize, usize, usize)], enabled: bool) -> Vec<usize> {
    if !enabled {
        return Vec::new();
    }
    let mut lines = Vec::new();
    for &(start_line, end_line, caret_line) in selections {
        if start_line == end_line {
            lines.push(caret_line);
        }
    }
    lines.sort_unstable();
    lines.dedup();
    lines
}

/// Convenience function to compute highlighted lines directly from [`Selection`] slices.
pub fn highlighted_lines_from_selections(selections: &[Selection], enabled: bool) -> Vec<usize> {
    let tuples: Vec<(usize, usize, usize)> = selections
        .iter()
        .map(|s| {
            (
                s.start().line_index,
                s.end().line_index,
                s.cursor.position.line_index,
            )
        })
        .collect();
    highlighted_lines(&tuples, enabled)
}
