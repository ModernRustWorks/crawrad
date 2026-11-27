//! Diagnostic dialog for ACP agent connections.
//!
//! Provides a visual status report on agent connection checks
//! (binary path, network reachability, process execution, ACP handshake)
//! and actionable steps to recover when the agent is unavailable.

use std::path::Path;

use super::super::*;
use crawrad_agent::{
    CheckStatus, DiagnosticAction, DiagnosticCheck, DiagnosticReport, DiagnosticStep,
};

/// Number of allocated button slots for actions in the dialog footer.
///
/// These are fixed widgets in the live_design (`diag_act_0` .. `diag_act_7`),
/// so an action past the last slot is never drawn and the user never learns it
/// existed. The const assertion below ties this count to the engine's own
/// promise: raise one and the build stops until the other follows.
pub const DIAGNOSTIC_ACTION_SLOTS: usize = 8;

const _: () = assert!(
    DIAGNOSTIC_ACTION_SLOTS >= crawrad_agent::acp_diagnostic::MAX_DIAGNOSTIC_ACTIONS,
    "the diagnostic dialog has fewer button slots than the engine can produce      actions, so the last actions would be dropped without a word"
);

/// Number of allocated check rows in the dialog (`diag_check_0` .. `diag_check_3`).
pub const DIAGNOSTIC_CHECK_SLOTS: usize = 4;

/// Returns the widget LiveId path for action button `i`.
pub fn diag_action_id(i: usize) -> [LiveId; 1] {
    [LiveId::from_str(&format!("diag_act_{i}"))]
}

/// Returns the widget LiveId path for check label `i`.
pub fn diag_check_id(i: usize) -> [LiveId; 1] {
    [LiveId::from_str(&format!("diag_check_{i}"))]
}

/// Returns the visual status mark corresponding to CheckStatus.
pub fn check_status_mark(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Passed => "✓",
        CheckStatus::Failed => "✕",
        CheckStatus::NotAttempted => "—",
    }
}

/// Formats a single diagnostic check line.
pub fn format_check_line(check: &DiagnosticCheck) -> String {
    let mark = check_status_mark(check.status);
    format!("{mark} {}: {}", check.step.label(), check.detail)
}

/// Formats all check lines in report order.
pub fn format_check_lines(report: &DiagnosticReport) -> Vec<String> {
    report.checks.iter().map(format_check_line).collect()
}

/// Returns all button labels for the actions in the report.
pub fn action_button_labels(report: &DiagnosticReport) -> Vec<String> {
    report.actions.iter().map(|a| a.label()).collect()
}

/// Pure decision: whether clicking on the agent chip should open the diagnostic.
pub fn should_open_diagnostic_on_chip_click(
    link: crawrad_agent::chat_layout::ChipLink,
    live_failed: bool,
) -> bool {
    link == crawrad_agent::chat_layout::ChipLink::Failed
        || link == crawrad_agent::chat_layout::ChipLink::Cli
        || live_failed
}

/// Fallback helper to open a file in the system's default program if the IDE editor cannot open it.
pub fn open_file_in_system(target: &Path) {
    #[cfg(windows)]
    {
        let mut cmd = crawrad_os::process::no_console("cmd");
        cmd.args(["/c", "start", ""]).arg(target);
        let _ = cmd.spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = crawrad_os::process::no_console("open").arg(target).spawn();
    }
    #[cfg(all(not(windows), not(target_os = "macos")))]
    {
        let _ = crawrad_os::process::no_console("xdg-open")
            .arg(target)
            .spawn();
    }
}

impl IdeApp {
    /// Determines whether the active agent connection is failing.
    pub fn is_agent_failing(&self) -> bool {
        let default_sender = self.get_active_agent_name();
        let chip = crawrad_agent::chat_layout::agent_chip(
            &self.chat.session.messages,
            &default_sender,
            self.chat.live.status(),
        );
        let live_failed = self.chat.live.live_failure().is_some();
        should_open_diagnostic_on_chip_click(chip.link, live_failed)
    }

    /// Opens the ACP diagnostic dialog and triggers background execution.
    pub fn open_acp_diagnostic(&mut self, cx: &mut Cx) {
        self.acp_diagnostic_open = true;
        self.start_acp_diagnostic_task();
        self.refresh(cx);
    }

    /// Spawns the diagnostic execution on a background thread so the window never freezes.
    pub fn start_acp_diagnostic_task(&mut self) {
        let cfg = self.acp_config.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        self.acp_diagnostic_rx = Some(rx);
        self.acp_diagnostic_running = true;
        std::thread::spawn(move || {
            let hooks = crawrad_agent::DiagnosticHooks::live();
            let report = crawrad_agent::run_diagnostic(&cfg, &hooks);
            let _ = tx.send(report);
            crawrad_os::tasks::wake();
        });
    }

    /// Draws the ACP diagnostic dialog and checks for background results.
    pub fn draw_acp_diagnostic(&mut self, cx: &mut Cx) {
        if let Some(report) = self
            .acp_diagnostic_rx
            .as_ref()
            .and_then(|rx| rx.try_recv().ok())
        {
            self.acp_diagnostic_report = Some(report);
            self.acp_diagnostic_running = false;
            self.acp_diagnostic_rx = None;
        }

        self.show_veil(cx, id!(acp_diagnostic_dialog), self.acp_diagnostic_open);
        if !self.acp_diagnostic_open {
            return;
        }

        let dlg = self.ui.view(id!(acp_diagnostic_dialog.dlg));
        dlg.button(id!(diag_close)).set_text(cx, "Close");

        if self.acp_diagnostic_running && self.acp_diagnostic_report.is_none() {
            dlg.label(id!(diag_status))
                .set_text(cx, "Running diagnostic checks in background\u{2026}");
            for (i, step) in [
                DiagnosticStep::ProgramExists,
                DiagnosticStep::NetworkReachable,
                DiagnosticStep::ProcessStarted,
                DiagnosticStep::HandshakeResponded,
            ]
            .into_iter()
            .enumerate()
            {
                dlg.label(&diag_check_id(i))
                    .set_text(cx, &format!("— {}: in progress\u{2026}", step.label()));
            }
            dlg.label(id!(diag_config_path))
                .set_text(cx, "Config file: checking\u{2026}");
            dlg.label(id!(diag_detected_agents))
                .set_text(cx, "Detected agents: searching\u{2026}");

            for i in 0..DIAGNOSTIC_ACTION_SLOTS {
                dlg.button(&diag_action_id(i)).set_visible(cx, false);
            }
            return;
        }

        let Some(ref report) = self.acp_diagnostic_report else {
            return;
        };

        if self.acp_diagnostic_running {
            dlg.label(id!(diag_status))
                .set_text(cx, "Retrying diagnostic in background\u{2026}");
        } else if report.is_success() {
            dlg.label(id!(diag_status))
                .set_text(cx, "✓ All diagnostic checks passed");
        } else if let Some(failed) = report.failed_step() {
            dlg.label(id!(diag_status)).set_text(
                cx,
                &format!("✕ Diagnostic failed at step: {}", failed.step.label()),
            );
        } else {
            dlg.label(id!(diag_status))
                .set_text(cx, "✕ Diagnostic found issues");
        }

        // 1. The checks, in order
        let check_lines = format_check_lines(report);
        for (i, line) in check_lines.iter().enumerate().take(DIAGNOSTIC_CHECK_SLOTS) {
            dlg.label(&diag_check_id(i)).set_text(cx, line);
        }

        // 2. The two informative items
        dlg.label(id!(diag_config_path)).set_text(
            cx,
            &format!("Config file: {}", report.config_path.display()),
        );

        let detected_str = if report.detected_agents.is_empty() {
            "None detected".to_string()
        } else {
            report.detected_agents.join(", ")
        };
        dlg.label(id!(diag_detected_agents))
            .set_text(cx, &format!("Detected agents: {detected_str}"));

        // 3. The buttons, one per DiagnosticAction
        let button_labels = action_button_labels(report);
        for i in 0..DIAGNOSTIC_ACTION_SLOTS {
            let btn = dlg.button(&diag_action_id(i));
            if let Some(label) = button_labels.get(i) {
                btn.set_text(cx, label);
                btn.set_visible(cx, true);
            } else {
                btn.set_visible(cx, false);
            }
        }
    }

    /// Handles actions and button clicks in the ACP diagnostic dialog.
    pub fn atender_acp_diagnostic(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.acp_diagnostic_open {
            return;
        }

        let dlg = self.ui.view(id!(acp_diagnostic_dialog.dlg));

        if dlg.button(id!(diag_close)).clicked(actions) {
            self.acp_diagnostic_open = false;
            self.refresh(cx);
            return;
        }

        let Some(report) = self.acp_diagnostic_report.clone() else {
            return;
        };

        for (i, action) in report
            .actions
            .iter()
            .enumerate()
            .take(DIAGNOSTIC_ACTION_SLOTS)
        {
            if dlg.button(&diag_action_id(i)).clicked(actions) {
                match action {
                    DiagnosticAction::Retry => {
                        self.start_acp_diagnostic_task();
                        self.draw_acp_diagnostic(cx);
                        self.refresh(cx);
                    }
                    DiagnosticAction::UseAgent(name) => {
                        self.acp_diagnostic_open = false;
                        self.selecionar_provedor_ia(cx, name);
                        self.open_options(cx);
                        self.secao = options::Category::AiAcp;
                        self.redraw_options(cx);
                        self.refresh(cx);
                    }
                    DiagnosticAction::SelectAnother => {
                        self.acp_diagnostic_open = false;
                        self.open_options(cx);
                        self.secao = options::Category::AiAcp;
                        self.redraw_options(cx);
                        self.refresh(cx);
                    }
                    DiagnosticAction::OpenConfigFile => {
                        let path = report.config_path.clone();
                        if path.is_file() {
                            self.abrir_no_editor(cx, path.clone());
                        } else {
                            open_file_in_system(&path);
                        }
                        self.status = Some(format!("Opened config: {}", path.display()));
                    }
                    DiagnosticAction::CopyError(err) => {
                        cx.copy_to_clipboard(err);
                        self.status = Some("Error details copied to clipboard".to_string());
                    }
                }
                break;
            }
        }
    }
}

#[cfg(test)]
#[path = "../../test/ide_app/acp_diagnostic/test.rs"]
mod test;
