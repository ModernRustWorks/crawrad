//! A rename planned against the editor's text: every file or none.

use super::{Planned, plan};
use crate::edit::{TextEdit, WorkspaceEdit};
use crate::encoding::PositionEncoding::Utf8;
use crate::protocol::{Position, Range};
use std::path::{Path, PathBuf};

fn edit(line: u32, from: u32, to: u32, text: &str) -> TextEdit {
    TextEdit {
        range: Range {
            start: Position {
                line,
                character: from,
            },
            end: Position {
                line,
                character: to,
            },
        },
        new_text: text.into(),
    }
}

fn uri(p: &str) -> String {
    crate::path_to_uri(Path::new(p))
}

/// **Each file's text from the reader, its edits applied, counted, in path
/// order** — the reader's text (an unsaved buffer), not the disk.
#[test]
fn every_file_is_planned_from_the_editors_text() {
    let mut edits = WorkspaceEdit::new();
    edits.insert(
        uri("C:/p/src/main.rs"),
        vec![edit(0, 8, 12, "size"), edit(1, 4, 8, "size")],
    );
    edits.insert(uri("C:/p/src/area.rs"), vec![edit(0, 7, 11, "size")]);
    let mut read = |f: &Path| match f.ends_with("main.rs") {
        true => Some("let a = area(1);\nlet area = 2;\n".to_string()),
        false => Some("pub fn area() {}\n".to_string()),
    };
    let got = plan(&edits, Utf8, &mut read).unwrap();
    let files: Vec<_> = got
        .iter()
        .map(|p| p.file.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert_eq!(files, ["area.rs", "main.rs"]);
    assert_eq!(got[0].after, "pub fn size() {}\n");
    assert_eq!(got[1].after, "let a = size(1);\nlet size = 2;\n");
    assert_eq!(got[1].count, 2);
    assert_eq!(got[1].before, "let a = area(1);\nlet area = 2;\n");
}

/// **A file that cannot be read stops the whole plan: nothing is applied.**
#[test]
fn an_unreadable_file_stops_the_whole_plan() {
    let mut edits = WorkspaceEdit::new();
    edits.insert(uri("C:/p/src/a.rs"), vec![edit(0, 0, 1, "b")]);
    edits.insert(uri("C:/p/src/gone.rs"), vec![edit(0, 0, 1, "b")]);
    let mut read = |f: &Path| (!f.ends_with("gone.rs")).then(|| "a\n".to_string());
    let got: Result<Vec<Planned>, PathBuf> = plan(&edits, Utf8, &mut read);
    assert!(got.unwrap_err().ends_with("gone.rs"));
}
