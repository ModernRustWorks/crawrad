//! Tests for current-line highlighting logic.

use crate::current_line::*;
use crate::selection::{Cursor, Selection};
use crate::text::Position;

#[test]
fn test_disabled_returns_empty() {
    let selections = [(2, 2, 2)];
    let result = highlighted_lines(&selections, false);
    assert!(result.is_empty());
}

#[test]
fn test_empty_selections_returns_empty() {
    let selections: [(usize, usize, usize); 0] = [];
    let result = highlighted_lines(&selections, true);
    assert!(result.is_empty());
}

#[test]
fn test_single_caret_returns_line() {
    let selections = [(4, 4, 4)];
    let result = highlighted_lines(&selections, true);
    assert_eq!(result, vec![4]);
}

#[test]
fn test_single_line_selection_returns_line() {
    let selections = [(7, 7, 7)];
    let result = highlighted_lines(&selections, true);
    assert_eq!(result, vec![7]);
}

#[test]
fn test_multi_line_selection_returns_empty() {
    let selections = [(2, 5, 5)];
    let result = highlighted_lines(&selections, true);
    assert!(result.is_empty());
}

#[test]
fn test_inverted_multi_line_selection_returns_empty() {
    let selections = [(6, 3, 3)];
    let result = highlighted_lines(&selections, true);
    assert!(result.is_empty());
}

#[test]
fn test_multiple_carets_sorted_and_deduplicated() {
    let selections = [(12, 12, 12), (3, 3, 3), (8, 8, 8), (3, 3, 3)];
    let result = highlighted_lines(&selections, true);
    assert_eq!(result, vec![3, 8, 12]);
}

#[test]
fn test_mixed_single_and_multi_line_selections() {
    let selections = [(1, 1, 1), (3, 7, 7), (9, 9, 9)];
    let result = highlighted_lines(&selections, true);
    assert_eq!(result, vec![1, 9]);
}

#[test]
fn test_disabled_with_mixed_selections_returns_empty() {
    let selections = [(1, 1, 1), (9, 9, 9)];
    let result = highlighted_lines(&selections, false);
    assert!(result.is_empty());
}

#[test]
fn test_from_selections_helper_single_line() {
    let sel = Selection {
        cursor: Cursor {
            position: Position {
                line_index: 5,
                byte_index: 0,
            },
            affinity: Default::default(),
            preferred_column_index: None,
        },
        anchor: Position {
            line_index: 5,
            byte_index: 10,
        },
    };
    let result = highlighted_lines_from_selections(&[sel], true);
    assert_eq!(result, vec![5]);
}

#[test]
fn test_from_selections_helper_multi_line() {
    let sel = Selection {
        cursor: Cursor {
            position: Position {
                line_index: 5,
                byte_index: 0,
            },
            affinity: Default::default(),
            preferred_column_index: None,
        },
        anchor: Position {
            line_index: 2,
            byte_index: 0,
        },
    };
    let result = highlighted_lines_from_selections(&[sel], true);
    assert!(result.is_empty());
}
