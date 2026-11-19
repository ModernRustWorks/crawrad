//! **The `EditMask` of a `MaskEdit`**: one pure engine, written once,
//! that both targets, the designer canvas and the Preview call — so a mask
//! cannot mean one thing on Makepad and another on egui.
//!
//! # The syntax (the classic subset)
//!
//! | char | meaning                                   |
//! |------|-------------------------------------------|
//! | `0`  | a digit, required                         |
//! | `9`  | a digit, optional                         |
//! | `#`  | a digit or a sign (`+`/`-`), optional     |
//! | `L`  | a letter, required                        |
//! | `l`  | a letter, optional                        |
//! | `A`  | a letter or a digit, required             |
//! | `a`  | a letter or a digit, optional             |
//! | `C`  | any character, required                   |
//! | `c`  | any character, optional                   |
//! | `>`  | the characters that follow go upper-case  |
//! | `<`  | the characters that follow go lower-case  |
//! | `<>` | case conversion off again                 |
//! | `\`  | the next character is a literal           |
//! | else | a literal, shown as is                    |
//!
//! A leading `!` is accepted and ignored. Two optional trailing sections
//! follow the mask, separated by `;`: `;1` / `;0` says whether the `Text`
//! keeps the literals (`1`, the default) or holds only what was typed (`0`),
//! and `;_` gives the blank shown in an empty position (`_` by default).
//! Example: `!99/99/0000;1;_`.
//!
//! An empty mask is no mask at all: every function hands the text back
//! untouched, so a `MaskEdit` without one is an `Edit`.
//!
//! # The two texts
//!
//! - the **shown** text is what the box displays: every position of the mask,
//!   literals in place and blanks where nothing was typed (`12/__/____`);
//! - the **typed** text is only what the user typed (`12`).
//!
//! On every change the generated app takes the edited box text, recovers the
//! typed characters ([`EditMask::raw`]) and lays them into the mask again
//! ([`EditMask::format`]) — [`EditMask::reshape`] does both — so only what
//! the mask allows can stay in the box.
//!
//! A character that is also one of the mask's literals cannot be typed into
//! an input position: [`EditMask::raw`] takes it for the literal.

use crawrad_note::note;
#[note("mask-accepts")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Accepts {
    Digit,
    DigitOrSign,
    Letter,
    LetterOrDigit,
    Any,
}

impl Accepts {
    fn takes(self, ch: char) -> bool {
        match self {
            Self::Digit => ch.is_ascii_digit(),
            Self::DigitOrSign => ch.is_ascii_digit() || ch == '+' || ch == '-',
            Self::Letter => ch.is_alphabetic(),
            Self::LetterOrDigit => ch.is_alphanumeric(),
            Self::Any => !ch.is_control(),
        }
    }
}

#[note("mask-case")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Case {
    Keep,
    Upper,
    Lower,
}

impl Case {
    fn apply(self, ch: char) -> char {
        match self {
            Self::Keep => ch,
            Self::Upper => ch.to_uppercase().next().unwrap_or(ch),
            Self::Lower => ch.to_lowercase().next().unwrap_or(ch),
        }
    }
}

#[note("mask-slot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Literal(char),
    Input {
        accepts: Accepts,
        required: bool,
        case: Case,
    },
}

#[note("mask-editmask")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditMask {
    slots: Vec<Slot>,
    keep_literals: bool,
    blank: char,
}

impl Default for EditMask {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            keep_literals: true,
            blank: '_',
        }
    }
}

#[note("mask-input-of")]
fn input_of(ch: char) -> Option<(Accepts, bool)> {
    Some(match ch {
        '0' => (Accepts::Digit, true),
        '9' => (Accepts::Digit, false),
        '#' => (Accepts::DigitOrSign, false),
        'L' => (Accepts::Letter, true),
        'l' => (Accepts::Letter, false),
        'A' => (Accepts::LetterOrDigit, true),
        'a' => (Accepts::LetterOrDigit, false),
        'C' => (Accepts::Any, true),
        'c' => (Accepts::Any, false),
        _ => return None,
    })
}

impl EditMask {
    #[note("mask-editmask-parse")]
    pub fn parse(mask: &str) -> Self {
        let mut out = Self::default();
        let mut chars = mask.chars().peekable();
        if chars.peek() == Some(&'!') {
            chars.next();
        }
        let mut case = Case::Keep;
        let mut sections = String::new();
        while let Some(ch) = chars.next() {
            match ch {
                ';' => {
                    sections = chars.collect();
                    break;
                }
                '\\' => {
                    if let Some(lit) = chars.next() {
                        out.slots.push(Slot::Literal(lit));
                    }
                }
                '>' => case = Case::Upper,
                '<' if chars.peek() == Some(&'>') => {
                    chars.next();
                    case = Case::Keep;
                }
                '<' => case = Case::Lower,
                _ => out.slots.push(match input_of(ch) {
                    Some((accepts, required)) => Slot::Input {
                        accepts,
                        required,
                        case,
                    },
                    None => Slot::Literal(ch),
                }),
            }
        }
        let mut parts = sections.split(';');
        if let Some(keep) = parts.next() {
            out.keep_literals = keep.trim() != "0";
        }
        if let Some(blank) = parts.next().and_then(|b| b.chars().next()) {
            out.blank = blank;
        }
        out
    }

    #[note("mask-editmask-is-empty")]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    #[note("mask-editmask-blank")]
    pub fn blank(&self) -> char {
        self.blank
    }

    #[note("mask-editmask-keeps-literals")]
    pub fn keeps_literals(&self) -> bool {
        self.keep_literals
    }

    #[note("mask-editmask-is-literal")]
    fn is_literal(&self, ch: char) -> bool {
        self.slots.contains(&Slot::Literal(ch))
    }

    #[note("mask-editmask-fill")]
    fn fill(&self, typed: &str) -> Vec<Option<char>> {
        let mut typed = typed.chars();
        self.slots
            .iter()
            .filter_map(|slot| match *slot {
                Slot::Literal(_) => None,
                Slot::Input { accepts, case, .. } => Some(
                    typed
                        .by_ref()
                        .find(|c| accepts.takes(*c))
                        .map(|c| case.apply(c)),
                ),
            })
            .collect()
    }

    #[note("mask-editmask-format")]
    pub fn format(&self, typed: &str) -> String {
        if self.is_empty() {
            return typed.to_string();
        }
        let mut filled = self.fill(typed).into_iter();
        self.slots
            .iter()
            .map(|slot| match *slot {
                Slot::Literal(ch) => ch,
                Slot::Input { .. } => filled.next().flatten().unwrap_or(self.blank),
            })
            .collect()
    }

    #[note("mask-editmask-raw")]
    pub fn raw(&self, shown: &str) -> String {
        if self.is_empty() {
            return shown.to_string();
        }
        shown
            .chars()
            .filter(|c| *c != self.blank && !self.is_literal(*c))
            .collect()
    }

    #[note("mask-editmask-reshape")]
    pub fn reshape(&self, edited: &str) -> String {
        self.format(&self.raw(edited))
    }

    #[note("mask-editmask-is-complete")]
    pub fn is_complete(&self, shown: &str) -> bool {
        let filled = self.fill(&self.raw(shown));
        let required = self.slots.iter().filter_map(|slot| match slot {
            Slot::Literal(_) => None,
            Slot::Input { required, .. } => Some(*required),
        });
        required.zip(filled).all(|(req, ch)| !req || ch.is_some())
    }

    #[note("mask-editmask-text")]
    pub fn text(&self, shown: &str) -> String {
        if self.keep_literals {
            self.reshape(shown)
        } else {
            self.fill(&self.raw(shown)).into_iter().flatten().collect()
        }
    }

    #[note("mask-editmask-hold-edit")]
    pub fn hold_edit(&self, before: &str, edited: &str, cursor: usize) -> (String, usize) {
        if self.is_empty() {
            return (edited.to_string(), cursor);
        }
        let before_count = before.chars().count();
        let edited_count = edited.chars().count();
        if edited_count < before_count && before_count == self.slots.len() {
            let cursor = cursor.min(edited_count);
            let missing = before_count - edited_count;
            let mut chars = edited.chars();
            let mut result = String::with_capacity(before.len());
            for _ in 0..cursor {
                if let Some(ch) = chars.next() {
                    result.push(ch);
                }
            }
            for i in cursor..cursor + missing {
                let placeholder = match self.slots.get(i) {
                    Some(Slot::Literal(ch)) => *ch,
                    Some(Slot::Input { .. }) | None => self.blank,
                };
                result.push(placeholder);
            }
            result.extend(chars);
            (result, cursor)
        } else {
            self.hold(edited, cursor)
        }
    }

    #[note("mask-editmask-hold")]
    pub fn hold(&self, edited: &str, cursor: usize) -> (String, usize) {
        if self.is_empty() {
            return (edited.to_string(), cursor);
        }
        (self.reshape(edited), self.cursor_after(edited, cursor))
    }

    #[note("mask-editmask-cursor-after")]
    pub fn cursor_after(&self, edited: &str, cursor: usize) -> usize {
        if self.is_empty() {
            return cursor;
        }
        let before: String = edited.chars().take(cursor).collect();
        let taken = self.fill(&self.raw(&before)).into_iter().flatten().count();
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| matches!(slot, Slot::Input { .. }))
            .nth(taken)
            .map_or(self.slots.len(), |(at, _)| at)
    }
}

#[note("mask-shown-text")]
pub fn shown_text(mask: &str, text: &str) -> String {
    EditMask::parse(mask).reshape(text)
}

#[cfg(test)]
#[path = "test/mask/test.rs"]
mod test;
