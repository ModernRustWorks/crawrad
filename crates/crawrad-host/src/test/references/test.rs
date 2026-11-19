//! The References panel: groups, marks, rows, the line cut, the places.

use super::{AFTER, BEFORE, Hit, References, Row, cut, same_file};
use std::path::PathBuf;

const SUFFIX: &str = ".designer.rs";

fn hit(line: usize, text: &str, word: &str) -> Hit {
    let start = text.find(word).unwrap();
    Hit {
        line,
        start,
        end: start + word.len(),
        text: text.into(),
    }
}

fn found() -> References {
    let roots = vec![PathBuf::from("C:/Users/u/.rustup")];
    let mut r = References::default();
    r.show(
        "area",
        vec![
            (
                PathBuf::from("C:/p/src/main.rs"),
                hit(12, "    let b = area(3);", "area"),
            ),
            (
                PathBuf::from("C:/p/src/form_main.designer.rs"),
                hit(8, "        area(0);", "area"),
            ),
            (
                PathBuf::from("C:/p/src/main.rs"),
                hit(4, "    let a = area(2);", "area"),
            ),
            (
                PathBuf::from("C:/p/src/area.rs"),
                hit(2, "pub fn area(x: i32) -> i32 {", "area"),
            ),
            (
                PathBuf::from("C:/Users/u/.rustup/lib.rs"),
                hit(0, "area", "area"),
            ),
        ],
        &roots,
        SUFFIX,
    );
    r
}

/// **One group per file, by path; occurrences by line; the title counts.**
#[test]
fn grouped_by_file_sorted_and_counted() {
    let r = found();
    let files: Vec<_> = r
        .groups
        .iter()
        .map(|g| g.file.to_string_lossy().to_string())
        .collect();
    assert_eq!(
        files,
        [
            "C:/Users/u/.rustup/lib.rs",
            "C:/p/src/area.rs",
            "C:/p/src/form_main.designer.rs",
            "C:/p/src/main.rs"
        ]
    );
    let main = &r.groups[3];
    assert_eq!(
        main.hits.iter().map(|h| h.line).collect::<Vec<_>>(),
        [4, 12]
    );
    assert_eq!(r.total(), 5);
    assert_eq!(r.title(), "`area` \u{2014} 5 references in 4 files");
}

/// **The generated form source is marked generated, the toolchain
/// read-only; both open locked; the user's files carry no mark.**
#[test]
fn the_generated_source_and_the_toolchain_are_marked() {
    let r = found();
    let marks: Vec<_> = r
        .rows()
        .into_iter()
        .filter_map(|row| match row {
            Row::File { name, mark, .. } => Some((name, mark)),
            Row::Hit { .. } => None,
        })
        .collect();
    assert_eq!(
        marks,
        [
            ("lib.rs".to_string(), Some("read-only")),
            ("area.rs".to_string(), None),
            ("form_main.designer.rs".to_string(), Some("generated")),
            ("main.rs".to_string(), None),
        ]
    );
    let locked: Vec<bool> = r.groups.iter().map(|g| g.read_only).collect();
    assert_eq!(locked, [true, false, true, false]);
}

/// **A closed group hides its hits; a click on a hit is the place of the
/// name, in the right file.**
#[test]
fn closing_a_group_hides_its_hits_and_a_hit_is_a_place() {
    let mut r = found();
    assert_eq!(r.rows().len(), 4 + 5);
    r.toggle(3);
    assert_eq!(r.rows().len(), 4 + 3);
    assert!(matches!(
        r.rows().last(),
        Some(Row::File {
            open: false,
            count: 2,
            ..
        })
    ));
    let p = r.place(3, 1).unwrap();
    assert_eq!(
        (p.file, p.line, p.byte),
        (PathBuf::from("C:/p/src/main.rs"), 12, 12)
    );
    assert!(r.place(9, 0).is_none() && r.place(3, 9).is_none());
}

/// **The hit row: 1-based line, indent dropped, the name apart.**
#[test]
fn a_hit_row_is_the_line_in_three() {
    let r = found();
    let row = r.rows().into_iter().find(|row| {
        matches!(
            row,
            Row::Hit {
                group: 3,
                hit: 0,
                ..
            }
        )
    });
    assert_eq!(
        row,
        Some(Row::Hit {
            group: 3,
            hit: 0,
            line: "5".into(),
            before: "let a = ".into(),
            word: "area".into(),
            after: "(2);".into(),
        })
    );
}

/// **A long line is cut around the name, never inside a character.**
#[test]
fn a_long_line_is_cut_around_the_name() {
    let pre = "\u{1F980}".repeat(BEFORE + 10);
    let post = "\u{e9}".repeat(AFTER + 10);
    let text = format!("{pre}area{post}");
    let (before, word, after) = cut(&hit(0, &text, "area"));
    assert_eq!(word, "area");
    assert_eq!(before.chars().count(), BEFORE + 1);
    assert!(before.starts_with('\u{2026}'));
    assert_eq!(after.chars().count(), AFTER + 1);
    assert!(after.ends_with('\u{2026}'));
    // A span past the line's end, or inside a character, does not panic.
    let odd = Hit {
        line: 0,
        start: 1,
        end: 999,
        text: "\u{e9}x".into(),
    };
    assert_eq!(cut(&odd), (String::new(), "\u{e9}x".into(), String::new()));
}

/// **Nothing found is said by name; before any search, how to start.**
#[test]
fn nothing_and_never_asked_are_said() {
    let mut r = References::default();
    assert!(r.title().contains("put the caret"));
    r.show("ghost", vec![], &[], SUFFIX);
    assert_eq!(r.title(), "No references to `ghost`");
    assert!(r.rows().is_empty());
}

/// **What MCP reads: every file with its marks and its occurrences.**
#[test]
fn the_json_has_the_files_marks_and_lines() {
    let j = found().to_json();
    assert_eq!(j["total"], 5);
    let designer = &j["files"][2];
    assert_eq!(designer["generated"], true);
    assert_eq!(designer["hits"][0]["line"], 9);
    assert_eq!(designer["hits"][0]["word"], "area");
    assert_eq!(j["files"][3]["hits"][1]["text"], "let b = area(3);");
}

/// **The server's path and the editor's are the same file whatever the
/// drive letter's case or the slashes; another file is not.**
#[test]
fn same_file_ignores_case_and_slashes() {
    use std::path::Path;
    assert!(same_file(
        Path::new("c:/p/src/main.rs"),
        Path::new("C:\\p\\src\\main.rs")
    ));
    assert!(!same_file(
        Path::new("c:/p/src/main.rs"),
        Path::new("C:/p/src/area.rs")
    ));
}
