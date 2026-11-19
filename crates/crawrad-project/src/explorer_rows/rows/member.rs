//! The node of a MEMBER of the group — the chevron, the folder, the name, and
//! the word the member earns inside the group.
//!
//! # The click is an action, and it used to not be
//!
//! Before this file the shell folded the node by reaching into its own field
//! (`self.folds.toggle(...)`) straight out of the click handler. The ruler of
//! the house is one line — *does an agent get to do this by tool, without a
//! mouse?* — and the answer was no, by any door. The row now carries
//! `explorer.toggle_folder` with the folder it was drawn for, and the shell
//! hands the pair to the same door the menu, the keyboard and the protocol
//! use.
//!
//! The argument travels **with the row that was drawn**, and not read again at
//! click time: reading the tree again on the click is the window in which what
//! was drawn and what runs disagree.

use std::path::{Path, PathBuf};

use crawrad_actions::Args;

use super::super::icon::{CHEVRON_CLOSED, CHEVRON_OPEN, MEMBER, MEMBER_OPEN, RowIcon};
use super::super::problems::Problems;
use super::super::row::{Activation, ExplorerRow};
use crawrad_note::note;

#[note("explorer-rows-rows-member-member")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    folder: PathBuf,
    name: String,
    /// `active`, `default`, or nothing — `model::member_badge` — with the
    /// count of the problems under the folder before it.
    mark: String,
    open: bool,
    /// The active project: its name is bold (approved mockup, 13/11).
    active: bool,
    problems: Problems,
}

impl Member {
    pub fn new(
        folder: &Path,
        name: impl Into<String>,
        mark: Option<&'static str>,
        open: bool,
    ) -> Self {
        Self {
            folder: folder.to_path_buf(),
            name: name.into(),
            // The pill already says it: no "active" text beside it.
            mark: mark
                .filter(|m| *m != crate::explorer_model::MARK_ACTIVE)
                .unwrap_or("")
                .to_string(),
            open,
            active: mark == Some(crate::explorer_model::MARK_ACTIVE),
            problems: Problems::default(),
        }
    }

    #[note("explorer-rows-rows-member-member-with-problems")]
    pub fn with_problems(mut self, problems: Problems) -> Self {
        self.mark = problems.badge(&self.mark);
        self.problems = problems;
        self
    }
}

impl ExplorerRow for Member {
    #[note("explorer-rows-rows-member-member-template")]
    fn template(&self) -> &'static str {
        match self.problems.tone() {
            Some("Error") => "MemberError",
            Some(_) => "MemberWarn",
            None if self.active => "MemberActive",
            None => "Member",
        }
    }

    fn text(&self) -> &str {
        &self.name
    }

    fn icon(&self) -> Option<RowIcon> {
        Some(if self.open { MEMBER_OPEN } else { MEMBER })
    }

    fn chevron(&self) -> Option<RowIcon> {
        Some(if self.open {
            CHEVRON_OPEN
        } else {
            CHEVRON_CLOSED
        })
    }

    fn badge(&self) -> &str {
        &self.mark
    }

    fn activation(&self) -> Activation {
        Activation::Run(
            "explorer.toggle_folder",
            Args::text("path", self.folder.to_string_lossy().into_owned()),
        )
    }

    fn item_target(&self) -> Option<(PathBuf, bool)> {
        Some((self.folder.clone(), true))
    }
}
