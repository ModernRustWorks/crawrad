//! The sidecar notes: anchors, markdown and the notes folder.
use super::*;

#[test]
fn parser_identifies_anchors_in_various_styles() {
    let code = r#"
fn calculate() {
// [crawrad:note#calc_tax_2026]
let tax = 0.15;
/* [crawrad:note#block_rate] */
let total = 100.0 * tax;
// [crawrad:note:begin#critical_region]
run_sensitive_step();
// [crawrad:note:end#critical_region]
}
"#;
    let anchors = extract_anchors(code);
    assert_eq!(anchors.len(), 4);

    assert_eq!(anchors[0].note_id, "calc_tax_2026");
    assert_eq!(anchors[0].line_number, 3);
    assert!(!anchors[0].is_region_begin);

    assert_eq!(anchors[1].note_id, "block_rate");
    assert_eq!(anchors[1].line_number, 5);

    assert_eq!(anchors[2].note_id, "critical_region");
    assert_eq!(anchors[2].line_number, 7);
    assert!(anchors[2].is_region_begin);

    assert_eq!(anchors[3].note_id, "critical_region");
    assert_eq!(anchors[3].line_number, 9);
    assert!(anchors[3].is_region_end);
}

#[test]
fn parser_identifies_comment_anchors() {
    let code = "let a = 1;\n// [crawrad:comment#review_comment_1]\nlet b = 2;\n";
    let anchors = extract_anchors(code);
    assert_eq!(anchors.len(), 1);
    assert_eq!(anchors[0].note_id, "review_comment_1");
    assert_eq!(anchors[0].line_number, 2);
    assert!(anchors[0].is_comment);
    assert!(!anchors[0].is_region_begin);
    assert!(!anchors[0].is_region_end);
}

#[test]
fn serialization_and_deserialization_roundtrip() {
    let note = SidecarNote {
        id: "test_2026".to_string(),
        kind: NoteKind::Internal,
        resolved: false,
        title: "Architectural Decision".to_string(),
        target_file: "src/main.rs".to_string(),
        line_hint: 42,
        author: "isaque".to_string(),
        created: "2026-09-23T22:30:00-03:00".to_string(),
        tags: vec!["security".to_string(), "architecture".to_string()],
        signature: String::new(),
        content: "Here lies a detailed explanation of why we use stdio null.".to_string(),
    };

    let md = note.to_markdown();
    let recovered = SidecarNote::from_markdown(&md).expect("Should deserialize successfully");

    assert_eq!(recovered.id, note.id);
    assert_eq!(recovered.kind, note.kind);
    assert_eq!(recovered.resolved, note.resolved);
    assert_eq!(recovered.title, note.title);
    assert_eq!(recovered.target_file, note.target_file);
    assert_eq!(recovered.line_hint, note.line_hint);
    assert_eq!(recovered.author, note.author);
    assert_eq!(recovered.created, note.created);
    assert_eq!(recovered.tags, note.tags);
    assert_eq!(recovered.content, note.content);
}

#[test]
fn save_and_load_in_temp_disk() {
    let tmp = std::env::temp_dir().join("crawrad_test_sidecar_notes");
    let _ = fs::remove_dir_all(&tmp);

    let note = SidecarNote {
        id: "disk_note_01".to_string(),
        kind: NoteKind::Internal,
        resolved: false,
        title: "Disk Note".to_string(),
        target_file: "crates/crawrad-ide/src/lib.rs".to_string(),
        line_hint: 10,
        author: "agent".to_string(),
        created: "2026-09-23T22:00:00Z".to_string(),
        tags: vec!["io".to_string()],
        signature: String::new(),
        content: "Disk text saved successfully.".to_string(),
    };

    let saved_path = save_note(&tmp, &note).expect("Disk save should succeed");
    assert!(saved_path.exists());

    let loaded = load_note(&tmp, "disk_note_01").expect("Should load saved note");
    assert_eq!(loaded.id, "disk_note_01");
    assert_eq!(loaded.title, "Disk Note");

    let all = list_notes(&tmp);
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, "disk_note_01");

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn resolved_survives_save_and_load() {
    let tmp = std::env::temp_dir().join("crawrad_test_sidecar_notes_resolved");
    let _ = fs::remove_dir_all(&tmp);

    let resolved_comment = SidecarNote {
        id: "resolved_comment_01".to_string(),
        kind: NoteKind::Comment,
        resolved: true,
        title: "Resolved Comment".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 5,
        author: "reviewer".to_string(),
        created: "2026-11-04T10:00:00Z".to_string(),
        tags: Vec::new(),
        signature: String::new(),
        content: "Fixed.".to_string(),
    };

    save_note(&tmp, &resolved_comment).expect("save resolved comment");
    let loaded = load_note(&tmp, "resolved_comment_01").expect("load resolved comment");
    assert_eq!(loaded.id, resolved_comment.id);
    assert_eq!(loaded.kind, NoteKind::Comment);
    assert!(loaded.resolved);

    let open_comment = SidecarNote {
        id: "open_comment_02".to_string(),
        kind: NoteKind::Comment,
        resolved: false,
        title: "Open Comment".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 12,
        author: "reviewer".to_string(),
        created: "2026-11-04T10:05:00Z".to_string(),
        tags: Vec::new(),
        signature: String::new(),
        content: "Pending review.".to_string(),
    };

    save_note(&tmp, &open_comment).expect("save open comment");
    let loaded_open = load_note(&tmp, "open_comment_02").expect("load open comment");
    assert_eq!(loaded_open.id, open_comment.id);
    assert_eq!(loaded_open.kind, NoteKind::Comment);
    assert!(!loaded_open.resolved);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn a_new_note_is_stamped_with_the_real_time() {
    assert_eq!(rfc3339_local(0, 0), "1970-01-01T00:00:00+00:00");
    // 2026-11-16 15:03:14 UTC, seen from Brasilia (-03:00) and from India (+05:30).
    assert_eq!(
        rfc3339_local(1_794_841_394, -3 * 3600),
        "2026-11-16T12:03:14-03:00"
    );
    assert_eq!(
        rfc3339_local(1_794_841_394, 5 * 3600 + 1800),
        "2026-11-16T20:33:14+05:30"
    );
    // Crossing midnight backwards changes the date.
    assert_eq!(
        rfc3339_local(1_794_787_200, -3 * 3600),
        "2026-11-15T21:00:00-03:00"
    );

    let stamp = SidecarNote::current_timestamp();
    assert_ne!(stamp, "2026-09-23T22:45:00-03:00", "no longer a fixed date");
    assert_eq!(stamp.len(), 25, "{stamp}");
}
