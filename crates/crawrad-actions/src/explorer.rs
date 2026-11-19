//! What an agent can do to the **EXPLORER tree** — fold it, and unfold it.
//!
//! # Why a family of its own, and not a corner of `LayoutCommand`
//!
//! [`LayoutCommand`](super::LayoutCommand) answers *where the panels are*. This
//! answers *what is open inside one of them*, and the two never travel
//! together: resetting the layout must not close a folder, and opening a
//! folder must not move a panel. An agent reading an [`Effect`](super::Effect)
//! to find out what is about to happen reads the family and already knows —
//! this one writes nothing to disk, launches no process and does not touch the
//! document.
//!
//! # Why it is in the catalogue at all
//!
//! The rule of the house: *does an agent get to do this with a tool, without a
//! mouse?* Folding a member node and opening the legend are the two pieces of
//! state the panel owns, and before this module the shell toggled both by
//! reaching into its own fields. That is exactly the kind of capability that
//! never reaches the protocol, because nobody notices it is missing.
//!
//! # What is deliberately NOT here
//!
//! The panel also **explains** a file when it is clicked — the sentence that
//! says whether typing in it survives the next `Generate`. That is a readout
//! and not a capability: it changes nothing, and an agent that wants the
//! answer reads the generation plan instead of asking a panel. Making it an
//! action would be an action whose argument is its own answer.

use crawrad_note::note;
use std::path::PathBuf;

#[note("explorer-explorercommand")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorerCommand {
    /// Open the member node when it is closed, close it when it is open.
    ///
    /// **Toggle**, and not `Open`/`Close`, for the same reason
    /// [`LayoutCommand::Toggle`](super::LayoutCommand::Toggle) is: the user's
    /// gesture is one click that goes both ways, and two commands over one
    /// chevron would be two things to keep in step.
    ToggleFolder(PathBuf),
    /// Open or close the legend at the foot of the panel.
    ///
    /// The legend explains `Rewritten` / `Yours` / `active`, which is a
    /// concept of THIS product and of no other IDE — nobody arrives knowing
    /// it, unlike the `M`/`U` of git. So it cannot be deleted; it is folded
    /// away, closed by default, the way VS Code folds `Outline` and
    /// `Timeline`.
    ToggleLegend,
    /// Open or close the Open Editors card section.
    ToggleOpenEditors,
    /// Open or close the Outline card section.
    ToggleOutline,
    /// Open or close the Timeline card section.
    ToggleTimeline,
    /// Open a local history copy in the History view by its timeline index.
    OpenTimelineEntry(usize),
    /// Open or close the Rust Dependencies card section.
    ToggleDependencies,
    /// Create a new file in specified path or active project.
    NewFile(PathBuf),
    /// Create a new form in specified path or active project.
    NewForm(PathBuf),
    /// Create a new folder in specified path or active project.
    NewFolder(PathBuf),
    /// Reveal file or folder in system file explorer.
    Reveal(PathBuf),
    /// Copy absolute path to clipboard.
    CopyPath(PathBuf),
    /// Copy relative path to clipboard.
    CopyRelativePath(PathBuf),
    /// Rename file or folder.
    Rename(PathBuf),
    /// Delete file or folder.
    Delete(PathBuf),
    /// Refresh explorer contents.
    Refresh,
    /// Collapse all explorer folders.
    CollapseAll,
}

#[cfg(test)]
#[path = "test/explorer/test.rs"]
mod tests;

#[note("explorer-workspacerole")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkspaceRole {
    /// Any other folder, or a file.
    #[default]
    Other,
    /// The workspace's root (its `Cargo.toml` has `[workspace]`).
    Root,
    /// A project the root lists in `members`; `active` = in `default-members`.
    Member { active: bool },
}
