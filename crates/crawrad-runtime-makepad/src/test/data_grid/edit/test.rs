use super::*;

fn key_event(key_code: KeyCode, shift: bool) -> KeyEvent {
    KeyEvent {
        key_code,
        is_repeat: false,
        modifiers: KeyModifiers {
            shift,
            control: false,
            alt: false,
            logo: false,
        },
        time: 0.0,
    }
}

#[test]
fn test_edit_key_of_escape() {
    let ke = key_event(KeyCode::Escape, false);
    assert_eq!(edit_key_of(&ke), Some(EditKey::Escape));
}

#[test]
fn test_edit_key_of_return() {
    let ke = key_event(KeyCode::ReturnKey, false);
    assert_eq!(edit_key_of(&ke), Some(EditKey::Enter));
}

#[test]
fn test_edit_key_of_ctrl_return() {
    let mut ke = key_event(KeyCode::ReturnKey, false);
    ke.modifiers.control = true;
    assert_eq!(edit_key_of(&ke), Some(EditKey::CtrlEnter));
}

#[test]
fn test_edit_key_of_f2() {
    let ke = key_event(KeyCode::F2, false);
    assert_eq!(edit_key_of(&ke), Some(EditKey::F2));
}

#[test]
fn test_edit_key_of_tab() {
    let ke = key_event(KeyCode::Tab, false);
    assert_eq!(edit_key_of(&ke), Some(EditKey::Tab { back: false }));
}

#[test]
fn test_edit_key_of_shift_tab() {
    let ke = key_event(KeyCode::Tab, true);
    assert_eq!(edit_key_of(&ke), Some(EditKey::Tab { back: true }));
}

#[test]
fn test_edit_key_of_insert() {
    let ke = key_event(KeyCode::Insert, false);
    assert_eq!(edit_key_of(&ke), Some(EditKey::Insert));
}

#[test]
fn test_edit_key_of_arrows() {
    assert_eq!(
        edit_key_of(&key_event(KeyCode::ArrowLeft, false)),
        Some(EditKey::Left)
    );
    assert_eq!(
        edit_key_of(&key_event(KeyCode::ArrowRight, false)),
        Some(EditKey::Right)
    );
    assert_eq!(
        edit_key_of(&key_event(KeyCode::ArrowUp, false)),
        Some(EditKey::Up)
    );
    assert_eq!(
        edit_key_of(&key_event(KeyCode::ArrowDown, false)),
        Some(EditKey::Down)
    );
}

#[test]
fn test_edit_key_of_unknown() {
    assert_eq!(edit_key_of(&key_event(KeyCode::KeyA, false)), None);
    assert_eq!(edit_key_of(&key_event(KeyCode::Unknown, false)), None);
}

#[test]
fn the_row_is_scrolled_wholly_into_view_only_when_it_is_out() {
    assert_eq!(rows_that_fit(86.0, 24.0), 3);
    assert_eq!(rows_that_fit(10.0, 24.0), 1, "at least one");
    assert_eq!(scroll_target(10, 3, 11), None);
    assert_eq!(scroll_target(10, 3, 12), None, "the last row that fits");
    assert_eq!(
        scroll_target(10, 3, 13),
        Some(11),
        "lands as the last one in view"
    );
    assert_eq!(
        scroll_target(10, 3, 4),
        Some(4),
        "above: lands as the first"
    );
}

#[test]
fn the_error_box_goes_over_the_cell_when_there_is_no_room_below() {
    assert_eq!(error_box_top(10.0, 24.0, 200.0), 35.0);
    assert_eq!(
        error_box_top(72.0, 24.0, 100.0),
        72.0 - 1.0 - ERROR_BOX_HEIGHT
    );
}

#[test]
fn the_error_box_is_pulled_left_instead_of_being_cut_by_the_right_edge() {
    assert_eq!(error_box_left(40.0, 160.0, 470.0), 40.0);
    assert_eq!(error_box_left(374.0, 160.0, 470.0), 310.0);
    assert_eq!(error_box_left(10.0, 160.0, 100.0), 0.0);
}

#[test]
fn the_lookup_window_keeps_the_highlighted_row_in_sight() {
    assert_eq!(lookup_window(0, 0, 5, 0, true), (0, 0));
    assert_eq!(lookup_window(3, 2, 5, 0, true), (0, 3));
    assert_eq!(lookup_window(20, 0, 5, 0, true), (0, 5));
    assert_eq!(
        lookup_window(20, 7, 5, 0, true),
        (3, 5),
        "the highlighted row is the last shown"
    );
    assert_eq!(
        lookup_window(20, 2, 5, 6, true),
        (2, 5),
        "a highlight above the window pulls it up"
    );
    assert_eq!(
        lookup_window(40, 39, 50, 0, true),
        (24, 16),
        "never more than LOOKUP_ROWS"
    );
}

#[test]
fn the_wheel_scrolls_the_lookup_list_without_moving_the_highlight() {
    // The highlight did not move: the window stays where the wheel put it.
    assert_eq!(lookup_window(20, 0, 5, 9, false), (9, 5));
    // Past the end, it is held to the last full page.
    assert_eq!(lookup_window(20, 0, 5, 30, false), (15, 5));
}

#[test]
fn the_lookup_scroll_bar_shows_only_when_rows_overflow() {
    assert_eq!(lookup_thumb(5, 5, 0, 110.0), None);
    let (top, h) = lookup_thumb(20, 5, 0, 110.0).unwrap();
    assert_eq!((top, h), (0.0, 27.5));
    let (top, _) = lookup_thumb(20, 5, 15, 110.0).unwrap();
    assert_eq!(
        top,
        110.0 - 27.5,
        "the last page puts the thumb at the bottom"
    );
    let (_, h) = lookup_thumb(1000, 5, 0, 110.0).unwrap();
    assert_eq!(h, 12.0, "a thumb never shrinks below grabbing size");
}

#[test]
fn lookup_lines_put_the_display_columns_side_by_side() {
    let names = vec!["name".to_string(), "uf".to_string()];
    let rows = vec![
        vec!["Natal".to_string(), "RN".to_string()],
        vec!["São Paulo".to_string(), "SP".to_string()],
    ];
    let (header, lines) = lookup_lines(&names, &rows);
    assert_eq!(header, "name        uf");
    assert_eq!(lines, vec!["Natal       RN", "São Paulo   SP"]);
}

#[test]
fn the_lookup_list_opens_above_only_when_the_window_has_no_room_below() {
    // Room below: it stays under the cell.
    assert!(!lookup_opens_above(100.0, 24.0, 220.0, 800.0));
    // No room below, room above: it flips.
    assert!(lookup_opens_above(600.0, 24.0, 220.0, 800.0));
    // No room either way: it stays below rather than run off the top.
    assert!(!lookup_opens_above(100.0, 24.0, 220.0, 300.0));
    // The window not measured yet: never flips.
    assert!(!lookup_opens_above(600.0, 24.0, 220.0, 0.0));
}

/// After Insert + Save the row count can drop under the list's first row for a frame;
/// the first row is pulled back to the last one instead of crashing the app.
#[test]
fn a_first_row_past_the_end_moves_back_into_range() {
    assert_eq!(first_in_range(4, 1), Some(0));
    assert_eq!(first_in_range(8, 8), Some(7));
    assert_eq!(first_in_range(3, 8), None, "inside the range it stays");
    assert_eq!(first_in_range(4, 0), None, "an empty list draws nothing");
}
