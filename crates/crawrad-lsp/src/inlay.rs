//! **Inlay hints** — the types and parameter names the code does not write
//! `let total: usize = …`, `read_dir(path: dir)`.
//!
//! The request runs on a worker thread, never on the window's: rust-analyzer
//! answers in tens to hundreds of milliseconds, more while it indexes. The
//! worker waits [`SETTLE`] after the last request (typing must not send one
//! request per key), keeps only the newest, and hands back the answer tagged
//! with the document version it was asked for — the caller drops an answer
//! whose version is no longer the editor's.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use serde::Deserialize;

use crate::client::LspClient;
use crate::protocol::Position;
use crate::settler::{Settle, Settler};
use crawrad_note::note;

#[note("inlay-settle")]
pub const SETTLE: Duration = Duration::from_millis(300);

#[note("inlay-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(5);

#[note("inlay-inlayhint")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHint {
    pub position: Position,
    pub label: InlayHintLabel,
    /// `1` a type, `2` a parameter name; absent for anything else.
    #[serde(default)]
    pub kind: Option<u8>,
    #[serde(default)]
    pub padding_left: Option<bool>,
    #[serde(default)]
    pub padding_right: Option<bool>,
}

#[note("inlay-inlayhintlabel")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum InlayHintLabel {
    Text(String),
    Parts(Vec<InlayHintLabelPart>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InlayHintLabelPart {
    pub value: String,
}

impl InlayHint {
    #[note("inlay-inlayhint-text")]
    pub fn text(&self) -> String {
        let label = match &self.label {
            InlayHintLabel::Text(t) => t.clone(),
            InlayHintLabel::Parts(parts) => parts.iter().map(|p| p.value.as_str()).collect(),
        };
        let left = if self.padding_left == Some(true) {
            " "
        } else {
            ""
        };
        let right = if self.padding_right == Some(true) {
            " "
        } else {
            ""
        };
        format!("{left}{label}{right}")
    }
}

#[note("inlay-placed")]
pub fn placed(
    hints: &[InlayHint],
    text: &str,
    enc: crate::encoding::PositionEncoding,
) -> Vec<(usize, usize, String)> {
    let lines: Vec<&str> = text.lines().collect();
    hints
        .iter()
        .filter_map(|h| {
            let line = h.position.line as usize;
            let code = lines.get(line)?;
            Some((line, enc.byte(code, h.position.character), h.text()))
        })
        .collect()
}

#[note("inlay-utf16-to-byte")]
pub fn utf16_to_byte(line: &str, column: u32) -> usize {
    crate::encoding::PositionEncoding::Utf16.byte(line, column)
}

#[note("inlay-inlayanswer")]
#[derive(Debug)]
pub struct InlayAnswer {
    pub file: PathBuf,
    pub version: i32,
    pub hints: Vec<InlayHint>,
    /// The column unit the answer was asked in.
    pub encoding: crate::encoding::PositionEncoding,
}

struct Job {
    client: Arc<LspClient>,
    file: PathBuf,
    version: i32,
    end: Position,
}

#[note("inlay-end-of")]
pub fn end_of(text: &str, enc: crate::encoding::PositionEncoding) -> Position {
    let lines: Vec<&str> = text.split('\n').collect();
    let last = lines.last().copied().unwrap_or("").trim_end_matches('\r');
    Position {
        line: u32::try_from(lines.len().saturating_sub(1)).unwrap_or(u32::MAX),
        character: enc.column(last, last.len()),
    }
}

#[note("inlay-inlayworker")]
pub struct InlayWorker {
    inner: Settler<Job, InlayAnswer>,
}

impl Default for InlayWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl InlayWorker {
    pub fn new() -> Self {
        let inner = Settler::new(
            "lsp-inlay-hints",
            Settle::Quiet(SETTLE),
            |job: Job, jobs: &Receiver<Job>| {
                let mut newer = None;
                let result =
                    job.client
                        .request_inlay_hints(&job.file, job.end, TIMEOUT, &mut || {
                            newer = jobs.try_iter().last();
                            newer.is_some()
                        });
                if let Some(n) = newer {
                    // A newer version replaced this one while the server worked: it
                    // was told (`$/cancelRequest`), and the newer one goes next.
                    return Err(Some(n));
                }
                let hints = result.unwrap_or_default();
                Ok(InlayAnswer {
                    file: job.file,
                    version: job.version,
                    hints,
                    encoding: job.client.position_encoding(),
                })
            },
        );
        Self { inner }
    }

    #[note("inlay-inlayworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.inner.set_waker(wake);
    }

    #[note("inlay-inlayworker-request")]
    pub fn request(&self, client: Arc<LspClient>, file: &Path, version: i32, text: &str) {
        let end = end_of(text, client.position_encoding());
        let _ = self.inner.send(Job {
            client,
            file: file.to_path_buf(),
            version,
            end,
        });
    }

    #[note("inlay-inlayworker-poll")]
    pub fn poll(
        &self,
        client: Arc<LspClient>,
        file: &Path,
        version: i32,
        text: &str,
    ) -> Option<Vec<(usize, usize, String)>> {
        if client.take_inlay_refresh() {
            self.request(client, file, version, text);
        }
        let answer = self.take()?;
        (answer.file == file && answer.version == version)
            .then(|| placed(&answer.hints, text, answer.encoding))
    }

    #[note("inlay-inlayworker-take")]
    pub fn take(&self) -> Option<InlayAnswer> {
        self.inner.answers().try_iter().last()
    }
}

#[cfg(test)]
#[path = "test/inlay.rs"]
mod tests;
