//! What the bridge does with one line from the agent.
//!
//! The bridge and the IDE talk one line out, at most one line back, with no
//! request ids to pair them: if the two sides disagree on whether a line gets
//! an answer, every answer after it goes to the wrong request. They disagreed
//! twice (measured 01/10/2026):
//!
//! - a line that is not a request — the IDE answers `-32700`, the bridge did
//!   not read it, and the next request got that error as its answer;
//! - `"id": null` — the IDE reads it as a notification and answers nothing,
//!   the bridge saw an `id` key and waited forever.
//!
//! So the bridge reads the line with the IDE's own type,
//! [`JsonRpcRequest`], and decides from that.

use serde_json::Value;

use crate::protocol::JsonRpcRequest;
use crawrad_note::note;

#[note("route-route")]
#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    /// Send it to the IDE; read one answer back only when it is a request.
    Forward {
        expects_answer: bool,
        /// The parsed line, for the toolset checks.
        request: Value,
    },
    /// Not a request the IDE can read: answer it here and send nothing on.
    AnswerHere(Value),
}

#[note("route-route-2")]
pub fn route(line: &str) -> Route {
    match serde_json::from_str::<JsonRpcRequest>(line) {
        Ok(req) => Route::Forward {
            expects_answer: req.id.is_some(),
            request: serde_json::from_str(line).unwrap_or(Value::Null),
        },
        Err(e) => Route::AnswerHere(serde_json::json!({
            "jsonrpc": "2.0",
            "id": Value::Null,
            "error": { "code": -32700, "message": format!("Parse error: {e}") }
        })),
    }
}

#[cfg(test)]
#[path = "test/route/test.rs"]
mod tests;
