// This file is EMITTED VERBATIM into the user's project. Everything in it --
// comments included -- ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules:
// 1. No `//!` and no `#![...]`.
// 2. No `use`, and no type of ours. Only `std`.
//
// SHORTCUT & KEY -- keyboard shortcuts combining modifiers with a key.

use std::fmt;
use std::str::FromStr;

/// A key on the keyboard, as events and shortcuts report it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Key {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Escape,
    Tab,
    Backspace,
    Enter,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    Copy,
    Cut,
    Paste,
    Colon,
    Comma,
    Backslash,
    Slash,
    Pipe,
    Questionmark,
    Exclamationmark,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Backtick,
    Minus,
    Period,
    Plus,
    Equals,
    Semicolon,
    Quote,
    BrowserBack,
    IntlBackslash,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
    /// A key this vocabulary has no name for, or no key assigned.
    #[default]
    Unknown,
}

impl Key {
    /// Checks if this key is empty or unknown.
    pub const fn is_none(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns the canonical display name for this key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "",
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::ArrowUp => "Up",
            Self::Escape => "Esc",
            Self::Tab => "Tab",
            Self::Backspace => "BkSp",
            Self::Enter => "Enter",
            Self::Space => "Space",
            Self::Insert => "Ins",
            Self::Delete => "Del",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PgUp",
            Self::PageDown => "PgDn",
            Self::Num0 => "0",
            Self::Num1 => "1",
            Self::Num2 => "2",
            Self::Num3 => "3",
            Self::Num4 => "4",
            Self::Num5 => "5",
            Self::Num6 => "6",
            Self::Num7 => "7",
            Self::Num8 => "8",
            Self::Num9 => "9",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::F21 => "F21",
            Self::F22 => "F22",
            Self::F23 => "F23",
            Self::F24 => "F24",
            Self::F25 => "F25",
            Self::F26 => "F26",
            Self::F27 => "F27",
            Self::F28 => "F28",
            Self::F29 => "F29",
            Self::F30 => "F30",
            Self::F31 => "F31",
            Self::F32 => "F32",
            Self::F33 => "F33",
            Self::F34 => "F34",
            Self::F35 => "F35",
            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Backslash => "\\",
            Self::Slash => "/",
            Self::Pipe => "|",
            Self::Questionmark => "?",
            Self::Exclamationmark => "!",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenCurlyBracket => "{",
            Self::CloseCurlyBracket => "}",
            Self::Backtick => "`",
            Self::Minus => "-",
            Self::Period => ".",
            Self::Plus => "+",
            Self::Equals => "=",
            Self::Semicolon => ";",
            Self::Quote => "\"",
            Self::BrowserBack => "BrowserBack",
            Self::IntlBackslash => "IntlBackslash",
            Self::ShiftLeft => "ShiftLeft",
            Self::ShiftRight => "ShiftRight",
            Self::ControlLeft => "ControlLeft",
            Self::ControlRight => "ControlRight",
            Self::AltLeft => "AltLeft",
            Self::AltRight => "AltRight",
            Self::SuperLeft => "SuperLeft",
            Self::SuperRight => "SuperRight",
        }
    }

    /// Parses a key from a string slice (case-insensitive).
    pub fn from_str_name(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty()
            || trimmed.eq_ignore_ascii_case("none")
            || trimmed.eq_ignore_ascii_case("unknown")
        {
            return Some(Self::Unknown);
        }

        if trimmed.len() == 1 {
            let c = trimmed.chars().next().unwrap();
            return match c.to_ascii_uppercase() {
                'A' => Some(Self::A),
                'B' => Some(Self::B),
                'C' => Some(Self::C),
                'D' => Some(Self::D),
                'E' => Some(Self::E),
                'F' => Some(Self::F),
                'G' => Some(Self::G),
                'H' => Some(Self::H),
                'I' => Some(Self::I),
                'J' => Some(Self::J),
                'K' => Some(Self::K),
                'L' => Some(Self::L),
                'M' => Some(Self::M),
                'N' => Some(Self::N),
                'O' => Some(Self::O),
                'P' => Some(Self::P),
                'Q' => Some(Self::Q),
                'R' => Some(Self::R),
                'S' => Some(Self::S),
                'T' => Some(Self::T),
                'U' => Some(Self::U),
                'V' => Some(Self::V),
                'W' => Some(Self::W),
                'X' => Some(Self::X),
                'Y' => Some(Self::Y),
                'Z' => Some(Self::Z),
                '0' => Some(Self::Num0),
                '1' => Some(Self::Num1),
                '2' => Some(Self::Num2),
                '3' => Some(Self::Num3),
                '4' => Some(Self::Num4),
                '5' => Some(Self::Num5),
                '6' => Some(Self::Num6),
                '7' => Some(Self::Num7),
                '8' => Some(Self::Num8),
                '9' => Some(Self::Num9),
                _ => None,
            };
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "f1" => Some(Self::F1),
            "f2" => Some(Self::F2),
            "f3" => Some(Self::F3),
            "f4" => Some(Self::F4),
            "f5" => Some(Self::F5),
            "f6" => Some(Self::F6),
            "f7" => Some(Self::F7),
            "f8" => Some(Self::F8),
            "f9" => Some(Self::F9),
            "f10" => Some(Self::F10),
            "f11" => Some(Self::F11),
            "f12" => Some(Self::F12),
            "enter" | "return" => Some(Self::Enter),
            "esc" | "escape" => Some(Self::Escape),
            "space" => Some(Self::Space),
            "tab" => Some(Self::Tab),
            "backspace" | "bksp" => Some(Self::Backspace),
            "del" | "delete" => Some(Self::Delete),
            "ins" | "insert" => Some(Self::Insert),
            "home" => Some(Self::Home),
            "end" => Some(Self::End),
            "pgup" | "pageup" => Some(Self::PageUp),
            "pgdn" | "pagedown" => Some(Self::PageDown),
            "up" | "arrowup" => Some(Self::ArrowUp),
            "down" | "arrowdown" => Some(Self::ArrowDown),
            "left" | "arrowleft" => Some(Self::ArrowLeft),
            "right" | "arrowright" => Some(Self::ArrowRight),
            "f13" => Some(Self::F13),
            "f14" => Some(Self::F14),
            "f15" => Some(Self::F15),
            "f16" => Some(Self::F16),
            "f17" => Some(Self::F17),
            "f18" => Some(Self::F18),
            "f19" => Some(Self::F19),
            "f20" => Some(Self::F20),
            "f21" => Some(Self::F21),
            "f22" => Some(Self::F22),
            "f23" => Some(Self::F23),
            "f24" => Some(Self::F24),
            "f25" => Some(Self::F25),
            "f26" => Some(Self::F26),
            "f27" => Some(Self::F27),
            "f28" => Some(Self::F28),
            "f29" => Some(Self::F29),
            "f30" => Some(Self::F30),
            "f31" => Some(Self::F31),
            "f32" => Some(Self::F32),
            "f33" => Some(Self::F33),
            "f34" => Some(Self::F34),
            "f35" => Some(Self::F35),
            "copy" => Some(Self::Copy),
            "cut" => Some(Self::Cut),
            "paste" => Some(Self::Paste),
            ":" | "colon" => Some(Self::Colon),
            "," | "comma" => Some(Self::Comma),
            "\\" | "backslash" => Some(Self::Backslash),
            "/" | "slash" => Some(Self::Slash),
            "|" | "pipe" => Some(Self::Pipe),
            "?" | "questionmark" => Some(Self::Questionmark),
            "!" | "exclamationmark" => Some(Self::Exclamationmark),
            "[" | "openbracket" => Some(Self::OpenBracket),
            "]" | "closebracket" => Some(Self::CloseBracket),
            "{" | "opencurlybracket" => Some(Self::OpenCurlyBracket),
            "}" | "closecurlybracket" => Some(Self::CloseCurlyBracket),
            "`" | "backtick" => Some(Self::Backtick),
            "-" | "minus" => Some(Self::Minus),
            "." | "period" => Some(Self::Period),
            "+" | "plus" => Some(Self::Plus),
            "=" | "equals" => Some(Self::Equals),
            ";" | "semicolon" => Some(Self::Semicolon),
            "\"" | "quote" => Some(Self::Quote),
            "browserback" => Some(Self::BrowserBack),
            "intlbackslash" => Some(Self::IntlBackslash),
            "shiftleft" => Some(Self::ShiftLeft),
            "shiftright" => Some(Self::ShiftRight),
            "controlleft" => Some(Self::ControlLeft),
            "controlright" => Some(Self::ControlRight),
            "altleft" => Some(Self::AltLeft),
            "altright" => Some(Self::AltRight),
            "superleft" => Some(Self::SuperLeft),
            "superright" => Some(Self::SuperRight),
            _ => None,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}

/// A keyboard shortcut combining modifier flags with a primary key.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Shortcut {
    /// The primary keyboard key.
    pub key: Key,
    /// Whether Control (or Command) modifier is active.
    pub ctrl: bool,
    /// Whether Alt (or Option) modifier is active.
    pub alt: bool,
    /// Whether Shift modifier is active.
    pub shift: bool,
    /// Whether Meta (Windows key or Command) is active.
    pub meta: bool,
}

impl Shortcut {
    /// Represents no assigned shortcut.
    pub const NONE: Self = Self {
        key: Key::Unknown,
        ctrl: false,
        alt: false,
        shift: false,
        meta: false,
    };

    /// Checks if this shortcut is empty.
    pub const fn is_none(self) -> bool {
        self.key.is_none()
    }

    /// Formats the shortcut as a human-readable text (e.g., "Ctrl+S", "Ctrl+Shift+F").
    pub fn to_string_formatted(self) -> String {
        if self.is_none() {
            return String::new();
        }

        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.meta {
            parts.push("Meta");
        }

        let key_str = self.key.as_str();
        if !key_str.is_empty() {
            parts.push(key_str);
        }

        parts.join("+")
    }

    /// Parses a human-readable shortcut string (e.g., "Ctrl+S", "Ctrl+Shift+Z", "F5", "Esc").
    pub fn from_str_formatted(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
            return Some(Self::NONE);
        }

        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut meta = false;
        let mut key = Key::Unknown;

        for part in trimmed.split('+') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            match part.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "meta" | "win" | "cmd" | "command" => meta = true,
                _ => {
                    if let Some(parsed_key) = Key::from_str_name(part) {
                        key = parsed_key;
                    }
                }
            }
        }

        if key.is_none() && !ctrl && !alt && !shift && !meta {
            None
        } else {
            Some(Self {
                key,
                ctrl,
                alt,
                shift,
                meta,
            })
        }
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_formatted())
    }
}

impl FromStr for Shortcut {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_formatted(s).ok_or(())
    }
}
