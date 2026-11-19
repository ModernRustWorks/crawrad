use super::{Focus, rewrite};
use makepad_widgets::*;

fn key(code: KeyCode, shift: bool) -> KeyEvent {
    KeyEvent {
        key_code: code,
        modifiers: KeyModifiers {
            shift,
            ..KeyModifiers::default()
        },
        ..KeyEvent::default()
    }
}

#[test]
fn enter_in_a_memo_reaches_the_widget_as_the_key_that_breaks_the_line() {
    let got =
        rewrite(&key(KeyCode::ReturnKey, false), Focus::Memo).expect("Enter must be rewritten");
    assert_eq!(got.key_code, KeyCode::ReturnKey);
    assert!(
        got.modifiers.shift,
        "makepad breaks the line on Shift+Enter"
    );
}

#[test]
fn shift_enter_in_a_memo_already_breaks_the_line() {
    assert_eq!(rewrite(&key(KeyCode::ReturnKey, true), Focus::Memo), None);
}

#[test]
fn shift_enter_in_an_edit_reaches_it_as_a_plain_enter() {
    let got = rewrite(&key(KeyCode::ReturnKey, true), Focus::Edit)
        .expect("Shift+Enter must be rewritten");
    assert!(
        !got.modifiers.shift,
        "a one-line box never gets the line-break key"
    );
}

#[test]
fn enter_in_an_edit_is_left_alone() {
    assert_eq!(rewrite(&key(KeyCode::ReturnKey, false), Focus::Edit), None);
}

#[test]
fn other_keys_and_other_modifiers_go_through_as_they_are() {
    assert_eq!(rewrite(&key(KeyCode::KeyA, false), Focus::Memo), None);
    let mut ctrl_enter = key(KeyCode::ReturnKey, false);
    ctrl_enter.modifiers.control = true;
    let got = rewrite(&ctrl_enter, Focus::Memo).unwrap();
    assert!(got.modifiers.control, "the other modifiers are kept");
}

#[test]
fn key_by_name_translates_function_keys() {
    use super::key_by_name;
    assert_eq!(key_by_name("F1"), Some(KeyCode::F1));
    assert_eq!(key_by_name("F5"), Some(KeyCode::F5));
    assert_eq!(key_by_name("F12"), Some(KeyCode::F12));
    assert_eq!(key_by_name("F"), Some(KeyCode::KeyF));
    assert_eq!(key_by_name("F0"), None);
    assert_eq!(key_by_name("F13"), None);
}

#[test]
fn key_by_name_translates_alphanumeric_and_named_keys() {
    use super::key_by_name;
    assert_eq!(key_by_name("a"), Some(KeyCode::KeyA));
    assert_eq!(key_by_name("Z"), Some(KeyCode::KeyZ));
    assert_eq!(key_by_name("1"), Some(KeyCode::Key1));
    assert_eq!(key_by_name("Del"), Some(KeyCode::Delete));
    assert_eq!(key_by_name("Delete"), Some(KeyCode::Delete));
    assert_eq!(key_by_name("Esc"), Some(KeyCode::Escape));
    assert_eq!(key_by_name("Return"), Some(KeyCode::ReturnKey));
    assert_eq!(key_by_name("Space"), Some(KeyCode::Space));
    assert_eq!(key_by_name("Left"), Some(KeyCode::ArrowLeft));
    assert_eq!(key_by_name("NumpadAdd"), Some(KeyCode::NumpadAdd));
    assert_eq!(key_by_name("nonexistent_key"), None);
}

#[test]
fn parse_chord_decomposes_modifiers_and_key() {
    use super::{Chord, parse_chord};
    let (key, chord) = parse_chord("Ctrl+Shift+B").expect("valid chord");
    assert_eq!(key, KeyCode::KeyB);
    assert_eq!(
        chord,
        Chord {
            primary: true,
            shift: true,
            alt: false,
        }
    );

    let (key2, chord2) = parse_chord("Alt+F4").expect("valid chord");
    assert_eq!(key2, KeyCode::F4);
    assert_eq!(
        chord2,
        Chord {
            primary: false,
            shift: false,
            alt: true,
        }
    );

    assert_eq!(parse_chord("UnknownKey"), None);
}

#[test]
fn arrow_direction_reports_unit_signs() {
    use super::arrow_direction;
    assert_eq!(arrow_direction(KeyCode::ArrowLeft), Some((-1.0, 0.0)));
    assert_eq!(arrow_direction(KeyCode::ArrowRight), Some((1.0, 0.0)));
    assert_eq!(arrow_direction(KeyCode::ArrowUp), Some((0.0, -1.0)));
    assert_eq!(arrow_direction(KeyCode::ArrowDown), Some((0.0, 1.0)));
    assert_eq!(arrow_direction(KeyCode::Space), None);
}

#[test]
fn digit_of_extracts_bookmark_numbers() {
    use super::digit_of;
    assert_eq!(digit_of(KeyCode::Key1), Some(1));
    assert_eq!(digit_of(KeyCode::Key5), Some(5));
    assert_eq!(digit_of(KeyCode::Key9), Some(9));
    assert_eq!(digit_of(KeyCode::Key0), None);
    assert_eq!(digit_of(KeyCode::KeyA), None);
}

#[test]
fn matches_shortcut_requires_exact_modifiers() {
    use super::matches_shortcut;
    let mods = KeyModifiers {
        control: true,
        ..Default::default()
    };
    assert!(matches_shortcut("Ctrl+Z", KeyCode::KeyZ, &mods));

    let mut shift_mods = mods;
    shift_mods.shift = true;
    assert!(!matches_shortcut("Ctrl+Z", KeyCode::KeyZ, &shift_mods));
}
