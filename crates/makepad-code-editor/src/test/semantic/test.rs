//! The semantic spans and the cursor that walks them.

use super::*;

fn p(line_index: usize, byte_index: usize) -> Position {
    Position {
        line_index,
        byte_index,
    }
}

/// The cursor answers each position by the span over it, in one pass.
#[test]
fn the_cursor_finds_the_span_over_each_position() {
    let mut set = SemanticSet::default();
    set.set(vec![
        SemanticSpan {
            start: p(1, 4),
            end: p(1, 9),
            kind: Some(SemanticKind::Macro),
            struck: false,
        },
        SemanticSpan {
            start: p(0, 8),
            end: p(0, 13),
            kind: Some(SemanticKind::Mutable),
            struck: false,
        },
    ]);
    let mut cur = SemanticCursor::new(set.as_spans());
    let mut kind = |at| cur.at(at).and_then(|s| s.kind);
    assert_eq!(kind(p(0, 7)), None);
    assert_eq!(kind(p(0, 8)), Some(SemanticKind::Mutable));
    assert_eq!(kind(p(0, 12)), Some(SemanticKind::Mutable));
    assert_eq!(kind(p(0, 13)), None, "the end is not in the span");
    assert_eq!(kind(p(1, 4)), Some(SemanticKind::Macro));
    assert_eq!(kind(p(2, 0)), None);
}

/// **The spans follow an edit:** a newline above moves them down a
/// line; deleting the coloured text drops its span. Without this, typing
/// would show the old colours on the wrong text until the server answers.
#[test]
fn the_spans_follow_an_edit() {
    use crate::text::{Change, Drift, Edit, Length, Text};
    let mut set = SemanticSet::default();
    set.set(vec![
        SemanticSpan {
            start: p(1, 8),
            end: p(1, 13),
            kind: Some(SemanticKind::Mutable),
            struck: false,
        },
        SemanticSpan {
            start: p(2, 0),
            end: p(2, 5),
            kind: Some(SemanticKind::Macro),
            struck: false,
        },
    ]);
    let newline = Edit {
        change: Change::Insert(p(0, 0), Text::from("\n")),
        drift: Drift::Before,
    };
    set.apply_edit(&newline);
    let lines: Vec<_> = set.as_spans().iter().map(|s| (s.start, s.end)).collect();
    assert_eq!(lines, vec![(p(2, 8), p(2, 13)), (p(3, 0), p(3, 5))]);
    let delete = Edit {
        change: Change::Delete(
            p(3, 0),
            Length {
                line_count: 0,
                byte_count: 5,
            },
        ),
        drift: Drift::Before,
    };
    set.apply_edit(&delete);
    assert_eq!(set.as_spans().len(), 1, "the deleted macro's span is gone");
}
