//! What the server tells us without being asked — and the one thing that matters:
//! whether it managed to load the project at all.
//!
//! # The defect this closes
//!
//! The owner, with `FormMain` on screen and two `impl ... for FormMain` twenty lines
//! below it, ran Find All References and read:
//!
//! > *"No references to `FormMain`"*
//!
//! Nothing was broken in the search. The project's `Cargo.toml` pointed at a folder
//! that no longer existed, cargo could not load the manifest, and rust-analyzer
//! indexed nothing — so it answered, correctly, with an empty list. The IDE showed
//! what it was given.
//!
//! And rust-analyzer HAD said so, on `window/showMessage`, in the words
//! *"failed to load manifest"*. The client dropped the notification on the floor
//! (`handle_incoming_message` handled `textDocument/publishDiagnostics` and nothing
//! else), so the one signal that existed never reached anyone.
//!
//! # What this module is
//!
//! A single place that reads those notifications and keeps the last thing worth
//! saying. Three callers want it, and they are three symptoms of the same silence:
//!
//! 1. an empty answer on a project that never loaded must say so, not "none found";
//! 2. a server error should appear, instead of vanishing.
//!
//! Indexing is the third such moment, and the client already answers for it:
//! `track_progress` keeps the work map and `LspClient::progress` reports it.
//!
//! It stores a verdict, never a stream: the caller asks "is the project all right?"
//! at the moment it has an empty answer to explain, which is the only moment it
//! matters.

use std::sync::{Arc, Mutex};

use crawrad_note::note;

/// What the server last told us about its own health.
#[note("server-says-health")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Health {
    /// Nothing said yet, or nothing worth saying: assume it is working.
    #[default]
    Fine,
    /// The project itself could not be loaded — the manifest, the workspace, cargo.
    /// An empty answer now means nothing at all, and saying "no references" lies.
    ProjectFailed(String),
    /// The server complained about something else.
    Complained(String),
}

impl Health {
    /// True while an empty answer cannot be trusted to mean "there are none".
    /// Indexing is the other such moment, and the client answers for it with
    /// [`crate::client::LspClient::progress`].
    #[note("server-says-cannot-trust-empty")]
    pub fn cannot_trust_empty(&self) -> bool {
        matches!(self, Health::ProjectFailed(_))
    }

    /// What to show the person instead of a count of zero. `None` when an empty
    /// answer is the honest answer.
    #[note("server-says-instead-of-empty")]
    pub fn instead_of_empty(&self) -> Option<String> {
        match self {
            Health::Fine | Health::Complained(_) => None,
            Health::ProjectFailed(why) => Some(format!("The project did not load: {why}")),
        }
    }
}

/// The health the client keeps, shared with whoever asks.
#[note("server-says-watch")]
#[derive(Debug, Clone, Default)]
pub struct Watch(Arc<Mutex<Health>>);

impl Watch {
    #[note("server-says-watch-new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[note("server-says-watch-health")]
    pub fn health(&self) -> Health {
        self.0.lock().map(|h| h.clone()).unwrap_or_default()
    }

    /// Reads one server notification and updates the verdict. Anything that is not
    /// one of the notifications we listen for leaves the verdict alone.
    #[note("server-says-watch-heard")]
    pub fn heard(&self, method: &str, params: &serde_json::Value) {
        let Some(next) = read(method, params) else {
            return;
        };
        if let Ok(mut slot) = self.0.lock() {
            *slot = next;
        }
    }
}

/// Turns one notification into a verdict, or `None` when it says nothing about health.
///
/// Kept free of the lock so it can be read and tested on its own.
#[note("server-says-read")]
pub fn read(method: &str, params: &serde_json::Value) -> Option<Health> {
    match method {
        "window/showMessage" | "window/logMessage" => {
            let text = params.get("message").and_then(|m| m.as_str())?;
            // Type 1 is Error in LSP; anything milder is not worth overriding a
            // verdict with, unless it names a load failure outright.
            let is_error = params.get("type").and_then(|t| t.as_u64()) == Some(1);
            if says_the_project_failed(text) {
                Some(Health::ProjectFailed(first_line(text)))
            } else if is_error {
                Some(Health::Complained(first_line(text)))
            } else {
                None
            }
        }
        // `$/progress` is NOT read here: the client already tracks it into its
        // work map and answers with `LspClient::progress()`. Asking twice would
        // give two answers to drift apart.
        _ => None,
    }
}

/// Whether the text is the server saying it could not load the project.
///
/// Matched on the words cargo and rust-analyzer actually use. Deliberately narrow:
/// a false positive here would hide a real empty answer behind a scary message.
#[note("server-says-the-project-failed")]
pub fn says_the_project_failed(text: &str) -> bool {
    let t = text.to_ascii_lowercase();
    t.contains("failed to load manifest")
        || t.contains("failed to load workspace")
        || t.contains("cargo metadata failed")
        || t.contains("failed to run cargo metadata")
        || t.contains("no cargo.toml")
        || (t.contains("failed to load") && t.contains("cargo.toml"))
}

/// The first line, trimmed — a server message can be a page long, and the panel
/// has one line to say it in.
#[note("server-says-first-line")]
pub fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or("").trim().to_string()
}
