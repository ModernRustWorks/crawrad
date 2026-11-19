use super::*;

#[test]
fn diff_row_tints_match_approved_spec() {
    assert_eq!(diff_row_tint(DiffRowKind::Same), "#181825");
    assert_eq!(diff_row_tint(DiffRowKind::Removed), "#2a1c24");
    assert_eq!(diff_row_tint(DiffRowKind::Added), "#1c2a24");
    assert_eq!(diff_row_tint(DiffRowKind::Changed), "#1d2236");
    assert_eq!(diff_row_accent(DiffRowKind::Same), None);
    assert_eq!(diff_row_accent(DiffRowKind::Removed), Some("#f38ba8"));
    assert_eq!(diff_row_accent(DiffRowKind::Added), Some("#a6e3a1"));
    assert_eq!(diff_row_accent(DiffRowKind::Changed), Some("#89b4fa"));
}

#[test]
fn diff_view_row_creates_correct_stored_rows() {
    let row = DiffViewRow {
        kind: DiffRowKind::Removed,
        left: Some((42, "let x = 1;")),
        right: None,
    };
    let stored = StoredDiffRow::from(&row);
    assert_eq!(stored.kind, DiffRowKind::Removed);
    assert_eq!(stored.left_line, Some(42));
    assert_eq!(stored.left_text, "let x = 1;");
    assert_eq!(stored.right_line, None);
    assert_eq!(stored.right_text, "");
}

#[test]
fn unbound_diff_view_ref_is_safe_noop() {
    let view_ref = CrawradDiffViewRef::default();
    assert!(view_ref.borrow().is_none());
}

#[test]
fn a_side_scrolls_by_whole_columns() {
    assert_eq!(skip_columns("let ação = 1;", 4), "ação = 1;");
    assert_eq!(skip_columns("let", 3), "");
    assert_eq!(skip_columns("let", 9), "");
    assert_eq!(skip_columns("let", 0), "let");
}

#[test]
fn the_thumb_shows_only_when_the_side_overflows() {
    assert_eq!(thumb_geometry(400.0, 50, 40, 0), None);
    assert_eq!(thumb_geometry(400.0, 50, 100, 0), Some((0.0, 200.0)));
    // at the farthest offset the thumb touches the track's end
    let (x, w) = thumb_geometry(400.0, 50, 100, max_offset(100, 50)).unwrap();
    assert_eq!(x + w, 400.0);
    // a very long line still leaves a thumb that can be grabbed
    assert_eq!(thumb_geometry(400.0, 50, 100_000, 0).unwrap().1, MIN_THUMB);
}

#[test]
fn line_numbers_line_up_on_their_last_digit() {
    assert_eq!(gutter_text(9), "   9");
    assert_eq!(gutter_text(10), "  10");
    assert_eq!(gutter_text(1234), "1234");
}
