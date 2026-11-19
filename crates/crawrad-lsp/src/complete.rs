//! **Completion off the window's thread** (ADR 0050).
//!
//! `Ctrl+Space` used to ask rust-analyzer on the window's thread with a
//! 600 ms cap. Measured on a fresh Makepad project: empty answers for ~38 s
//! while it indexes, then a first real answer after 18.9 s — so the user got
//! a guessed list, then a frozen window, then the real one, for the same key.
//!
//! Here the request runs on its own thread with a long [`TIMEOUT`]; the
//! editor shows what it already knows at once, and merges these items when
//! they arrive. Each request carries a `ticket`: an answer for a list that is
//! no longer open (or was reopened) is dropped by the caller.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::client::LspClient;
use crate::protocol::{CompletionItem, Position};
use crate::settler::{Settle, Settler};
use crawrad_note::note;

#[note("complete-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(30);

#[note("complete-busy-limit")]
pub const BUSY_LIMIT: Duration = Duration::from_secs(90);

#[note("complete-retry")]
pub const RETRY: Duration = Duration::from_millis(400);

#[note("complete-trigger")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Trigger {
    /// `Ctrl+Space` (`Invoked = 1`).
    #[default]
    Invoked,
    /// A character the server listed in `triggerCharacters` (`2`).
    Character(char),
    /// The last list was `isIncomplete` and the user typed on (`3`).
    Incomplete,
}

impl Trigger {
    #[note("complete-trigger-context")]
    pub fn context(self) -> serde_json::Value {
        match self {
            Self::Invoked => serde_json::json!({ "triggerKind": 1 }),
            Self::Character(c) => {
                serde_json::json!({ "triggerKind": 2, "triggerCharacter": c.to_string() })
            }
            Self::Incomplete => serde_json::json!({ "triggerKind": 3 }),
        }
    }
}

#[note("complete-completionanswer")]
#[derive(Debug)]
pub struct CompletionAnswer {
    pub ticket: u64,
    pub items: Vec<CompletionItem>,
    /// The server's `isIncomplete`: ask again as the user types.
    pub incomplete: bool,
}

struct Job {
    client: Arc<LspClient>,
    file: PathBuf,
    at: Position,
    ticket: u64,
    trigger: Trigger,
}

#[note("complete-completionworker")]
pub struct CompletionWorker {
    inner: Settler<Job, CompletionAnswer>,
}

impl Default for CompletionWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl CompletionWorker {
    pub fn new() -> Self {
        let inner = Settler::new(
            "lsp-completion",
            Settle::Newest,
            |job: Job, jobs: &Receiver<Job>| {
                // While it indexes, rust-analyzer answers EMPTY at once (measured:
                // 0 items for ~38 s on a cold Makepad project). An empty answer from
                // a busy server is "not yet", not "nothing": ask again until it has
                // something, goes idle, or a newer request replaces this one.
                let started = std::time::Instant::now();
                let (items, incomplete) = loop {
                    let mut newer = None;
                    let result = job.client.request_completion_list(
                        &job.file,
                        job.at,
                        job.trigger.context(),
                        TIMEOUT,
                        &mut || {
                            newer = jobs.try_iter().last();
                            newer.is_some()
                        },
                    );
                    if newer.is_some() {
                        // Replaced while the server worked: cancelled, the newer goes next.
                        return Err(newer);
                    }
                    let (items, incomplete) = result.unwrap_or_default();
                    if !items.is_empty()
                        || job.client.progress().is_none()
                        || started.elapsed() > BUSY_LIMIT
                    {
                        break (items, incomplete);
                    }
                    std::thread::sleep(RETRY);
                    if let Some(n) = jobs.try_iter().last() {
                        return Err(Some(n));
                    }
                };
                Ok(CompletionAnswer {
                    ticket: job.ticket,
                    items,
                    incomplete,
                })
            },
        );
        Self { inner }
    }

    #[note("complete-completionworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.inner.set_waker(wake);
    }

    #[note("complete-completionworker-request")]
    pub fn request(
        &self,
        client: Arc<LspClient>,
        file: &Path,
        at: Position,
        ticket: u64,
        trigger: Trigger,
    ) {
        let _ = self.inner.send(Job {
            client,
            file: file.to_path_buf(),
            at,
            ticket,
            trigger,
        });
    }

    #[note("complete-completionworker-take")]
    pub fn take(&self, ticket: u64) -> Option<CompletionAnswer> {
        self.inner
            .answers()
            .try_iter()
            .filter(|a| a.ticket == ticket)
            .last()
    }
}
