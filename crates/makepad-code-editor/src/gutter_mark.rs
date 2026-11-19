//! Marks that belong to a LINE and are drawn in the gutter's glyph margin —
//! the breakpoint dot.
//!
//! A mark follows the code it was put on while the text is edited, the way a
//! breakpoint does in classic RAD IDEs and VS Code:
//!
//! - lines inserted or deleted **above** move it;
//! - `Enter` at the very start of its line pushes the line's code down, and
//!   the mark goes with the code;
//! - typing inside its line, or `Enter` after the line's first byte, leaves it
//!   where it is — the code it points at is still there;
//! - deleting its line joins it to the line the deletion started on.
//!
//! [`crate::text::Position::apply_edit`] cannot be used for this: an
//! insertion exactly at the mark resolves by the edit's drift, and a line mark
//! needs the rule above instead.

use crate::text::{Change, Edit};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GutterMarkKind {
    Breakpoint,
    /// Where the program under the debugger stands: the yellow arrow.
    ExecutionPoint,
    /// A numbered bookmark, 1 to 9: one line per number in a file.
    Bookmark(u8),
}

/// A mark on a (zero-based) line.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GutterMark {
    pub line: usize,
    pub kind: GutterMarkKind,
}

/// Where the mark on `line` is after `edit`.
pub fn shift_line(line: usize, edit: &Edit) -> usize {
    match &edit.change {
        Change::Insert(at, text) => {
            let added = text.length().line_count;
            if added == 0 {
                line
            } else if at.line_index < line || (at.line_index == line && at.byte_index == 0) {
                line + added
            } else {
                line
            }
        }
        Change::Delete(start, length) => {
            let end_line = start.line_index + length.line_count;
            if line > end_line {
                line - length.line_count
            } else if line > start.line_index {
                start.line_index
            } else {
                line
            }
        }
    }
}

/// The marks of one document, kept sorted and without two of a kind on a line.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GutterMarkSet {
    marks: Vec<GutterMark>,
}

impl GutterMarkSet {
    pub fn as_marks(&self) -> &[GutterMark] {
        &self.marks
    }

    /// The lines that carry a mark of `kind`, in order.
    pub fn lines(&self, kind: GutterMarkKind) -> Vec<usize> {
        self.marks
            .iter()
            .filter(|m| m.kind == kind)
            .map(|m| m.line)
            .collect()
    }

    /// Replaces the marks of `kind` with one on each of `lines`.
    pub fn set_lines(&mut self, kind: GutterMarkKind, lines: &[usize]) {
        self.marks.retain(|m| m.kind != kind);
        self.marks
            .extend(lines.iter().map(|&line| GutterMark { line, kind }));
        self.normalize();
    }

    /// Adds the mark when the line has none, removes it when it has one.
    /// Returns whether the line carries it afterwards.
    pub fn toggle(&mut self, kind: GutterMarkKind, line: usize) -> bool {
        let mark = GutterMark { line, kind };
        if let Some(i) = self.marks.iter().position(|m| *m == mark) {
            self.marks.remove(i);
            false
        } else {
            self.marks.push(mark);
            self.normalize();
            true
        }
    }

    /// Every bookmark as `(number, line)`, in line order.
    pub fn bookmarks(&self) -> Vec<(u8, usize)> {
        self.marks
            .iter()
            .filter_map(|m| match m.kind {
                GutterMarkKind::Bookmark(n) => Some((n, m.line)),
                _ => None,
            })
            .collect()
    }

    /// Replaces every bookmark; a number given twice keeps its first line.
    pub fn set_bookmarks(&mut self, bookmarks: &[(u8, usize)]) {
        self.marks
            .retain(|m| !matches!(m.kind, GutterMarkKind::Bookmark(_)));
        let mut seen = Vec::new();
        for &(n, line) in bookmarks {
            if !seen.contains(&n) {
                seen.push(n);
                self.marks.push(GutterMark {
                    line,
                    kind: GutterMarkKind::Bookmark(n),
                });
            }
        }
        self.normalize();
    }

    /// Bookmark `n` on `line`: removed when it is already there, otherwise
    /// moved there from wherever it was — a number names one place. Answers
    /// whether the line carries it now.
    pub fn toggle_bookmark(&mut self, n: u8, line: usize) -> bool {
        let kind = GutterMarkKind::Bookmark(n);
        let here = self.marks.iter().any(|m| m.kind == kind && m.line == line);
        self.marks.retain(|m| m.kind != kind);
        if !here {
            self.marks.push(GutterMark { line, kind });
            self.normalize();
        }
        !here
    }

    pub fn apply_edit(&mut self, edit: &Edit) {
        for mark in &mut self.marks {
            mark.line = shift_line(mark.line, edit);
        }
        // Two marks on lines that were joined become one.
        self.normalize();
    }

    /// Drops the marks past the last of `line_count` lines — a mark on a line
    /// that no longer exists would be drawn nowhere and stop nowhere.
    pub fn clamp(&mut self, line_count: usize) {
        self.marks.retain(|m| m.line < line_count);
    }

    fn normalize(&mut self) {
        self.marks.sort();
        self.marks.dedup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{Drift, Length, Position, Text};

    fn insert(line_index: usize, byte_index: usize, text: &str) -> Edit {
        Edit {
            change: Change::Insert(
                Position {
                    line_index,
                    byte_index,
                },
                Text::from(text),
            ),
            drift: Drift::Before,
        }
    }

    fn delete(line_index: usize, byte_index: usize, line_count: usize, byte_count: usize) -> Edit {
        Edit {
            change: Change::Delete(
                Position {
                    line_index,
                    byte_index,
                },
                Length {
                    line_count,
                    byte_count,
                },
            ),
            drift: Drift::Before,
        }
    }

    #[test]
    fn lines_added_above_push_the_mark_down_and_below_leave_it() {
        assert_eq!(shift_line(5, &insert(2, 3, "a\nb\n")), 7);
        assert_eq!(shift_line(5, &insert(8, 0, "\n")), 5);
    }

    #[test]
    fn enter_at_the_start_of_its_line_moves_it_with_the_code() {
        assert_eq!(shift_line(5, &insert(5, 0, "\n")), 6);
        // After the first byte the code stays on the line: so does the mark.
        assert_eq!(shift_line(5, &insert(5, 4, "\n")), 5);
    }

    #[test]
    fn typing_inside_a_line_moves_nothing() {
        assert_eq!(shift_line(5, &insert(5, 0, "let x = 1;")), 5);
        assert_eq!(shift_line(5, &insert(2, 0, "abc")), 5);
    }

    #[test]
    fn lines_deleted_above_pull_it_up_and_its_own_deletion_joins_it_upward() {
        // Lines 2..=3 go (two line breaks removed starting on line 1).
        assert_eq!(shift_line(6, &delete(1, 4, 2, 0)), 4);
        // Backspace at the start of line 5 joins it to line 4.
        assert_eq!(shift_line(5, &delete(4, 10, 1, 0)), 4);
        // A deletion within one line changes no line.
        assert_eq!(shift_line(5, &delete(5, 0, 0, 3)), 5);
        // Below the mark: nothing.
        assert_eq!(shift_line(5, &delete(7, 0, 2, 0)), 5);
    }

    #[test]
    fn the_set_toggles_keeps_order_and_merges_joined_marks() {
        let mut set = GutterMarkSet::default();
        assert!(set.toggle(GutterMarkKind::Breakpoint, 9));
        assert!(set.toggle(GutterMarkKind::Breakpoint, 3));
        assert_eq!(set.lines(GutterMarkKind::Breakpoint), vec![3, 9]);
        assert!(!set.toggle(GutterMarkKind::Breakpoint, 9));
        assert_eq!(set.lines(GutterMarkKind::Breakpoint), vec![3]);
        set.set_lines(GutterMarkKind::Breakpoint, &[4, 5]);
        // Deleting line 5 joins it to 4: one mark, not two on the same line.
        set.apply_edit(&delete(4, 2, 1, 0));
        assert_eq!(set.lines(GutterMarkKind::Breakpoint), vec![4]);
        set.set_lines(GutterMarkKind::Breakpoint, &[1, 30]);
        set.clamp(10);
        assert_eq!(set.lines(GutterMarkKind::Breakpoint), vec![1]);
    }
}

#[cfg(test)]
#[path = "test/gutter_mark/test.rs"]
mod bookmark_tests;
