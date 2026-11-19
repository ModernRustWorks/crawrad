//! Unit tests for the NoteDialogModel headless model.

use super::*;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "crawrad-note-dlg-{tag}-{}-{id}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn from_caret_item_with_existing_note_loaded_and_missing_template() {
    let temp_dir = unique_temp_dir("existing");
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    // Pre-save an existing doc note
    let pre_note = SidecarNote {
        id: "existing-fn".to_string(),
        kind: NoteKind::Doc,
        title: "Existing Title".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 1,
        content: "Existing content".to_string(),
        ..Default::default()
    };
    sidecar_notes::save_note(&temp_dir, &pre_note).expect("save pre note");

    let src = "/// Old doc.\n#[crawrad_note::note(\"existing-fn\")]\npub fn existing_fn() {}\n\n#[note(\"missing-fn\")]\npub fn missing_fn() {}\n";

    // Caret on existing_fn (line 2)
    let open_existing = NoteDialogModel::from_caret(src, 2, "src/lib.rs", &temp_dir);
    assert_eq!(open_existing.model.note.id, "existing-fn");
    assert_eq!(open_existing.model.kind(), NoteKind::Doc);
    assert_eq!(open_existing.model.title, "Existing Title");
    assert_eq!(open_existing.model.content, "Existing content");
    assert!(open_existing.model.target.is_some());
    assert_eq!(open_existing.insert_anchor, None);
    assert_eq!(open_existing.model.status, None);

    // Caret on missing_fn (line 5)
    let open_missing = NoteDialogModel::from_caret(src, 5, "src/lib.rs", &temp_dir);
    assert_eq!(open_missing.model.note.id, "missing-fn");
    assert_eq!(open_missing.model.kind(), NoteKind::Doc);
    assert_eq!(open_missing.model.title, "missing_fn");
    assert!(open_missing.model.target.is_some());
    assert_eq!(open_missing.insert_anchor, None);
    assert!(
        open_missing
            .model
            .status
            .as_ref()
            .unwrap()
            .contains("New note template")
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn from_caret_item_without_note_creates_doc_note() {
    let temp_dir = unique_temp_dir("newdoc");
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src = "impl MyStruct {\n    /// First line doc.\n    /// Second line doc.\n    pub fn calculate(&self) -> i32 {\n        42\n    }\n}\n";

    // Caret on calculate (line 3)
    let open = NoteDialogModel::from_caret(src, 3, "src/lib.rs", &temp_dir);
    assert_eq!(open.model.note.id, "my-struct-calculate");
    assert_eq!(open.model.kind(), NoteKind::Doc);
    assert_eq!(open.model.title, "MyStruct::calculate");
    assert_eq!(open.model.content, "First line doc.\nSecond line doc.");
    assert!(open.model.target.is_some());
    assert_eq!(open.insert_anchor, None);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn from_caret_anchor_line_loads_internal_note() {
    let temp_dir = unique_temp_dir("anchor");
    let pre_note = SidecarNote {
        id: "anchor_123".to_string(),
        kind: NoteKind::Internal,
        title: "Internal Anchor Note".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 1,
        content: "Anchor note body".to_string(),
        ..Default::default()
    };
    sidecar_notes::save_note(&temp_dir, &pre_note).expect("save internal");

    let src = "// [crawrad:note#anchor_123]\nlet x = 1;\n";

    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    assert_eq!(open.model.note.id, "anchor_123");
    assert_eq!(open.model.kind(), NoteKind::Internal);
    assert_eq!(open.model.title, "Internal Anchor Note");
    assert_eq!(open.model.content, "Anchor note body");
    assert_eq!(open.model.target, None);
    assert_eq!(open.insert_anchor, None);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn from_caret_plain_line_creates_new_internal_note_with_anchor() {
    let temp_dir = unique_temp_dir("plain");
    let src = "let x = 1;\n";

    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    assert_eq!(open.model.kind(), NoteKind::Internal);
    assert_eq!(open.model.target, None);
    assert!(
        open.model
            .title
            .contains("Architectural note for src/lib.rs:1")
    );
    assert!(open.insert_anchor.is_some());

    let (line, new_src) = open.insert_anchor.unwrap();
    assert_eq!(line, 0);
    assert!(new_src.starts_with("let x = 1; // [crawrad:note#"));
    assert!(new_src.contains(&open.model.note.id));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn set_kind_refusal_when_target_is_none() {
    let temp_dir = unique_temp_dir("refuse");
    let note = SidecarNote {
        id: "test-note".to_string(),
        kind: NoteKind::Internal,
        ..Default::default()
    };
    let mut model = NoteDialogModel::from_note(note, &temp_dir);
    assert_eq!(model.kind(), NoteKind::Internal);
    assert_eq!(model.target, None);

    // Attempt to set Doc when target is None -> refused
    model.set_kind(NoteKind::Doc);
    assert_eq!(model.kind(), NoteKind::Internal);
    assert_eq!(
        model.status.as_deref(),
        Some("Documentation needs the caret on an item (fn, struct, enum…)")
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn doc_save_writes_kind_doc_under_crate_dir_and_updates_source() {
    let temp_dir = unique_temp_dir("docsave");
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src = "impl Demo {\n    /// The original documentation.\n    pub fn do_it() {}\n}\n";
    let open = NoteDialogModel::from_caret(src, 2, "src/lib.rs", &temp_dir);
    let mut model = open.model;

    model.content = "New markdown documentation for do_it.".to_string();
    let res = model.save(src).expect("save doc note");
    let new_src = res.expect("source was updated");

    // Attribute added, /// line removed
    assert!(new_src.contains("#[crawrad_note::note(\"demo-do-it\")]"));
    assert!(!new_src.contains("/// The original documentation."));

    // Check file on disk under crate_dir/.crawrad/notes/
    let note_path = temp_dir
        .join(".crawrad")
        .join("notes")
        .join("demo-do-it.md");
    assert!(note_path.is_file());

    let saved = sidecar_notes::load_note(&temp_dir, "demo-do-it").expect("load note");
    assert_eq!(saved.kind, NoteKind::Doc);
    assert_eq!(saved.content, "New markdown documentation for do_it.");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn internal_save_path_writes_to_project_root_and_returns_none() {
    let temp_dir = unique_temp_dir("internalsave");
    let src = "fn dummy() {}\n";
    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    let mut model = open.model;
    // the caret is on an item, so it opens as Documentation; the user switches
    model.set_kind(NoteKind::Internal);
    model.title = "My Internal Note".to_string();
    model.content = "Internal details here.".to_string();

    let res = model.save(src).expect("save internal note");
    assert_eq!(res, None);

    let note_path = temp_dir
        .join(".crawrad")
        .join("notes")
        .join(format!("{}.md", model.note.id));
    assert!(note_path.is_file());

    let saved = sidecar_notes::load_note(&temp_dir, &model.note.id).expect("load saved");
    assert_eq!(saved.kind, NoteKind::Internal);
    assert_eq!(saved.title, "My Internal Note");
    assert_eq!(saved.content, "Internal details here.");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn missing_dependency_status_vs_present() {
    let temp_dir = unique_temp_dir("depstatus");

    // Case 1: Cargo.toml lacks crawrad-note
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src = "pub fn foo() {}\n";
    let open1 = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    let mut model1 = open1.model;
    model1.save(src).expect("save 1");
    let st1 = model1.status.expect("status 1");
    assert!(st1.contains("Saved:"));
    assert!(
        st1.contains("Added crawrad-note to [dependencies] in Cargo.toml."),
        "Save adds the crate the attribute needs: {st1}"
    );

    // Case 2: Cargo.toml includes crawrad-note
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ncrawrad-note = \"0.1\"\n",
    )
    .expect("write Cargo.toml");

    let open2 = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    let mut model2 = open2.model;
    model2.save(src).expect("save 2");
    let st2 = model2.status.expect("status 2");
    assert!(st2.starts_with("Saved:"));
    assert!(!st2.contains("Add crawrad-note to [dependencies]"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn suggestion_text_warning_and_footer_texts() {
    let temp_dir = unique_temp_dir("sugwarn");
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src = "pub fn fetch(&self, id: &str) -> Result<Package, RegistryError> {\n    let p = self.get(id).unwrap();\n    Ok(p)\n}\n";

    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    let mut model = open.model;

    // Header
    assert_eq!(model.header(), format!("Note — {}", model.note.id));

    // Item label
    assert_eq!(
        model.item_label(),
        "pub fn fetch(&self, id: &str) -> Result<Package, RegistryError>"
    );

    // Meta
    assert_eq!(model.meta(), "src/lib.rs:1");

    // Subtitle
    assert_eq!(
        model.view(src, false).subtitle,
        "Public \u{b7} shows in cargo doc and docs.rs"
    );

    // Suggestions & suggestion text
    assert_eq!(
        model.suggestion_text(src),
        "From the signature: returns Result → Errors · calls unwrap() on line 2 → Panics"
    );

    // Warning: first missing section is Errors
    model.content.clear();
    assert_eq!(
        model.warning(src),
        Some("Missing # Errors: returns Result.".to_string())
    );

    // Insert Errors
    model.insert(Section::Errors);
    assert_eq!(
        model.warning(src),
        Some("Missing # Panics: calls unwrap() on line 2.".to_string())
    );

    // Insert Panics
    model.insert(Section::Panics);
    assert_eq!(
        model.warning(src),
        Some("Missing summary: the first paragraph is what rustdoc shows in lists.".to_string())
    );

    // Insert Summary
    model.insert(Section::Summary);
    assert_eq!(
        model.warning(src).as_deref(),
        Some("Missing # Examples section."),
        "a public item: the build asks for examples"
    );

    // Insert Examples
    model.insert(Section::Examples);
    assert_eq!(model.warning(src), None);

    // Footer
    assert_eq!(
        model.footer(),
        format!(
            "Writes #[crawrad_note::note(\"{}\")] above the item",
            model.note.id
        )
    );

    // If switched to Internal
    model.set_kind(NoteKind::Internal);
    assert_eq!(
        model.view(src, false).subtitle,
        "Private \u{b7} stays in the IDE, never becomes docs"
    );
    assert_eq!(model.item_label(), "");
    assert_eq!(model.suggestions(src), Vec::<Suggestion>::new());
    assert_eq!(model.suggestion_text(src), "");
    assert_eq!(model.warning(src), None);
    assert_eq!(
        model.footer(),
        format!(".crawrad/notes/{}.md", model.note.id)
    );
    assert!(model.meta().starts_with("Target: src/lib.rs:1"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn preview_check_and_doctest_run_follow_the_note() {
    let temp_dir = unique_temp_dir("preview");
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");
    let src = "impl Client {\n    pub fn fetch(&self) -> u8 {\n        1\n    }\n}\n";
    let mut model = NoteDialogModel::from_caret(src, 1, "src/lib.rs", &temp_dir).model;

    model.content = "Fetches one.".to_string();
    assert_eq!(
        model.preview(),
        "```rust\npub fn fetch(&self) -> u8\n```\n\n---\n\nFetches one."
    );
    assert_eq!(
        model.warning(src).as_deref(),
        Some("Missing # Examples section."),
        "the build warns on a public item without examples: so does the dialog"
    );
    assert_eq!(model.check(src), None, "never green while the build warns");
    assert_eq!(model.doctest_run(), None, "no code block, nothing to run");

    model
        .content
        .push_str("\n\n# Examples\n\n```\nassert_eq!(1, 1);\n```\n");
    assert_eq!(
        model.check(src).as_deref(),
        Some("Summary 12 chars · every section the signature asks for is present")
    );
    let run = model.doctest_run().expect("an example to run");
    assert_eq!(run.package, "demo");
    assert_eq!(run.path, "Client::fetch");

    model.content = String::new();
    assert_eq!(
        model.check(src),
        None,
        "a missing summary is a warning, not a check"
    );

    model.set_kind(NoteKind::Internal);
    model.content = "```\nx\n```".to_string();
    assert_eq!(model.preview(), "```\nx\n```", "internal: the body alone");
    assert_eq!(
        model.doctest_run(),
        None,
        "an internal note never reaches rustdoc"
    );
    assert_eq!(model.check(src), None);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn the_view_follows_the_tab_the_check_and_the_test() {
    let temp_dir = unique_temp_dir("view");
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");
    let src = "pub fn parse(s: &str) -> Result<u8, String> {\n    s.parse().map_err(|_| s.to_string())\n}\n";
    let mut m = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir).model;
    m.content.clear();

    let v = m.view(src, false);
    assert!(v.is_doc);
    assert_eq!(
        v.check,
        ("Missing # Errors: returns Result.".to_string(), Tone::Warn)
    );
    assert_eq!(v.suggested, [Section::Errors]);
    assert_eq!(v.preview, None, "the Edit tab");
    assert_eq!(
        v.tab_hint,
        format!("Markdown · .crawrad/notes/{}.md", m.note.id)
    );
    assert!(!v.can_test, "no example yet");
    assert_eq!(v.test_line, (String::new(), Tone::Muted));

    m.typed("Parses one byte.\n\n# Errors\n\nWhen it is not a number.\n\n# Examples\n\n```\nassert!(true);\n```".into());
    m.preview_tab = true;
    m.typed("ignored on the Preview tab".into());
    let v = m.view(src, true);
    assert_eq!(v.check.1, Tone::Ok);
    assert!(
        v.preview
            .as_deref()
            .is_some_and(|p| p.contains("Parses one byte."))
    );
    assert_eq!(v.tab_hint, "As cargo doc will show it");
    assert!(v.can_test);
    assert_eq!(v.test_button, "Testing…");

    m.testing();
    assert_eq!(m.view(src, true).test_line.1, Tone::Muted);
    m.tested(&crate::note_preview::DoctestOutcome::Passed {
        count: 1,
        secs: 0.4,
    });
    assert_eq!(
        m.view(src, false).test_line,
        ("Doctest passed · 0.4 s".to_string(), Tone::Ok)
    );
    m.tested(&crate::note_preview::DoctestOutcome::NoTest);
    assert_eq!(m.view(src, false).test_line.1, Tone::Failed);

    let _ = fs::remove_dir_all(&temp_dir);
}

/// **Link item… writes at the caret** — a byte past the end lands at the
/// end, a byte inside `é` lands before it.
#[test]
fn the_link_lands_at_the_caret() {
    let root = unique_temp_dir("link");
    let mut m = NoteDialogModel::from_id("fetch", "src/lib.rs", &root);
    m.content = "Returns é when missing.".into();
    m.insert_link(8, "[`E`] ");
    assert_eq!(m.content, "Returns [`E`] é when missing.");
    m.content = "é".into();
    m.insert_link(1, "x");
    assert_eq!(m.content, "xé");
    m.insert_link(99, "!");
    assert_eq!(m.content, "xé!");
    let _ = fs::remove_dir_all(&root);
}

/// **Save records where the item is and its signature; a changed signature
/// warns until the note is saved again**.
#[test]
fn a_changed_signature_warns_until_the_note_is_saved_again() {
    let dir = unique_temp_dir("stale");
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");
    let src = "\n#[note(\"add-two\")]\npub fn add_two(n: i32) -> i32 {\n    n + 2\n}\n";
    let mut m = NoteDialogModel::from_caret(src, 2, "src/lib.rs", &dir).model;
    m.content = "Adds two.

# Examples

```
assert_eq!(demo::add_two(1), 3);
```"
    .into();
    m.save(src).expect("saved");
    let saved = sidecar_notes::load_note(&dir, "add-two").expect("on disk");
    assert_eq!(saved.signature, "pub fn add_two(n: i32) -> i32");
    assert_eq!(
        (saved.target_file.as_str(), saved.line_hint),
        ("src/lib.rs", 3)
    );
    assert_eq!(m.meta(), "src/lib.rs:3");
    assert_eq!(m.warning(src), None, "just saved: not stale");

    let changed = src.replace("n: i32) -> i32", "n: i64) -> i64");
    let reopened = NoteDialogModel::from_caret(&changed, 2, "src/lib.rs", &dir).model;
    let warning = reopened.warning(&changed).expect("stale");
    assert!(
        warning.contains("Written for `pub fn add_two(n: i32) -> i32`"),
        "{warning}"
    );

    let reformatted = src.replace("(n: i32)", "(\n    n: i32,\n)");
    let same = NoteDialogModel::from_caret(&reformatted, 2, "src/lib.rs", &dir).model;
    assert_eq!(same.warning(&reformatted), None, "rustfmt is not a change");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reopening_on_comment_anchor_loads_as_comment() {
    let dir = unique_temp_dir("comment_open");
    let pre_note = SidecarNote {
        id: "cm_open_1".to_string(),
        kind: NoteKind::Comment,
        title: "Review comment".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 1,
        content: "Please check this.".to_string(),
        ..Default::default()
    };
    sidecar_notes::save_note(&dir, &pre_note).expect("save comment");

    let src = "// [crawrad:comment#cm_open_1]\nlet x = 10;\n";
    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &dir);
    assert_eq!(open.model.kind(), NoteKind::Comment);
    assert_eq!(open.model.note.id, "cm_open_1");
    assert_eq!(open.model.title, "Review comment");
    assert_eq!(open.model.content, "Please check this.");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn saving_comment_updates_note_tag_to_comment_tag() {
    let dir = unique_temp_dir("comment_save");
    let src = "let x = 1;\n";
    let open = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &dir);
    let mut model = open.model;
    let (_, anchor_src) = open.insert_anchor.expect("inserted note anchor");
    assert!(anchor_src.contains("// [crawrad:note#"));

    model.set_kind(NoteKind::Comment);
    model.title = "A review comment".to_string();
    model.content = "Looks good".to_string();

    let res = model.save(&anchor_src).expect("save comment");
    let updated = res.expect("source was updated to comment tag");
    assert!(updated.contains(&format!("// [crawrad:comment#{}", model.note.id)));
    assert!(!updated.contains("// [crawrad:note#"));

    let loaded = sidecar_notes::load_note(&dir, &model.note.id).expect("load saved");
    assert_eq!(loaded.kind, NoteKind::Comment);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn note_popup_on_fn_line_all_enabled_add() {
    let src = "pub fn calculate() -> i32 {\n    42\n}\n";
    let popup = note_popup(src, 0);
    assert_eq!(popup.header, "Line 1 · pub fn calculate() -> i32");
    assert_eq!(popup.rows.len(), 3);
    assert!(popup.rows[0].enabled);
    assert_eq!(popup.rows[0].title, "Add documentation");
    assert_eq!(popup.rows[0].kind, NoteKind::Doc);
    assert_eq!(popup.rows[0].shortcut, "Ctrl+Alt+D");

    assert!(popup.rows[1].enabled);
    assert_eq!(popup.rows[1].title, "Add note");
    assert_eq!(popup.rows[1].kind, NoteKind::Internal);
    assert_eq!(popup.rows[1].shortcut, "Ctrl+Alt+N");

    assert!(popup.rows[2].enabled);
    assert_eq!(popup.rows[2].title, "Add comment");
    assert_eq!(popup.rows[2].kind, NoteKind::Comment);
    assert_eq!(popup.rows[2].shortcut, "Ctrl+Alt+M");
}

#[test]
fn note_popup_on_fn_with_note_attr_edits_documentation() {
    let src = "#[crawrad_note::note(\"my-doc\")]\npub struct Date {\n    pub year: i32,\n}\n";
    let popup = note_popup(src, 1);
    assert_eq!(popup.header, "Line 2 · pub struct Date");
    assert!(popup.rows[0].enabled);
    assert_eq!(popup.rows[0].title, "Edit documentation");
    assert_eq!(popup.rows[0].kind, NoteKind::Doc);
    assert_eq!(popup.rows[1].title, "Add note");
    assert_eq!(popup.rows[2].title, "Add comment");
}

#[test]
fn note_popup_on_note_anchor_edits_note() {
    let src = "let x = 10; // [crawrad:note#my_note_1]\n";
    let popup = note_popup(src, 0);
    assert_eq!(popup.header, "Line 1");
    assert!(!popup.rows[0].enabled);
    assert_eq!(popup.rows[0].title, "Add documentation");
    assert_eq!(
        popup.rows[0].description,
        "Put the caret on a fn, struct, enum, trait or const to document it."
    );
    assert!(popup.rows[1].enabled);
    assert_eq!(popup.rows[1].title, "Edit note");
    assert_eq!(popup.rows[1].kind, NoteKind::Internal);
    assert!(popup.rows[2].enabled);
    assert_eq!(popup.rows[2].title, "Add comment");
}

#[test]
fn note_popup_on_comment_anchor_edits_comment() {
    let src = "let x = 10; // [crawrad:comment#cm_1]\n";
    let popup = note_popup(src, 0);
    assert_eq!(popup.header, "Line 1");
    assert!(!popup.rows[0].enabled);
    assert_eq!(popup.rows[1].title, "Add note");
    assert!(popup.rows[2].enabled);
    assert_eq!(popup.rows[2].title, "Edit comment");
    assert_eq!(popup.rows[2].kind, NoteKind::Comment);
}

#[test]
fn note_popup_on_blank_line_disables_doc_with_reason() {
    let src = "fn foo() {}\n\nfn bar() {}\n";
    let popup = note_popup(src, 1);
    assert_eq!(popup.header, "Line 2");
    assert!(!popup.rows[0].enabled);
    assert_eq!(popup.rows[0].title, "Add documentation");
    assert_eq!(
        popup.rows[0].description,
        "Put the caret on a fn, struct, enum, trait or const to document it."
    );
    assert!(popup.rows[1].enabled);
    assert_eq!(popup.rows[1].title, "Add note");
    assert!(popup.rows[2].enabled);
    assert_eq!(popup.rows[2].title, "Add comment");
}

#[test]
fn from_caret_as_for_each_kind() {
    let dir = unique_temp_dir("from_caret_as");
    let src = "let val = 42;\n";

    // 1. Kind::Doc on a line without target (fallback)
    let open_doc = NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, NoteKind::Doc);
    assert_eq!(open_doc.model.kind(), NoteKind::Doc);
    assert_eq!(open_doc.insert_anchor, None);

    // 2. Kind::Internal creates note anchor
    let open_internal =
        NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, NoteKind::Internal);
    assert_eq!(open_internal.model.kind(), NoteKind::Internal);
    let (line, new_src) = open_internal
        .insert_anchor
        .expect("internal anchor inserted");
    assert_eq!(line, 0);
    assert!(new_src.contains("// [crawrad:note#"));

    // 3. Kind::Comment creates comment anchor
    let open_comment =
        NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, NoteKind::Comment);
    assert_eq!(open_comment.model.kind(), NoteKind::Comment);
    let (line_c, new_src_c) = open_comment.insert_anchor.expect("comment anchor inserted");
    assert_eq!(line_c, 0);
    assert!(new_src_c.contains("// [crawrad:comment#"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn at_caret_opens_the_popup_until_a_kind_is_chosen() {
    use crawrad_actions::NoteKindChoice;
    let dir = unique_temp_dir("at_caret");
    let src = "pub fn f() {}
let x = 1;
";
    let open = |line, choice| NoteDialogModel::at_caret(src, line, "src/lib.rs", &dir, choice);

    let NoteOpen::Popup(header, rows) = open(0, None) else {
        panic!("no choice opens the popup");
    };
    assert!(header.starts_with("Line 1"));
    assert_eq!(rows.len(), 3);
    // Documentation off an item cannot open its dialog: the popup says why.
    assert!(matches!(
        open(1, Some(NoteKindChoice::Doc)),
        NoteOpen::Popup(..)
    ));

    let kind_of = |line, choice| match open(line, Some(choice)) {
        NoteOpen::Dialog(o) => o.model.kind(),
        NoteOpen::Popup(..) => panic!("a chosen kind opens its dialog"),
    };
    assert_eq!(kind_of(0, NoteKindChoice::Doc), NoteKind::Doc);
    assert_eq!(kind_of(1, NoteKindChoice::Note), NoteKind::Internal);
    assert_eq!(kind_of(1, NoteKindChoice::Comment), NoteKind::Comment);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn add_documentation_opens_skeleton_for_undocumented_and_keeps_existing_docs() {
    let temp_dir = unique_temp_dir("skeleton_fixture");
    let cargo_toml = temp_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[package]\nname = \"my-demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml");

    let src = "pub fn add_numbers(a: i32, b: i32) -> i32 {\n    a + b\n}\n\n/// Pre-existing doc comment.\npub fn multiply(a: i32, b: i32) -> i32 {\n    a * b\n}\n";

    // 1. Caret on add_numbers (undocumented pub fn) -> skeleton built from signature
    let open_undoc = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir);
    assert_eq!(open_undoc.model.kind(), NoteKind::Doc);
    assert_eq!(open_undoc.model.title, "add_numbers");
    assert!(
        open_undoc.model.content.starts_with("Adds numbers."),
        "expected summary in skeleton, got: {}",
        open_undoc.model.content
    );
    assert!(
        open_undoc
            .model
            .content
            .contains("use my_demo::add_numbers;")
    );
    assert!(open_undoc.model.content.contains("let a: i32 = todo!();"));
    assert!(open_undoc.model.content.contains("let b: i32 = todo!();"));
    assert!(
        open_undoc
            .model
            .content
            .contains("let result = add_numbers(a, b);")
    );

    // 2. Caret on multiply (documented with ///) -> keeps existing docs
    let open_doc = NoteDialogModel::from_caret(src, 5, "src/lib.rs", &temp_dir);
    assert_eq!(open_doc.model.kind(), NoteKind::Doc);
    assert_eq!(open_doc.model.title, "multiply");
    assert_eq!(open_doc.model.content, "Pre-existing doc comment.");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn view_documentation_kind_flags_and_content() {
    let dir = unique_temp_dir("view_doc");
    let src = "pub fn calculate(val: i32) -> i32 {\n    val * 2\n}\n";
    let model = NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, NoteKind::Doc).model;
    let v = model.view(src, false);

    assert_eq!(v.kind, NoteKind::Doc);
    assert_eq!(v.title_text, "Documentation");
    assert_eq!(v.subtitle, "Public \u{b7} shows in cargo doc and docs.rs");
    assert_eq!(v.badge, None);
    assert!(v.location.is_empty());
    assert!(v.code_line.is_empty());
    assert!(v.byline.is_empty());
    assert!(v.footer.starts_with("Writes #[crawrad_note::note("));
    assert!(!v.show_title_tags);
    assert!(v.show_tabs);
    assert_eq!(v.tab_kind_hint, "rustdoc Markdown");
    assert_eq!(v.resolve_label, None);
    assert!(v.is_doc);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn view_note_kind_flags_and_location_with_40_char_cut() {
    let dir = unique_temp_dir("view_note");
    // Long code line (55 chars trimmed code before anchor):
    let src =
        "    let extremely_long_configuration_variable_name = 100; // [crawrad:note#long_note_1]\n";
    let model = NoteDialogModel::from_caret_as(
        src,
        0,
        "crates/my_app/src/main.rs",
        &dir,
        NoteKind::Internal,
    )
    .model;
    let v = model.view(src, false);

    assert_eq!(v.kind, NoteKind::Internal);
    assert_eq!(v.title_text, "Note");
    assert_eq!(
        v.subtitle,
        "Private \u{b7} stays in the IDE, never becomes docs"
    );
    assert_eq!(v.badge, None);
    // Trimmed code: "let extremely_long_configuration_variable_name = 100;"
    // First 40 chars: "let extremely_long_configuration_variabl"
    // Location: "main.rs · line 1 · let extremely_long_configuration_variabl…"
    assert_eq!(
        v.location,
        "main.rs \u{b7} line 1 \u{b7} let extremely_long_configuration_variabl…"
    );
    assert!(v.code_line.is_empty());
    assert!(v.byline.is_empty());
    assert!(v.show_title_tags);
    assert!(v.show_tabs);
    assert_eq!(v.tab_kind_hint, "Markdown");
    assert_eq!(v.resolve_label, None);
    assert!(!v.is_doc);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn view_comment_kind_flags_badge_code_and_byline() {
    let dir = unique_temp_dir("view_comment");
    let pre_note = SidecarNote {
        id: "cm_view_1".to_string(),
        kind: NoteKind::Comment,
        resolved: false,
        title: "Review comment".to_string(),
        target_file: "src/utils.rs".to_string(),
        line_hint: 1,
        author: "isaque".to_string(),
        created: "2026-09-23T22:45:00-03:00".to_string(),
        content: "Check performance here.".to_string(),
        ..Default::default()
    };
    sidecar_notes::save_note(&dir, &pre_note).expect("save note");

    let src = "    let threshold = 50; // [crawrad:comment#cm_view_1]\n";
    let mut model = NoteDialogModel::from_id("cm_view_1", "src/utils.rs", &dir);
    model.note.kind = NoteKind::Comment;
    let v_open = model.view(src, false);

    assert_eq!(v_open.kind, NoteKind::Comment);
    assert_eq!(v_open.title_text, "Comment");
    assert!(v_open.subtitle.is_empty());
    assert_eq!(v_open.badge, Some(("Open", Tone::Warn)));
    assert_eq!(v_open.location, "utils.rs \u{b7} line 1");
    assert_eq!(v_open.code_line, "let threshold = 50;");
    assert_eq!(v_open.byline, "isaque \u{b7} 23 Sep 2026");
    assert_eq!(v_open.footer, ".crawrad/notes/cm_view_1.md");
    assert!(!v_open.show_title_tags);
    assert!(!v_open.show_tabs);
    assert_eq!(v_open.resolve_label, Some("Resolve"));

    // Toggle resolved -> badge is Resolved (muted) and resolve_label is Reopen
    model.toggle_resolved();
    let v_res = model.view(src, false);
    assert_eq!(v_res.badge, Some(("Resolved", Tone::Muted)));
    assert_eq!(v_res.resolve_label, Some("Reopen"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn byline_formats_with_missing_parts() {
    assert_eq!(
        format_byline("alice", "2026-09-23T12:00:00Z"),
        "alice \u{b7} 23 Sep 2026"
    );
    assert_eq!(format_byline("alice", ""), "alice");
    assert_eq!(format_byline("", "2026-04-05T00:00:00Z"), "5 Apr 2026");
    assert!(format_byline("", "").is_empty());
    assert!(format_byline("  ", "invalid-date").is_empty());
}

#[test]
fn toggle_resolved_flips_and_survives_save_load() {
    let dir = unique_temp_dir("toggle_res");
    let note = SidecarNote {
        id: "comment_toggle".to_string(),
        kind: NoteKind::Comment,
        resolved: false,
        title: "Test comment".to_string(),
        target_file: "src/lib.rs".to_string(),
        line_hint: 1,
        author: "user".to_string(),
        created: "2026-09-23T22:45:00-03:00".to_string(),
        content: "Body text".to_string(),
        ..Default::default()
    };
    sidecar_notes::save_note(&dir, &note).expect("save");

    let src = "let x = 1; // [crawrad:comment#comment_toggle]\n";
    let mut model = NoteDialogModel::from_id("comment_toggle", "src/lib.rs", &dir);
    model.note.kind = NoteKind::Comment;
    assert!(!model.note.resolved);

    // Flip to resolved
    model.toggle_resolved();
    assert!(model.note.resolved);
    assert_eq!(
        model.view(src, false).badge,
        Some(("Resolved", Tone::Muted))
    );
    assert_eq!(model.view(src, false).resolve_label, Some("Reopen"));

    // Save and load
    model.save(src).expect("save model");
    let loaded = sidecar_notes::load_note(&dir, "comment_toggle").expect("load");
    assert!(loaded.resolved);

    // Flip back to open
    let mut model2 = NoteDialogModel::from_note(loaded, &dir);
    model2.toggle_resolved();
    assert!(!model2.note.resolved);
    assert_eq!(model2.view(src, false).badge, Some(("Open", Tone::Warn)));
    assert_eq!(model2.view(src, false).resolve_label, Some("Resolve"));

    model2.save(src).expect("save model2");
    let loaded2 = sidecar_notes::load_note(&dir, "comment_toggle").expect("load2");
    assert!(!loaded2.resolved);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn toggle_resolved_on_doc_note_is_noop() {
    let dir = unique_temp_dir("toggle_doc");
    let src = "pub fn my_func() {}\n";
    let mut model = NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, NoteKind::Doc).model;
    assert_eq!(model.kind(), NoteKind::Doc);
    assert!(!model.note.resolved);

    model.toggle_resolved();
    assert!(!model.note.resolved);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn paint_lists_follow_the_kind() {
    let dir = unique_temp_dir("paint_lists");
    let src = "    let threshold = 50;\n";
    let shown = |v: &NoteView, name: &str| {
        let label = v.labels().into_iter().find(|l| l.0 == name);
        let boxed = v.boxes().into_iter().find(|b| b.0 == name);
        label.map(|l| l.2).or(boxed.map(|b| b.1)).expect(name)
    };
    let button = |v: &NoteView, name: &str| -> Option<String> {
        let found = v.buttons().into_iter().find(|b| b.0 == name).expect(name);
        found.1.map(str::to_string)
    };
    for (kind, icon) in [
        (NoteKind::Doc, 0),
        (NoteKind::Internal, 1),
        (NoteKind::Comment, 2),
    ] {
        let model = NoteDialogModel::from_caret_as(src, 0, "src/lib.rs", &dir, kind).model;
        let v = model.view(src, false);
        assert_eq!(v.icon(), icon);
        let comment = kind == NoteKind::Comment;
        assert_eq!(shown(&v, "note_code_box"), comment);
        assert_eq!(shown(&v, "note_item_row"), !comment);
        assert_eq!(shown(&v, "note_warning_row"), kind != NoteKind::Internal);
        assert_eq!(shown(&v, "note_location_right"), comment);
        let above = v
            .labels()
            .into_iter()
            .find(|l| l.0 == "note_item")
            .unwrap()
            .1;
        let want = match kind {
            NoteKind::Doc => v.item.as_str(),
            _ => v.location.as_str(),
        };
        assert_eq!(above, want);
        assert_eq!(shown(&v, "note_insert_row"), kind == NoteKind::Doc);
        assert_eq!(shown(&v, "note_id_row"), kind == NoteKind::Internal);
        assert_eq!(shown(&v, "note_tab_row"), !comment);
        assert!(shown(&v, "note_title"));
        let resolve = comment.then(|| "Resolve".to_string());
        assert_eq!(button(&v, "note_resolve"), resolve);
        assert_eq!(button(&v, "note_save").as_deref(), Some("Save"));
        assert_eq!(button(&v, "note_cancel").as_deref(), Some("Close"));
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn comment_location_follows_the_anchor_not_the_hint() {
    let dir = unique_temp_dir("anchor_line");
    let src = "//! top\n\nfn f() {\n    let n = 2; // [crawrad:comment#c-anchor]\n}\n";
    let mut model = NoteDialogModel::from_id("c-anchor", "src/lib.rs", &dir);
    model.note.kind = NoteKind::Comment;
    model.note.line_hint = 1;
    let v = model.view(src, false);
    assert_eq!(v.location, "lib.rs \u{b7} line 4");
    assert_eq!(v.code_line, "let n = 2;");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_dialog_warns_whenever_the_build_would() {
    let temp_dir = unique_temp_dir("build-agrees");
    let src = "pub struct Calc;\nimpl Calc {\n    pub fn equals(&mut self) {}\n}\n";
    let mut model = NoteDialogModel::from_caret(src, 2, "src/lib.rs", &temp_dir).model;
    // The note that warned on the calculator: an example in prose, no code block.
    model.content =
        "Evaluates the display.\n\n# Examples\n\nTyping `7+8*2` shows `23`.".to_string();

    let target = model.target.clone().expect("a documentable item");
    let facts = crate::doc_note::item_facts_for_target(src, &target);
    let build = crawrad_note_format::rustdoc::doc_problems(&model.content, &facts);
    assert_eq!(build, vec!["# Examples has no rust code block".to_string()]);
    assert_eq!(
        model.warning(src).as_deref(),
        Some("# Examples has no rust code block.")
    );
    assert_eq!(model.check(src), None);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn saving_documentation_adds_the_crawrad_note_dependency() {
    let temp_dir = unique_temp_dir("adds-dep");
    let manifest =
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1\"\n";
    fs::write(temp_dir.join("Cargo.toml"), manifest).expect("write Cargo.toml");
    let src = "pub fn one() -> u8 {\n    1\n}\n";
    let mut model = NoteDialogModel::from_caret(src, 0, "src/lib.rs", &temp_dir).model;
    model.content =
        "Returns one.\n\n# Examples\n\n```\nassert_eq!(demo::one(), 1);\n```".to_string();

    let written = model
        .save(src)
        .expect("saved")
        .expect("the attribute is written");
    assert!(written.contains("#[crawrad_note::note("));
    let after = fs::read_to_string(temp_dir.join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        crate::doc_note::depends_on_crawrad_note(&after),
        "the build must find the crate the attribute names:\n{after}"
    );
    assert!(
        after.contains("serde = \"1\""),
        "nothing else changes:\n{after}"
    );
    assert!(
        model
            .status
            .as_deref()
            .is_some_and(|s| s.contains("Added crawrad-note")),
        "{:?}",
        model.status
    );

    // A second save leaves the manifest alone.
    model.save(src).expect("saved again");
    let again = fs::read_to_string(temp_dir.join("Cargo.toml")).expect("read Cargo.toml");
    assert_eq!(again, after);

    let _ = fs::remove_dir_all(&temp_dir);
}

/// The editor hands an absolute path; the note's title names the file as the
/// project sees it (seen on the manual's capture: the whole disk path).
#[test]
fn from_caret_titles_the_note_with_the_project_relative_path() {
    let temp_dir = unique_temp_dir("relative-title");
    let file = temp_dir.join("src").join("main.rs");
    let open = NoteDialogModel::from_caret("let x = 1;\n", 0, &file.to_string_lossy(), &temp_dir);
    assert_eq!(open.model.title, "Architectural note for src/main.rs:1");
    let _ = fs::remove_dir_all(&temp_dir);
}
