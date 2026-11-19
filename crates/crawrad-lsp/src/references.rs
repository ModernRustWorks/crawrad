//! **Find All References** off the window's thread.
//!
//! The editor asks with a `ticket`; the worker asks the server
//! (`textDocument/references`, LSP 3.17, `context.includeDeclaration: true`)
//! and answers with the server's places as they came. The worker does NOT
//! turn them into bytes: [`spans`] does, on the window's side, against the
//! text the editor holds (an open, unsaved buffer first), which only the
//! window has.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::client::LspClient;
use crate::definition::{RETRY, retrying};
use crate::protocol::{Location, Position};
use crawrad_note::note;

#[note("references-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(10);

#[note("references-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The places, in the server's order, the declaration among them.
    Found(Vec<Location>),
    /// The server answered: no references there.
    Nothing,
    /// The server did not answer in time, or the request failed.
    NoAnswer(String),
}

#[note("references-referencesanswer")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencesAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
}

#[note("references-referencesworker")]
pub struct ReferencesWorker(crate::asker::Asker<ReferencesAnswer>);

impl Default for ReferencesWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferencesWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-references"))
    }

    #[note("references-referencesworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("references-referencesworker-request")]
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
            let mut request = |left| client.request_references(&file, at, true, left);
            let outcome = ask(&mut request, TIMEOUT, RETRY);
            ReferencesAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("references-referencesworker-take")]
    pub fn take(&self, ticket: u64) -> Option<ReferencesAnswer> {
        self.0.take(ticket)
    }
}

#[note("references-ask")]
pub fn ask(
    request: &mut dyn FnMut(Duration) -> Result<Vec<Location>, crate::LspError>,
    timeout: Duration,
    retry: Duration,
) -> Outcome {
    match retrying(request, timeout, retry) {
        Ok(places) if places.is_empty() => Outcome::Nothing,
        Ok(places) => Outcome::Found(places),
        Err(e) => Outcome::NoAnswer(e.to_string()),
    }
}

#[note("references-span")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub file: PathBuf,
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[note("references-spans")]
pub fn spans(
    places: Vec<Location>,
    enc: crate::encoding::PositionEncoding,
    read: &mut dyn FnMut(&Path) -> Option<String>,
) -> Vec<Span> {
    let mut texts: std::collections::HashMap<PathBuf, Option<String>> = Default::default();
    places
        .into_iter()
        .filter_map(|p| {
            let file = crate::uri_to_path(&p.uri)?;
            let text = texts.entry(file.clone()).or_insert_with(|| read(&file));
            let line = p.range.start.line as usize;
            let text = text
                .as_deref()
                .and_then(|t| t.lines().nth(line))
                .unwrap_or("")
                .to_string();
            let start = enc.byte(&text, p.range.start.character);
            let end = match p.range.end.line == p.range.start.line {
                true => enc.byte(&text, p.range.end.character).max(start),
                false => text.len(),
            };
            Some(Span {
                file,
                line,
                start,
                end,
                text,
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "test/references/test.rs"]
mod tests;
