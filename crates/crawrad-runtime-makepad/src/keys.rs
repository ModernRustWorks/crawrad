//! **Enter in a text box does what it does in a classic RAD IDE**.
//!
//! A generated app draws `Memo` and `Edit` as makepad's `TextInput`, and that
//! widget answers Enter its own way (`makepad-widgets-1.0.0/src/text_input.rs`,
//! the two `KeyCode::ReturnKey` arms): plain Enter only reports `Returned`,
//! Shift+Enter inserts a line break. A classic RAD IDE is the other way round for a
//! `TMemo` — Enter breaks the line (`WantReturns` is on by default) — and a
//! `TEdit` holds one line, so no key puts a break in it.
//!
//! The generated `handle_event` hands every key to [`text_box_enter`] before the
//! widgets see it, with the form's memos and edits; the key the widget gets is
//! the one that produces the classic result:
//!
//! | focus | pressed | the widget gets | result |
//! |---|---|---|---|
//! | Memo | Enter | Shift+Enter | line break |
//! | Edit | Shift+Enter | Enter | no line break |
//!
//! Everything else goes through untouched.

use crawrad_note::note;
use makepad_widgets::*;

#[note("keys-text-box-enter")]
pub fn text_box_enter(
    cx: &Cx,
    memos: &[WidgetRef],
    edits: &[WidgetRef],
    event: &Event,
) -> Option<Event> {
    let Event::KeyDown(key) = event else {
        return None;
    };
    if key.key_code != KeyCode::ReturnKey {
        return None;
    }
    let focused = |boxes: &[WidgetRef]| boxes.iter().any(|b| b.key_focus(cx));
    let focus = if focused(memos) {
        Focus::Memo
    } else if focused(edits) {
        Focus::Edit
    } else {
        return None;
    };
    rewrite(key, focus).map(Event::KeyDown)
}

#[note("keys-focus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Memo,
    Edit,
}

#[note("keys-rewrite")]
pub fn rewrite(key: &KeyEvent, focus: Focus) -> Option<KeyEvent> {
    if key.key_code != KeyCode::ReturnKey {
        return None;
    }
    let shift = match (focus, key.modifiers.shift) {
        // makepad breaks the line on Shift+Enter: a Memo's Enter becomes that.
        (Focus::Memo, false) => true,
        // An Edit holds one line: its Shift+Enter becomes a plain Enter.
        (Focus::Edit, true) => false,
        _ => return None,
    };
    Some(KeyEvent {
        modifiers: KeyModifiers {
            shift,
            ..key.modifiers
        },
        ..*key
    })
}

#[note("keys-chord")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Chord {
    pub primary: bool,
    pub shift: bool,
    pub alt: bool,
}

#[note("keys-char-keys")]
pub const CHAR_KEYS: [KeyCode; 36] = [
    KeyCode::KeyA,
    KeyCode::KeyB,
    KeyCode::KeyC,
    KeyCode::KeyD,
    KeyCode::KeyE,
    KeyCode::KeyF,
    KeyCode::KeyG,
    KeyCode::KeyH,
    KeyCode::KeyI,
    KeyCode::KeyJ,
    KeyCode::KeyK,
    KeyCode::KeyL,
    KeyCode::KeyM,
    KeyCode::KeyN,
    KeyCode::KeyO,
    KeyCode::KeyP,
    KeyCode::KeyQ,
    KeyCode::KeyR,
    KeyCode::KeyS,
    KeyCode::KeyT,
    KeyCode::KeyU,
    KeyCode::KeyV,
    KeyCode::KeyW,
    KeyCode::KeyX,
    KeyCode::KeyY,
    KeyCode::KeyZ,
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

const NAMED_KEYS: &[(&str, KeyCode)] = &[
    ("Del", KeyCode::Delete),
    ("Delete", KeyCode::Delete),
    ("Ins", KeyCode::Insert),
    ("Insert", KeyCode::Insert),
    ("Esc", KeyCode::Escape),
    ("Escape", KeyCode::Escape),
    ("Enter", KeyCode::ReturnKey),
    ("Return", KeyCode::ReturnKey),
    ("Tab", KeyCode::Tab),
    ("Backspace", KeyCode::Backspace),
    ("Home", KeyCode::Home),
    ("End", KeyCode::End),
    ("PageUp", KeyCode::PageUp),
    ("PageDown", KeyCode::PageDown),
    ("Space", KeyCode::Space),
    ("Left", KeyCode::ArrowLeft),
    ("Right", KeyCode::ArrowRight),
    ("Up", KeyCode::ArrowUp),
    ("Down", KeyCode::ArrowDown),
    (".", KeyCode::Period),
    (",", KeyCode::Comma),
    ("-", KeyCode::Minus),
    ("=", KeyCode::Equals),
    ("/", KeyCode::Slash),
    (";", KeyCode::Semicolon),
    ("'", KeyCode::Quote),
    ("\\", KeyCode::Backslash),
    ("`", KeyCode::Backtick),
    ("NumpadAdd", KeyCode::NumpadAdd),
    ("NumpadSubtract", KeyCode::NumpadSubtract),
    ("Numpad0", KeyCode::Numpad0),
    ("[", KeyCode::LBracket),
    ("]", KeyCode::RBracket),
];

#[note("keys-key-by-name")]
pub fn key_by_name(name: &str) -> Option<KeyCode> {
    if let Some(n) = name.strip_prefix('F')
        && let Ok(n) = n.parse::<u8>()
    {
        const F_KEYS: [KeyCode; 12] = [
            KeyCode::F1,
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
            KeyCode::F8,
            KeyCode::F9,
            KeyCode::F10,
            KeyCode::F11,
            KeyCode::F12,
        ];
        if (1..=12).contains(&n) {
            return Some(F_KEYS[(n - 1) as usize]);
        }
        return None;
    }
    if name.chars().count() == 1 {
        let c = name.chars().next().unwrap_or('\0').to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            return CHAR_KEYS
                .iter()
                .copied()
                .find(|k| k.to_char(false) == Some(c));
        }
    }
    NAMED_KEYS
        .iter()
        .find(|(k, _)| *k == name)
        .map(|(_, code)| *code)
}

#[note("keys-parse-chord")]
pub fn parse_chord(chord: &str) -> Option<(KeyCode, Chord)> {
    let mut req = Chord::default();
    let mut name = "";
    for part in chord.split('+') {
        match part.trim() {
            "" => {}
            "Ctrl" | "Cmd" => req.primary = true,
            "Shift" => req.shift = true,
            "Alt" => req.alt = true,
            other => name = other,
        }
    }
    Some((key_by_name(name)?, req))
}

#[note("keys-matches-shortcut")]
pub fn matches_shortcut(chord: &str, key: KeyCode, m: &KeyModifiers) -> bool {
    let Some((expected_key, req)) = parse_chord(chord) else {
        return false;
    };
    expected_key == key && req.primary == m.is_primary() && req.shift == m.shift && req.alt == m.alt
}

#[note("keys-arrow-direction")]
pub fn arrow_direction(key: KeyCode) -> Option<(f32, f32)> {
    match key {
        KeyCode::ArrowLeft => Some((-1.0, 0.0)),
        KeyCode::ArrowRight => Some((1.0, 0.0)),
        KeyCode::ArrowUp => Some((0.0, -1.0)),
        KeyCode::ArrowDown => Some((0.0, 1.0)),
        _ => None,
    }
}

#[note("keys-digit-of")]
pub fn digit_of(key: KeyCode) -> Option<u8> {
    let i = CHAR_KEYS.iter().position(|k| *k == key)?;
    (27..=35).contains(&i).then(|| (i - 26) as u8)
}

#[cfg(test)]
#[path = "test/keys/test.rs"]
mod tests;
