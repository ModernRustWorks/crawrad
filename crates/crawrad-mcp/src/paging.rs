//! Paging a text for MCP reads: a byte budget that never splits a character.

use crawrad_note::note;
#[note("paging-slice-utf8-budget")]
pub fn slice_utf8_budget(text: &str, offset: usize, limit: usize) -> (String, bool) {
    if offset >= text.len() {
        return (String::new(), false);
    }

    let start = text.floor_char_boundary(offset);
    let end_target = (start + limit).min(text.len());
    let end = text.floor_char_boundary(end_target);

    let sliced = &text[start..end];
    let has_more = end < text.len();
    (sliced.to_string(), has_more)
}

#[note("paging-read-answer")]
pub fn read_answer(
    path: &str,
    text: &str,
    hash: &str,
    args: &serde_json::Value,
    predicted: Option<String>,
) -> (serde_json::Value, String) {
    let n = |k: &str, d: u64| args.get(k).and_then(serde_json::Value::as_u64).unwrap_or(d) as usize;
    let (offset, limit) = (n("offset", 0), n("limit", 65536));
    let (slice, has_more) = slice_utf8_budget(text, offset, limit);
    let lines = text.lines().count();
    let pending = predicted.is_some();
    let note = if pending {
        " [NOTE: file has a pending diff review awaiting user decision]"
    } else {
        ""
    };
    let said = format!(
        "Read {} bytes ({lines} lines) from '{path}' (hash: {hash}){note}",
        slice.len()
    );
    let answer = serde_json::json!({
        "path": path, "content": slice, "content_hash": hash, "lines_count": lines,
        "has_more": has_more, "total_bytes": text.len(), "offset": offset,
        "pending_diff_review": pending, "predicted_hash": predicted,
    });
    (answer, said)
}

/// Directly formats a successful paged read response into a [`crate::protocol::ToolCallResult`].
pub fn paged_read_success(
    path: &str,
    text: &str,
    hash: &str,
    args: &serde_json::Value,
    predicted: Option<String>,
) -> crate::protocol::ToolCallResult {
    let (answer, said) = read_answer(path, text, hash, args, predicted);
    crate::protocol::ToolCallResult::success_json(answer, said)
}

#[cfg(test)]
#[path = "test/paging/test.rs"]
mod tests;
