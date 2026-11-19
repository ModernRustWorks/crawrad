use super::shortcut::{Key, Shortcut};
use std::str::FromStr;

#[test]
fn default_shortcut_is_none() {
    let s = Shortcut::default();
    assert!(s.is_none());
    assert_eq!(s.key, Key::Unknown);
    assert_eq!(s.to_string(), "");
}

#[test]
fn parse_and_display_shortcuts() {
    let cases = [
        ("Ctrl+S", Key::S, true, false, false),
        ("Ctrl+Shift+P", Key::P, true, false, true),
        ("Alt+F4", Key::F4, false, true, false),
        ("F5", Key::F5, false, false, false),
        ("Ctrl+Alt+Del", Key::Delete, true, true, false),
        ("Esc", Key::Escape, false, false, false),
        ("Enter", Key::Enter, false, false, false),
        ("", Key::Unknown, false, false, false),
        ("None", Key::Unknown, false, false, false),
    ];

    for (input, expected_key, ctrl, alt, shift) in cases {
        let parsed =
            Shortcut::from_str(input).unwrap_or_else(|_| panic!("failed to parse {input}"));
        assert_eq!(parsed.key, expected_key, "key mismatch for {input}");
        assert_eq!(parsed.ctrl, ctrl, "ctrl mismatch for {input}");
        assert_eq!(parsed.alt, alt, "alt mismatch for {input}");
        assert_eq!(parsed.shift, shift, "shift mismatch for {input}");

        if !parsed.is_none() {
            let formatted = parsed.to_string();
            let roundtrip = Shortcut::from_str(&formatted).expect("roundtrip parse");
            assert_eq!(
                parsed, roundtrip,
                "roundtrip failed for {input} -> {formatted}"
            );
        }
    }
}

#[test]
fn construction() {
    let s = Shortcut {
        key: Key::S,
        ctrl: true,
        alt: false,
        shift: false,
        meta: false,
    };
    assert_eq!(s.key, Key::S);
    assert!(s.ctrl);
    assert!(!s.alt);
}
