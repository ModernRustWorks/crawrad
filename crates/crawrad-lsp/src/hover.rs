//! **Hover** off the window's thread.
//!
//! The editor asks with a `ticket` when the mouse rests on a name (or
//! `editor.show_hover` at the caret); the worker asks the server
//! (`textDocument/hover`, LSP 3.17) and answers with the text to show. The
//! three shapes the spec allows for `contents` are read — `MarkupContent`,
//! `MarkedString` and `MarkedString[]` — and turned into plain text the card
//! can show and the user can select. An answer whose ticket is not the newest
//! is dropped: the mouse moved on.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::client::LspClient;
use crate::protocol::Position;
use crawrad_note::note;

#[note("hover-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(3);

#[note("hover-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The text to show, already plain.
    Text(String),
    /// Nothing to show there (a blank, a comment): no card at all.
    Nothing,
    /// The server did not answer in time, or the request failed.
    NoAnswer(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
    /// The server's markdown, as it came (empty when there is no text).
    pub markdown: String,
    /// Where the name is defined, asked only when there is text: a card
    /// over a noted item shows the note.
    pub defined_at: Option<crate::definition::Target>,
}

#[note("hover-hoverworker")]
pub struct HoverWorker(crate::asker::Asker<HoverAnswer>);

impl Default for HoverWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl HoverWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-hover"))
    }

    #[note("hover-hoverworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("hover-hoverworker-request")]
    pub fn request(
        &self,
        client: Arc<LspClient>,
        ticket: u64,
        file: &Path,
        version: i32,
        text: &str,
        caret: (usize, usize),
    ) -> bool {
        let line = text.lines().nth(caret.0).unwrap_or("");
        let at = Position {
            line: u32::try_from(caret.0).unwrap_or(u32::MAX),
            character: client.position_encoding().column(line, caret.1),
        };
        let file = file.to_path_buf();
        self.0.ask(ticket, move || {
            let params = serde_json::json!({
                "textDocument": { "uri": crate::path_to_uri(&file) },
                "position": { "line": at.line, "character": at.character },
            });
            let mut request =
                |left| client.send_request("textDocument/hover", params.clone(), left);
            let retry = crate::definition::RETRY;
            let (outcome, markdown) =
                match crate::definition::retrying(&mut request, TIMEOUT, retry) {
                    Ok(v) => match contents_markdown(&v) {
                        Some(md) if !plain(&md).is_empty() => (Outcome::Text(plain(&md)), md),
                        _ => (Outcome::Nothing, String::new()),
                    },
                    Err(e) => (Outcome::NoAnswer(e.to_string()), String::new()),
                };
            let under = client.diagnostics_of(&file).unwrap_or_default();
            let outcome = with_diagnostics(outcome, &diagnostics_at(&under, at));
            let defined_at = match outcome {
                Outcome::Text(_) => {
                    let mut ask =
                        |left| client.request_definition(&file, at.line, at.character, left);
                    let mut read = |f: &Path| std::fs::read_to_string(f).ok();
                    let enc = client.position_encoding();
                    match crate::definition::ask(&mut ask, enc, &mut read, TIMEOUT, retry) {
                        crate::definition::Outcome::Found(t) => t.into_iter().next(),
                        _ => None,
                    }
                }
                _ => None,
            };
            HoverAnswer {
                ticket,
                file,
                version,
                outcome,
                markdown,
                defined_at,
            }
        })
    }

    #[note("hover-hoverworker-take")]
    pub fn take(&self, ticket: u64) -> Option<HoverAnswer> {
        self.0.take(ticket)
    }
}

#[note("hover-diagnostics-at")]
pub fn diagnostics_at(diagnostics: &[crate::protocol::Diagnostic], at: Position) -> String {
    use crate::protocol::DiagnosticSeverity as S;
    let held = diagnostics
        .iter()
        .filter(|d| d.range.start <= at && at <= d.range.end);
    let written = held.filter_map(|d| {
        let severity = match d.severity {
            Some(S::Error) => "error",
            Some(S::Warning) | None => "warning",
            Some(S::Information) => "info",
            Some(S::Hint) => return None,
        };
        let lines = d.message.lines().map(str::trim);
        let message: Vec<&str> = lines.filter(|l| !l.ends_with("on by default")).collect();
        let from = [d.source.as_deref(), d.code.as_deref()];
        let from: Vec<&str> = from.into_iter().flatten().collect();
        Some(format!(
            "{severity}: {}\n{}",
            message.join("\n"),
            from.join(" \u{b7} ")
        ))
    });
    written.collect::<Vec<_>>().join("\n\n").trim().to_string()
}

#[note("hover-with-diagnostics")]
fn with_diagnostics(outcome: Outcome, diagnostics: &str) -> Outcome {
    match outcome {
        _ if diagnostics.is_empty() => outcome,
        Outcome::Text(text) => Outcome::Text(format!("{diagnostics}\n\n{text}")),
        _ => Outcome::Text(diagnostics.to_string()),
    }
}

#[note("hover-contents-markdown")]
pub fn contents_markdown(hover: &Value) -> Option<String> {
    fn one(v: &Value) -> Option<String> {
        match v {
            Value::String(s) => Some(s.clone()),
            Value::Object(o) => match (o.get("language"), o.get("value")) {
                (Some(Value::String(lang)), Some(Value::String(code))) => {
                    Some(format!("```{lang}\n{code}\n```"))
                }
                (_, Some(Value::String(value))) => Some(value.clone()),
                _ => None,
            },
            _ => None,
        }
    }
    let text = match hover.get("contents")? {
        Value::Array(parts) => parts
            .iter()
            .filter_map(one)
            .collect::<Vec<_>>()
            .join("\n\n"),
        other => one(other)?,
    };
    (!text.trim().is_empty()).then_some(text)
}

#[note("hover-plain")]
pub fn plain(markdown: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut in_code = false;
    for line in markdown.lines() {
        let t = line.trim_end();
        if t.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        let shown = if in_code {
            t.to_string()
        } else if t.trim() == "---" {
            "\u{2500}".repeat(24)
        } else {
            inline_plain(t.trim_start_matches('#').trim_start())
        };
        if shown.is_empty() && out.last().is_none_or(String::is_empty) {
            continue;
        }
        out.push(shown);
    }
    while out.last().is_some_and(String::is_empty) {
        out.pop();
    }
    out.join("\n")
}

#[note("hover-inline-plain")]
fn inline_plain(line: &str) -> String {
    let mut s = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let link = after.find("](").and_then(|close| {
            after[close + 2..]
                .find(')')
                .map(|end| (close, close + 2 + end))
        });
        match link {
            Some((close, end)) => {
                s.push_str(&rest[..open]);
                s.push_str(&after[..close]);
                rest = &after[end + 1..];
            }
            None => {
                s.push_str(&rest[..=open]);
                rest = after;
            }
        }
    }
    s.push_str(rest);
    s.replace("**", "").replace("__", "").replace('`', "")
}

#[cfg(test)]
#[path = "test/hover/test.rs"]
mod tests;
