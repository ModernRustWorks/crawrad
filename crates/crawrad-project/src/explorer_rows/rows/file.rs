//! A file of the generation plan — and **who owns it**, which is the one thing
//! this panel exists to say.
//!
//! # Three carriers for one fact, and each carries a different part
//!
//! The reference has nothing like this: VS Code says `M`/`U`, which is git,
//! which everybody already knows. `Rewritten` / `Yours` is the promise of the
//! generator — that what the user typed survives the next `Generate` — and it
//! is the most expensive information on this screen, because getting it wrong
//! costs somebody's work.
//!
//! So it is said three times, and not three times the same way:
//!
//! | carrier | says |
//! |---|---|
//! | the **drawing** | what the file IS — code, manifest, not written yet, leftover |
//! | the **colour of the drawing** | who owns it: the user's files are bright, ours recede |
//! | the **badge** | the word — and a click puts the whole sentence in the status bar |
//!
//! The colour rule reads the right way round on purpose. The panel is not here
//! to show off what the generator wrote; it is here to promise that the file
//! with the user's name on it is untouched. So the user's files are the ones
//! that stand out, and the generated ones are the quiet background they sit in.

use std::path::{Path, PathBuf};

use crawrad_actions::Args;
use crawrad_codegen::Dono;

use super::super::icon::{MANIFEST, PENDING, RowIcon, SOURCE, WARNING};
use super::super::problems::{Problems, state_icon};
use super::super::row::{Activation, ExplorerRow};
use crate::explorer_model as model;
use crawrad_note::note;

#[note("explorer-rows-rows-file-manifest-name")]
const MANIFEST_NAME: &str = "Cargo.toml";

#[note("explorer-rows-rows-file-file")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// Relative to the member, which is what `project.open_editor` wants.
    path: PathBuf,
    name: String,
    badge: String,
    state: model::FileState,
    /// What the language server said about it.
    problems: Problems,
    /// Has unsaved edits in an open editor buffer.
    unsaved: bool,
    /// `None` only for a file outside the plan — see `model::Line::owner`.
    owner: Option<Dono>,
    indent: usize,
}

impl File {
    pub fn new(path: &Path, state: model::FileState, owner: Option<Dono>, indent: usize) -> Self {
        Self {
            path: path.to_path_buf(),
            name: model::name_of(path).to_string(),
            badge: owner
                .map(model::badge_for)
                .unwrap_or(model::BADGE_STALE)
                .to_string(),
            state,
            problems: Problems::default(),
            unsaved: false,
            owner,
            indent,
        }
    }

    #[note("explorer-rows-rows-file-file-with-problems")]
    pub fn with_problems(mut self, problems: Problems) -> Self {
        self.badge = problems.badge(&self.badge);
        self.problems = problems;
        self
    }

    #[note("explorer-rows-rows-file-file-with-unsaved")]
    pub fn with_unsaved(mut self, unsaved: bool) -> Self {
        self.unsaved = unsaved;
        self
    }

    #[note("explorer-rows-rows-file-file-is-the-users")]
    fn is_the_users(&self) -> bool {
        matches!(self.owner, Some(Dono::Usuario) | Some(Dono::PrimeiraVez))
    }
}

impl ExplorerRow for File {
    fn template(&self) -> &'static str {
        match self.problems.tone() {
            Some("Error") => return "FileError",
            Some(_) => return "FileWarn",
            None => {}
        }
        match self.state {
            model::FileState::Stale => "FileStale",
            model::FileState::Pending => "FilePending",
            model::FileState::OnDisk | model::FileState::User => {
                if self.is_the_users() {
                    "FileYours"
                } else {
                    "File"
                }
            }
        }
    }

    fn text(&self) -> &str {
        &self.name
    }

    fn indent(&self) -> usize {
        self.indent
    }

    fn icon(&self) -> Option<RowIcon> {
        if let Some(state) = state_icon(self.problems, self.unsaved) {
            return Some(state);
        }
        Some(match self.state {
            // Stale is not a file to keep: it is the only line of this panel
            // that asks someone to do something.
            model::FileState::Stale => WARNING,
            model::FileState::Pending => PENDING,
            _ if self.name == MANIFEST_NAME => MANIFEST,
            _ => SOURCE,
        })
    }

    fn badge(&self) -> &str {
        &self.badge
    }

    fn activation(&self) -> Activation {
        // `editor.open`, and NOT `project.open_editor`. Both exist and do
        // different things: that one drops the file into the SYSTEM editor —
        // still the right answer for whoever wants full rust-analyzer —, and
        // this one opens a TAB inside the IDE, next to the drawing. The owner asked
        // for the second on 2026-09-17: *"we need tabs for each .RS opened in
        // the editor"*.
        //
        // Via the CATALOGUE, like every line of this panel: an agent opens the same
        // file via `editor.open` without any mouse.
        //
        // Here it used to say that `project.open_editor` had no handler in this
        // shell and therefore the click only informed. It still has no handler,
        // and the outcome is no longer just reporting: it became the action that
        // this shell knows how to fulfill.
        Activation::Run(
            "editor.open",
            Args::text("path", self.path.to_string_lossy()),
        )
    }

    #[note("explorer-rows-rows-file-file-note")]
    fn note(&self) -> Option<String> {
        Some(hint(self.state, self.owner))
    }

    fn item_target(&self) -> Option<(PathBuf, bool)> {
        Some((self.path.clone(), false))
    }
}

#[note("explorer-rows-rows-file-hint")]
pub fn hint(state: model::FileState, owner: Option<Dono>) -> String {
    match (state, owner) {
        (model::FileState::Stale, _) | (_, None) => {
            "Left over from an older form. The next Generate deletes it.".to_string()
        }
        (model::FileState::Pending, Some(d)) => {
            format!("Not generated yet. {}", model::explanation(d))
        }
        (model::FileState::OnDisk, Some(d)) => model::explanation(d).to_string(),
        (model::FileState::User, Some(d)) => format!(
            "Not part of the plan — CrawRAD never writes it. {}",
            model::explanation(d)
        ),
    }
}
