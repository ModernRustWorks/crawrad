//! A snippet's stops: visited in order, ending after `$0`, following edits.

use super::*;
use crate::text::{Change, Drift, Text};

fn p(line_index: usize, byte_index: usize) -> Position {
    Position {
        line_index,
        byte_index,
    }
}

/// Typing in `$1` moves the later stops; Tab goes through them in order,
/// and after `$0` there is none left.
#[test]
fn stops_follow_typing_and_end_after_the_last() {
    let mut s = SnippetStops::default();
    s.start(vec![
        (p(0, 5), p(0, 6)),
        (p(0, 8), p(0, 9)),
        (p(0, 10), p(0, 10)),
    ]);
    assert_eq!(s.advance(), Some((p(0, 5), p(0, 6))));
    // the user typed "xy" over the first placeholder: 2 bytes in, 1 out
    let delete = Edit {
        change: Change::Delete(
            p(0, 5),
            crate::text::Length {
                line_count: 0,
                byte_count: 1,
            },
        ),
        drift: Drift::Before,
    };
    let insert = Edit {
        change: Change::Insert(p(0, 5), Text::from("xy")),
        drift: Drift::Before,
    };
    s.apply_edit(&delete);
    s.apply_edit(&insert);
    assert!(s.active());
    assert_eq!(
        s.advance(),
        Some((p(0, 9), p(0, 10))),
        "`$2` moved one byte right"
    );
    assert_eq!(s.advance(), Some((p(0, 11), p(0, 11))), "`$0` too");
    assert!(!s.active());
    assert_eq!(s.advance(), None);
    s.start(vec![(p(1, 0), p(1, 0))]);
    s.end();
    assert!(!s.active(), "Esc ends it");
}
