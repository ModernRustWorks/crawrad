//! The gate over how a file says it was written.

use super::*;

/// **A file with no newline at all is LF.**
///
/// It is the generator's own first output, and answering `CRLF` for it would
/// make the first save rewrite every line of a file nobody edited.
#[test]
fn a_file_with_no_newline_is_lf() {
    assert_eq!(LineEnding::detect(""), LineEnding::Lf);
    assert_eq!(LineEnding::detect("fn main() {}"), LineEnding::Lf);
}

/// **The FIRST ending decides, even when the file mixes them.**
///
/// A mixed file is already inconsistent, and the IDE is not a normalisation
/// tool. What it must not do is the opposite — silently rewrite the endings of
/// a whole file nobody asked it to touch.
#[test]
fn the_first_ending_decides_for_the_whole_file() {
    assert_eq!(LineEnding::detect("a\r\nb\nc\n"), LineEnding::CrLf);
    assert_eq!(LineEnding::detect("a\nb\r\nc\r\n"), LineEnding::Lf);
}

/// **A file that starts with a newline is LF, and does not read out of bounds.**
///
/// The `Some(0)` arm is not style: without it the code asks for `bytes[p - 1]`
/// with `p == 0` and panics on a file whose first character is a newline.
#[test]
fn a_leading_newline_is_lf_and_does_not_panic() {
    assert_eq!(LineEnding::detect("\nrest"), LineEnding::Lf);
    assert_eq!(LineEnding::detect("\n"), LineEnding::Lf);
}

/// **Applying an ending is the exact inverse of normalising it.**
#[test]
fn applying_crlf_puts_every_ending_back() {
    assert_eq!(LineEnding::CrLf.apply("a\nb\n"), "a\r\nb\r\n");
    assert_eq!(LineEnding::Lf.apply("a\nb\n"), "a\nb\n");
    // And it does not duplicate a `\r` that is already there: text arriving here
    // is NORMALIZED, which is what the caller promises.
    assert_eq!(LineEnding::CrLf.apply("without newline"), "without newline");
}

/// **The BOM is read off the BYTES, and it is stripped before the ending is
/// looked for.**
#[test]
fn the_bom_is_found_in_the_bytes() {
    let with_bom = [BOM, b"a\r\nb"].concat();
    let e = Encoding::of(&with_bom);
    assert!(e.bom, "the byte order mark was not seen");
    assert_eq!(e.line_ending, LineEnding::CrLf);

    let without_bom = Encoding::of(b"a\nb");
    assert!(!without_bom.bom);
    assert_eq!(without_bom.line_ending, LineEnding::Lf);
}

/// **A file that merely CONTAINS those bytes later is not a BOM file.**
///
/// The mark is a prefix, and `contains` would call any file with a stray
/// `U+FEFF` in the middle a BOM file — and then the save would put a second
/// one at the front.
#[test]
fn the_mark_only_counts_at_the_front() {
    let in_middle = [b"fn main() {}", BOM].concat();
    assert!(!Encoding::of(&in_middle).bom);
}

/// **The four labels are the four combinations, and none repeats.**
///
/// The footer is the only place the user learns any of this, so a label that
/// says the same as another is a footer that stopped answering.
#[test]
fn the_four_labels_are_distinct() {
    let all: Vec<&str> = [
        (true, LineEnding::CrLf),
        (true, LineEnding::Lf),
        (false, LineEnding::CrLf),
        (false, LineEnding::Lf),
    ]
    .into_iter()
    .map(|(bom, line_ending)| Encoding { bom, line_ending }.label())
    .collect();
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a, b, "two combinations write the same label");
        }
    }
    // And the common case — what the generator writes — is the shortest.
    assert_eq!(Encoding::default().label(), "UTF-8  LF");
}

/// **Empty bytes do not panic and answer the generator's own default.**
#[test]
fn an_empty_file_answers_the_default() {
    assert_eq!(Encoding::of(b""), Encoding::default());
}

/// **What was read comes back byte for byte — the round trip.**
///
/// This is the property that makes saving safe, and the one a user notices the
/// day it breaks: a file found with CRLF and a byte order mark has to go back
/// with both. The editor holds neither — it normalises on open, because a `\r`
/// in the middle draws as a missing-glyph box — so `to_bytes` is the only thing
/// standing between a one-word edit and a diff of every line in the file.
#[test]
fn what_was_read_comes_back_byte_for_byte() {
    for raw in [
        &b"line one\nother\n"[..],
        &b"line one\r\nother\r\n"[..],
        &b"\xEF\xBB\xBFline one\nother\n"[..],
        &b"\xEF\xBB\xBFline one\r\nother\r\n"[..],
    ] {
        let e = Encoding::of(raw);
        // What the editor stores: no mark, and always `\n`.
        let text = String::from_utf8(raw.to_vec())
            .expect("test setup is UTF-8")
            .trim_start_matches('\u{FEFF}')
            .replace("\r\n", "\n");
        assert_eq!(
            e.to_bytes(&text),
            raw,
            "round trip changed file, and {} was read",
            e.label()
        );
    }
}

/// **A file with no mark does not gain one, and one with a mark does not lose
/// it.**
///
/// The two halves are separate mistakes: adding a BOM breaks `#!` scripts and
/// some compilers, and dropping one changes a file that nobody edited.
#[test]
fn the_mark_is_neither_invented_nor_dropped() {
    let without = Encoding {
        bom: false,
        line_ending: LineEnding::Lf,
    };
    assert!(!without.to_bytes("x").starts_with(crate::encoding::BOM));

    let with = Encoding {
        bom: true,
        line_ending: LineEnding::Lf,
    };
    assert!(with.to_bytes("x").starts_with(crate::encoding::BOM));
    assert_eq!(with.to_bytes("x").len(), crate::encoding::BOM.len() + 1);
}

#[test]
fn file_encoding_detects_bytes_from_disk_or_defaults() {
    assert_eq!(file_encoding(None), Encoding::default());
    let missing = std::path::Path::new("nonexistent/test/path/file.rs");
    assert_eq!(file_encoding(Some(missing)), Encoding::default());
}
