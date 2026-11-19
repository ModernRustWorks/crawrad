//! **Code actions and quick fixes** off the window's thread.
//!
//! When the caret rests, the editor asks the server
//! (`textDocument/codeAction`, LSP 3.17) what it can do there; when the
//! answer has actions with edits, a light bulb shows on the line and
//! `Ctrl+.` lists them. Choosing one applies its `WorkspaceEdit` through
//! the same path as Rename Symbol: every file or none, the generated form
//! source refused (the RAD guard), one undo step per file.
//!
//! # Edits inline, no resolve
//!
//! The client does not announce `codeAction.resolveSupport`, so the server
//! computes each action's `edit` in the answer. Measured on rust-analyzer
//! (2026-10-04): for `HashMap::new()` without its `use`, without
//! `resolveSupport` the answer carries `Import std::collections::HashMap`
//! with its `documentChanges`; with it, only `data` and a later
//! `codeAction/resolve`. A `Command` (no edit) is left out: the IDE does not
//! run server commands.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::client::LspClient;
use crate::edit::WorkspaceEdit;
use crate::protocol::Position;
use crawrad_note::note;

#[note("code-action-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(3);

#[note("code-action-codeaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAction {
    pub title: String,
    pub kind: Option<String>,
    pub preferred: bool,
    pub edit: WorkspaceEdit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeActionAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    /// Line and byte of the caret it was asked at.
    pub caret: (usize, usize),
    /// `Err` when the server did not answer.
    pub actions: Result<Vec<CodeAction>, String>,
}

#[note("code-action-place")]
pub type Place = (PathBuf, i32, (usize, usize));

#[note("code-action-offer")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Offer {
    tickets: u64,
    /// The newest request: its ticket, its place, and whether asked by key.
    pending: Option<(u64, Place, bool)>,
    /// The answer for a place, and its actions.
    pub here: Option<(Place, Vec<CodeAction>)>,
}

impl Offer {
    #[note("code-action-offer-knows")]
    pub fn knows(&self, place: &Place) -> bool {
        self.here.as_ref().is_some_and(|(p, _)| p == place)
            || self.pending.as_ref().is_some_and(|(_, p, _)| p == place)
    }

    #[note("code-action-offer-ask")]
    pub fn ask(&mut self, place: Place, by_key: bool) -> u64 {
        self.tickets += 1;
        self.pending = Some((self.tickets, place, by_key));
        self.tickets
    }

    pub fn pending_ticket(&self) -> Option<u64> {
        self.pending.as_ref().map(|p| p.0)
    }

    #[note("code-action-offer-answered")]
    pub fn answered(&mut self, answer: CodeActionAnswer) -> Option<bool> {
        let (ticket, _, by_key) = self.pending.clone()?;
        if answer.ticket != ticket {
            return None;
        }
        self.pending = None;
        let actions = answer.actions.unwrap_or_default();
        self.here = Some(((answer.file, answer.version, answer.caret), actions));
        Some(by_key)
    }

    #[note("code-action-offer-to-json")]
    pub fn to_json(&self, place: Option<Place>, bulb: bool) -> Value {
        let actions = place.as_ref().and_then(|p| self.at(p)).map(|a| {
            a.iter()
                .map(|x| serde_json::json!({ "title": x.title, "kind": x.kind, "preferred": x.preferred }))
                .collect::<Vec<_>>()
        });
        serde_json::json!({
            "line": place.map(|p| p.2 .0 + 1),
            "waiting": actions.is_none() && self.pending.is_some(),
            "actions": actions.unwrap_or_default(),
            "bulb": bulb,
        })
    }

    #[note("code-action-offer-at")]
    pub fn at(&self, place: &Place) -> Option<&[CodeAction]> {
        match &self.here {
            Some((p, a)) if p == place => Some(a),
            _ => None,
        }
    }
}

#[note("code-action-codeactionworker")]
pub struct CodeActionWorker(crate::asker::Asker<CodeActionAnswer>);

impl Default for CodeActionWorker {
    fn default() -> Self {
        Self::new()
    }
}

#[note("code-action-asked")]
pub struct Asked<'a> {
    pub file: &'a Path,
    pub version: i32,
    pub text: &'a str,
    /// Line and byte of the caret.
    pub caret: (usize, usize),
}

impl CodeActionWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-code-action"))
    }

    #[note("code-action-codeactionworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("code-action-codeactionworker-request")]
    pub fn request(&self, client: Arc<LspClient>, ticket: u64, asked: &Asked<'_>) -> bool {
        let line = asked.text.lines().nth(asked.caret.0).unwrap_or("");
        let at = Position {
            line: u32::try_from(asked.caret.0).unwrap_or(u32::MAX),
            character: client.position_encoding().column(line, asked.caret.1),
        };
        let (file, version, caret) = (asked.file.to_path_buf(), asked.version, asked.caret);
        self.0.ask(ticket, move || {
            let diagnostics = client.diagnostics_on_line(&file, at.line);
            let point = serde_json::json!({ "line": at.line, "character": at.character });
            let params = serde_json::json!({
                "textDocument": { "uri": crate::path_to_uri(&file) },
                "range": { "start": point, "end": point },
                "context": { "diagnostics": diagnostics, "triggerKind": 1 },
            });
            let mut request =
                |left| client.send_request("textDocument/codeAction", params.clone(), left);
            let retry = crate::definition::RETRY;
            let actions = crate::definition::retrying(&mut request, TIMEOUT, retry)
                .map(|v| parse(&v))
                .map_err(|e| e.to_string());
            CodeActionAnswer {
                ticket,
                file,
                version,
                caret,
                actions,
            }
        })
    }

    #[note("code-action-codeactionworker-take")]
    pub fn take(&self, ticket: u64) -> Option<CodeActionAnswer> {
        self.0.take(ticket)
    }
}

#[note("code-action-parse")]
pub fn parse(answer: &Value) -> Vec<CodeAction> {
    answer
        .as_array()
        .into_iter()
        .flatten()
        .filter(|a| a.get("disabled").is_none())
        .filter_map(|a| {
            let title = a.get("title")?.as_str()?.to_string();
            let edit = crate::edit::workspace_edit_from(a.get("edit")?.clone()).ok()?;
            (!edit.is_empty()).then(|| CodeAction {
                title,
                kind: a.get("kind").and_then(Value::as_str).map(str::to_string),
                preferred: a.get("isPreferred").and_then(Value::as_bool) == Some(true),
                edit,
            })
        })
        .collect()
}

#[note("code-action-on-line")]
pub fn on_line(published: &[Value], line: u32) -> Vec<Value> {
    let at = |d: &Value, end: &str| {
        d.pointer(&format!("/range/{end}/line"))
            .and_then(Value::as_u64)
    };
    published
        .iter()
        .filter(|d| {
            let (start, end) = (at(d, "start"), at(d, "end"));
            start.is_some_and(|s| s <= u64::from(line)) && end.is_some_and(|e| e >= u64::from(line))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "test/code_action/test.rs"]
mod tests;
