//! Translatable property keys and key formatting rule.

use crate::NodeId;

/// Translatable property keys declared across components.
pub const TRANSLATABLE: &[&str] = &["text", "hint", "title", "text_hint"];

/// Computes the persistent translation key `"{form}.{node}.{prop}"`.
pub fn text_key(form: &str, node: &NodeId, prop: &str) -> String {
    format!("{form}.{node}.{prop}")
}

#[cfg(test)]
#[path = "test/translation_key/test.rs"]
mod tests;
