use super::{EditMask, shown_text};

const DATE: &str = "!99/99/0000;1;_";

#[test]
fn an_empty_mask_passes_the_text_untouched() {
    let m = EditMask::parse("");
    assert!(m.is_empty());
    assert_eq!(m.format("abc 1"), "abc 1");
    assert_eq!(m.raw("abc_/1"), "abc_/1");
    assert_eq!(m.reshape("x"), "x");
    assert_eq!(m.text("x/y"), "x/y");
    assert!(m.is_complete(""));
    assert_eq!(m.cursor_after("abc", 2), 2);
    assert_eq!(shown_text("", "free"), "free");
}

#[test]
fn an_empty_text_shows_the_blank_mask() {
    assert_eq!(EditMask::parse(DATE).format(""), "__/__/____");
    assert_eq!(shown_text("(000) 000-0000", ""), "(___) ___-____");
}

#[test]
fn typed_characters_are_laid_into_the_mask() {
    let m = EditMask::parse(DATE);
    assert_eq!(m.format("1234"), "12/34/____");
    assert_eq!(m.format("12345678"), "12/34/5678");
    // what does not fit is dropped
    assert_eq!(m.format("123456789"), "12/34/5678");
}

#[test]
fn a_rejected_character_is_dropped_and_the_next_one_tried() {
    let m = EditMask::parse("000");
    assert_eq!(m.format("1a2-3"), "123");
    let letters = EditMask::parse("LL");
    assert_eq!(letters.format("1x9y"), "xy");
}

#[test]
fn each_position_accepts_its_kind() {
    assert_eq!(EditMask::parse("#99").format("-12"), "-12");
    assert_eq!(EditMask::parse("#").format("+"), "+");
    assert_eq!(EditMask::parse("A").format("-z"), "z");
    assert_eq!(EditMask::parse("a").format("7"), "7");
    assert_eq!(EditMask::parse("C").format("%"), "%");
    assert_eq!(EditMask::parse("l").format("5"), "_");
}

#[test]
fn case_rules_turn_the_following_characters() {
    assert_eq!(EditMask::parse(">LL<LL").format("abCD"), "ABcd");
    assert_eq!(EditMask::parse(">L<>L").format("ab"), "Ab");
}

#[test]
fn a_backslash_makes_the_next_character_a_literal() {
    let m = EditMask::parse("\\0\\L-00");
    assert_eq!(m.format(""), "0L-__");
    assert_eq!(m.format("12"), "0L-12");
}

#[test]
fn the_sections_set_the_blank_and_the_literals_of_the_text() {
    let m = EditMask::parse("00-00;0;*");
    assert_eq!(m.blank(), '*');
    assert!(!m.keeps_literals());
    assert_eq!(m.format("1"), "1*-**");
    assert_eq!(m.text("12-34"), "1234");
    let keep = EditMask::parse("00-00");
    assert!(keep.keeps_literals());
    assert_eq!(keep.blank(), '_');
    assert_eq!(keep.text("12-3_"), "12-3_");
}

#[test]
fn raw_recovers_what_was_typed() {
    let m = EditMask::parse(DATE);
    assert_eq!(m.raw("12/3_/____"), "123");
    // an edited text, one character longer than the mask
    assert_eq!(m.raw("1__/__/____"), "1");
}

#[test]
fn reshape_takes_an_edited_text_back_into_the_mask() {
    let m = EditMask::parse(DATE);
    // typed '2' after the '1'
    assert_eq!(m.reshape("12_/__/____"), "12/__/____");
    // the separator deleted: the mask puts it back
    assert_eq!(m.reshape("1234/5678"), "12/34/5678");
    // a letter typed into a digit position does not stay
    assert_eq!(m.reshape("1x_/__/____"), "1_/__/____");
}

#[test]
fn complete_means_every_required_position_is_typed() {
    let m = EditMask::parse(DATE);
    assert!(!m.is_complete("12/34/567_"));
    assert!(m.is_complete("12/34/5678"));
    // the optional positions may stay blank
    assert!(EditMask::parse("0099").is_complete("12__"));
    assert!(!EditMask::parse("0099").is_complete("1___"));
}

#[test]
fn the_cursor_goes_to_the_next_editable_position() {
    let m = EditMask::parse(DATE);
    // '2' typed after '1': two characters before the cursor, past the '/'
    assert_eq!(m.cursor_after("12_/__/____", 2), 3);
    assert_eq!(m.cursor_after("1__/__/____", 1), 1);
    // a full mask leaves the cursor at the end
    assert_eq!(m.cursor_after("12/34/56789", 11), 10);
    // a deletion keeps the cursor where it was
    assert_eq!(m.cursor_after("12/3/5678", 4), 4);
}

#[test]
fn hold_reshapes_and_moves_the_cursor_and_an_empty_mask_hands_back() {
    let phone = EditMask::parse("(00) 00000-0000;1;_");
    assert_eq!(
        phone.hold("(11_) _____-____", 3),
        ("(11) _____-____".to_string(), 5)
    );
    assert_eq!(EditMask::parse("").hold("abc", 1), ("abc".to_string(), 1));
}

#[test]
fn backspace_blanks_slots_and_crosses_literals_without_shifting() {
    let m = EditMask::parse(DATE);

    // Initial: "11/05/2024", caret at 10.
    // 1st Backspace: deletes '4' at index 9, caret goes to 9
    let (t1, c1) = m.hold_edit("11/05/2024", "11/05/202", 9);
    assert_eq!(t1, "11/05/202_");
    assert_eq!(c1, 9);

    // 2nd Backspace: deletes '2' at index 8, caret goes to 8
    let (t2, c2) = m.hold_edit(&t1, "11/05/20_", 8);
    assert_eq!(t2, "11/05/20__");
    assert_eq!(c2, 8);

    // 3rd Backspace: deletes '0' at index 7, caret goes to 7
    let (t3, c3) = m.hold_edit(&t2, "11/05/2__", 7);
    assert_eq!(t3, "11/05/2___");
    assert_eq!(c3, 7);

    // 4th Backspace: deletes '2' at index 6, caret goes to 6
    let (t4, c4) = m.hold_edit(&t3, "11/05/___", 6);
    assert_eq!(t4, "11/05/____");
    assert_eq!(c4, 6);

    // 5th Backspace: over the '/' at index 5, caret goes to 5.
    // The '/' literal is re-inserted, leaving text unchanged and caret at 5.
    let (t5, c5) = m.hold_edit(&t4, "11/05____", 5);
    assert_eq!(t5, "11/05/____");
    assert_eq!(c5, 5);

    // 6th Backspace: deletes '5' at index 4, caret goes to 4
    let (t6, c6) = m.hold_edit(&t5, "11/0/____", 4);
    assert_eq!(t6, "11/0_/____");
    assert_eq!(c6, 4);
}

#[test]
fn delete_forward_in_the_middle_blanks_the_slot() {
    let m = EditMask::parse(DATE);
    // Delete forward at caret 3 deletes '0' from "11/05/2024", caret stays at 3
    let (text, caret) = m.hold_edit("11/05/2024", "11/5/2024", 3);
    assert_eq!(text, "11/_5/2024");
    assert_eq!(caret, 3);
}

#[test]
fn deleting_a_selection_blanks_all_selected_slots() {
    let m = EditMask::parse(DATE);
    // Selecting "05/" (indices 3..6) and deleting it leaves caret at 3
    let (text, caret) = m.hold_edit("11/05/2024", "11/2024", 3);
    assert_eq!(text, "11/__/2024");
    assert_eq!(caret, 3);
}

#[test]
fn typing_after_a_deletion_lands_in_the_blank_slot() {
    let m = EditMask::parse(DATE);
    // Box is "11/05/____", caret is at 6. User types '2'.
    // Text becomes "11/05/2____", cursor 7.
    let (text, caret) = m.hold_edit("11/05/____", "11/05/2____", 7);
    assert_eq!(text, "11/05/2___");
    assert_eq!(caret, 7);
}

#[test]
fn hold_edit_empty_mask_untouched() {
    let m = EditMask::parse("");
    assert_eq!(m.hold_edit("abc", "ab", 2), ("ab".to_string(), 2));
    assert_eq!(m.hold_edit("", "xyz", 3), ("xyz".to_string(), 3));
}

#[test]
fn hold_edit_box_not_yet_full_falls_back_to_hold() {
    let m = EditMask::parse(DATE);
    // "12" is 2 chars, mask has 10 slots -> not yet full. Deleting '2' leaves "1".
    let (text, caret) = m.hold_edit("12", "1", 1);
    assert_eq!(text, "1_/__/____");
    assert_eq!(caret, 1);
}
