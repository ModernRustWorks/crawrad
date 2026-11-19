//! **Rename Symbol** off the window's thread.
//!
//! Two requests, both on the one engine ([`crate::asker`]):
//! `textDocument/prepareRename` (LSP 3.17: is there something to rename at
//! the caret, and where is it — the client announces `prepareSupport`) and
//! `textDocument/rename` (the `WorkspaceEdit`, [`crate::edit`]). Nothing is
//! applied here: the window checks the edits against the form (the
//! guard), shows them, and only then applies them.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::client::LspClient;
use crate::definition::{RETRY, retrying};
use crate::edit::WorkspaceEdit;
use crate::protocol::Position;
use crawrad_note::note;

#[note("rename-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(10);

#[note("rename-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// `prepareRename`: the name at the caret can be renamed; its bytes on
    /// the caret's line, and the files where it is DECLARED (its
    /// `textDocument/definition`). The server's edits do not always reach the
    /// declaration (measured: renaming the form's struct from the user's unit
    /// edited only the user's unit), so the window judges the declaration
    /// itself.
    Renamable {
        start: usize,
        end: usize,
        declared: Vec<PathBuf>,
    },
    /// `prepareRename`: nothing to rename there (a keyword, a literal…).
    NotRenamable,
    /// `rename`: the edits, by file URI.
    Edits(WorkspaceEdit),
    /// The server did not answer in time, refused, or the request failed —
    /// with its words (rust-analyzer says why a name is invalid).
    NoAnswer(String),
}

#[note("rename-renameanswer")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
}

#[note("rename-asked")]
#[derive(Debug, Clone, Copy)]
pub struct Asked<'a> {
    pub file: &'a Path,
    pub version: i32,
    pub text: &'a str,
    pub caret: (usize, usize),
}

impl Asked<'_> {
    fn line(&self) -> &str {
        self.text.lines().nth(self.caret.0).unwrap_or("")
    }
}

#[note("rename-renameworker")]
pub struct RenameWorker(crate::asker::Asker<RenameAnswer>);

impl Default for RenameWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl RenameWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-rename"))
    }

    #[note("rename-renameworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("rename-renameworker-prepare")]
    pub fn prepare(&self, client: Arc<LspClient>, ticket: u64, asked: Asked) -> bool {
        let line = asked.line().to_string();
        let at = position(&client, &line, asked.caret);
        let (file, version) = (asked.file.to_path_buf(), asked.version);
        self.0.ask(ticket, move || {
            let enc = client.position_encoding();
            let mut request = |left| client.request_prepare_rename(&file, at, left);
            let outcome = match retrying(&mut request, TIMEOUT, RETRY) {
                Ok(Some(r)) if r.start.line == at.line => Outcome::Renamable {
                    start: enc.byte(&line, r.start.character),
                    end: enc.byte(&line, r.end.character),
                    declared: declared(&client, &file, at),
                },
                Ok(_) => Outcome::NotRenamable,
                Err(e) => Outcome::NoAnswer(e.to_string()),
            };
            RenameAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("rename-renameworker-rename")]
    pub fn rename(
        &self,
        client: Arc<LspClient>,
        ticket: u64,
        asked: Asked,
        new_name: &str,
    ) -> bool {
        let at = position(&client, asked.line(), asked.caret);
        let (file, version) = (asked.file.to_path_buf(), asked.version);
        let new_name = new_name.to_string();
        self.0.ask(ticket, move || {
            let mut request = |left| client.request_rename(&file, at, &new_name, left);
            let outcome = match retrying(&mut request, TIMEOUT, RETRY) {
                Ok(edits) => Outcome::Edits(edits),
                Err(e) => Outcome::NoAnswer(e.to_string()),
            };
            RenameAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("rename-renameworker-take")]
    pub fn take(&self, ticket: u64) -> Option<RenameAnswer> {
        self.0.take(ticket)
    }
}

#[note("rename-declared")]
fn declared(client: &LspClient, file: &Path, at: Position) -> Vec<PathBuf> {
    let mut request = |left| client.request_definition(file, at.line, at.character, left);
    let places = retrying(&mut request, TIMEOUT, RETRY).unwrap_or_default();
    places
        .iter()
        .filter_map(|p| crate::uri_to_path(&p.uri))
        .collect()
}

fn position(client: &LspClient, line: &str, caret: (usize, usize)) -> Position {
    Position {
        line: u32::try_from(caret.0).unwrap_or(u32::MAX),
        character: client.position_encoding().column(line, caret.1),
    }
}

#[note("rename-plan")]
pub fn plan(
    edits: &WorkspaceEdit,
    enc: crate::encoding::PositionEncoding,
    read: &mut dyn FnMut(&Path) -> Option<String>,
) -> Result<Vec<Planned>, PathBuf> {
    let mut out = Vec::new();
    for (uri, file_edits) in edits {
        let Some(file) = crate::uri_to_path(uri) else {
            return Err(PathBuf::from(uri));
        };
        let Some(before) = read(&file) else {
            return Err(file);
        };
        let after = crate::edit::apply_text_edits(&before, file_edits, enc);
        out.push(Planned {
            file,
            count: file_edits.len(),
            before,
            after,
        });
    }
    out.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(out)
}

#[note("rename-planned")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Planned {
    pub file: PathBuf,
    pub count: usize,
    pub before: String,
    pub after: String,
}

#[cfg(test)]
#[path = "test/rename/test.rs"]
mod tests;
