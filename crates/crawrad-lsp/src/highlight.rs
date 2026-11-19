//! **Occurrences** of the name under the caret — `textDocument/documentHighlight`.
//!
//! With the caret on a function, a variable or a type, every place in the file
//! that names THE SAME symbol gets a background, as a modern editor and the classic RAD
//! IDEs show it. The server answers by symbol, not by text: a `total` in
//! another function is another variable and is not lit, and a write
//! (`total = …`) is told apart from a read.
//!
//! Same shape as [`crate::inlay`]: the request runs on a worker thread, waits
//! [`SETTLE`] for the caret to rest, keeps only the newest, and the answer is
//! tagged with the file, the document version and the caret it was asked for —
//! an answer for a caret that already moved is dropped.

use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Deserialize;

use crate::client::LspClient;
use crate::protocol::{Position, Range};
use crate::settler::{Settle, Settler};
use crawrad_note::note;

#[note("highlight-settle")]
pub const SETTLE: Duration = Duration::from_millis(120);

#[note("highlight-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(3);

#[note("highlight-documenthighlight")]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentHighlight {
    pub range: Range,
    #[serde(default)]
    pub kind: Option<u8>,
}

#[note("highlight-occurrence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occurrence {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub write: bool,
}

impl Occurrence {
    #[note("highlight-occurrence-parts")]
    pub fn parts(&self) -> (usize, usize, usize, bool) {
        (self.line, self.start, self.end, self.write)
    }
}

#[note("highlight-placed")]
pub fn placed(
    highlights: &[DocumentHighlight],
    text: &str,
    enc: crate::encoding::PositionEncoding,
) -> Vec<Occurrence> {
    let lines: Vec<&str> = text.lines().collect();
    highlights
        .iter()
        .filter(|h| h.range.start.line == h.range.end.line)
        .filter_map(|h| {
            let line = h.range.start.line as usize;
            let code = lines.get(line)?;
            let start = enc.byte(code, h.range.start.character);
            let end = enc.byte(code, h.range.end.character);
            (end > start).then_some(Occurrence {
                line,
                start,
                end,
                write: h.kind == Some(3),
            })
        })
        .collect()
}

#[note("highlight-byte-to-utf16")]
pub fn byte_to_utf16(line: &str, byte: usize) -> u32 {
    crate::encoding::PositionEncoding::Utf16.column(line, byte)
}

#[note("highlight-caret")]
pub type Caret = (usize, usize);

#[note("highlight-highlightanswer")]
#[derive(Debug)]
pub struct HighlightAnswer {
    pub file: PathBuf,
    pub version: i32,
    pub caret: Caret,
    pub highlights: Vec<DocumentHighlight>,
    /// The column unit the answer was asked in.
    pub encoding: crate::encoding::PositionEncoding,
}

struct Job {
    client: Arc<LspClient>,
    file: PathBuf,
    version: i32,
    caret: Caret,
    at: Position,
}

#[note("highlight-highlightworker")]
pub struct HighlightWorker {
    inner: Settler<Job, HighlightAnswer>,
    /// The last place asked about: the caret resting costs no request.
    asked: Mutex<Option<(PathBuf, i32, Caret)>>,
}

impl Default for HighlightWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightWorker {
    pub fn new() -> Self {
        let inner = Settler::new(
            "lsp-document-highlight",
            Settle::Quiet(SETTLE),
            |job: Job, jobs: &Receiver<Job>| {
                let mut newer = None;
                let result =
                    job.client
                        .request_document_highlight(&job.file, job.at, TIMEOUT, &mut || {
                            newer = jobs.try_iter().last();
                            newer.is_some()
                        });
                if let Some(n) = newer {
                    // A newer place replaced this one while the server worked: it
                    // was told (`$/cancelRequest`), and the newer one goes next.
                    return Err(Some(n));
                }
                let highlights = result.unwrap_or_default();
                Ok(HighlightAnswer {
                    file: job.file,
                    version: job.version,
                    caret: job.caret,
                    highlights,
                    encoding: job.client.position_encoding(),
                })
            },
        );
        Self {
            inner,
            asked: Mutex::new(None),
        }
    }

    #[note("highlight-highlightworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.inner.set_waker(wake);
    }

    #[note("highlight-highlightworker-request")]
    pub fn request(
        &self,
        client: Arc<LspClient>,
        file: &Path,
        version: i32,
        text: &str,
        caret: Caret,
    ) -> bool {
        let place = (file.to_path_buf(), version, caret);
        if let Ok(mut asked) = self.asked.lock() {
            if asked.as_ref() == Some(&place) {
                return false;
            }
            *asked = Some(place);
        }
        let line = text.lines().nth(caret.0).unwrap_or("");
        let at = Position {
            line: u32::try_from(caret.0).unwrap_or(u32::MAX),
            character: client.position_encoding().column(line, caret.1),
        };
        self.inner.send(Job {
            client,
            file: file.to_path_buf(),
            version,
            caret,
            at,
        })
    }

    #[note("highlight-highlightworker-poll")]
    pub fn poll(
        &self,
        file: &Path,
        version: i32,
        text: &str,
        caret: Caret,
    ) -> Option<Vec<Occurrence>> {
        let answer = self.inner.answers().try_iter().last()?;
        (answer.file == file && answer.version == version && answer.caret == caret)
            .then(|| placed(&answer.highlights, text, answer.encoding))
    }
}

#[note("highlight-editorassist")]
#[derive(Default)]
pub struct EditorAssist {
    pub hints: crate::inlay::InlayWorker,
    pub occurrences: HighlightWorker,
    pub completion: crate::complete::CompletionWorker,
    pub definition: crate::definition::DefinitionWorker,
    pub hover: crate::hover::HoverWorker,
    pub format: crate::format::FormatWorker,
    pub references: crate::references::ReferencesWorker,
    pub rename: crate::rename::RenameWorker,
    pub signature: crate::signature::SignatureWorker,
    pub code_actions: crate::code_action::CodeActionWorker,
    pub docs: crate::resolve::DocWorker,
    pub semantic: crate::semantic::SemanticWorker,
    pub symbols: crate::symbols::SymbolWorker,
    pub code_lenses: crate::code_lens::CodeLensWorker,
}

impl EditorAssist {
    #[note("highlight-editorassist-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + Sync + Clone + 'static) {
        self.hints.set_waker(wake.clone());
        self.occurrences.set_waker(wake.clone());
        self.completion.set_waker(wake.clone());
        self.definition.set_waker(wake.clone());
        self.hover.set_waker(wake.clone());
        self.format.set_waker(wake.clone());
        self.references.set_waker(wake.clone());
        self.rename.set_waker(wake.clone());
        self.signature.set_waker(wake.clone());
        self.code_actions.set_waker(wake.clone());
        self.docs.set_waker(wake.clone());
        self.semantic.set_waker(wake.clone());
        self.symbols.set_waker(wake.clone());
        self.code_lenses.set_waker(wake);
    }
}

#[cfg(test)]
#[path = "test/highlight/test.rs"]
mod tests;
