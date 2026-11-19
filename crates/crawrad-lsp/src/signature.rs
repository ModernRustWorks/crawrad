//! **Signature help** off the window's thread.
//!
//! When `(` or `,` is typed inside a call, the editor asks the server
//! (`textDocument/signatureHelp`, LSP 3.17) which signature the call is to
//! and which of its parameters the caret is on. The card above the caret
//! shows the signature with that parameter marked; every later edit asks
//! again (a *retrigger*), so a typed `,` moves the mark and a deleted one
//! moves it back. An empty answer — the caret left the call — closes it.
//!
//! # The parameter's place in the label is in UTF-16
//!
//! LSP 3.17 *ParameterInformation*: the label is either a substring of the
//! signature's label or `[start, end]` offsets into it, and *"the offsets are
//! based on a UTF-16 string representation"*. Measured on rust-analyzer
//! (2026-10-03): with `utf-8` negotiated as the position encoding,
//! `fn soma(ação: i32, …` still sends `[8, 17]` for the first parameter —
//! nine UTF-16 units, eleven bytes. So the offsets are converted from
//! UTF-16, whatever the encoding of positions.

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::client::LspClient;
use crate::protocol::Position;
use crawrad_note::note;

#[note("signature-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(3);

#[note("signature-trigger")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    /// Asked by the user, by key or by the MCP (`1`).
    Invoked,
    /// A character the server listed in `triggerCharacters` was typed (`2`).
    Character(char),
    /// The text or the caret changed while the card was up (`3`).
    ContentChange,
}

impl Trigger {
    #[note("signature-trigger-context")]
    pub fn context(self, retrigger: bool) -> Value {
        let (kind, character) = match self {
            Trigger::Invoked => (1, None),
            Trigger::Character(c) => (2, Some(c.to_string())),
            Trigger::ContentChange => (3, None),
        };
        let mut context = serde_json::json!({ "triggerKind": kind, "isRetrigger": retrigger });
        if let Some(c) = character {
            context["triggerCharacter"] = Value::String(c);
        }
        context
    }
}

#[note("signature-signature")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub label: String,
    pub params: Vec<Range<usize>>,
    pub active: Option<usize>,
    /// How many signatures the server offered (overloads); the shown one is
    /// its `activeSignature`.
    pub count: usize,
}

impl Signature {
    #[note("signature-signature-parts")]
    pub fn parts(&self) -> (&str, &str, &str) {
        let range = self
            .active
            .and_then(|i| self.params.get(i))
            .filter(|r| self.label.get((*r).clone()).is_some());
        match range {
            Some(r) => (
                &self.label[..r.start],
                &self.label[r.clone()],
                &self.label[r.end..],
            ),
            None => (self.label.as_str(), "", ""),
        }
    }
}

#[note("signature-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Shown(Signature),
    /// No call around the caret: the card closes.
    Nothing,
    /// The server did not answer in time, or the request failed.
    NoAnswer(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
}

#[note("signature-signatureworker")]
pub struct SignatureWorker(crate::asker::Asker<SignatureAnswer>);

impl Default for SignatureWorker {
    fn default() -> Self {
        Self::new()
    }
}

#[note("signature-asked")]
pub struct Asked<'a> {
    pub file: &'a Path,
    pub version: i32,
    pub text: &'a str,
    /// Line and byte of the caret.
    pub caret: (usize, usize),
    pub trigger: Trigger,
    pub retrigger: bool,
}

impl SignatureWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-signature"))
    }

    #[note("signature-signatureworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("signature-signatureworker-request")]
    pub fn request(&self, client: Arc<LspClient>, ticket: u64, asked: &Asked<'_>) -> bool {
        let line = asked.text.lines().nth(asked.caret.0).unwrap_or("");
        let at = Position {
            line: u32::try_from(asked.caret.0).unwrap_or(u32::MAX),
            character: client.position_encoding().column(line, asked.caret.1),
        };
        let file = asked.file.to_path_buf();
        let version = asked.version;
        let context = asked.trigger.context(asked.retrigger);
        self.0.ask(ticket, move || {
            let params = serde_json::json!({
                "textDocument": { "uri": crate::path_to_uri(&file) },
                "position": { "line": at.line, "character": at.character },
                "context": context,
            });
            let mut request =
                |left| client.send_request("textDocument/signatureHelp", params.clone(), left);
            let retry = crate::definition::RETRY;
            let outcome = match crate::definition::retrying(&mut request, TIMEOUT, retry) {
                Ok(v) => parse(&v).map_or(Outcome::Nothing, Outcome::Shown),
                Err(e) => Outcome::NoAnswer(e.to_string()),
            };
            SignatureAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("signature-signatureworker-take")]
    pub fn take(&self, ticket: u64) -> Option<SignatureAnswer> {
        self.0.take(ticket)
    }
}

#[note("signature-parse")]
pub fn parse(help: &Value) -> Option<Signature> {
    let signatures = help.get("signatures")?.as_array()?;
    let index = help
        .get("activeSignature")
        .and_then(Value::as_u64)
        .and_then(|i| usize::try_from(i).ok())
        .filter(|i| *i < signatures.len())
        .unwrap_or(0);
    let chosen = signatures.get(index)?;
    let label = chosen.get("label")?.as_str()?.to_string();
    let mut params = Vec::new();
    let mut from = 0;
    for p in chosen
        .get("parameters")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let range = match p.get("label") {
            Some(Value::String(s)) => label[from..].find(s.as_str()).map(|i| {
                let start = from + i;
                start..start + s.len()
            }),
            Some(Value::Array(pair)) => match (
                pair.first().and_then(Value::as_u64),
                pair.get(1).and_then(Value::as_u64),
            ) {
                (Some(a), Some(b)) => Some(utf16_to_byte(&label, a)..utf16_to_byte(&label, b)),
                _ => None,
            },
            _ => None,
        };
        let range = range.unwrap_or(from..from);
        from = range.end;
        params.push(range);
    }
    let active = chosen
        .get("activeParameter")
        .or_else(|| help.get("activeParameter"))
        .and_then(Value::as_u64)
        .and_then(|i| usize::try_from(i).ok())
        .filter(|i| *i < params.len());
    Some(Signature {
        label,
        params,
        active,
        count: signatures.len(),
    })
}

#[note("signature-utf16-to-byte")]
fn utf16_to_byte(s: &str, units: u64) -> usize {
    let mut seen = 0u64;
    for (byte, c) in s.char_indices() {
        if seen >= units {
            return byte;
        }
        seen += c.len_utf16() as u64;
    }
    s.len()
}

#[note("signature-triggers-from")]
pub fn triggers_from(capabilities: Option<&Value>) -> Vec<String> {
    ["triggerCharacters", "retriggerCharacters"]
        .iter()
        .filter_map(|k| {
            capabilities?
                .pointer(&format!("/signatureHelpProvider/{k}"))?
                .as_array()
        })
        .flatten()
        .filter_map(|t| t.as_str().map(str::to_string))
        .collect()
}

#[cfg(test)]
#[path = "test/signature/test.rs"]
mod tests;
