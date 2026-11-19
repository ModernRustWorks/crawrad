//! What the server answers when it changes code — Rename Symbol and Format
//! Document — and how those edits land on a text.
//!
//! The server only proposes: nothing here writes a file. Whoever owns the
//! buffers decides whether the edits apply (the RAD guard refuses a
//! rename that would touch a name the form owns).

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::protocol::Range;
use crawrad_note::note;

#[note("edit-textedit")]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[note("edit-workspaceedit")]
pub type WorkspaceEdit = BTreeMap<String, Vec<TextEdit>>;

#[note("edit-workspace-edit-from")]
pub fn workspace_edit_from(value: serde_json::Value) -> Result<WorkspaceEdit, String> {
    let mut out = WorkspaceEdit::new();
    if value.is_null() {
        return Ok(out);
    }
    if let Some(changes) = value.get("changes").and_then(|c| c.as_object()) {
        for (uri, edits) in changes {
            let edits: Vec<TextEdit> =
                serde_json::from_value(edits.clone()).map_err(|e| e.to_string())?;
            out.entry(uri.clone()).or_default().extend(edits);
        }
    }
    if let Some(changes) = value.get("documentChanges").and_then(|c| c.as_array()) {
        for change in changes {
            if let Some(kind) = change.get("kind").and_then(|k| k.as_str()) {
                return Err(format!("the server asked for a file {kind}"));
            }
            let uri = change
                .pointer("/textDocument/uri")
                .and_then(|u| u.as_str())
                .ok_or("a document change without textDocument.uri")?;
            let edits: Vec<TextEdit> =
                serde_json::from_value(change.get("edits").cloned().unwrap_or_default())
                    .map_err(|e| e.to_string())?;
            out.entry(uri.to_string()).or_default().extend(edits);
        }
    }
    Ok(out)
}

#[note("edit-apply-text-edits")]
pub fn apply_text_edits(
    text: &str,
    edits: &[TextEdit],
    enc: crate::encoding::PositionEncoding,
) -> String {
    let line_starts = line_starts(text);
    let offset = |line: u32, character: u32| -> usize {
        let Some(&start) = line_starts.get(line as usize) else {
            return text.len();
        };
        let end = text[start..].find('\n').map_or(text.len(), |n| start + n);
        let line_text = text[start..end]
            .strip_suffix('\r')
            .unwrap_or(&text[start..end]);
        start + enc.byte(line_text, character)
    };
    let mut spans: Vec<(usize, usize, &str)> = edits
        .iter()
        .map(|e| {
            let from = offset(e.range.start.line, e.range.start.character);
            let to = offset(e.range.end.line, e.range.end.character).max(from);
            (from, to, e.new_text.as_str())
        })
        .collect();
    spans.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    let mut out = text.to_string();
    for (from, to, new_text) in spans {
        out.replace_range(from..to, new_text);
    }
    out
}

#[note("edit-line-starts")]
fn line_starts(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(text.match_indices('\n').map(|(i, _)| i + 1))
        .collect()
}

#[cfg(test)]
#[path = "test/edit/test.rs"]
mod tests;
