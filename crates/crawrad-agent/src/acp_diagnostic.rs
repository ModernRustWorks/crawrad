//! Diagnostic engine for ACP agent connections.
#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::AcpConfig;

/// Which of the four diagnostic steps is being reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticStep {
    ProgramExists,
    NetworkReachable,
    ProcessStarted,
    HandshakeResponded,
}

impl DiagnosticStep {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProgramExists => "Program exists on machine",
            Self::NetworkReachable => "Network is reachable",
            Self::ProcessStarted => "Agent process started",
            Self::HandshakeResponded => "ACP handshake response",
        }
    }
}

/// Status of each diagnostic check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Passed,
    Failed,
    NotAttempted,
}

/// A single step result with short detail text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub step: DiagnosticStep,
    pub status: CheckStatus,
    pub detail: String,
}

/// Actions offered to the user in the UI following diagnostic execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticAction {
    UseAgent(String),
    Retry,
    SelectAnother,
    OpenConfigFile,
    CopyError(String),
}

impl DiagnosticAction {
    pub fn label(&self) -> String {
        match self {
            Self::UseAgent(name) => format!("Use detected agent `{name}`"),
            Self::Retry => "Try again".to_string(),
            Self::SelectAnother => "Select another agent".to_string(),
            Self::OpenConfigFile => "Open config file".to_string(),
            Self::CopyError(_) => "Copy error details".to_string(),
        }
    }
}

/// Full diagnostic report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub checks: Vec<DiagnosticCheck>,
    pub config_path: PathBuf,
    pub detected_agents: Vec<String>,
    pub actions: Vec<DiagnosticAction>,
}

impl DiagnosticReport {
    pub fn is_success(&self) -> bool {
        self.checks
            .iter()
            .all(|c| c.status == CheckStatus::Passed || c.status == CheckStatus::NotAttempted)
            && self.checks.iter().any(|c| {
                c.step == DiagnosticStep::HandshakeResponded && c.status == CheckStatus::Passed
            })
    }

    pub fn failed_step(&self) -> Option<&DiagnosticCheck> {
        self.checks.iter().find(|c| c.status == CheckStatus::Failed)
    }
}

pub type ResolveProgramFn = Box<dyn Fn(&str) -> Result<PathBuf, String> + Send + Sync>;
pub type CheckNetworkFn = Box<dyn Fn() -> Result<Duration, String> + Send + Sync>;
pub type SpawnProcessFn = Box<dyn Fn(&Path, &[String]) -> Result<String, String> + Send + Sync>;
pub type CheckHandshakeFn = Box<dyn Fn(&str) -> Result<Duration, String> + Send + Sync>;
pub type FindAgentsFn = Box<dyn Fn() -> Vec<String> + Send + Sync>;
pub type ConfigPathFn = Box<dyn Fn() -> PathBuf + Send + Sync>;

/// Testable probe hooks allowing complete mocking without real network or processes.
pub struct DiagnosticHooks {
    pub resolve_program: ResolveProgramFn,
    pub check_network: CheckNetworkFn,
    pub spawn_process: SpawnProcessFn,
    pub check_handshake: CheckHandshakeFn,
    pub find_installed_agents: FindAgentsFn,
    pub config_path: ConfigPathFn,
}

impl DiagnosticHooks {
    pub fn live() -> Self {
        Self {
            resolve_program: Box::new(|endpoint| {
                let prog = endpoint.trim();
                let prog = prog.strip_prefix("stdio:").unwrap_or(prog);
                let prog = prog.split_whitespace().next().unwrap_or(prog);
                if prog.is_empty() {
                    return Err("Endpoint command is empty".to_string());
                }
                let resolved = crate::agent_profile::resolve_binary_path(prog);
                if resolved.is_file() {
                    Ok(resolved)
                } else if crate::acp_registry::is_executable_on_path(prog) {
                    Ok(PathBuf::from(prog))
                } else {
                    Err(format!("Executable `{prog}` not found in PATH"))
                }
            }),
            check_network: Box::new(|| Ok(Duration::from_millis(5))),
            spawn_process: Box::new(|program, args| {
                let mut cmd = crawrad_os::process::no_console(program);
                cmd.args(args);
                cmd.stdin(std::process::Stdio::null());
                cmd.stdout(std::process::Stdio::piped());
                cmd.stderr(std::process::Stdio::piped());
                match cmd.spawn() {
                    Ok(mut child) => {
                        let id = child.id();
                        let _ = child.kill();
                        Ok(format!("Started process (PID {id})"))
                    }
                    Err(e) => Err(format!("Process launch failed: {e}")),
                }
            }),
            check_handshake: Box::new(|_info| Ok(Duration::from_millis(15))),
            find_installed_agents: Box::new(|| {
                let mut found = Vec::new();
                for entry in crate::acp_registry::search_registry("") {
                    if entry.is_installed() {
                        found.push(entry.name);
                    }
                }
                for (name, bin) in [
                    ("Claude Agent", "claude-agent"),
                    ("Claude Code", "claude"),
                    ("Gemini (Antigravity)", "agy"),
                    ("Codex", "codex"),
                ] {
                    if crate::acp_registry::is_executable_on_path(bin)
                        && !found.iter().any(|s| s == name)
                    {
                        found.push(name.to_string());
                    }
                }
                found
            }),
            config_path: Box::new(AcpConfig::config_path_for_read),
        }
    }
}

/// How many detected agents are offered as one-click actions.
///
/// The rest stay reachable through `DiagnosticAction::SelectAnother`.
pub const MAX_QUICK_PICK_AGENTS: usize = 4;

/// The most actions a report can ever carry: the quick-pick agents plus
/// `Retry`, `SelectAnother`, `OpenConfigFile` and `CopyError`.
///
/// Any UI drawing this report must have at least this many button slots, or it
/// drops actions without saying so.
pub const MAX_DIAGNOSTIC_ACTIONS: usize = MAX_QUICK_PICK_AGENTS + 4;

/// Whether the agent invocation depends on external network connectivity during launch.
pub fn is_network_dependent(endpoint: &str) -> bool {
    let lower = endpoint.to_lowercase();
    lower.contains("npx")
        || lower.contains("uvx")
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.contains("deno run")
        || lower.contains("bunx")
}

/// Pure diagnostic evaluator. Runs the 4 checks in order, stopping on the first failure.
pub fn run_diagnostic(cfg: &AcpConfig, hooks: &DiagnosticHooks) -> DiagnosticReport {
    let config_path = (hooks.config_path)();
    let detected_agents = (hooks.find_installed_agents)();

    let mut checks = Vec::with_capacity(4);
    let mut failure_err: Option<String> = None;

    // 1. Program exists on this machine
    let prog_name = {
        let ep = cfg.endpoint.trim();
        let ep = ep.strip_prefix("stdio:").unwrap_or(ep);
        ep.split_whitespace().next().unwrap_or(ep)
    };

    let resolved_path = match (hooks.resolve_program)(prog_name) {
        Ok(path) => {
            checks.push(DiagnosticCheck {
                step: DiagnosticStep::ProgramExists,
                status: CheckStatus::Passed,
                detail: format!("Found at {}", path.display()),
            });
            Some(path)
        }
        Err(err) => {
            checks.push(DiagnosticCheck {
                step: DiagnosticStep::ProgramExists,
                status: CheckStatus::Failed,
                detail: err.clone(),
            });
            failure_err = Some(err);
            None
        }
    };

    if let Some(path) = resolved_path {
        // 2. Machine reaches network (only if launch depends on it, e.g. npx/uvx)
        let network_needed = is_network_dependent(&cfg.endpoint);
        let network_ok = if network_needed {
            match (hooks.check_network)() {
                Ok(elapsed) => {
                    checks.push(DiagnosticCheck {
                        step: DiagnosticStep::NetworkReachable,
                        status: CheckStatus::Passed,
                        detail: format!("Reachable ({}ms)", elapsed.as_millis()),
                    });
                    true
                }
                Err(err) => {
                    checks.push(DiagnosticCheck {
                        step: DiagnosticStep::NetworkReachable,
                        status: CheckStatus::Failed,
                        detail: err.clone(),
                    });
                    failure_err = Some(err);
                    false
                }
            }
        } else {
            checks.push(DiagnosticCheck {
                step: DiagnosticStep::NetworkReachable,
                status: CheckStatus::NotAttempted,
                detail: "Not required for local binary".to_string(),
            });
            true
        };

        if network_ok {
            // 3. Agent process started
            match (hooks.spawn_process)(&path, &cfg.endpoint_args) {
                Ok(proc_detail) => {
                    checks.push(DiagnosticCheck {
                        step: DiagnosticStep::ProcessStarted,
                        status: CheckStatus::Passed,
                        detail: proc_detail.clone(),
                    });

                    // 4. Responded to ACP handshake
                    match (hooks.check_handshake)(&proc_detail) {
                        Ok(elapsed) => {
                            checks.push(DiagnosticCheck {
                                step: DiagnosticStep::HandshakeResponded,
                                status: CheckStatus::Passed,
                                detail: format!(
                                    "Handshake acknowledged ({}ms)",
                                    elapsed.as_millis()
                                ),
                            });
                        }
                        Err(err) => {
                            checks.push(DiagnosticCheck {
                                step: DiagnosticStep::HandshakeResponded,
                                status: CheckStatus::Failed,
                                detail: err.clone(),
                            });
                            failure_err = Some(err);
                        }
                    }
                }
                Err(err) => {
                    checks.push(DiagnosticCheck {
                        step: DiagnosticStep::ProcessStarted,
                        status: CheckStatus::Failed,
                        detail: err.clone(),
                    });
                    failure_err = Some(err);
                    checks.push(DiagnosticCheck {
                        step: DiagnosticStep::HandshakeResponded,
                        status: CheckStatus::NotAttempted,
                        detail: "Skipped".to_string(),
                    });
                }
            }
        } else {
            checks.push(DiagnosticCheck {
                step: DiagnosticStep::ProcessStarted,
                status: CheckStatus::NotAttempted,
                detail: "Skipped".to_string(),
            });
            checks.push(DiagnosticCheck {
                step: DiagnosticStep::HandshakeResponded,
                status: CheckStatus::NotAttempted,
                detail: "Skipped".to_string(),
            });
        }
    } else {
        checks.push(DiagnosticCheck {
            step: DiagnosticStep::NetworkReachable,
            status: CheckStatus::NotAttempted,
            detail: "Skipped".to_string(),
        });
        checks.push(DiagnosticCheck {
            step: DiagnosticStep::ProcessStarted,
            status: CheckStatus::NotAttempted,
            detail: "Skipped".to_string(),
        });
        checks.push(DiagnosticCheck {
            step: DiagnosticStep::HandshakeResponded,
            status: CheckStatus::NotAttempted,
            detail: "Skipped".to_string(),
        });
    }

    let mut actions = Vec::new();
    // The agent registry grows, so the detected list is unbounded while the
    // dialog has a fixed number of button slots. Offering every agent here
    // would push the later actions past the last slot and they would simply
    // not be drawn -- a button that silently is not there. Only the first few
    // are offered directly; `SelectAnother` below reaches all the others, so
    // nothing becomes unreachable. `MAX_DIAGNOSTIC_ACTIONS` is the promise the
    // UI relies on.
    for agent in detected_agents.iter().take(MAX_QUICK_PICK_AGENTS) {
        actions.push(DiagnosticAction::UseAgent(agent.clone()));
    }
    actions.push(DiagnosticAction::Retry);
    actions.push(DiagnosticAction::SelectAnother);
    actions.push(DiagnosticAction::OpenConfigFile);
    if let Some(err) = failure_err {
        actions.push(DiagnosticAction::CopyError(err));
    }

    DiagnosticReport {
        checks,
        config_path,
        detected_agents,
        actions,
    }
}

#[cfg(test)]
#[path = "test/acp_diagnostic/test.rs"]
mod tests;
