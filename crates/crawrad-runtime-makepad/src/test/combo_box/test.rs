use makepad_widgets::KeyCode;

use crate::combo_box::{offset_showing, step, thumb, visible_rows};

#[test]
fn makepad_visible_rows_respects_count_and_zero_means_all() {
    assert_eq!(visible_rows(8, 20), 8);
    assert_eq!(visible_rows(8, 3), 3);
    assert_eq!(visible_rows(0, 20), 20);
    assert_eq!(visible_rows(0, 0), 0);
    assert_eq!(visible_rows(5, 0), 0);
}

#[test]
fn makepad_offset_showing_keeps_target_visible_with_minimal_change() {
    // Empty collection returns zero offset
    assert_eq!(offset_showing(0, 0, 5, 0), 0);
    assert_eq!(offset_showing(3, 2, 0, 10), 0);

    // Row already inside the visible window keeps current offset
    assert_eq!(offset_showing(0, 0, 5, 10), 0);
    assert_eq!(offset_showing(4, 0, 5, 10), 0);
    assert_eq!(offset_showing(3, 2, 5, 10), 2);

    // Row below visible window scrolls down minimally
    assert_eq!(offset_showing(5, 0, 5, 10), 1);
    assert_eq!(offset_showing(9, 0, 5, 10), 5);

    // Row above visible window scrolls up minimally
    assert_eq!(offset_showing(1, 4, 5, 10), 1);
    assert_eq!(offset_showing(0, 4, 5, 10), 0);

    // Out of bounds row is clamped to last item
    assert_eq!(offset_showing(15, 0, 5, 10), 5);
}

#[test]
fn makepad_thumb_geometry_proportional_and_none_when_fits() {
    // Fits entirely -> None
    assert_eq!(thumb(0, 5, 5, 100.0), None);
    assert_eq!(thumb(0, 10, 5, 100.0), None);
    assert_eq!(thumb(0, 5, 0, 100.0), None);
    assert_eq!(thumb(0, 0, 5, 100.0), None);
    assert_eq!(thumb(0, 5, 10, 0.0), None);

    // Does not fit -> Some((y, height))
    // 5 visible out of 10 items in 100px track: thumb height is 50px
    assert_eq!(thumb(0, 5, 10, 100.0), Some((0.0, 50.0)));
    // Scrolled to end (offset 5 out of max 5)
    assert_eq!(thumb(5, 5, 10, 100.0), Some((50.0, 50.0)));
    // Halfway (offset 2 out of max 5) -> y is (100 - 50) * (2 / 5) = 20.0
    assert_eq!(thumb(2, 5, 10, 100.0), Some((20.0, 50.0)));
}

#[test]
fn makepad_step_navigates_rows_and_clamps() {
    assert_eq!(step(None, KeyCode::ArrowDown, 5, 0), None);

    // When none hovered, ArrowDown picks first and ArrowUp picks last
    assert_eq!(step(None, KeyCode::ArrowDown, 5, 10), Some(0));
    assert_eq!(step(None, KeyCode::ArrowUp, 5, 10), Some(9));
    assert_eq!(step(None, KeyCode::Home, 5, 10), Some(0));
    assert_eq!(step(None, KeyCode::End, 5, 10), Some(9));

    // Arrow navigation moves by one row
    assert_eq!(step(Some(0), KeyCode::ArrowDown, 5, 10), Some(1));
    assert_eq!(step(Some(1), KeyCode::ArrowUp, 5, 10), Some(0));
    assert_eq!(step(Some(0), KeyCode::ArrowUp, 5, 10), Some(0));
    assert_eq!(step(Some(9), KeyCode::ArrowDown, 5, 10), Some(9));

    // PageUp and PageDown move by visible_rows
    assert_eq!(step(Some(0), KeyCode::PageDown, 5, 10), Some(5));
    assert_eq!(step(Some(5), KeyCode::PageDown, 5, 10), Some(9));
    assert_eq!(step(Some(8), KeyCode::PageUp, 5, 10), Some(3));
    assert_eq!(step(Some(2), KeyCode::PageUp, 5, 10), Some(0));

    // Home and End move to boundaries
    assert_eq!(step(Some(4), KeyCode::Home, 5, 10), Some(0));
    assert_eq!(step(Some(4), KeyCode::End, 5, 10), Some(9));
}
