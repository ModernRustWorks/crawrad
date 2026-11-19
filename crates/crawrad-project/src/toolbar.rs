//! The command strip of the top bar — Undo, Redo, Save, Run and Preview.
//!
//! # Why a strip and not a menu
//!
//! A menu is a VIEW over a group of the catalogue: everything in the group is
//! drawn, and the shell chooses nothing. A toolbar is the opposite — it is a
//! CHOICE of commands out of thirty-one, and the choice is the design.
//! There is no property in `crawrad-actions` that says "this one goes on the bar",
//! and inventing one would put a decision about the Makepad screen inside a
//! crate that knows nothing about screens.
//!
//! So the choice is its own table, here — out of the shell like the icons it
//! draws (`crate::icons`), and still not a property of the catalogue.
//!
//! # What this module buys, and it is not style
//!
//! Before it, the command ids were string literals inside an `else if` chain
//! in `ide_app.rs`. Renaming `project.run` in the catalogue left that chain
//! compiling and the Run button answering `unknown action: project.run` — a
//! button that looks alive and is not. Here the command ids are one table, and
//! `every_toolbar_command_exists_in_the_catalogue` fails the moment one of them
//! stops existing.

use crate::icons as icon;
use crawrad_note::note;

#[note("toolbar-command")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Undo,
    Redo,
    Save,
    SaveAll,
    /// Opens the Build window.
    Build,
    /// The only one with the accent surface — it is the primary command.
    Run,
    /// The designer's Preview: the form answers clicks and typing, the user's handlers do not run — that is Run.
    Preview,
    /// The debugger group beside Run, lit only while a program is debugged.
    DebugPause,
    DebugStop,
    StepOver,
    StepInto,
    StepOut,
}

impl Command {
    #[note("toolbar-command-all")]
    pub const ALL: &'static [Self] = &[
        Self::Undo,
        Self::Redo,
        Self::Save,
        Self::SaveAll,
        Self::Build,
        Self::Run,
        Self::Preview,
        Self::DebugPause,
        Self::DebugStop,
        Self::StepOver,
        Self::StepInto,
        Self::StepOut,
    ];

    #[note("toolbar-command-icon")]
    pub fn icon(self) -> icon::Chrome {
        match self {
            Self::Undo => icon::Chrome::Undo,
            Self::Redo => icon::Chrome::Redo,
            Self::Save => icon::Chrome::Save,
            Self::SaveAll => icon::Chrome::SaveAll,
            Self::Build => icon::Chrome::Build,
            Self::Run => icon::Chrome::Start,
            Self::Preview => icon::Chrome::Preview,
            Self::DebugPause => icon::Chrome::DebugPause,
            Self::DebugStop => icon::Chrome::DebugStop,
            Self::StepOver => icon::Chrome::StepOver,
            Self::StepInto => icon::Chrome::StepInto,
            Self::StepOut => icon::Chrome::StepOut,
        }
    }

    #[note("toolbar-command-action")]
    pub fn action(self) -> &'static str {
        match self {
            Self::Undo => "edit.undo",
            Self::Redo => "edit.redo",
            Self::Save => "file.save",
            Self::SaveAll => "file.save_all",
            Self::Build => "project.build_manager",
            Self::Run => "project.run",
            Self::Preview => "view.toggle_live",
            Self::DebugPause => "debug.pause",
            Self::DebugStop => "debug.stop",
            Self::StepOver => "debug.step_over",
            Self::StepInto => "debug.step_into",
            Self::StepOut => "debug.step_out",
        }
    }

    #[note("toolbar-command-widget")]
    pub fn widget(self) -> &'static str {
        match self {
            Self::Undo => "btn_undo",
            Self::Redo => "btn_redo",
            Self::Save => "btn_save",
            Self::SaveAll => "btn_save_all",
            Self::Build => "btn_build",
            Self::Run => "btn_play",
            Self::Preview => "btn_preview",
            Self::DebugPause => "tb_debug_pause",
            Self::DebugStop => "tb_debug_stop",
            Self::StepOver => "tb_step_over",
            Self::StepInto => "tb_step_into",
            Self::StepOut => "tb_step_out",
        }
    }

    #[note("toolbar-command-caption")]
    pub fn caption(self) -> Option<&'static str> {
        match self {
            Self::Build => Some("Build"),
            Self::Run => Some("Run"),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/toolbar/test.rs"]
mod tests;
