//! Whole-line edits.

use super::{
    apply_line_edit, delete_lines, duplicate_lines, move_lines, next_occurrence,
    selection_line_range, toggle_line_comment,
};
use crate::editor_chords::{Command, LineDirection};

fn lines(text: &[&str]) -> Vec<String> {
    text.iter().map(|s| s.to_string()).collect()
}

/// **Comment a block, then uncomment it back to the original.**
#[test]
fn a_block_is_commented_at_its_indentation_and_back() {
    let code = lines(&[
        "    fn test() {",
        "        let x = 10;",
        "        let y = 20;",
        "    }",
    ]);
    let commented = toggle_line_comment(&code, 1, 2);
    assert_eq!(commented[0], "    fn test() {");
    assert_eq!(commented[1], "        // let x = 10;");
    assert_eq!(commented[2], "        // let y = 20;");
    assert_eq!(commented[3], "    }");
    assert_eq!(toggle_line_comment(&commented, 1, 2), code);
}

/// **A mixed block is commented whole**, the already-commented line too.
#[test]
fn a_mixed_block_is_commented_whole() {
    let mixed = lines(&["    // let a = 1;", "    let b = 2;"]);
    let all = toggle_line_comment(&mixed, 0, 1);
    assert_eq!(all[0], "    // // let a = 1;");
    assert_eq!(all[1], "    // let b = 2;");
}

/// **Empty lines are left alone, and a range past the end is clamped.**
#[test]
fn empty_lines_stay_and_the_range_is_clamped() {
    let code = lines(&["  a", "", "    b"]);
    let commented = toggle_line_comment(&code, 0, 99);
    assert_eq!(commented, lines(&["  // a", "", "  //   b"]));
    assert!(toggle_line_comment(&[], 0, 1).is_empty());
}

/// **A line goes down and back up; at the top it stays.**
#[test]
fn a_line_moves_down_and_back_and_stops_at_the_edge() {
    let code = lines(&["line 0", "line 1", "line 2"]);
    let (down, s, e) = move_lines(&code, 1, 1, false);
    assert_eq!(down, lines(&["line 0", "line 2", "line 1"]));
    assert_eq!((s, e), (2, 2));
    let (up, s, e) = move_lines(&down, 2, 2, true);
    assert_eq!(up, code);
    assert_eq!((s, e), (1, 1));
    assert_eq!(move_lines(&code, 0, 0, true).0, code);
    assert_eq!(move_lines(&code, 2, 2, false).0, code);
}

/// **A copy goes below (the caret follows it) or above (the caret stays).**
#[test]
fn a_line_is_duplicated_below_or_above() {
    let code = lines(&["a", "b"]);
    let (below, s, e) = duplicate_lines(&code, 1, 1, false);
    assert_eq!(below, lines(&["a", "b", "b"]));
    assert_eq!((s, e), (2, 2));
    let (above, s, e) = duplicate_lines(&code, 0, 0, true);
    assert_eq!(above, lines(&["a", "a", "b"]));
    assert_eq!((s, e), (0, 0));
}

/// **A deleted line puts the caret on the next; the last line leaves one empty.**
#[test]
fn a_deleted_line_moves_the_caret_and_never_empties_the_text() {
    let (rest, caret) = delete_lines(&lines(&["a", "b", "c"]), 1, 1);
    assert_eq!(rest, lines(&["a", "c"]));
    assert_eq!(caret, 1);
    assert_eq!(delete_lines(&lines(&["x"]), 0, 0).0, lines(&[""]));
}

/// **The next occurrence, wrapping round, never past where it started.**
#[test]
fn the_next_occurrence_wraps_round_and_stops() {
    let code = lines(&["let apple = 1;", "let banana = 2;", "let apple = 3;"]);
    assert_eq!(
        next_occurrence(&code, "apple", (0, 9), (0, 4)),
        Some((2, 4))
    );
    assert_eq!(next_occurrence(&code, "apple", (2, 9), (0, 4)), None);
    assert_eq!(
        next_occurrence(&code, "apple", (2, 9), (2, 4)),
        Some((0, 4))
    );
    assert_eq!(next_occurrence(&code, "orange", (0, 0), (0, 0)), None);
    // A byte inside a two-byte char starts at the next boundary.
    assert_eq!(
        next_occurrence(&lines(&["éa"]), "a", (0, 1), (0, 0)),
        Some((0, 2))
    );
}

#[test]
fn selection_line_range_excludes_trailing_empty_cursor() {
    assert_eq!(selection_line_range(1, 1, 5), (1, 1));
    assert_eq!(selection_line_range(1, 3, 0), (1, 2));
    assert_eq!(selection_line_range(1, 3, 4), (1, 3));
    assert_eq!(selection_line_range(2, 2, 0), (2, 2));
}

#[test]
fn apply_line_edit_dispatches_commands() {
    let code = lines(&["    let a = 1;", "    let b = 2;"]);
    let (toggled, cur) = apply_line_edit(Command::ToggleComment, &code, 0, 0).expect("handled");
    assert_eq!(toggled[0], "    // let a = 1;");
    assert_eq!(cur, 0);

    let (moved, cur) =
        apply_line_edit(Command::MoveLines(LineDirection::Down), &code, 0, 0).expect("handled");
    assert_eq!(moved[0], "    let b = 2;");
    assert_eq!(moved[1], "    let a = 1;");
    assert_eq!(cur, 1);

    let (duped, cur) = apply_line_edit(Command::DuplicateLines(LineDirection::Down), &code, 0, 0)
        .expect("handled");
    assert_eq!(duped.len(), 3);
    assert_eq!(cur, 1);

    let (deleted, cur) = apply_line_edit(Command::DeleteLine, &code, 0, 0).expect("handled");
    assert_eq!(deleted, lines(&["    let b = 2;"]));
    assert_eq!(cur, 0);

    assert!(apply_line_edit(Command::OpenFind(false), &code, 0, 0).is_none());
}
