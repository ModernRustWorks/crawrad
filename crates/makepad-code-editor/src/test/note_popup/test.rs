//! Unit tests for note popup pure state and geometry.

use crate::note_anchor::AnchorKind;
use crate::note_popup::{NotePopupState, PopupItem, PopupMouseResult};
use makepad_widgets::*;

fn sample_items() -> Vec<PopupItem> {
    vec![
        PopupItem {
            kind: AnchorKind::Doc,
            title: "Add documentation".to_string(),
            description: "Public docs".to_string(),
            shortcut: "Ctrl+Alt+D",
            enabled: true,
        },
        PopupItem {
            kind: AnchorKind::Note,
            title: "Add note".to_string(),
            description: "Private context".to_string(),
            shortcut: "Ctrl+Alt+N",
            enabled: true,
        },
        PopupItem {
            kind: AnchorKind::Comment,
            title: "Add comment".to_string(),
            description: "A short remark".to_string(),
            shortcut: "Ctrl+Alt+M",
            enabled: true,
        },
    ]
}

#[test]
fn opening_selects_first_enabled_row() {
    let mut state = NotePopupState::default();
    let items = sample_items();
    state.open("Line 10 · Date".to_string(), items);
    assert!(state.open);
    assert_eq!(state.selected, Some(0));

    // When first row is disabled, second row is selected on open
    let mut disabled_first = sample_items();
    disabled_first[0].enabled = false;
    let mut state2 = NotePopupState::default();
    state2.open("Line 10".to_string(), disabled_first);
    assert_eq!(state2.selected, Some(1));
}

#[test]
fn arrow_keys_move_skipping_disabled_rows() {
    let mut items = sample_items();
    items[0].enabled = false; // Doc disabled
    let mut state = NotePopupState::default();
    state.open("Line 10".to_string(), items);

    assert_eq!(state.selected, Some(1));
    // Up should not select disabled row 0
    state.move_up();
    assert_eq!(state.selected, Some(1));

    // Down moves to row 2
    state.move_down();
    assert_eq!(state.selected, Some(2));

    // Down at bottom stays at row 2
    state.move_down();
    assert_eq!(state.selected, Some(2));

    // Up moves back to row 1
    state.move_up();
    assert_eq!(state.selected, Some(1));
}

#[test]
fn enter_chooses_selected_row() {
    let mut state = NotePopupState::default();
    state.open("Line 5".to_string(), sample_items());
    state.selected = Some(1);
    let chosen = state.choose_selected();
    assert_eq!(chosen, Some(1));
    assert!(!state.open);
    assert_eq!(state.take_choice(), Some(1));
    assert_eq!(state.take_choice(), None);
}

#[test]
fn letter_shortcuts_choose_directly_if_enabled() {
    let mut items = sample_items();
    items[0].enabled = false;
    let mut state = NotePopupState::default();
    state.open("Line 1".to_string(), items);

    // 'd' or 'D' is disabled -> returns None and popup stays open
    assert_eq!(state.handle_letter('d'), None);
    assert!(state.open);

    // 'n' chooses note (row 1) and closes
    assert_eq!(state.handle_letter('n'), Some(1));
    assert!(!state.open);

    // 'M' chooses comment (row 2)
    let mut state2 = NotePopupState::default();
    state2.open("Line 1".to_string(), sample_items());
    assert_eq!(state2.handle_letter('M'), Some(2));
    assert!(!state2.open);
}

#[test]
fn mouse_interactions_hover_and_click() {
    let mut state = NotePopupState::default();
    state.open("Line 1".to_string(), sample_items());
    let viewport = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(800.0, 600.0),
    };
    state.compute_layout(dvec2(50.0, 100.0), 20.0, viewport, 1.0, 0.0);

    // Hover over row 2
    let row2_center = state.row_rects[2].pos + state.row_rects[2].size * 0.5;
    state.handle_mouse_move(row2_center);
    assert_eq!(state.selected, Some(2));

    // Click inside row 2 chooses it
    let res = state.handle_mouse_down(row2_center);
    assert_eq!(res, PopupMouseResult::Chosen(2));
    assert!(!state.open);

    // Reopen and click outside closes
    let mut state3 = NotePopupState::default();
    state3.open("Line 1".to_string(), sample_items());
    state3.compute_layout(dvec2(50.0, 100.0), 20.0, viewport, 1.0, 0.0);
    let outside_pos = dvec2(700.0, 500.0);
    let res3 = state3.handle_mouse_down(outside_pos);
    assert_eq!(res3, PopupMouseResult::Closed);
    assert!(!state3.open);
}

#[test]
fn layout_anchors_under_caret_and_flips_if_overflow() {
    let mut state = NotePopupState::default();
    state.open("Line 1".to_string(), sample_items());
    let viewport = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(800.0, 600.0),
    };

    // Ample space below (caret at y = 100)
    state.compute_layout(dvec2(100.0, 100.0), 20.0, viewport, 1.0, 0.0);
    assert!(state.panel_rect.pos.y > 100.0);

    // Near bottom of viewport (caret at y = 550, viewport height 600)
    state.compute_layout(dvec2(100.0, 550.0), 20.0, viewport, 1.0, 0.0);
    // Flips above line: panel y should be < 550.0
    assert!(state.panel_rect.pos.y < 550.0);

    // Near right edge of viewport (caret at x = 750, panel width ~380)
    state.compute_layout(dvec2(750.0, 100.0), 20.0, viewport, 1.0, 0.0);
    // Panel should be shifted left to remain inside viewport
    assert!(state.panel_rect.pos.x + state.panel_rect.size.x <= viewport.size.x);

    // Short texts keep the 380 floor; a wider text widens the panel to fit.
    assert_eq!(state.panel_rect.size.x, 380.0);
    state.compute_layout(dvec2(100.0, 100.0), 20.0, viewport, 1.0, 500.0);
    assert!(state.panel_rect.size.x >= 500.0);
    assert!(state.row_rects.iter().all(|r| r.size.x >= 500.0));
}
