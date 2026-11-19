//! The palette ListBox reports where its rows are.

use super::{DETAIL_FONT_SIZE, detail_pos, rows_fit, to_list_space};
use makepad_widgets::{Rect, dvec2};

fn r(x: f64, y: f64) -> Rect {
    Rect {
        pos: dvec2(x, y),
        size: dvec2(140.0, 26.0),
    }
}

/// Measured in the New Project: the rows came out in the dialog's draw pass
/// (675, 226) while the list was at (1123, 418) on screen.
#[test]
fn rows_land_where_the_list_is() {
    let rows = vec![
        (0, ("Makepad".to_string(), r(675.0, 226.0))),
        (1, ("egui".to_string(), r(675.0, 252.0))),
    ];
    let out = to_list_space(&rows, dvec2(1123.0, 418.0));
    assert_eq!(out[0].1.pos, dvec2(1123.0, 418.0));
    assert_eq!(out[1].1.pos, dvec2(1123.0, 444.0));
    assert_eq!(out[1].0, "egui");
}

/// When the first drawn row is not row 0, its index places the list's top.
#[test]
fn a_later_first_row_still_lands_right() {
    let rows = vec![(2, ("c".to_string(), r(10.0, 52.0)))];
    let out = to_list_space(&rows, dvec2(100.0, 200.0));
    assert_eq!(out[0].1.pos, dvec2(100.0, 252.0));
    assert!(to_list_space(&[], dvec2(0.0, 0.0)).is_empty());
}

/// A row's detail (a count) sits at the row's right end, centred in height,
/// and a longer one starts further left.
#[test]
fn the_detail_is_right_aligned_in_its_row() {
    let row = r(100.0, 200.0);
    let one = detail_pos(row, "2", DETAIL_FONT_SIZE);
    let two = detail_pos(row, "12", DETAIL_FONT_SIZE);
    assert!(one.x < 240.0 && one.x > 220.0, "{one:?}");
    assert!(two.x < one.x);
    assert!(one.y > 200.0 && one.y < 213.0, "{one:?}");
}

/// The scroll bar shows only when the rows overflow: the New
/// Project's 9 categories of 32 px fit in its 475 px column.
#[test]
fn the_scroll_bar_is_for_rows_that_overflow() {
    assert!(rows_fit(9, 32.0, 475.0));
    assert!(rows_fit(2, 26.0, 52.0));
    assert!(!rows_fit(3, 26.0, 52.0));
    assert!(rows_fit(0, 26.0, 10.0));
}
