//! `strings.set`: the arguments and the answer; the shell only writes the file.

use serde_json::{Value, json};

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;

/// `key`, `lang` and `text` of a `strings.set` call, or `None` when one is missing.
pub fn params(args: &Value) -> Option<(&str, &str, &str)> {
    let get = |k: &str| args.get(k).and_then(Value::as_str);
    Some((get("key")?, get("lang")?, get("text")?))
}

/// The answer to `strings.set`: the shell's result, or the missing-arguments error.
pub fn answer(result: Option<Result<String, String>>) -> Answer {
    match result {
        Some(Ok(said)) => (
            ToolCallResult::success_json(json!({ "status": "ok", "message": said }), said),
            "applied",
        ),
        Some(Err(e)) => (ToolCallResult::error_text("set-failed", e), "error"),
        None => (
            ToolCallResult::error_text(
                "invalid-params",
                "strings.set requires 'key', 'lang', and 'text'",
            ),
            "error",
        ),
    }
}

#[cfg(test)]
#[path = "test/strings_tool/test.rs"]
mod tests;
