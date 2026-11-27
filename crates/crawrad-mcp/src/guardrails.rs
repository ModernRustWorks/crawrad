//! Guardrails: every command checks itself BEFORE it runs.
//!
//! The owner's rule (23/11/2026): every command fired through the MCP must carry
//! a guardrail — and one of them is the dialog. A person cannot touch
//! the IDE behind an open modal: the veil stops the pointer and the dialog owns
//! the keyboard. The MCP could, and that was visible on screen — the editor
//! changing underneath an open Note dialog.
//!
//! The shape is deliberate, and it is the one that works: the command refuses
//! **without mutating** and answers with a reason a machine can act on, so the
//! caller fixes itself and sends the command again. Never a bare "no".
//!
//! This is the place to add the next ones. A guardrail belongs here when it is
//! about *whether the command may run at all*, not about its arguments.
//!
//! # Already guarded elsewhere
//!
//! The **view** guardrail lives on the action descriptor (`ViewAffinity`) and is
//! checked in the MCP host: a Design command with the editor in front, or a Code
//! command with the Designer in front, is refused with `wrong-view-design` /
//! `wrong-view-code` and the move that fixes it ("Call 'view.toggle_code' and
//! resend"). That one is done; it is named here so nobody writes it twice.
//!
//! # Next ones the owner asked for (23/11/2026)
//!
//! Each answers with a code plus the move that fixes it, never a bare refusal:
//!
//! - a page/tab command while the wrong page is in front;
//! - a command that needs a selection with nothing selected;
//! - a command that needs a saved project on a project that was never saved;
//! - a build or run command while a build is already running.

/// A command that was refused, and what the caller must do about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// Stable code the caller can branch on, never prose.
    pub code: &'static str,
    /// What happened, in English, for a human reading the log.
    pub message: String,
    /// The move that makes the command work. The caller does this and resends.
    pub fix: &'static str,
}

/// Commands that only look. These stay allowed while a dialog is open, so the
/// caller can read the dialog it must deal with — refusing those would leave it
/// blind and unable to recover.
fn only_reads(tool: &str) -> bool {
    tool.contains(".get_")
        || tool.contains(".list_")
        || matches!(
            tool,
            "component.installed"
                | "debug.state"
                | "chat.history"
                | "form.check"
                | "editor.get_active_file"
                | "designer.get_active_form"
        )
}

/// The whole state a guardrail may look at. One struct so adding the next
/// guardrail does not change every call site.
#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    /// Is a modal dialog on screen?
    pub modal_open: bool,
}

/// Checks a command against every guardrail. `None` means it may run.
pub fn check(tool: &str, state: State) -> Option<Refusal> {
    if state.modal_open && !only_reads(tool) {
        return Some(Refusal {
            code: "dialog-open",
            message: format!(
                "'{tool}' was not run: a dialog is open, and it owns the IDE while it is."
            ),
            fix: "Close the dialog, then send the command again.",
        });
    }
    None
}

#[cfg(test)]
#[path = "test/guardrails/test.rs"]
mod guardrails_test;
