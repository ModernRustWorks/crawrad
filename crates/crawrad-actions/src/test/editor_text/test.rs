//! Unit tests for EditorText pure constructor.

use super::EditorText;

#[test]
fn editor_text_for_view_when_code_not_in_front() {
    let text = EditorText::for_view(false, true, true, true);
    assert!(!text.code_in_front);
    assert!(!text.selection);
    assert!(!text.clipboard);
    assert!(!text.unit_has_form);
}

#[test]
fn editor_text_for_view_when_code_in_front() {
    let text = EditorText::for_view(true, true, true, false);
    assert!(text.code_in_front);
    assert!(text.selection);
    assert!(text.clipboard);
    assert!(!text.unit_has_form);

    let text2 = EditorText::for_view(true, false, false, true);
    assert!(text2.code_in_front);
    assert!(!text2.selection);
    assert!(!text2.clipboard);
    assert!(text2.unit_has_form);
}
