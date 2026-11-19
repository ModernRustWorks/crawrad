//! Numbered bookmarks in the gutter's mark set.

use super::{GutterMarkKind, GutterMarkSet};
use crate::text::{Change, Drift, Edit, Position, Text};

fn enter_at_start_of(line_index: usize) -> Edit {
    Edit {
        change: Change::Insert(
            Position {
                line_index,
                byte_index: 0,
            },
            Text::from("\n"),
        ),
        drift: Drift::Before,
    }
}

/// **A number names one place:** set on a line, off on the same line, and
/// moved — not copied — when set on another.
#[test]
fn a_bookmark_number_names_one_line() {
    let mut marks = GutterMarkSet::default();
    assert!(marks.toggle_bookmark(1, 3));
    assert_eq!(marks.bookmarks(), vec![(1, 3)]);
    assert!(marks.toggle_bookmark(1, 7));
    assert_eq!(marks.bookmarks(), vec![(1, 7)], "it moved, it did not copy");
    assert!(!marks.toggle_bookmark(1, 7));
    assert!(marks.bookmarks().is_empty());
}

/// **Two numbers may share a line; a breakpoint is not touched.**
#[test]
fn bookmarks_live_beside_breakpoints() {
    let mut marks = GutterMarkSet::default();
    marks.toggle(GutterMarkKind::Breakpoint, 4);
    marks.set_bookmarks(&[(2, 4), (5, 4), (2, 9)]);
    assert_eq!(
        marks.bookmarks(),
        vec![(2, 4), (5, 4)],
        "a number given twice keeps its first line"
    );
    assert_eq!(marks.lines(GutterMarkKind::Breakpoint), vec![4]);
    marks.set_bookmarks(&[]);
    assert_eq!(marks.lines(GutterMarkKind::Breakpoint), vec![4]);
}

/// **The bookmark follows its code:** an Enter above pushes it down.
#[test]
fn a_bookmark_follows_its_line_through_an_edit() {
    let mut marks = GutterMarkSet::default();
    marks.toggle_bookmark(3, 10);
    marks.apply_edit(&enter_at_start_of(2));
    assert_eq!(marks.bookmarks(), vec![(3, 11)]);
}
