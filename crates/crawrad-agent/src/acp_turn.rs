//! One ACP prompt turn, folded from its stream of events: the reply's text,
//! the tool calls as records, and how it ended. Pure over a channel, so the
//! shell's worker thread keeps only the wiring.

use std::sync::Arc;
use std::sync::mpsc::Receiver;

use crate::client::AcpStreamEvent;
use crate::file_write::FileWriteRequest;
use crate::markdown::ToolCallRecord;
use crate::permission::PermissionRequest;
use crawrad_note::note;

#[note("acp-turn-turnend")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnEnd {
    /// The agent finished: its text and the tools it called.
    Reply(String, Vec<ToolCallRecord>),
    /// The user cancelled before the agent finished.
    Cancelled,
    /// The agent or the connection failed.
    Failed(String),
}

#[note("acp-turn-collect-turn")]
pub fn collect_turn(
    rx: &Receiver<AcpStreamEvent>,
    mut cancelled: impl FnMut() -> bool,
    mut on_status: impl FnMut(&AcpStreamEvent),
    mut ask: impl FnMut(Arc<PermissionRequest>),
    mut on_write: impl FnMut(Arc<FileWriteRequest>),
) -> TurnEnd {
    let mut reply = String::new();
    // Each call by its ACP id: an update may bring a new title (Claude's
    // `Terminal` becomes the command line), so the name cannot find it.
    let mut tools: Vec<(String, ToolCallRecord)> = Vec::new();
    let mut tool_since_text = false;
    while let Ok(event) = rx.recv() {
        if cancelled() {
            return TurnEnd::Cancelled;
        }
        on_status(&event);
        match event {
            AcpStreamEvent::PermissionRequest(req) => ask(req),
            AcpStreamEvent::WriteFile(req) => on_write(req),
            AcpStreamEvent::TextDelta(delta) => {
                if std::mem::take(&mut tool_since_text) {
                    reply.push_str(paragraph_break(&reply));
                }
                reply.push_str(&delta);
            }
            AcpStreamEvent::ToolCall { id, name, input } => {
                tool_since_text = true;
                let arguments = input.to_string();
                let status = crate::markdown::ToolCallStatus::Running;
                tools.push((
                    id,
                    ToolCallRecord {
                        name,
                        arguments,
                        status,
                    },
                ));
            }
            AcpStreamEvent::ToolCallUpdate {
                id,
                name,
                status,
                summary,
            } => {
                tool_since_text = true;
                match tools.iter_mut().find(|(i, _)| *i == id) {
                    Some((_, t)) => apply_update(t, name, status, summary),
                    None => tools.push((
                        id,
                        ToolCallRecord {
                            name,
                            arguments: summary,
                            status,
                        },
                    )),
                }
            }
            AcpStreamEvent::Done { .. } => break,
            AcpStreamEvent::Error(err) => return TurnEnd::Failed(err),
            _ => {}
        }
    }
    let tools = tools.into_iter().map(|(_, t)| t).collect();
    TurnEnd::Reply(reply, tools)
}

/// Text that resumes after a tool call starts a new paragraph, unless the
/// sentence was left open (it ends in a space): `request.` + `Now` must not
/// read `request.Now`.
fn paragraph_break(reply: &str) -> &'static str {
    let open = reply.is_empty() || reply.ends_with(char::is_whitespace);
    if open {
        ""
    } else {
        "

"
    }
}

/// An update keeps the card's first name (`Terminal`); a new title it brings
/// (the command line) becomes the card's detail when there is no summary.
fn apply_update(
    t: &mut ToolCallRecord,
    title: String,
    status: crate::markdown::ToolCallStatus,
    summary: String,
) {
    t.status = status;
    if !summary.is_empty() {
        t.arguments = summary;
    } else if title != t.name && (t.arguments.is_empty() || t.arguments == "{}") {
        t.arguments = title;
    }
}

#[cfg(test)]
#[path = "test/acp_turn/test.rs"]
mod tests;
