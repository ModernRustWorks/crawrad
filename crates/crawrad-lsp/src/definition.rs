//! **Go to Definition** off the window's thread.
//!
//! The editor asks with a `ticket`; the worker asks the server
//! (`textDocument/definition`, LSP 3.17) and answers with the places already
//! turned into the editor's unit: a file and a line and **byte** — the
//! server's column is in the negotiated encoding (`crate::encoding`), and
//! the target file is read to convert it. An answer whose ticket is not the
//! newest is dropped: the user moved on.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::client::LspClient;
use crate::protocol::{Location, Position};
use crawrad_note::note;

#[note("definition-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(5);

#[note("definition-target")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub file: PathBuf,
    pub line: usize,
    pub byte: usize,
}

#[note("definition-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// One or more places, in the server's order.
    Found(Vec<Target>),
    /// The server answered: nothing is defined there.
    Nothing,
    /// The server did not answer in time, or the request failed.
    NoAnswer(String),
}

#[note("definition-definitionanswer")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
}

#[note("definition-definitionworker")]
pub struct DefinitionWorker(crate::asker::Asker<DefinitionAnswer>);

impl Default for DefinitionWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl DefinitionWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-definition"))
    }

    #[note("definition-definitionworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("definition-definitionworker-request")]
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
            let enc = client.position_encoding();
            let mut request = |left| client.request_definition(&file, at.line, at.character, left);
            let mut read = |f: &Path| std::fs::read_to_string(f).ok();
            let outcome = ask(&mut request, enc, &mut read, TIMEOUT, RETRY);
            DefinitionAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("definition-definitionworker-take")]
    pub fn take(&self, ticket: u64) -> Option<DefinitionAnswer> {
        self.0.take(ticket)
    }
}

#[note("definition-targets")]
pub fn targets(
    places: Vec<Location>,
    enc: crate::encoding::PositionEncoding,
    read: &mut dyn FnMut(&Path) -> Option<String>,
) -> Vec<Target> {
    places
        .into_iter()
        .filter_map(|p| {
            let file = crate::uri_to_path(&p.uri)?;
            let line = p.range.start.line as usize;
            let byte = read(&file)
                .and_then(|text| text.lines().nth(line).map(str::to_owned))
                .map_or(0, |l| enc.byte(&l, p.range.start.character));
            Some(Target { file, line, byte })
        })
        .collect()
}

#[note("definition-content-modified")]
pub const CONTENT_MODIFIED: i64 = -32801;

#[note("definition-retry")]
pub const RETRY: Duration = Duration::from_millis(300);

#[note("definition-retrying")]
pub fn retrying<T>(
    request: &mut dyn FnMut(Duration) -> Result<T, crate::LspError>,
    timeout: Duration,
    retry: Duration,
) -> Result<T, crate::LspError> {
    let started = std::time::Instant::now();
    loop {
        match request(timeout.saturating_sub(started.elapsed())) {
            Err(crate::LspError::RpcError {
                code: CONTENT_MODIFIED,
                ..
            }) if started.elapsed() + retry < timeout => std::thread::sleep(retry),
            other => return other,
        }
    }
}

#[note("definition-ask")]
pub fn ask(
    request: &mut dyn FnMut(Duration) -> Result<Vec<Location>, crate::LspError>,
    enc: crate::encoding::PositionEncoding,
    read: &mut dyn FnMut(&Path) -> Option<String>,
    timeout: Duration,
    retry: Duration,
) -> Outcome {
    match retrying(request, timeout, retry) {
        Ok(places) => match targets(places, enc, read) {
            t if t.is_empty() => Outcome::Nothing,
            t => Outcome::Found(t),
        },
        Err(e) => Outcome::NoAnswer(e.to_string()),
    }
}

#[cfg(test)]
#[path = "test/definition/test.rs"]
mod tests;
