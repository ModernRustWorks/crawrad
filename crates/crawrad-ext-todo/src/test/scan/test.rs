//! The scan, and what it becomes: marks per file (cleared when a file loses
//! its last one), the tree grouped by file, the list an agent reads.

use super::{Todo, VIEW_ID, list, marks, scan_project, scan_text, tree};
use std::path::Path;

#[test]
fn a_tag_counts_in_a_comment_in_capitals_as_a_word_of_its_own() {
    let text = "fn a() {} // TODO: wire it\n\
                let s = \"TODO is not a comment\";\n\
                // todo lower case is prose\n\
                // TODOS is another word, so is MYFIXME\n\
                /* FIXME(ana) - the leak */\n\
                 * HACK the sort\n\
                // see TODO later, and FIXME first? no: TODO wins being first\n";
    let found = scan_text(text);
    let short: Vec<_> = found
        .iter()
        .map(|(l, s, e, k, t)| (*l, *s, *e, *k, t.as_str()))
        .collect();
    assert_eq!(
        short,
        [
            (0, 13, 17, "todo", "wire it"),
            (4, 3, 8, "fixme", "the leak"),
            (5, 2, 6, "hack", "the sort"),
            (
                6,
                7,
                11,
                "todo",
                "later, and FIXME first? no: TODO wins being first"
            ),
        ]
    );
}

fn todo(file: &str, line: u32, kind: &str, text: &str) -> Todo {
    Todo {
        file: file.into(),
        line,
        start: 3,
        end: 7,
        kind: kind.into(),
        text: text.into(),
    }
}

#[test]
fn marks_per_file_and_a_file_that_lost_its_last_one_is_cleared() {
    let root = Path::new("C:/p");
    let now = [
        todo("src/a.rs", 2, "todo", "one"),
        todo("src/a.rs", 5, "fixme", "two"),
    ];
    let before = [todo("src/b.rs", 1, "hack", "gone")];
    let sent = marks(root, &now, &before);
    assert_eq!(sent.len(), 2);
    assert_eq!(sent[0].file, "C:/p/src/a.rs");
    assert_eq!(sent[0].marks.len(), 2);
    assert_eq!(
        (
            sent[0].marks[1].label.as_str(),
            sent[0].marks[1].kind.as_str()
        ),
        ("FIXME", "fixme")
    );
    assert_eq!(sent[0].marks[0].id, "src/a.rs:2");
    assert_eq!(sent[1].file, "C:/p/src/b.rs");
    assert!(sent[1].marks.is_empty(), "the editor is told to clear it");
}

#[test]
fn the_tree_groups_by_file_and_the_list_counts_from_one() {
    let tagged = [
        todo("src/a.rs", 2, "todo", "one"),
        todo("src/a.rs", 5, "fixme", "two"),
        todo("src/b.rs", 0, "hack", "three"),
    ];
    let t = tree(&tagged);
    assert_eq!(t.view_id, VIEW_ID);
    assert_eq!(t.items.len(), 2);
    assert_eq!(t.items[0].detail.as_deref(), Some("2"));
    let leaf = &t.items[0].children[1];
    assert_eq!(leaf.label, "FIXME: two");
    assert_eq!(leaf.detail.as_deref(), Some("line 6"));
    assert_eq!(
        leaf.location.as_ref().map(|l| (l.file.as_str(), l.line)),
        Some(("src/a.rs", 5))
    );
    let l = list(&tagged);
    assert_eq!(l["count"], 3);
    assert_eq!(l["todos"][2]["line"], 1);
    assert_eq!(l["todos"][2]["kind"], "hack");
}

#[test]
fn the_project_walk_reads_rust_sources_and_skips_target_and_hidden() {
    let root = std::env::temp_dir().join(format!("crawrad_todo_walk_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for dir in ["src", "target/debug", ".crawrad"] {
        std::fs::create_dir_all(root.join(dir)).expect("dir");
    }
    std::fs::write(root.join("src/a.rs"), "// TODO: here\n").expect("a");
    std::fs::write(root.join("src/notes.md"), "// TODO: not rust\n").expect("md");
    std::fs::write(root.join("target/debug/b.rs"), "// TODO: built\n").expect("b");
    std::fs::write(root.join(".crawrad/c.rs"), "// TODO: ide's\n").expect("c");
    let found = scan_project(&root);
    assert_eq!(found.len(), 1);
    assert_eq!(
        (found[0].file.as_str(), found[0].text.as_str()),
        ("src/a.rs", "here")
    );
    let _ = std::fs::remove_dir_all(&root);
}
