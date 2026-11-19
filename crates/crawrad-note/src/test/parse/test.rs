use super::*;
use std::path::Path;

#[test]
fn rule1_validate_id_accepts_valid_identifiers() {
    assert!(validate_id("fetch-package").is_ok());
    assert!(validate_id("a").is_ok());
    assert!(validate_id("ABC_123-xyz").is_ok());
    assert!(validate_id("registry-category-label").is_ok());
}

#[test]
fn rule1_validate_id_rejects_empty_and_invalid_identifiers() {
    assert!(validate_id("").is_err());
    assert!(validate_id("path/separator").is_err());
    assert!(validate_id("path\\separator").is_err());
    assert!(validate_id("..").is_err());
    assert!(validate_id("../escape").is_err());
    assert!(validate_id("foo.bar").is_err());
    assert!(validate_id("with spaces").is_err());
    assert!(validate_id("special$char").is_err());
}

#[test]
fn rule2_missing_file_error_matches_expected_format() {
    let path = Path::new("/workspace/.crawrad/notes/fetch-package.md");
    let err = missing_file_error("fetch-package", path);
    assert_eq!(
        err,
        "note \"fetch-package\" not found: expected /workspace/.crawrad/notes/fetch-package.md"
    );
}

#[test]
fn rule3_rejects_missing_front_matter() {
    let text = "# Just markdown without front matter\n\nSome text.";
    let res = parse_note("sample", text);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("front matter missing"));
}

#[test]
fn rule3_rejects_unterminated_front_matter() {
    let text = "---\nid: sample\nkind: doc\ntitle: test\n";
    let res = parse_note("sample", text);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("front matter unterminated"));
}

#[test]
fn rule4_rejects_missing_or_non_doc_kind() {
    // Missing kind
    let text_no_kind = "---\nid: sample\ntitle: test\n---\n\nBody content.";
    let err_no_kind = parse_note("sample", text_no_kind).unwrap_err();
    assert_eq!(
        err_no_kind,
        "note \"sample\" is an internal note (kind: missing); only kind: doc notes become documentation"
    );

    // Internal kind
    let text_internal = "---\nid: sample\nkind: internal\n---\n\nBody content.";
    let err_internal = parse_note("sample", text_internal).unwrap_err();
    assert_eq!(
        err_internal,
        "note \"sample\" is an internal note (kind: internal); only kind: doc notes become documentation"
    );

    // Comment kind
    let text_comment = "---\nid: sample\nkind: comment\n---\n\nBody content.";
    let err_comment = parse_note("sample", text_comment).unwrap_err();
    assert_eq!(
        err_comment,
        "note \"sample\" is a comment note (kind: comment); only kind: doc notes become documentation"
    );

    // Unknown kind
    let text_todo = "---\nid: sample\nkind: todo\n---\n\nBody content.";
    let err_todo = parse_note("sample", text_todo).unwrap_err();
    assert_eq!(
        err_todo,
        "note \"sample\" unknown note kind \"todo\": expected \"doc\" or \"internal\""
    );
}

#[test]
fn rule4_accepts_doc_kind() {
    let text = "---\nid: sample\nkind: doc\n---\n\nValid body.";
    let res = parse_note("sample", text);
    assert_eq!(res.unwrap().body, "Valid body.");

    // Quoted kind
    let text_quoted = "---\nid: sample\nkind: \"doc\"\n---\n\nValid body.";
    let res_quoted = parse_note("sample", text_quoted);
    assert_eq!(res_quoted.unwrap().body, "Valid body.");
}

#[test]
fn rule5_front_matter_id_validation() {
    // Matches
    let text_matching = "---\nid: sample\nkind: doc\n---\n\nBody.";
    assert!(parse_note("sample", text_matching).is_ok());

    // Omitted id in front matter is rejected
    let text_no_id = "---\nkind: doc\n---\n\nBody.";
    let err_no_id = parse_note("sample", text_no_id).unwrap_err();
    assert_eq!(err_no_id, "note \"sample\" front matter missing \"id\"");

    // Mismatched id
    let text_mismatch = "---\nid: other-id\nkind: doc\n---\n\nBody.";
    let err = parse_note("sample", text_mismatch).unwrap_err();
    assert!(err.contains("front matter id does not match"));
    assert!(err.contains("expected \"sample\""));
    assert!(err.contains("found \"other-id\""));
}

#[test]
fn rule6_rejects_empty_body() {
    let empty_text = "---\nid: sample\nkind: doc\n---\n";
    let err = parse_note("sample", empty_text).unwrap_err();
    assert_eq!(err, "note \"sample\" has empty body");

    let whitespace_text = "---\nid: sample\nkind: doc\n---\n   \r\n\t  \n";
    let err_ws = parse_note("sample", whitespace_text).unwrap_err();
    assert_eq!(err_ws, "note \"sample\" has empty body");
}

#[test]
fn handles_crlf_and_trims_body() {
    let crlf_text = "---\r\nid: sample\r\nkind: doc\r\n---\r\n\r\n   CRLF body content.\r\n\r\n";
    let body = parse_note("sample", crlf_text).unwrap();
    assert_eq!(body.body, "CRLF body content.");
}
