//! Keeps an agent's file edits under the IDE's review, whatever the agent's own mode.
#![forbid(unsafe_code)]

use crawrad_note::note;

/// The tools a Claude agent edits files with.
const EDIT_TOOLS: &str = "Edit|Write|MultiEdit|NotebookEdit";

/// The hook answer that makes the agent ask before an edit, even in `auto`.
const ASK_HOOK: &str = "node -e \"console.log(JSON.stringify({hookSpecificOutput:{hookEventName:'PreToolUse',permissionDecision:'ask',permissionDecisionReason:'CrawRAD reviews every edit'}}))\"";

#[note("edit-review-ask-before-edits-meta")]
pub fn ask_before_edits_meta() -> serde_json::Value {
    serde_json::json!({ "claudeCode": { "options": { "settings": { "hooks": { "PreToolUse": [
        { "matcher": EDIT_TOOLS, "hooks": [ { "type": "command", "command": ASK_HOOK } ] }
    ] } } } } })
}

#[note("edit-review-kind-is-read")]
pub fn kind_is_read(kind: Option<&str>) -> bool {
    matches!(kind, None | Some("read" | "search"))
}

#[cfg(test)]
#[path = "test/edit_review/test.rs"]
mod tests;
