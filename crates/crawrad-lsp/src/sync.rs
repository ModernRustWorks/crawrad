//! **Document synchronization: only the part that changed** (LSP 3.17,
//! *Text Document Synchronization*, `TextDocumentSyncKind.Incremental`).
//!
//! The IDE used to send the whole file on every key (`"change": 1`, the
//! `contentChanges` with only `text`). The shell cannot hand over the edits
//! — the editor drops its `TextDidChange` — so it passes the new full text,
//! and the delta is computed here from the text last sent: the common
//! prefix and suffix, and one `{ range, text }` for what is between.
//!
//! Positions are in the negotiated encoding (`crate::encoding`). The change never splits a
//! character nor a `\r\n` (rust-analyzer counts `\r\n` as one line break).

use serde_json::Value;

use crate::protocol::{Position, Range};
use crawrad_note::note;

#[note("sync-synckind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncKind {
    /// The server wants no change notifications.
    None,
    /// The whole text each time.
    Full,
    /// Only the ranges that changed.
    Incremental,
}

#[note("sync-sync-kind-from")]
pub fn sync_kind_from(capabilities: &Value) -> SyncKind {
    let sync = capabilities.get("textDocumentSync");
    let kind = sync
        .and_then(Value::as_u64)
        .or_else(|| sync.and_then(|s| s.get("change")).and_then(Value::as_u64));
    match kind {
        Some(0) => SyncKind::None,
        Some(2) => SyncKind::Incremental,
        _ => SyncKind::Full,
    }
}

#[note("sync-contentchange")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentChange {
    pub range: Range,
    pub text: String,
}

#[note("sync-incremental-change")]
pub fn incremental_change(
    old: &str,
    new: &str,
    enc: crate::encoding::PositionEncoding,
) -> Option<ContentChange> {
    if old == new {
        return None;
    }
    // Common prefix, on char boundaries of both texts.
    let mut prefix = old
        .char_indices()
        .zip(new.chars())
        .find(|((_, a), b)| a != b)
        .map_or(old.len().min(new.len()), |((i, _), _)| i);
    while !new.is_char_boundary(prefix) {
        prefix -= 1;
    }
    // Never start between `\r` and `\n`.
    if prefix > 0 && old.as_bytes()[prefix - 1] == b'\r' && old[prefix..].starts_with('\n') {
        prefix -= 1;
    }
    // Common suffix, not overlapping the prefix in either text.
    let max_suffix = (old.len() - prefix).min(new.len() - prefix);
    let mut suffix = old
        .bytes()
        .rev()
        .zip(new.bytes().rev())
        .take(max_suffix)
        .take_while(|(a, b)| a == b)
        .count();
    while !old.is_char_boundary(old.len() - suffix) || !new.is_char_boundary(new.len() - suffix) {
        suffix -= 1;
    }
    // Never end between `\r` and `\n`.
    let old_end = old.len() - suffix;
    if old_end > 0 && old.as_bytes()[old_end - 1] == b'\r' && old[old_end..].starts_with('\n') {
        suffix -= 1;
    }
    let old_end = old.len() - suffix;
    Some(ContentChange {
        range: Range {
            start: position_of(old, prefix, enc),
            end: position_of(old, old_end, enc),
        },
        text: new[prefix..new.len() - suffix].to_string(),
    })
}

#[note("sync-position-of")]
pub fn position_of(text: &str, offset: usize, enc: crate::encoding::PositionEncoding) -> Position {
    let before = &text[..offset];
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    Position {
        line: u32::try_from(before.matches('\n').count()).unwrap_or(u32::MAX),
        character: enc.column(&before[line_start..], offset - line_start),
    }
}

#[note("sync-offset-of")]
pub fn offset_of(text: &str, position: Position, enc: crate::encoding::PositionEncoding) -> usize {
    let mut line_start = 0;
    for _ in 0..position.line {
        match text[line_start..].find('\n') {
            Some(i) => line_start += i + 1,
            None => return text.len(),
        }
    }
    let line_end = text[line_start..]
        .find('\n')
        .map_or(text.len(), |i| line_start + i);
    line_start + enc.byte(&text[line_start..line_end], position.character)
}

#[note("sync-apply-change")]
pub fn apply_change(
    text: &str,
    change: &ContentChange,
    enc: crate::encoding::PositionEncoding,
) -> String {
    let start = offset_of(text, change.range.start, enc);
    let end = offset_of(text, change.range.end, enc);
    format!("{}{}{}", &text[..start], change.text, &text[end..])
}

#[cfg(test)]
#[path = "test/sync/test.rs"]
mod tests;
