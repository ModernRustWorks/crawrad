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
//! # Guardrails active here
//!
//! Each answers with a stable machine code plus the move that fixes it:
//!
//! - a modal dialog is open (`dialog-open`), blocking any mutating action;
//! - a build or run is already active in background (`build-running`), preventing
//!   overlapping compiles and confusing hangs;
//! - a command that operates on the ambient selected component is sent with
//!   nothing selected (`no-selection`), prompting selection via `designer.select`.

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

/// Commands that only look. These stay allowed while a dialog is open, while a
/// build runs, or when nothing is selected, so the caller can read the state it
/// must deal with — refusing read tools would leave it blind and unable to recover.
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
                | "view.sizes"
        )
}

/// Commands that start a build or program execution. Refused when a build is
/// already running so callers do not queue duplicate builds or wait on dead runs.
fn starts_build_or_run(tool: &str) -> bool {
    matches!(
        tool,
        "run.start"
            | "build.build"
            | "build.check"
            | "project.run"
            | "project.run_without_debugging"
            | "project.run_language"
            | "project.build_all"
            | "project.build"
    )
}

/// Working out WHICH tools need a selection from the action catalogue rather than
/// hardcoding a list: the action catalogue in `crawrad-actions` is the single source
/// of truth for action preconditions (`Gate { need, check }`). Querying it keeps
/// guardrails in sync with newly registered actions without risking list drift.
fn needs_selection(tool: &str) -> bool {
    crawrad_actions::catalogue()
        .into_iter()
        .any(|a| a.id == tool && a.enabled.need == crawrad_actions::Need::Selection)
}

/// The whole state a guardrail may look at. One struct so adding the next
/// guardrail does not change every call site.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct State {
    /// Is a modal dialog on screen?
    pub modal_open: bool,
    /// Is a background build or run currently active?
    pub build_running: bool,
    /// Is a component currently selected in the active form?
    pub has_selection: bool,
}

/// Checks a command against every guardrail. `None` means it may run.
pub fn check(tool: &str, state: State) -> Option<Refusal> {
    // Read tools are exempted across all guardrails so agents never lose visibility.
    if only_reads(tool) {
        return None;
    }
    if state.modal_open {
        return Some(Refusal {
            code: "dialog-open",
            message: format!(
                "'{tool}' was not run: a dialog is open, and it owns the IDE while it is."
            ),
            fix: "Close the dialog, then send the command again.",
        });
    }
    if state.build_running && starts_build_or_run(tool) {
        return Some(Refusal {
            code: "build-running",
            message: format!("'{tool}' was not run: a build is already running."),
            fix: "Wait for the build to finish, or call 'run.stop' and send the command again.",
        });
    }
    if !state.has_selection && needs_selection(tool) {
        return Some(Refusal {
            code: "no-selection",
            message: format!(
                "'{tool}' was not run: it acts on the selected component, but nothing is selected."
            ),
            fix: "Call 'designer.select' to select a component, then send the command again.",
        });
    }
    None
}

pub type CallRefusal = Box<(
    String,
    serde_json::Value,
    crate::protocol::ToolCallResult,
    &'static str,
)>;

/// Prepares and validates a tools/call request before any IDE action is executed.
///
/// Extracts the tool name and arguments, unwraps `action.execute` wrappers,
/// resolves relative file paths to absolute, and checks both modal/run/selection
/// guardrails and the pending-diff-review save guard.
pub fn prepare_tools_call<'a>(
    params: &'a serde_json::Value,
    roots: &[Option<std::path::PathBuf>],
    state: State,
    disable_diff_review_gate: bool,
    has_pending_diff_review: bool,
) -> Result<(&'a str, serde_json::Value), CallRefusal> {
    let fail = |n: String,
                a: serde_json::Value,
                r: crate::protocol::ToolCallResult,
                o: &'static str| { Box::new((n, a, r, o)) };
    let Some(name) = params.get("name").and_then(serde_json::Value::as_str) else {
        let err = crate::protocol::ToolCallResult::error_text(
            "invalid-request",
            "Missing 'name' field in tools/call parameters",
        );
        return Err(fail("unknown".to_string(), params.clone(), err, "error"));
    };
    let args = params.get("arguments").unwrap_or(&serde_json::Value::Null);
    let (name, args) = crate::tools::unwrap_action_execute(name, args);
    let rewritten = crate::edit::absolute_path_arg(args, roots);
    let args_val = rewritten.unwrap_or_else(|| args.clone());

    if let Some(refusal) = check(name, state) {
        let text = format!("{} {}", refusal.message, refusal.fix);
        let err = crate::protocol::ToolCallResult::error_text(refusal.code, &text);
        return Err(fail(name.to_string(), args_val, err, "rejected"));
    }

    if let Err((refused, outcome)) = crate::protocol::check_pending_review_save_guard(
        disable_diff_review_gate,
        name,
        has_pending_diff_review,
    ) {
        return Err(fail(
            "action.execute".to_string(),
            args_val,
            refused,
            outcome,
        ));
    }

    Ok((name, args_val))
}

#[cfg(test)]
#[path = "test/guardrails/test.rs"]
mod guardrails_test;
