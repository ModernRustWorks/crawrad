//! **Format Document** off the window's thread.
//!
//! `textDocument/formatting` (LSP 3.17) answers the whole file's `TextEdit`s;
//! [`formatted`] applies them to the text the request was made from. An
//! empty answer — or edits that change nothing — means *already formatted*,
//! and the editor must not mark the file as changed.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::client::LspClient;
use crate::edit::TextEdit;
use crawrad_note::note;

#[note("format-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(10);

#[note("format-outcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The edits (possibly none: already formatted).
    Edits(Vec<TextEdit>),
    /// The server did not answer in time, or the request failed.
    NoAnswer(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    pub outcome: Outcome,
}

#[note("format-formatworker")]
pub struct FormatWorker(crate::asker::Asker<FormatAnswer>);

impl Default for FormatWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-format"))
    }

    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("format-formatworker-request")]
    pub fn request(
        &self,
        client: Arc<LspClient>,
        ticket: u64,
        file: &Path,
        version: i32,
        tab_size: u32,
    ) -> bool {
        let file = file.to_path_buf();
        self.0.ask(ticket, move || {
            let mut request = |left| client.request_formatting(&file, tab_size, true, left);
            let retry = crate::definition::RETRY;
            let outcome = match crate::definition::retrying(&mut request, TIMEOUT, retry) {
                Ok(edits) => Outcome::Edits(edits),
                Err(e) => Outcome::NoAnswer(e.to_string()),
            };
            FormatAnswer {
                ticket,
                file,
                version,
                outcome,
            }
        })
    }

    #[note("format-formatworker-take")]
    pub fn take(&self, ticket: u64) -> Option<FormatAnswer> {
        self.0.take(ticket)
    }
}

#[note("format-formatted")]
pub fn formatted(
    text: &str,
    edits: &[TextEdit],
    enc: crate::encoding::PositionEncoding,
) -> Option<String> {
    let out = crate::edit::apply_text_edits(text, edits, enc);
    (out != text).then_some(out)
}

#[note("format-verdict")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Put this whole text in the editor, as one undo step.
    Apply(String),
    /// Say this on the status bar and change nothing.
    Say(&'static str),
}

#[note("format-verdict-2")]
pub fn verdict(
    outcome: Outcome,
    still_same: bool,
    text: &str,
    enc: crate::encoding::PositionEncoding,
) -> Verdict {
    match outcome {
        _ if !still_same => Verdict::Say("The file changed while formatting; format again"),
        Outcome::NoAnswer(_) => Verdict::Say("Language server did not answer"),
        Outcome::Edits(e) => match formatted(text, &e, enc) {
            Some(new) => Verdict::Apply(new),
            None => Verdict::Say("Already formatted"),
        },
    }
}

#[note("format-refusal")]
pub fn refusal(front: Option<&Path>, generated_suffix: &str) -> &'static str {
    match front.is_some_and(|f| f.to_string_lossy().ends_with(generated_suffix)) {
        true => "The generated form source is formatted by the generator: change the form",
        false => "Format Document needs rust-analyzer and a file that takes typing",
    }
}

#[cfg(test)]
#[path = "test/format/test.rs"]
mod tests;
