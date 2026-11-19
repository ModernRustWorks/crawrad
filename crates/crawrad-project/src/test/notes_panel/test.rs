//! Unit tests for the NotesPanel model and indexer.

use super::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "crawrad-notes-pnl-{tag}-{}-{id}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn sample_notes() -> Vec<IndexedNote> {
    vec![
        IndexedNote {
            id: "doc-1".to_string(),
            kind: NoteKind::Doc,
            title: "Doc 1".to_string(),
            file: Some("src/a.rs".to_string()),
            line: Some(10),
            resolved: false,
            alert: Some("Signature changed".to_string()),
        },
        IndexedNote {
            id: "doc-2".to_string(),
            kind: NoteKind::Doc,
            title: "Doc 2".to_string(),
            file: Some("src/b.rs".to_string()),
            line: Some(5),
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "int-1".to_string(),
            kind: NoteKind::Internal,
            title: "Internal 1".to_string(),
            file: Some("src/a.rs".to_string()),
            line: Some(2),
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "int-2".to_string(),
            kind: NoteKind::Internal,
            title: "Internal 2".to_string(),
            file: None,
            line: None,
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "cmt-1".to_string(),
            kind: NoteKind::Comment,
            title: "Comment 1".to_string(),
            file: Some("src/a.rs".to_string()),
            line: Some(20),
            resolved: true,
            alert: None,
        },
        IndexedNote {
            id: "cmt-2".to_string(),
            kind: NoteKind::Comment,
            title: "Comment 2".to_string(),
            file: Some("src/b.rs".to_string()),
            line: Some(15),
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "cmt-3".to_string(),
            kind: NoteKind::Comment,
            title: "Comment 3".to_string(),
            file: None,
            line: None,
            resolved: false,
            alert: None,
        },
    ]
}

#[test]
fn test_counts_over_all_notes() {
    let notes = sample_notes();
    let panel = NotesPanel::new(notes);

    let counts = panel.counts();
    assert_eq!(counts.all, 7);
    assert_eq!(counts.doc, 2);
    assert_eq!(counts.internal, 2);
    assert_eq!(counts.comment, 3);
    assert_eq!(counts.alert, 1);
}

#[test]
fn test_kind_chips_filtering() {
    let mut panel = NotesPanel::new(sample_notes());

    // KindFilter::Doc
    panel.kind = KindFilter::Doc;
    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(note_ids, vec!["doc-1", "doc-2"]);

    // KindFilter::Internal
    panel.kind = KindFilter::Internal;
    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(note_ids, vec!["int-1", "int-2"]);

    // KindFilter::Comment
    panel.kind = KindFilter::Comment;
    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(note_ids, vec!["cmt-1", "cmt-2", "cmt-3"]);

    // KindFilter::All
    panel.kind = KindFilter::All;
    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(note_ids.len(), 7);
}

#[test]
fn test_file_filter_and_drop_on_vanish() {
    let mut panel = NotesPanel::new(sample_notes());

    // Filter to "src/a.rs"
    panel.file = Some("src/a.rs".to_string());
    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    // Unanchored notes must not show when a file is selected
    assert_eq!(note_ids, vec!["int-1", "doc-1", "cmt-1"]);

    // Files list
    assert_eq!(panel.files(), vec!["src/a.rs", "src/b.rs"]);

    // Set notes without src/a.rs -> file filter drops to None
    let remaining_notes = vec![IndexedNote {
        id: "doc-2".to_string(),
        kind: NoteKind::Doc,
        title: "Doc 2".to_string(),
        file: Some("src/b.rs".to_string()),
        line: Some(5),
        resolved: false,
        alert: None,
    }];
    panel.set_notes(remaining_notes);
    assert_eq!(panel.file, None);
}

#[test]
fn test_in_alert_filter() {
    let mut panel = NotesPanel::new(sample_notes());
    panel.in_alert = true;

    let rows = panel.rows();
    let note_ids: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Note { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(note_ids, vec!["doc-1"]);
}

#[test]
fn test_group_ordering_and_without_anchor_last() {
    let notes = vec![
        IndexedNote {
            id: "z-note".to_string(),
            kind: NoteKind::Internal,
            title: "Z Note".to_string(),
            file: Some("src/z.rs".to_string()),
            line: Some(1),
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "a-note".to_string(),
            kind: NoteKind::Internal,
            title: "A Note".to_string(),
            file: Some("src/a.rs".to_string()),
            line: Some(1),
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "unanchored-note".to_string(),
            kind: NoteKind::Internal,
            title: "Unanchored".to_string(),
            file: None,
            line: None,
            resolved: false,
            alert: None,
        },
    ];

    let panel = NotesPanel::new(notes);
    let rows = panel.rows();

    let groups: Vec<(&str, &str)> = rows
        .iter()
        .filter_map(|r| match r {
            PanelRow::Group { key, title, .. } => Some((key.as_str(), title.as_str())),
            _ => None,
        })
        .collect();

    assert_eq!(
        groups,
        vec![
            ("src/a.rs", "src/a.rs"),
            ("src/z.rs", "src/z.rs"),
            ("", "Without anchor")
        ]
    );
}

#[test]
fn test_collapse_group() {
    let mut panel = NotesPanel::new(sample_notes());

    panel.toggle_group("src/a.rs");

    let rows = panel.rows();
    let src_a_group = rows.iter().find(|r| match r {
        PanelRow::Group { key, .. } => key == "src/a.rs",
        _ => false,
    });

    match src_a_group {
        Some(PanelRow::Group {
            count, collapsed, ..
        }) => {
            assert_eq!(*count, 3);
            assert!(*collapsed);
        }
        _ => panic!("src/a.rs group header not found"),
    }

    // No notes from src/a.rs should be present
    for row in &rows {
        if let PanelRow::Note { file, .. } = row {
            assert_ne!(file.as_deref(), Some("src/a.rs"));
        }
    }

    // Toggle again to expand
    panel.toggle_group("src/a.rs");
    let rows_expanded = panel.rows();
    let has_src_a_note = rows_expanded.iter().any(|r| match r {
        PanelRow::Note { file, .. } => file.as_deref() == Some("src/a.rs"),
        _ => false,
    });
    assert!(has_src_a_note);
}

#[test]
fn test_resolved_comment_faded_and_struck() {
    let notes = vec![
        IndexedNote {
            id: "resolved-cmt".to_string(),
            kind: NoteKind::Comment,
            title: "Resolved Comment".to_string(),
            file: Some("src/lib.rs".to_string()),
            line: Some(10),
            resolved: true,
            alert: None,
        },
        IndexedNote {
            id: "open-cmt".to_string(),
            kind: NoteKind::Comment,
            title: "Open Comment".to_string(),
            file: Some("src/lib.rs".to_string()),
            line: Some(20),
            resolved: false,
            alert: None,
        },
    ];

    let panel = NotesPanel::new(notes);
    let rows = panel.rows();

    let resolved_row = rows
        .iter()
        .find(|r| match r {
            PanelRow::Note { id, .. } => id == "resolved-cmt",
            _ => false,
        })
        .unwrap();

    match resolved_row {
        PanelRow::Note {
            badge,
            faded,
            struck,
            line_text,
            ..
        } => {
            assert_eq!(*badge, Some(("Resolved".to_string(), BadgeTone::Done)));
            assert!(*faded);
            assert!(*struck);
            assert_eq!(line_text, "10");
        }
        _ => unreachable!(),
    }

    let open_row = rows
        .iter()
        .find(|r| match r {
            PanelRow::Note { id, .. } => id == "open-cmt",
            _ => false,
        })
        .unwrap();

    match open_row {
        PanelRow::Note {
            badge,
            faded,
            struck,
            ..
        } => {
            assert_eq!(*badge, None);
            assert!(!*faded);
            assert!(!*struck);
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_no_anchor_badge_and_line_text() {
    let notes = vec![
        IndexedNote {
            id: "unanchored-int".to_string(),
            kind: NoteKind::Internal,
            title: "Unanchored Internal".to_string(),
            file: None,
            line: None,
            resolved: false,
            alert: None,
        },
        IndexedNote {
            id: "unanchored-doc".to_string(),
            kind: NoteKind::Doc,
            title: "Unanchored Doc".to_string(),
            file: None,
            line: None,
            resolved: false,
            alert: Some("Orphan".to_string()),
        },
    ];

    let panel = NotesPanel::new(notes);
    let rows = panel.rows();

    let int_row = rows
        .iter()
        .find(|r| match r {
            PanelRow::Note { id, .. } => id == "unanchored-int",
            _ => false,
        })
        .unwrap();

    match int_row {
        PanelRow::Note {
            badge, line_text, ..
        } => {
            assert_eq!(*badge, Some(("No anchor".to_string(), BadgeTone::Done)));
            assert_eq!(line_text, "\u{2014}");
        }
        _ => unreachable!(),
    }

    let doc_row = rows
        .iter()
        .find(|r| match r {
            PanelRow::Note { id, .. } => id == "unanchored-doc",
            _ => false,
        })
        .unwrap();

    match doc_row {
        PanelRow::Note {
            badge, line_text, ..
        } => {
            assert_eq!(*badge, Some(("Orphan".to_string(), BadgeTone::Warn)));
            assert_eq!(line_text, "\u{2014}");
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_footer_singular_plural() {
    let empty_panel = NotesPanel::new(Vec::new());
    assert_eq!(empty_panel.footer(), ".crawrad/notes \u{00B7} 0 files");

    let single_panel = NotesPanel::new(vec![IndexedNote {
        id: "one".to_string(),
        kind: NoteKind::Doc,
        title: "One".to_string(),
        file: None,
        line: None,
        resolved: false,
        alert: None,
    }]);
    assert_eq!(single_panel.footer(), ".crawrad/notes \u{00B7} 1 file");

    let multi_panel = NotesPanel::new(sample_notes());
    assert_eq!(multi_panel.footer(), ".crawrad/notes \u{00B7} 7 files");
}

#[test]
fn test_chip_alert_and_file_labels() {
    let mut panel = NotesPanel::new(sample_notes());

    assert_eq!(panel.chip_label(KindFilter::All), "All 7");
    assert_eq!(panel.chip_label(KindFilter::Doc), "Documentation 2");
    assert_eq!(panel.chip_label(KindFilter::Internal), "Notes 2");
    assert_eq!(panel.chip_label(KindFilter::Comment), "Comments 3");

    assert_eq!(panel.alert_label(), "In alert 1");

    assert_eq!(panel.file_label(), "All files");
    panel.file = Some("src/main.rs".to_string());
    assert_eq!(panel.file_label(), "src/main.rs");
}

#[test]
fn test_alert_reason_stale_signature() {
    let note = SidecarNote {
        id: "calc-fn".to_string(),
        kind: NoteKind::Doc,
        signature: "pub fn calculate(a: i32) -> i32".to_string(),
        content: "Calculates a number.\n\n# Examples\n\n```\nlet x = 1;\n```".to_string(),
        ..Default::default()
    };

    let source = "pub fn calculate(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
    let alert = alert_reason(&note, Some(source), Some(1));
    assert_eq!(alert, Some("Signature changed".to_string()));
}

#[test]
fn test_alert_reason_missing_section_shortening() {
    let note = SidecarNote {
        id: "try-calc".to_string(),
        kind: NoteKind::Doc,
        signature: "pub fn try_calculate() -> Result<i32, ()>".to_string(),
        content: "Calculates or fails.\n\n# Examples\n\n```\nlet _ = 1;\n```".to_string(),
        ..Default::default()
    };

    let source = "pub fn try_calculate() -> Result<i32, ()> {\n    Ok(42)\n}\n";
    let alert = alert_reason(&note, Some(source), Some(1));
    assert_eq!(alert, Some("Missing # Errors".to_string()));
}

#[test]
fn test_alert_reason_orphan_doc() {
    let note = SidecarNote {
        id: "doc-orphan".to_string(),
        kind: NoteKind::Doc,
        title: "Orphan Doc".to_string(),
        ..Default::default()
    };

    // Unanchored (no source or line)
    assert_eq!(alert_reason(&note, None, None), Some("Orphan".to_string()));

    // Anchor pointing to line with no doc target
    let source = "// Just a comment\n";
    assert_eq!(
        alert_reason(&note, Some(source), Some(1)),
        Some("Orphan".to_string())
    );
}

#[test]
fn test_alert_reason_internal_never_alert() {
    let internal_note = SidecarNote {
        id: "int-orphan".to_string(),
        kind: NoteKind::Internal,
        title: "Internal Note".to_string(),
        ..Default::default()
    };
    assert_eq!(alert_reason(&internal_note, None, None), None);

    let comment_note = SidecarNote {
        id: "cmt-orphan".to_string(),
        kind: NoteKind::Comment,
        title: "Comment Note".to_string(),
        ..Default::default()
    };
    assert_eq!(alert_reason(&comment_note, None, None), None);
}

#[test]
fn test_index_project_walks_files_and_anchors() {
    let temp_dir = unique_temp_dir("index");

    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir).expect("create src dir");

    let lib_rs = src_dir.join("lib.rs");
    let lib_source = "// [crawrad:note#my-internal]\n// [crawrad:comment#my-comment]\n#[note(\"my-doc\")]\npub fn hello() {}\n";
    fs::write(&lib_rs, lib_source).expect("write lib.rs");

    // Save notes to .crawrad/notes/
    let note_int = SidecarNote {
        id: "my-internal".to_string(),
        kind: NoteKind::Internal,
        title: "Internal Title".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&temp_dir, &note_int).expect("save int note");

    let note_cmt = SidecarNote {
        id: "my-comment".to_string(),
        kind: NoteKind::Comment,
        title: "Comment Title".to_string(),
        resolved: false,
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&temp_dir, &note_cmt).expect("save cmt note");

    let note_doc = SidecarNote {
        id: "my-doc".to_string(),
        kind: NoteKind::Doc,
        title: String::new(), // empty title should pick up item name from doc_target_at
        signature: "pub fn hello()".to_string(),
        content: "Says hello.\n\n# Examples\n\n```\nlet _ = 1;\n```".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&temp_dir, &note_doc).expect("save doc note");

    let note_unanchored = SidecarNote {
        id: "my-unanchored".to_string(),
        kind: NoteKind::Internal,
        title: "Unanchored Title".to_string(),
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&temp_dir, &note_unanchored).expect("save unanchored note");

    let indexed = index_project(&temp_dir);

    assert_eq!(indexed.len(), 4);

    let doc = indexed.iter().find(|n| n.id == "my-doc").unwrap();
    assert_eq!(doc.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(doc.line, Some(3));
    assert_eq!(doc.title, "hello");

    let int = indexed.iter().find(|n| n.id == "my-internal").unwrap();
    assert_eq!(int.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(int.line, Some(1));
    assert_eq!(int.title, "Internal Title");

    let cmt = indexed.iter().find(|n| n.id == "my-comment").unwrap();
    assert_eq!(cmt.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(cmt.line, Some(2));
    assert_eq!(cmt.title, "Comment Title");

    let unanchored = indexed.iter().find(|n| n.id == "my-unanchored").unwrap();
    assert_eq!(unanchored.file, None);
    assert_eq!(unanchored.line, None);
    assert_eq!(unanchored.title, "Unanchored Title");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn view_rows_parts_for_group_and_resolved_comment() {
    let panel = NotesPanel::new(sample_notes());
    let view_rows = panel.view_rows();

    // First row should be the group header for "src/a.rs"
    let group_row = &view_rows[0];
    assert_eq!(
        *group_row,
        (
            true,
            0,
            "src/a.rs".to_string(),
            String::new(),
            false,
            "3".to_string(),
            false,
            false,
            false
        )
    );

    // Finding the resolved comment note row in the view rows
    let cmt_row = view_rows
        .iter()
        .find(|r| !r.0 && r.1 == 2 && r.2 == "Comment 1")
        .expect("resolved comment row");
    assert_eq!(
        *cmt_row,
        (
            false,
            2,
            "Comment 1".to_string(),
            "Resolved".to_string(),
            false,
            "20".to_string(),
            true,
            true,
            false
        )
    );

    // Doc note with alert
    let doc_alert_row = view_rows
        .iter()
        .find(|r| !r.0 && r.1 == 0 && r.2 == "Doc 1")
        .expect("doc alert row");
    assert_eq!(
        *doc_alert_row,
        (
            false,
            0,
            "Doc 1".to_string(),
            "Signature changed".to_string(),
            true,
            "10".to_string(),
            false,
            false,
            false
        )
    );
}

#[test]
fn view_head_reflects_chip_file_and_alert_selection() {
    let mut panel = NotesPanel::new(sample_notes());

    // Initially at default
    let (chips, active_chip, choices, selected_file, alert_lbl, in_alert, footer) =
        panel.view_head();
    assert_eq!(active_chip, 0);
    assert_eq!(chips[0], "All 7");
    assert_eq!(chips[1], "Documentation 2");
    assert_eq!(chips[2], "Notes 2");
    assert_eq!(chips[3], "Comments 3");
    assert_eq!(selected_file, 0);
    assert_eq!(choices[0], "All files");
    assert_eq!(choices[1], "src/a.rs");
    assert_eq!(choices[2], "src/b.rs");
    assert_eq!(alert_lbl, "In alert 1");
    assert!(!in_alert);
    assert_eq!(footer, ".crawrad/notes \u{00B7} 7 files");

    // Pick chip 1 (Doc)
    panel.pick_chip(1);
    let (_, active_chip, _, _, _, _, _) = panel.view_head();
    assert_eq!(active_chip, 1);

    // Pick file 2 ("src/b.rs")
    panel.pick_file(2);
    let (_, _, _, selected_file, _, _, _) = panel.view_head();
    assert_eq!(selected_file, 2);

    // Toggle alert
    panel.toggle_alert();
    let (_, _, _, _, _, in_alert, _) = panel.view_head();
    assert!(in_alert);
}

#[test]
fn pick_file_out_of_range_resets_to_all() {
    let mut panel = NotesPanel::new(sample_notes());
    panel.pick_file(1);
    assert_eq!(panel.file.as_deref(), Some("src/a.rs"));

    panel.pick_file(999);
    assert_eq!(panel.file, None);

    // 0 also resets to all
    panel.pick_file(1);
    assert_eq!(panel.file.as_deref(), Some("src/a.rs"));
    panel.pick_file(0);
    assert_eq!(panel.file, None);
}

#[test]
fn row_target_resolves_group_and_note() {
    let panel = NotesPanel::new(sample_notes());
    // Row 0 is group "src/a.rs"
    assert_eq!(
        panel.row_target(0),
        Some(RowTarget::Group("src/a.rs".to_string()))
    );

    // Row 1 is note "int-1" at line 2 in src/a.rs
    assert_eq!(
        panel.row_target(1),
        Some(RowTarget::Note {
            id: "int-1".to_string(),
            file: Some("src/a.rs".to_string()),
            line: Some(2)
        })
    );

    // Out of bounds
    assert_eq!(panel.row_target(9999), None);
}

#[test]
fn toggle_row_collapses_only_a_group() {
    let mut panel = NotesPanel::new(sample_notes());
    let before = panel.rows().len();
    // Row 1 is a note: nothing collapses
    panel.toggle_row(1);
    assert_eq!(panel.rows().len(), before);
    // Row 0 is the group "src/a.rs": its notes go
    panel.toggle_row(0);
    assert!(panel.rows().len() < before);
    panel.toggle_row(0);
    assert_eq!(panel.rows().len(), before);
}

#[test]
fn note_place_is_the_anchor_file_and_zero_based_line() {
    let panel = NotesPanel::new(sample_notes());
    // Row 1 is "int-1" at 1-based line 2 of src/a.rs
    assert_eq!(panel.note_place(1), Some(("src/a.rs".to_string(), 1)));
    // A group, and an unanchored note, have no place
    assert_eq!(panel.note_place(0), None);
    let unanchored = (0..panel.rows().len())
        .find(|&i| {
            matches!(
                panel.row_target(i),
                Some(RowTarget::Note { file: None, .. })
            )
        })
        .expect("the sample has an unanchored note");
    assert_eq!(panel.note_place(unanchored), None);
}

#[test]
fn a_workspace_lists_its_crates_notes_and_the_editors_unsaved_anchors() {
    // The calculator shape: the workspace at the root, the crate one folder down.
    let root = unique_temp_dir("index-workspace");
    let krate = root.join("calc");
    fs::create_dir_all(krate.join("src")).expect("create crate");
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"calc\"]\n",
    )
    .expect("ws");
    fs::write(krate.join("Cargo.toml"), "[package]\nname = \"calc\"\n").expect("crate");
    let lib = krate.join("src").join("lib.rs");
    fs::write(&lib, "pub fn press() {}\n").expect("lib.rs on disk, no anchors yet");

    // A documentation note lives in the CRATE's folder (cargo doc reads it there);
    // a comment in the workspace's.
    let doc = SidecarNote {
        id: "press-doc".into(),
        kind: NoteKind::Doc,
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&krate, &doc).expect("save doc");
    let cmt = SidecarNote {
        id: "c1".into(),
        kind: NoteKind::Comment,
        ..Default::default()
    };
    crate::sidecar_notes::save_note(&root, &cmt).expect("save comment");

    // On disk: both notes listed, neither anchored.
    let disk = index_project(&root);
    assert_eq!(
        disk.len(),
        2,
        "the crate's documentation note is listed too"
    );
    assert!(disk.iter().all(|n| n.file.is_none()));

    // The editor holds the unsaved anchors the dialog wrote.
    let buffer = "// [crawrad:comment#c1]\n#[note(\"press-doc\")]\npub fn press() {}\n".to_string();
    let live = index_project_live(&root, |p| (p == lib).then(|| buffer.clone()));
    let c1 = live.iter().find(|n| n.id == "c1").unwrap();
    assert_eq!(
        (c1.file.as_deref(), c1.line),
        (Some("calc/src/lib.rs"), Some(1))
    );
    let d = live.iter().find(|n| n.id == "press-doc").unwrap();
    assert_eq!(d.line, Some(2));

    let _ = fs::remove_dir_all(&root);
}
