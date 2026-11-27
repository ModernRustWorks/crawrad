//! The `review.*` tools' answers: what the agent reads when it asks whether a
//! diff review is waiting, and what it reads after accepting or rejecting one.
//!
//! Pure of the window, like the rest here: the shell finds the review and applies
//! the decision; these functions only say the result. The same cut `run_tool`
//! makes with cargo.

use serde_json::{Value, json};

use crate::protocol::ToolCallResult;
use crawrad_note::note;

/// No review is waiting — what `review.status` answers when the bar is clear.
/// Not an error: asking and hearing "nothing" is ordinary use.
#[note("review-tool-none-pending")]
pub fn none_pending() -> ToolCallResult {
    ToolCallResult::success_json(json!({ "pending": false }), "No diff review is waiting.")
}

/// The waiting review, counted: who proposed it, on which file, and how big.
#[note("review-tool-pending")]
pub fn pending(
    path: &str,
    author_tool: &str,
    additions: usize,
    deletions: usize,
    changes: usize,
) -> ToolCallResult {
    let answer = json!({
        "pending": true,
        "path": path,
        "proposed_by": author_tool,
        "additions": additions,
        "deletions": deletions,
        "changes": changes,
    });
    let said = format!(
        "A diff review proposed by '{author_tool}' is waiting on '{path}'. \
         Call review.accept or review.reject."
    );
    ToolCallResult::success_json(answer, said)
}

/// Asked to decide, and there is nothing to decide. `path` is absent when no
/// file was under review at all; present when that file has none.
#[note("review-tool-no-review")]
pub fn no_review(path: Option<&str>) -> ToolCallResult {
    let said = match path {
        Some(p) => format!("No diff review is waiting on '{p}'."),
        None => "No diff review is waiting. Nothing to decide.".to_string(),
    };
    ToolCallResult::error_text("no-review", said)
}

/// The decision was applied. `accept` picks the word the agent reads back, and
/// that same word goes in the JSON — for whoever reads the answer by program.
#[note("review-tool-decided")]
pub fn decided(path: &str, accept: bool) -> ToolCallResult {
    let decision = if accept { "accepted" } else { "rejected" };
    ToolCallResult::success_json(
        json!({ "path": path, "decision": decision }),
        format!("Diff review {decision} on '{path}'."),
    )
}

/// The decision failed to reach the buffer.
#[note("review-tool-failed")]
pub fn failed(reason: impl std::fmt::Display) -> ToolCallResult {
    ToolCallResult::error_text("diff-review-error", reason.to_string())
}

/// The tool name the audit line is written under, by the decision taken.
#[note("review-tool-tool-name")]
pub fn tool_name(accept: bool) -> &'static str {
    if accept {
        "review.accept"
    } else {
        "review.reject"
    }
}

/// The path the decision falls on when the agent names one. The shell supplies
/// the fallback; this keeps the reading of the argument in one place.
#[note("review-tool-path-asked")]
pub fn path_asked(args: &Value) -> Option<std::path::PathBuf> {
    args.get("path")
        .and_then(Value::as_str)
        .map(std::path::PathBuf::from)
}
