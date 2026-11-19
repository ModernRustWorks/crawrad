use super::*;
use std::path::Path;

#[test]
fn round_trip_doc_and_internal_with_quotes_and_escapes() {
    let doc_note = NoteFile {
        id: "doc-note-01".to_string(),
        kind: NoteKind::Doc,
        kind_declared: true,
        resolved: false,
        title: "Architecture: \"Pipeline\" \\ Safety".to_string(),
        target_file: "crates/crawrad-note/src/lib.rs".to_string(),
        line_hint: 42,
        author: "isaque".to_string(),
        created: "2026-10-23T12:00:00Z".to_string(),
        tags: vec!["architecture".to_string(), "docs".to_string()],
        signature: String::new(),
        body: "# Pipeline\n\nDetailed markdown description.".to_string(),
    };

    let md = doc_note.to_markdown();
    let parsed = NoteFile::parse(&md).expect("should parse serialized doc note");
    assert_eq!(parsed, doc_note);

    let internal_note = NoteFile {
        id: "internal-note-02".to_string(),
        kind: NoteKind::Internal,
        kind_declared: true,
        resolved: false,
        title: "Internal Note Without Tags".to_string(),
        target_file: "crates/crawrad-shell/src/main.rs".to_string(),
        line_hint: 1,
        author: "agent".to_string(),
        created: "2026-10-23T12:30:00Z".to_string(),
        tags: Vec::new(),
        signature: String::new(),
        body: "Internal notes are not exposed in rustdoc.".to_string(),
    };

    let md_internal = internal_note.to_markdown();
    assert!(md_internal.contains("tags: []\n"));
    let parsed_internal =
        NoteFile::parse(&md_internal).expect("should parse serialized internal note");
    assert_eq!(parsed_internal, internal_note);

    let comment_note = NoteFile {
        id: "comment-note-03".to_string(),
        kind: NoteKind::Comment,
        kind_declared: true,
        resolved: false,
        title: "Review Remark".to_string(),
        target_file: "crates/crawrad-note/src/lib.rs".to_string(),
        line_hint: 15,
        author: "reviewer".to_string(),
        created: "2026-10-23T13:00:00Z".to_string(),
        tags: vec!["review".to_string()],
        signature: String::new(),
        body: "Consider simplifying this expression.".to_string(),
    };
    let md_comment = comment_note.to_markdown();
    assert!(md_comment.contains("kind: comment\nstatus: open\n"));
    let parsed_comment =
        NoteFile::parse(&md_comment).expect("should parse serialized comment note");
    assert_eq!(parsed_comment, comment_note);
    assert_eq!(NoteKind::Comment.as_str(), "comment");
    assert_eq!(NoteKind::Comment.label(), "Comment");
}

#[test]
fn tolerates_utf8_bom() {
    let text = "\u{feff}---\nid: bom-note\nkind: doc\n---\n\nBody with BOM.";
    let parsed = NoteFile::parse(text).expect("should tolerate BOM");
    assert_eq!(parsed.id, "bom-note");
    assert_eq!(parsed.kind, NoteKind::Doc);
    assert!(parsed.kind_declared);
    assert_eq!(parsed.body, "Body with BOM.");
}

#[test]
fn tolerates_crlf_line_endings() {
    let text = "---\r\nid: crlf-note\r\nkind: doc\r\ntitle: \"CRLF Title\"\r\ntags: [a, b]\r\n---\r\n\r\nCRLF body content.\r\n";
    let parsed = NoteFile::parse(text).expect("should tolerate CRLF");
    assert_eq!(parsed.id, "crlf-note");
    assert_eq!(parsed.title, "CRLF Title");
    assert_eq!(parsed.tags, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(parsed.body, "CRLF body content.");
}

#[test]
fn rejects_missing_front_matter() {
    let text = "# Just markdown\n\nNo front matter here.";
    let err = NoteFile::parse(text).unwrap_err();
    assert_eq!(err, NoteError::MissingFrontMatter);

    let empty = "   \n\t\r\n";
    let err_empty = NoteFile::parse(empty).unwrap_err();
    assert_eq!(err_empty, NoteError::MissingFrontMatter);

    let text_with_other_first_line = "some-text\n---\nid: foo\n---\n";
    let err_other = NoteFile::parse(text_with_other_first_line).unwrap_err();
    assert_eq!(err_other, NoteError::MissingFrontMatter);
}

#[test]
fn rejects_unterminated_front_matter() {
    let text = "---\nid: unterminated\nkind: doc\ntitle: test\n";
    let err = NoteFile::parse(text).unwrap_err();
    assert_eq!(err, NoteError::UnterminatedFrontMatter);

    let text_only_open = "---";
    let err_only_open = NoteFile::parse(text_only_open).unwrap_err();
    assert_eq!(err_only_open, NoteError::UnterminatedFrontMatter);
}

#[test]
fn rejects_missing_id() {
    let text = "---\nkind: doc\ntitle: test\n---\n\nBody text.";
    let err = NoteFile::parse(text).unwrap_err();
    assert_eq!(err, NoteError::MissingId);

    let text_empty_id = "---\nid: \"\"\nkind: doc\n---\n\nBody text.";
    let err_empty = NoteFile::parse(text_empty_id).unwrap_err();
    assert_eq!(err_empty, NoteError::MissingId);
}

#[test]
fn rejects_unknown_kind() {
    let text = "---\nid: unknown-kind\nkind: custom_kind\n---\n\nBody text.";
    let err = NoteFile::parse(text).unwrap_err();
    assert_eq!(err, NoteError::UnknownKind("custom_kind".to_string()));
}

#[test]
fn old_note_without_kind_defaults_to_internal_undeclared() {
    let text = "---\nid: old-note\ntarget_file: src/main.rs\nline_hint: 10\nauthor: user\ncreated: 2026-09-23\ntitle: \"Old Note\"\ntags: [legacy]\n---\n\nOld note body.";
    let parsed = NoteFile::parse(text).expect("should parse note without kind");
    assert_eq!(parsed.id, "old-note");
    assert_eq!(parsed.kind, NoteKind::Internal);
    assert!(!parsed.kind_declared);
    assert_eq!(parsed.line_hint, 10);
    assert_eq!(parsed.body, "Old note body.");
}

#[test]
fn line_hint_defaults_to_one_on_parse_failure() {
    let text = "---\nid: bad-line-hint\nline_hint: not-a-number\n---\n\nBody.";
    let parsed = NoteFile::parse(text).expect("should parse");
    assert_eq!(parsed.line_hint, 1);
}

#[test]
fn single_quoted_and_unquoted_values_handled() {
    let text = "---\nid: 'single-quoted'\ntitle: 'Single Quoted Title'\nkind: 'doc'\n---\n\nBody.";
    let parsed = NoteFile::parse(text).expect("should parse single quotes");
    assert_eq!(parsed.id, "single-quoted");
    assert_eq!(parsed.title, "Single Quoted Title");
    assert_eq!(parsed.kind, NoteKind::Doc);
}

#[test]
fn validate_id_rules() {
    assert!(validate_id("fetch-package").is_ok());
    assert!(validate_id("a").is_ok());
    assert!(validate_id("ABC_123-xyz").is_ok());

    assert!(validate_id("").is_err());
    assert!(validate_id("path/separator").is_err());
    assert!(validate_id("path\\separator").is_err());
    assert!(validate_id("with spaces").is_err());
    assert!(validate_id("special$char").is_err());
}

#[test]
fn note_path_resolution() {
    let base = Path::new("/workspace");
    let path = note_path(base, "sample-id");
    assert_eq!(
        path,
        base.join(".crawrad").join("notes").join("sample-id.md")
    );
}

#[test]
fn title_is_written_as_a_valid_yaml_double_quoted_string() {
    let note = NoteFile {
        id: "n".to_string(),
        title: r#"say "hi" in C:\dir"#.to_string(),
        ..Default::default()
    };
    let md = note.to_markdown();
    assert!(md.contains(r#"title: "say \"hi\" in C:\\dir""#), "{md}");
}

/// **The saved signature goes and comes back** — quotes and backslashes
/// escaped as the title's; an empty one writes no line.
#[test]
fn the_signature_round_trips_and_is_left_out_when_empty() {
    let note = NoteFile {
        id: "n".to_string(),
        kind: NoteKind::Doc,
        signature: r#"pub fn f(s: &str) -> &'static str"#.to_string(),
        body: "B".to_string(),
        ..Default::default()
    };
    let md = note.to_markdown();
    assert!(
        md.contains(r#"signature: "pub fn f(s: &str) -> &'static str""#),
        "{md}"
    );
    assert_eq!(
        NoteFile::parse(&md).expect("parses").signature,
        note.signature
    );
    let plain = NoteFile {
        signature: String::new(),
        ..note
    };
    assert!(!plain.to_markdown().contains("signature"));
}

/// **Reformatting is not a change; a new parameter is** — and a note that
/// recorded no signature is never stale.
#[test]
fn only_a_real_signature_change_is_stale() {
    let saved = "pub fn add_two(n: i32) -> i32";
    assert_eq!(
        stale_signature(
            "a",
            saved,
            "pub fn add_two(
    n: i32,
) -> i32"
        ),
        None
    );
    assert_eq!(
        stale_signature("a", saved, "pub fn add_two (n : i32) -> i32"),
        None
    );
    let msg = stale_signature("a", saved, "pub fn add_two(n: i64) -> i64").expect("stale");
    assert!(
        msg.contains("note \"a\" was written for `pub fn add_two(n: i32) -> i32`"),
        "{msg}"
    );
    assert_eq!(stale_signature("a", "", "pub fn x()"), None);
}

#[test]
fn round_trip_open_and_resolved_comment() {
    let open_comment = NoteFile {
        id: "open-comment-01".to_string(),
        kind: NoteKind::Comment,
        kind_declared: true,
        resolved: false,
        title: "Open Review Remark".to_string(),
        target_file: "crates/crawrad-note/src/lib.rs".to_string(),
        line_hint: 10,
        author: "alice".to_string(),
        created: "2026-11-04T10:00:00Z".to_string(),
        tags: vec!["review".to_string()],
        signature: String::new(),
        body: "Needs refactoring.".to_string(),
    };
    let md_open = open_comment.to_markdown();
    assert!(md_open.contains("kind: comment\nstatus: open\n"));
    let parsed_open = NoteFile::parse(&md_open).expect("should parse open comment");
    assert_eq!(parsed_open, open_comment);
    assert!(!parsed_open.resolved);

    let resolved_comment = NoteFile {
        id: "resolved-comment-02".to_string(),
        kind: NoteKind::Comment,
        kind_declared: true,
        resolved: true,
        title: "Resolved Remark".to_string(),
        target_file: "crates/crawrad-note/src/lib.rs".to_string(),
        line_hint: 20,
        author: "bob".to_string(),
        created: "2026-11-04T10:30:00Z".to_string(),
        tags: vec!["done".to_string()],
        signature: String::new(),
        body: "Refactoring completed.".to_string(),
    };
    let md_resolved = resolved_comment.to_markdown();
    assert!(md_resolved.contains("kind: comment\nstatus: resolved\n"));
    let parsed_resolved = NoteFile::parse(&md_resolved).expect("should parse resolved comment");
    assert_eq!(parsed_resolved, resolved_comment);
    assert!(parsed_resolved.resolved);
}

#[test]
fn comment_without_status_parses_as_open() {
    let text =
        "---\nid: comment-no-status\nkind: comment\ntitle: \"Default to open\"\n---\n\nBody.";
    let parsed = NoteFile::parse(text).expect("should parse comment without status");
    assert_eq!(parsed.id, "comment-no-status");
    assert_eq!(parsed.kind, NoteKind::Comment);
    assert!(!parsed.resolved);
}

#[test]
fn status_resolved_on_doc_note_is_ignored() {
    let text = "---\nid: doc-with-status\nkind: doc\nstatus: resolved\ntitle: \"Doc with status\"\n---\n\nDoc body.";
    let parsed = NoteFile::parse(text).expect("should parse doc note with status");
    assert_eq!(parsed.id, "doc-with-status");
    assert_eq!(parsed.kind, NoteKind::Doc);
    assert!(!parsed.resolved);
}

#[test]
fn rejects_unknown_status() {
    let text = "---\nid: comment-bad-status\nkind: comment\nstatus: pending\n---\n\nBody.";
    let err = NoteFile::parse(text).unwrap_err();
    assert_eq!(err, NoteError::UnknownStatus("pending".to_string()));
    assert_eq!(
        err.to_string(),
        "unknown note status \"pending\": expected \"open\" or \"resolved\""
    );
}

#[test]
fn doc_note_markdown_has_no_status_line() {
    let doc_note = NoteFile {
        id: "doc-note-plain".to_string(),
        kind: NoteKind::Doc,
        title: "Clean Doc".to_string(),
        ..Default::default()
    };
    let md = doc_note.to_markdown();
    assert!(!md.contains("status:"));
}

/// **A signature spanning lines survives the header** — a `const` string
/// continued with `\` keeps its breaks, its backslashes and its quotes.
#[test]
fn a_multi_line_signature_round_trips() {
    let note = NoteFile {
        id: "n".to_string(),
        kind: NoteKind::Doc,
        signature: "pub const N : & str = \"one \\\ntwo \\\r\nthree\"".to_string(),
        body: "B".to_string(),
        ..Default::default()
    };
    let md = note.to_markdown();
    let line = md
        .lines()
        .find(|l| l.starts_with("signature: "))
        .expect("a signature line");
    assert!(
        line.ends_with(r#"three\"""#),
        "the signature stays on one line:\n{md}"
    );
    assert_eq!(
        NoteFile::parse(&md).expect("parses").signature,
        note.signature
    );
}
