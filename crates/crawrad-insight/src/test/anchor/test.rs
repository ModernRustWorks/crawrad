//! The list's anchor: at the word's start (the gutter counted), under the
//! line, inside the container.

use super::place;

const CELL: [f64; 2] = [8.0, 18.0];
/// The editor's container: from x 358 (after the left dock), 900 wide.
const BOX: [f64; 4] = [358.0, 70.0, 900.0, 800.0];
const LIST: [f64; 2] = [460.0, 200.0];

#[test]
fn the_list_opens_at_the_word_start_under_the_line() {
    // the caret after the gutter: x 500 in the window, a fresh word
    let a = place([500.0, 200.0], 0, CELL, BOX, LIST);
    assert_eq!((a.left, a.top), (142.0, 148.0));
    assert_eq!(a.word, [500.0, 200.0]);
    // two chars typed: the list stays at the word's start, not the caret
    let b = place([516.0, 200.0], 2, CELL, BOX, LIST);
    assert_eq!(b.left, a.left);
}

#[test]
fn near_the_right_or_the_bottom_it_stays_inside() {
    // far right: pulled left just enough to fit, not to a fixed column
    let a = place([1200.0, 200.0], 0, CELL, BOX, LIST);
    assert_eq!(a.left, 900.0 - 460.0);
    // the last lines: it opens above the caret's line
    let b = place([500.0, 800.0], 0, CELL, BOX, LIST);
    assert_eq!(b.top, 800.0 - 70.0 - 200.0);
    // no room above either: below, as far as it goes
    let tiny = [358.0, 70.0, 900.0, 150.0];
    assert_eq!(place([500.0, 120.0], 0, CELL, tiny, LIST).top, 68.0);
}
