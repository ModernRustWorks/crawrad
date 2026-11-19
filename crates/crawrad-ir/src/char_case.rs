//! **The case an edit holds its text to** (`CharCase`) — the rule, in
//! one place.
//!
//! This is the user's *text*, never a name: `ecUpperCase` turns what is typed
//! into `ABC`, as the classic edit does, and turns the design-time `Text` too
//! the moment the case is set, so the app opens with it turned.
//!
//! The Makepad runtime and the generator read it here. The egui app carries
//! its own copy (`crawrad-runtime-egui/src/desenho_comum.rs`, section
//! `char_case`), since a generated project depends on no crate of ours.

use crate::{Node, Value};
use crawrad_note::note;

#[note("char-case-char-cased")]
pub fn char_cased(text: &str, char_case: &str) -> String {
    match char_case {
        "ecUpperCase" => text.to_uppercase(),
        "ecLowerCase" => text.to_lowercase(),
        _ => text.to_string(),
    }
}

#[note("char-case-cased-for")]
pub fn cased_for(node: &Node, text: &str) -> String {
    match node.props.get("char_case") {
        Some(Value::Enum(c)) | Some(Value::Str(c)) => char_cased(text, c),
        _ => text.to_string(),
    }
}

#[cfg(test)]
#[path = "test/char_case/test.rs"]
mod tests;
