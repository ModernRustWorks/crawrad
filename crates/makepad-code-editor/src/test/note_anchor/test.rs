use super::*;

#[test]
fn the_tag_span_covers_the_brackets() {
    let line = "    // [crawrad:note#form_main_1] tail";
    let (s, e) = tag_span(line).unwrap();
    assert_eq!(&line[s..e], "[crawrad:note#form_main_1]");
}

#[test]
fn the_tag_span_covers_comment_and_doc_attribute() {
    let comment_line = "   // [crawrad:comment#review_1] tail";
    let (cs, ce) = tag_span(comment_line).unwrap();
    assert_eq!(&comment_line[cs..ce], "[crawrad:comment#review_1]");

    let doc_line = "    #[crawrad_note::note(\"doc_span\")] tail";
    let (ds, de) = tag_span(doc_line).unwrap();
    assert_eq!(&doc_line[ds..de], "#[crawrad_note::note(\"doc_span\")]");
}

#[test]
fn the_id_is_read_from_every_form() {
    assert_eq!(id_in("// [crawrad:note#a1]"), Some("a1"));
    assert_eq!(id_in("// [crawrad:note:begin#b2]"), Some("b2"));
    assert_eq!(id_in("// [crawrad:note:end# c3 ]"), Some("c3"));
    assert_eq!(id_in("// [crawrad:comment#cm1]"), Some("cm1"));
    assert_eq!(id_in("#[note(\"d1\")]"), Some("d1"));
}

#[test]
fn anchor_in_recognises_three_kinds_and_regions() {
    assert_eq!(
        anchor_in("#[note(\"doc_simple\")]"),
        Some((AnchorKind::Doc, "doc_simple"))
    );
    assert_eq!(
        anchor_in("  #[crawrad_note::note(\"doc_path\")]"),
        Some((AnchorKind::Doc, "doc_path"))
    );
    assert_eq!(
        anchor_in("  #[ crawrad_note :: note ( \"spaced\" ) ]  "),
        Some((AnchorKind::Doc, "spaced"))
    );
    assert_eq!(
        anchor_in("// [crawrad:note#n1]"),
        Some((AnchorKind::Note, "n1"))
    );
    assert_eq!(
        anchor_in("// [crawrad:note:begin#b1]"),
        Some((AnchorKind::Note, "b1"))
    );
    assert_eq!(
        anchor_in("// [crawrad:note:end#e1]"),
        Some((AnchorKind::Note, "e1"))
    );
    assert_eq!(
        anchor_in("// [crawrad:comment#c1]"),
        Some((AnchorKind::Comment, "c1"))
    );
}

#[test]
fn a_line_without_a_whole_tag_names_no_note() {
    assert_eq!(id_in("// [crawrad:note#]"), None);
    assert_eq!(id_in("// [crawrad:note#open"), None);
    assert_eq!(id_in("// [crawrad:comment#]"), None);
    assert_eq!(id_in("let x = 1;"), None);
    assert_eq!(tag_span("// [crawrad:note#open"), None);
    assert_eq!(anchor_in("#[note]"), None);
    assert_eq!(anchor_in("let x = note(\"abc\");"), None);
}

#[test]
fn is_link_only_line_accepts_doc_attributes_with_or_without_path_and_spaces() {
    assert!(is_link_only_line("#[note(\"doc_simple\")]"));
    assert!(is_link_only_line(
        "   #[crawrad_note::note(\"doc_path\")]   "
    ));
    assert!(is_link_only_line(
        "  #[ crawrad_note :: note ( \"spaced\" ) ]  "
    ));
}

#[test]
fn is_link_only_line_rejects_inner_attributes_and_mixed_code() {
    assert!(!is_link_only_line("#![note(\"inner\")]"));
    assert!(!is_link_only_line("let x = 1; #[note(\"id\")]"));
    assert!(!is_link_only_line("#[note(\"id\")] fn foo() {}"));
    assert!(!is_link_only_line("// [crawrad:note#a1]"));
    assert!(!is_link_only_line("let x = 1;"));
}

#[test]
fn hidden_span_covers_trailing_note_and_comment_tags() {
    let line_note = "let x = 1; // [crawrad:note#a]";
    let (s, e) = hidden_span(line_note).unwrap();
    assert_eq!(&line_note[s..e], " // [crawrad:note#a]");

    let line_comment = "let y = 2; // [crawrad:comment#cm1]";
    let (cs, ce) = hidden_span(line_comment).unwrap();
    assert_eq!(&line_comment[cs..ce], " // [crawrad:comment#cm1]");

    let line_begin = "let b = 3; // [crawrad:note:begin#b1]";
    let (bs, be) = hidden_span(line_begin).unwrap();
    assert_eq!(&line_begin[bs..be], " // [crawrad:note:begin#b1]");

    let line_end = "let e_line = 4; // [crawrad:note:end#e1]";
    let (es, ee) = hidden_span(line_end).unwrap();
    assert_eq!(&line_end[es..ee], " // [crawrad:note:end#e1]");
}

#[test]
fn hidden_span_for_tag_inside_longer_comment_hides_only_bracketed_tag() {
    let line = "// why: see [crawrad:note#a]";
    let (s, e) = hidden_span(line).unwrap();
    assert_eq!(&line[s..e], "[crawrad:note#a]");
}

#[test]
fn hidden_span_returns_none_when_no_tag_or_link_only_line() {
    assert!(hidden_span("let x = 1;").is_none());
    assert!(hidden_span("#[note(\"doc\")]").is_none());
}

#[test]
fn collapsed_lines_collects_sorted_link_only_lines_and_skips_caret_lines() {
    let lines = [
        "#[note(\"a\")]",
        "let x = 1;",
        "#[note(\"b\")]",
        "#[note(\"c\")]",
    ];
    let carets = [2];
    assert_eq!(collapsed_lines(&lines, &carets), vec![0, 3]);
}

#[test]
fn icon_line_skips_stacked_attributes_and_doc_comments() {
    let lines = [
        "#[note(\"a\")]",
        "#[note(\"b\")]",
        "/// doc comment",
        "#[derive(Debug, Clone)]",
        "pub struct Foo;",
    ];
    assert_eq!(icon_line(&lines, 0), 4);
    assert_eq!(icon_line(&lines, 1), 4);
}

#[test]
fn icon_line_at_end_of_file_falls_back_to_next_line() {
    let lines_eof = ["fn foo() {}", "#[note(\"a\")]"];
    assert_eq!(icon_line(&lines_eof, 1), 2);

    let lines_no_item = ["#[note(\"a\")]", "/// doc", "#[derive(Clone)]"];
    assert_eq!(icon_line(&lines_no_item, 0), 1);
}

#[test]
fn fit_label_truncates_with_ellipsis() {
    assert_eq!(fit_label("Short", 10), "Short");
    let cut = fit_label("A very long title that will not fit", 16);
    assert_eq!(cut.chars().count(), 16);
    assert!(cut.ends_with('…'));
}
