//! Placing the server's highlights on the editor's text.

use super::{DocumentHighlight, Occurrence, byte_to_utf16, placed};
use crate::encoding::PositionEncoding::Utf16 as U16;
use crate::protocol::{Position, Range};

fn at(line: u32, start: u32, end: u32, kind: Option<u8>) -> DocumentHighlight {
    DocumentHighlight {
        range: Range {
            start: Position {
                line,
                character: start,
            },
            end: Position {
                line,
                character: end,
            },
        },
        kind,
    }
}

/// **A read and a write of `total`, in bytes, the write told apart.**
#[test]
fn highlights_become_byte_ranges_with_the_write_marked() {
    let text = "let total = 1;\ntotal += 2;\n";
    let found = placed(&[at(0, 4, 9, Some(3)), at(1, 0, 5, Some(2))], text, U16);
    assert_eq!(
        found,
        vec![
            Occurrence {
                line: 0,
                start: 4,
                end: 9,
                write: true
            },
            Occurrence {
                line: 1,
                start: 0,
                end: 5,
                write: false
            },
        ]
    );
}

/// **The LSP counts UTF-16 units; the editor counts bytes.** `é` is one unit
/// and two bytes: the name after it starts one byte later than its column.
#[test]
fn a_column_after_an_accent_is_moved_to_its_byte() {
    let text = "let é = soma;\n";
    // `soma` is UTF-16 columns 8..12, bytes 9..13.
    let found = placed(&[at(0, 8, 12, None)], text, U16);
    assert_eq!(
        found,
        vec![Occurrence {
            line: 0,
            start: 9,
            end: 13,
            write: false
        }]
    );
    assert_eq!(byte_to_utf16("let é = soma;", 9), 8);
}

/// **A range across lines, or past the text, is dropped, not drawn wrong.**
#[test]
fn a_range_that_cannot_be_drawn_is_dropped() {
    let text = "x\n";
    let across = DocumentHighlight {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 1,
                character: 1,
            },
        },
        kind: None,
    };
    assert!(placed(&[across, at(5, 0, 1, None)], text, U16).is_empty());
}
