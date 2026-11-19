//! Word motion on one line. The shell's key tests drive these too.

use super::{column_of_byte, word_end_right, word_start_left};

const LINE: &str = "    self.edt_nome_.text = 1;";

/// **`Ctrl+Left` stops at each part of a path, and skips indentation whole.**
#[test]
fn left_stops_at_each_part_and_skips_spaces() {
    let mut at = LINE.len();
    let mut stops = Vec::new();
    while let Some(n) = word_start_left(LINE, at) {
        stops.push(n);
        at = n;
    }
    // `;` `1` `=` `text` `.` `edt_nome_` `.` `self`, then the indentation in
    // one step to 0, and from 0 nothing is left.
    assert_eq!(stops, vec![27, 26, 24, 19, 18, 9, 8, 4, 0]);
}

/// **`Ctrl+Right` ends at the word; at the edges there is nothing to cross.**
#[test]
fn right_ends_at_the_word_and_edges_ask_to_cross() {
    assert_eq!(
        word_end_right(LINE, 0),
        Some(8),
        "past the spaces, to the end of `self`"
    );
    assert_eq!(
        word_end_right(LINE, 8),
        Some(9),
        "the `.` is a step of its own"
    );
    assert_eq!(word_start_left(LINE, 0), None);
    assert_eq!(word_end_right(LINE, LINE.len()), None);
    assert_eq!(word_start_left("", 0), None);
}

/// **A byte inside a character rounds up to the next one, and does not panic.**
#[test]
fn a_byte_inside_a_character_rounds_up() {
    let line = "é x";
    // Byte 1 is inside `é` (bytes 0..2): the walk starts at the space.
    assert_eq!(word_end_right(line, 1), Some(4));
    assert_eq!(word_start_left(line, 1), Some(0));
}

// The column of a byte: moved here from the shell's footer with the function.

/// **A column is counted in CHARACTERS, and from one.**
///
/// This is the whole reason the conversion exists. `byte_index` and column
/// agree only in pure ASCII, and the first line of every unit this IDE
/// generates is a Portuguese comment — so they part company on the very first
/// line a user ever sees.
#[test]
fn a_column_counts_characters_and_starts_at_one() {
    assert_eq!(column_of_byte("abc", 0), 1, "the first column is 1, not 0");
    assert_eq!(column_of_byte("abc", 2), 3);

    // `é` occupies TWO bytes. After it, byte 3 is column 3.
    let linha = "aébc";
    assert_eq!(linha.len(), 5, "the test fixture changed shape");
    assert_eq!(column_of_byte(linha, 0), 1);
    assert_eq!(column_of_byte(linha, 1), 2, "before `é`");
    assert_eq!(column_of_byte(linha, 3), 3, "right after `é`");
    assert_eq!(column_of_byte(linha, 4), 4);
}

/// **The real first line of a generated unit.**
///
/// Not a synthetic string: this is what the generator writes at the top of
/// every `form_main.rs`, and it is where a byte count would first lie.
#[test]
fn the_first_line_of_a_generated_unit_is_where_bytes_lie() {
    let linha = "//! **Este arquivo é seu.** O CrawRAD nunca o sobrescreve.";
    assert!(
        linha.chars().count() < linha.len(),
        "the fixture lost the accent, and without it this test measures nothing"
    );
    // THE COLUMN AFTER THE LAST CHARACTER is `chars + 1`, and the byte after the
    // last is `len`. The first draft of this test compared BOTH
    // directly, giving 59 against 59: on a line with only ONE accent they
    // coincide by chance.
    //
    // The mistake that `byte_index` would make is not at the end of the line — it is in the
    // MIDDLE, right after the accent, and that is where this test looks now.
    assert_eq!(
        column_of_byte(linha, linha.len()),
        linha.chars().count() + 1
    );

    let byte_do_acento = linha.find('é').expect("the accent of this line");
    let depois = byte_do_acento + 'é'.len_utf8();
    // THE COUNT IS THE RELATION, not a hand-written number: right after the
    // accent the column is ONE behind the byte, because one character consumed two
    // bytes. Writing `22` here would tie the test to this sentence, and what is
    // measured is the conversion.
    assert_eq!(
        column_of_byte(linha, depois),
        linha[..depois].chars().count() + 1,
        "the column is not the character count up to that point"
    );
    assert!(
        column_of_byte(linha, depois) < depois + 1,
        "the column did not fall behind the byte: the conversion is not converting"
    );
}

/// **A byte past the end of the line answers the end, and does not panic.**
///
/// It is reachable: the cursor comes from the editor's session and the text
/// from this shell's last write, and a frame where the two disagree must not
/// take the IDE down.
#[test]
fn a_byte_past_the_end_answers_the_end() {
    assert_eq!(column_of_byte("abc", 99), 4);
    assert_eq!(column_of_byte("", 0), 1);
    assert_eq!(column_of_byte("", 7), 1);
}

/// **A byte INSIDE a character answers the next boundary.**
///
/// `byte_index` pointing at the middle of a multi-byte character is not a
/// state the editor should produce, but slicing a `&str` there panics — so the
/// conversion is written to walk boundaries instead of index into them.
#[test]
fn a_byte_inside_a_character_does_not_panic() {
    // `é` occupies bytes 1 and 2; byte 2 is INSIDE it.
    assert_eq!(column_of_byte("aébc", 2), 3);
}
