//! The marks store: replaced per file, cleared, forgotten with their extension.

use super::{LineMarkStore, file_key};
use crawrad_extension_api::marks::{LineMark, SetLineMarksParams};
use std::path::Path;

fn mark(id: &str, line: u32) -> LineMark {
    LineMark {
        id: id.into(),
        line,
        start: 4,
        end: 8,
        label: "TODO".into(),
        kind: "todo".into(),
    }
}

fn set(file: &str, marks: Vec<LineMark>) -> SetLineMarksParams {
    SetLineMarksParams {
        file: file.into(),
        marks,
    }
}

#[test]
fn a_file_s_marks_are_replaced_cleared_and_forgotten() {
    let mut s = LineMarkStore::default();
    s.set(
        "todo",
        set("C:/p/src/a.rs", vec![mark("1", 2), mark("2", 5)]),
    );
    s.set("lint", set("C:/p/src/a.rs", vec![mark("x", 9)]));
    s.set("todo", set("C:/p/src/b.rs", vec![mark("3", 1)]));
    assert_eq!(
        s.on(Path::new("C:/p/src/a.rs")).len(),
        3,
        "two extensions, one file"
    );
    let r = s.revision();
    s.set("todo", set("C:/p/src/a.rs", vec![mark("1", 2)]));
    assert_eq!(
        s.on(Path::new("C:/p/src/a.rs")).len(),
        2,
        "the todo ones replaced"
    );
    assert!(s.revision() > r);
    s.set("todo", set("C:/p/src/a.rs", vec![]));
    assert_eq!(
        s.on(Path::new("C:/p/src/a.rs")).len(),
        1,
        "an empty list clears"
    );
    s.forget("lint");
    assert!(s.on(Path::new("C:/p/src/a.rs")).is_empty());
    assert_eq!(
        s.on(Path::new("C:/p/src/b.rs"))[0].0,
        "todo",
        "the owner is kept"
    );
}

#[test]
fn file_keys_ignore_separators_and_case_on_windows() {
    assert_eq!(file_key("C:\\p\\a.rs"), file_key("C:/p/a.rs"));
    if cfg!(windows) {
        assert_eq!(file_key("c:/P/A.rs"), file_key("C:/p/a.rs"));
    }
}
