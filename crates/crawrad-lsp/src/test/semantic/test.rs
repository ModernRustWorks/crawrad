//! Semantic tokens: the classes, the relative data decoded, the delta.

use super::{Class, Legend, Span, apply_delta, decode, next_data};
use crate::encoding::PositionEncoding;
use serde_json::json;

fn legend() -> Legend {
    // The first entries of rust-analyzer's legend (captured 2026-10-04).
    let types = [
        "comment",
        "decorator",
        "enumMember",
        "enum",
        "function",
        "interface",
        "keyword",
        "macro",
        "method",
        "namespace",
        "number",
        "operator",
        "parameter",
        "property",
        "string",
        "struct",
        "typeParameter",
        "variable",
    ];
    let mods = [
        "async",
        "documentation",
        "declaration",
        "static",
        "defaultLibrary",
        "deprecated",
        "associated",
        "attribute",
        "callable",
        "constant",
        "consuming",
        "controlFlow",
        "crateRoot",
        "injected",
        "intraDocLink",
        "library",
        "macro",
        "mutable",
        "unsafe",
    ];
    Legend {
        types: types.iter().map(|s| s.to_string()).collect(),
        modifiers: mods.iter().map(|s| s.to_string()).collect(),
    }
}

/// **What the editor colours** — the rest keeps the tokenizer's colour.
#[test]
fn the_classes_come_from_type_and_modifiers() {
    let l = legend();
    let mutable = 1 << 17;
    assert_eq!(l.class(7, 0), Some(Class::Macro));
    assert_eq!(l.class(5, 0), Some(Class::Trait));
    assert_eq!(l.class(17, mutable), Some(Class::Mutable), "a mut binding");
    assert_eq!(
        l.class(12, mutable),
        Some(Class::Mutable),
        "a mut parameter"
    );
    assert_eq!(
        l.class(17, 1 << 2),
        None,
        "a plain binding keeps its colour"
    );
    assert_eq!(l.class(12, 0), Some(Class::Parameter));
    assert_eq!(l.class(13, 0), Some(Class::Field));
    assert_eq!(l.class(2, 0), Some(Class::EnumMember));
    assert_eq!(l.class(6, 0), None, "keywords are the tokenizer's");
    assert_eq!(l.class(99, 0), None, "an index past the legend");
}

/// The `unsafe` keyword (type `keyword`, modifier `unsafe`) gets
/// its own colour, over any other class; `deprecated` is apart from the
/// class, so a plain function still comes out struck through.
#[test]
fn unsafe_is_a_class_and_deprecated_a_flag() {
    let (l, unsafe_, deprecated) = (legend(), 1 << 18, 1 << 5);
    assert_eq!(l.class(6, unsafe_), Some(Class::Unsafe), "`unsafe` keyword");
    assert_eq!(
        l.class(17, unsafe_ | 1 << 17),
        Some(Class::Unsafe),
        "static mut"
    );
    assert!(l.deprecated(deprecated) && !l.deprecated(unsafe_));
    // `stale` (function, deprecated) and `fresh` (function): only `stale`
    // is kept, with no colour of its own.
    let text = "stale(); fresh();";
    let data = [0, 0, 5, 4, deprecated, 0, 9, 5, 4, 0];
    assert_eq!(
        decode(&data, &l, text, PositionEncoding::Utf8),
        vec![Span {
            line: 0,
            start: 0,
            end: 5,
            class: None,
            deprecated: true
        }]
    );
}

/// **Relative positions, columns in the position encoding.** Line 0 has
/// `ção` (5 bytes, 3 chars) before a `mut` binding; line 1 a macro.
#[test]
fn relative_data_becomes_byte_spans() {
    let text = "let ção = 1; let mut n = 2;\n    dobro!(n);";
    let mutable = 1 << 17;
    // utf-16 / chars: `n` starts at char 21; in utf-8 at byte 23.
    let utf16 = [0, 21, 1, 17, mutable, 1, 4, 5, 7, 0];
    let spans = decode(&utf16, &legend(), text, PositionEncoding::Utf16);
    assert_eq!(
        spans,
        vec![
            Span {
                line: 0,
                start: 23,
                end: 24,
                class: Some(Class::Mutable),
                deprecated: false
            },
            Span {
                line: 1,
                start: 4,
                end: 9,
                class: Some(Class::Macro),
                deprecated: false
            },
        ]
    );
    let utf8 = [0, 23, 1, 17, mutable, 1, 4, 5, 7, 0];
    assert_eq!(
        decode(&utf8, &legend(), text, PositionEncoding::Utf8),
        spans
    );
    // Two tokens on one line: the second start is RELATIVE to the first.
    let same_line = [0, 4, 3, 7, 0, 0, 6, 1, 17, mutable];
    let spans = decode(
        &same_line,
        &legend(),
        "abc dobro x n",
        PositionEncoding::Utf8,
    );
    assert_eq!((spans[1].start, spans[1].end), (10, 11), "4 + 6, not 6");
    let unclassified = [0, 0, 3, 6, 0];
    assert!(decode(&unclassified, &legend(), text, PositionEncoding::Utf8).is_empty());
}

/// **The delta:** edits applied from the last to the first, against the
/// kept data; a `data` answer is whole; a bad edit is no answer.
#[test]
fn a_delta_edits_the_kept_data() {
    let prev = [0, 0, 3, 6, 0, 0, 4, 5, 17, 0];
    let edits = [
        json!({ "start": 5, "deleteCount": 5, "data": [0, 4, 6, 17, 0] }),
        json!({ "start": 0, "deleteCount": 0, "data": [0, 0, 1, 11, 0] }),
    ];
    assert_eq!(
        apply_delta(&prev, &edits).unwrap(),
        vec![0, 0, 1, 11, 0, 0, 0, 3, 6, 0, 0, 4, 6, 17, 0]
    );
    assert_eq!(
        apply_delta(
            &prev,
            &[json!({ "start": 8, "deleteCount": 5, "data": [] })]
        ),
        None
    );

    let delta =
        json!({ "resultId": "2", "edits": [{ "start": 2, "deleteCount": 1, "data": [9] }] });
    let (id, data) = next_data(&delta, Some(&prev)).unwrap();
    assert_eq!((id.as_deref(), data[2]), (Some("2"), 9));
    let full = json!({ "resultId": "3", "data": [0, 0, 1, 7, 0] });
    assert_eq!(next_data(&full, None).unwrap().1, vec![0, 0, 1, 7, 0]);
    assert_eq!(next_data(&delta, None), None, "a delta needs the kept data");
}

/// The legend from the server's capabilities.
#[test]
fn the_legend_comes_from_the_capabilities() {
    let caps = json!({ "semanticTokensProvider": { "legend": {
        "tokenTypes": ["macro", "interface"], "tokenModifiers": ["mutable"] }, "full": { "delta": true } } });
    let l = Legend::from_capabilities(Some(&caps));
    assert_eq!(l.types, vec!["macro", "interface"]);
    assert_eq!(l.modifiers, vec!["mutable"]);
    assert_eq!(Legend::from_capabilities(None), Legend::default());
}
