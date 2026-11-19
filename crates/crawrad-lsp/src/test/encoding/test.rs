//! Columns in the three encodings, and the way back.

use super::PositionEncoding::{self, Utf8, Utf16, Utf32};

const LINE: &str = "let s = \"über🦀\"; s.";

/// **The same place, three columns:** after `"über🦀"` the byte is 19 (ü is
/// 2 bytes, 🦀 4), the UTF-16 column 16 (🦀 is 2 units), the char column 15.
#[test]
fn one_place_three_columns() {
    let byte = LINE.find("; s.").unwrap();
    assert_eq!(byte, 19);
    assert_eq!(Utf8.column(LINE, byte), 19);
    assert_eq!(Utf16.column(LINE, byte), 16);
    assert_eq!(Utf32.column(LINE, byte), 15);
}

/// **Every char boundary round-trips in every encoding.**
#[test]
fn every_boundary_round_trips() {
    for enc in [Utf8, Utf16, Utf32] {
        for (byte, _) in LINE
            .char_indices()
            .chain(std::iter::once((LINE.len(), ' ')))
        {
            assert_eq!(
                enc.byte(LINE, enc.column(LINE, byte)),
                byte,
                "{enc:?} at {byte}"
            );
        }
    }
}

/// **Out of range clamps to the line; inside a character goes to its start.**
#[test]
fn clamps_and_never_splits_a_character() {
    assert_eq!(Utf16.byte(LINE, 999), LINE.len());
    assert_eq!(Utf8.byte(LINE, 999), LINE.len());
    let crab = LINE.find('🦀').unwrap();
    assert_eq!(Utf8.byte(LINE, (crab + 2) as u32), crab, "middle of 🦀");
    assert_eq!(Utf8.column(LINE, crab + 1), crab as u32, "middle of 🦀");
}

/// **The server decides; absent means UTF-16** (the protocol's default).
#[test]
fn the_server_decides() {
    use serde_json::json;
    let pick = |v: serde_json::Value| PositionEncoding::from_capabilities(Some(&v));
    assert_eq!(pick(json!({ "positionEncoding": "utf-8" })), Utf8);
    assert_eq!(pick(json!({ "positionEncoding": "utf-16" })), Utf16);
    assert_eq!(pick(json!({ "positionEncoding": "utf-32" })), Utf32);
    assert_eq!(pick(json!({})), Utf16);
    assert_eq!(PositionEncoding::from_capabilities(None), Utf16);
}

fn pos(line: u32, character: u32) -> crate::protocol::Position {
    crate::protocol::Position { line, character }
}

/// **Every consumer reads the column in the negotiated unit:** the same `s`
/// after `"über🦀"` is column 21 in UTF-8 and 18 in UTF-16, and both land on
/// byte 21 — a squiggle, a highlight, an inlay, an edit.
#[test]
fn every_consumer_lands_on_the_same_byte() {
    use crate::protocol::Range;
    let text = format!("fn f() {{}}\n{LINE}len()\n");
    let s_byte = LINE.rfind('s').unwrap();
    assert_eq!(s_byte, 21);
    let lines: Vec<String> = text.lines().map(String::from).collect();
    for (enc, col) in [(Utf8, 21), (Utf16, 18)] {
        assert_eq!(enc.place(&lines, pos(1, col)), (1, s_byte), "{enc:?}");

        let h = crate::highlight::DocumentHighlight {
            range: Range {
                start: pos(1, col),
                end: pos(1, col + 1),
            },
            kind: None,
        };
        let found = crate::highlight::placed(&[h], &text, enc);
        assert_eq!(
            (found[0].start, found[0].end),
            (s_byte, s_byte + 1),
            "{enc:?}"
        );

        let e = crate::edit::TextEdit {
            range: Range {
                start: pos(1, col),
                end: pos(1, col + 1),
            },
            new_text: "t".into(),
        };
        let out = crate::edit::apply_text_edits(&text, &[e], enc);
        assert!(out.contains("🦀\"; t.len()"), "{enc:?}: {out}");
    }
}

/// **The delta in UTF-8 rebuilds the new text** across accents and emoji,
/// and its columns are bytes.
#[test]
fn the_delta_in_utf8_rebuilds_the_text() {
    use crate::sync::{apply_change, incremental_change};
    let old = format!("{LINE}\n");
    for new in [
        format!("{LINE}len\n"),
        LINE.replace("🦀", "🦀🦀"),
        LINE.replace("über", "uber"),
        format!("// é\n{LINE}\n"),
    ] {
        let change = incremental_change(&old, &new, Utf8).unwrap();
        assert_eq!(apply_change(&old, &change, Utf8), new);
    }
    let change = incremental_change(&old, &format!("{LINE}len\n"), Utf8).unwrap();
    assert_eq!(
        change.range.start,
        pos(0, LINE.len() as u32),
        "a byte column"
    );
}
