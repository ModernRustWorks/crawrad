//! First-start environment session model and installation coordinator.
//!
//! Tracks tool probe status, manages background installation pipelines, and
//! prepares display-ready parts for the environment check dialog.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, channel};

use crate::environment::{
    Host, InstallStep, Probe, Tool, ToolState, ToolStatus, check, install_plan, needs_dialog,
};
use crate::environment_install::{InstallError, InstallEvent, run_plan};

/// Order in which tools appear in the environment dialog.
pub const TOOL_ORDER: [Tool; 5] = [
    Tool::BuildTools,
    Tool::RustToolchain,
    Tool::RustAnalyzer,
    Tool::Debugger,
    Tool::WebTarget,
];

/// The intro shown while a required tool is missing.
pub const INTRO: &str = "You can design forms now. To Run your app and get completion and errors as you type, CrawRAD needs the tools marked Missing.";

/// Head parts of the dialog: heading, intro (empty = hidden), "Install all missing"
/// label (empty = hidden), and busy state.
pub type EnvironmentHeadParts = (String, String, String, bool);

/// Row parts of a single tool: icon color, title, mono tag, state word, purpose, detail text, progress ratio, output line, and button label.
pub type EnvironmentRowParts = (
    [f32; 4],
    String,
    String,
    String,
    String,
    String,
    Option<f32>,
    String,
    Option<String>,
);

/// Formats the dialog heading based on the number of missing required tools.
pub fn heading_text(required_missing: usize) -> String {
    match required_missing {
        0 => "Everything is ready".to_string(),
        1 => "1 required tool is missing".to_string(),
        n => format!("{n} required tools are missing"),
    }
}

/// Returns the button label corresponding to an installation step.
pub fn button_label(step: &InstallStep) -> &'static str {
    match step {
        InstallStep::DownloadAndRun { .. } => "Download and install",
        InstallStep::Run { .. } => "Install",
        InstallStep::OpenUrl { .. } => "Open download page",
        InstallStep::CopyCommand { .. } => "Copy command",
        InstallStep::Manual { .. } => "How to install",
    }
}

/// Counts how many steps in an installation plan can be run automatically.
/// Excludes OpenUrl, CopyCommand, and Manual steps.
pub fn install_all_count(plan: &[InstallStep]) -> usize {
    plan.iter()
        .filter(|s| {
            matches!(
                s,
                InstallStep::DownloadAndRun { .. } | InstallStep::Run { .. }
            )
        })
        .count()
}

/// Returns the small mono tag for a given tool.
pub fn tool_mono_tag(tool: Tool) -> &'static str {
    match tool {
        Tool::BuildTools => "",
        Tool::RustToolchain => "rustc · cargo",
        Tool::RustAnalyzer => "rust-analyzer",
        Tool::Debugger => "lldb-dap",
        Tool::WebTarget => "wasm32-unknown-unknown",
    }
}

/// Formats the status word for a given tool.
pub fn state_word(state: &ToolState, tool: Tool, is_running: bool) -> String {
    if is_running {
        return "Installing".to_string();
    }
    match state {
        ToolState::Installed { version } => {
            if version.is_empty() {
                "Installed".to_string()
            } else {
                format!("Installed {version}")
            }
        }
        ToolState::TooOld { version, minimum } => {
            let min_display = minimum.strip_suffix(".0").unwrap_or(minimum);
            format!("Too old {version}, needs {min_display}")
        }
        ToolState::Missing => {
            if tool.required() {
                "Missing".to_string()
            } else {
                "Optional".to_string()
            }
        }
    }
}

/// Returns the status indicator RGBA color for a tool row.
pub fn status_color(state: &ToolState, tool: Tool, is_running: bool) -> [f32; 4] {
    if is_running {
        // Amber #f0b35d
        [0.941, 0.702, 0.365, 1.0]
    } else {
        match state {
            ToolState::Installed { .. } => {
                // Green #7fd4a0
                [0.498, 0.831, 0.627, 1.0]
            }
            ToolState::TooOld { .. } => {
                // Red #ff8a80
                [1.0, 0.541, 0.502, 1.0]
            }
            ToolState::Missing => {
                if tool.required() {
                    // Red #ff8a80
                    [1.0, 0.541, 0.502, 1.0]
                } else {
                    // Grey #98989b
                    [0.596, 0.596, 0.608, 1.0]
                }
            }
        }
    }
}

/// Resolves the individual installation step for a given tool.
pub fn tool_install_step(tool: Tool, statuses: &[ToolStatus], host: Host) -> Option<InstallStep> {
    let status = statuses.iter().find(|s| s.tool == tool)?;
    if matches!(status.state, ToolState::Installed { .. }) {
        return None;
    }
    match tool {
        Tool::BuildTools => match host {
            Host::WindowsX64 | Host::WindowsArm64 => Some(InstallStep::OpenUrl {
                url: "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
            }),
            Host::MacX64 | Host::MacArm64 => Some(InstallStep::Run {
                program: "xcode-select",
                args: vec!["--install"],
            }),
            Host::LinuxX64 | Host::LinuxArm64 => Some(InstallStep::CopyCommand {
                command: "sudo apt install build-essential pkg-config libx11-dev libxcursor-dev libxkbcommon-dev libasound2-dev",
            }),
        },
        Tool::RustToolchain => match &status.state {
            ToolState::Missing => Some(InstallStep::DownloadAndRun {
                url: host.rustup_init_url(),
                file_name: host.rustup_init_file_name(),
                args: vec!["-y", "--profile", "default"],
            }),
            ToolState::TooOld { .. } => Some(InstallStep::Run {
                program: "rustup",
                args: vec!["update", "stable"],
            }),
            ToolState::Installed { .. } => None,
        },
        Tool::RustAnalyzer => Some(InstallStep::Run {
            program: "rustup",
            args: vec!["component", "add", "rust-analyzer"],
        }),
        Tool::WebTarget => Some(InstallStep::Run {
            program: "rustup",
            args: vec!["target", "add", "wasm32-unknown-unknown"],
        }),
        Tool::Debugger => Some(InstallStep::Manual {
            tool: Tool::Debugger,
            how: "Install lldb-dap via your system package manager or LLVM distribution.",
        }),
    }
}

/// What the shell must do after [`EnvironmentSession::pump`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pump {
    /// Nothing changed.
    Idle,
    /// The first-start probe found a required tool missing: open the dialog.
    Open,
    /// Something changed: redraw the dialog if it is open.
    Redraw,
}

/// State of background probing, tool installation, and dialog data.
#[derive(Default)]
pub struct EnvironmentSession {
    pub statuses: Vec<ToolStatus>,
    pub is_probing: bool,
    pub probe_rx: Option<Receiver<Vec<ToolStatus>>>,
    pub probe_factory: Option<fn() -> Box<dyn Probe + Send>>,
    pub startup_checked: bool,
    pub waiting_startup_decision: bool,
    pub install_cancel: Option<Arc<AtomicBool>>,
    pub install_rx: Option<Receiver<InstallEvent>>,
    pub install_done_rx: Option<Receiver<Result<(), InstallError>>>,
    pub running_tool: Option<Tool>,
    pub running_step: Option<usize>,
    pub active_plan: Vec<(Tool, InstallStep)>,
    pub last_output_line: String,
    pub progress_bytes: Option<(u64, Option<u64>)>,
    pub copied_tool: Option<Tool>,
    pub show_how_tool: Option<Tool>,
}

impl EnvironmentSession {
    /// Starts the single probe of the IDE's first start; `false` once it ran.
    pub fn begin_startup(&mut self, probe_factory: fn() -> Box<dyn Probe + Send>) -> bool {
        if self.startup_checked {
            return false;
        }
        (self.startup_checked, self.waiting_startup_decision) = (true, true);
        self.start_probe(probe_factory);
        true
    }

    /// Polls the workers and says what the shell must do next.
    pub fn pump(&mut self) -> Pump {
        if !self.poll() {
            return Pump::Idle;
        }
        let open = self.waiting_startup_decision && !self.is_probing && self.needs_dialog();
        if !self.is_probing {
            self.waiting_startup_decision = false;
        }
        if open { Pump::Open } else { Pump::Redraw }
    }

    /// The row button at `index`: the step the shell still has to carry out
    /// (open a page, copy a command — already marked copied), if any.
    pub fn press_primary(&mut self, index: usize) -> Option<InstallStep> {
        let tool = *TOOL_ORDER.get(index)?;
        let step = self.install_tool(tool)?;
        if matches!(step, InstallStep::CopyCommand { .. }) {
            self.mark_copied(tool);
        }
        Some(step)
    }

    /// [`Self::view`] for the host the IDE runs on.
    pub fn view_here(&self) -> (EnvironmentHeadParts, Vec<EnvironmentRowParts>) {
        self.view(Host::current().unwrap_or(Host::WindowsX64))
    }

    /// Launches an asynchronous probe using `probe_factory`.
    pub fn start_probe(&mut self, probe_factory: fn() -> Box<dyn Probe + Send>) {
        if self.is_probing {
            return;
        }
        self.is_probing = true;
        self.probe_factory = Some(probe_factory);
        let (tx, rx) = channel();
        self.probe_rx = Some(rx);

        std::thread::Builder::new()
            .name("environment-probe".into())
            .spawn(move || {
                let probe = probe_factory();
                let statuses = check(probe.as_ref());
                let _ = tx.send(statuses);
                crawrad_os::tasks::wake();
            })
            .ok();
    }

    /// Polls background probe and install threads. Returns `true` if state changed.
    pub fn poll(&mut self) -> bool {
        let mut changed = false;

        if let Some(ref rx) = self.probe_rx
            && let Ok(statuses) = rx.try_recv()
        {
            self.statuses = statuses;
            self.is_probing = false;
            self.probe_rx = None;
            changed = true;
        }

        if let Some(ref rx) = self.install_rx {
            while let Ok(event) = rx.try_recv() {
                changed = true;
                match event {
                    InstallEvent::Started { step, title } => {
                        self.running_step = Some(step);
                        self.last_output_line = title;
                    }
                    InstallEvent::Progress { step, done, total } => {
                        self.running_step = Some(step);
                        self.progress_bytes = Some((done, total));
                    }
                    InstallEvent::Line { step, text } => {
                        self.running_step = Some(step);
                        self.last_output_line = text;
                    }
                    InstallEvent::Finished {
                        step: _,
                        ok: _,
                        message,
                    } => {
                        if !message.is_empty() {
                            self.last_output_line = message;
                        }
                    }
                }
                if let Some(step) = self.running_step
                    && let Some((tool, _)) = self.active_plan.get(step)
                {
                    self.running_tool = Some(*tool);
                }
            }
        }

        if let Some(ref rx) = self.install_done_rx
            && let Ok(_result) = rx.try_recv()
        {
            self.install_cancel = None;
            self.install_rx = None;
            self.install_done_rx = None;
            self.running_tool = None;
            self.running_step = None;
            self.active_plan.clear();
            self.progress_bytes = None;
            changed = true;
            if let Some(probe_factory) = self.probe_factory {
                self.start_probe(probe_factory);
            }
        }

        changed
    }

    /// Triggers installation for a single tool or returns the non-automated step for user action.
    pub fn install_tool(&mut self, tool: Tool) -> Option<InstallStep> {
        let rust_is_missing = self
            .statuses
            .iter()
            .find(|s| s.tool == Tool::RustToolchain)
            .map(|s| matches!(s.state, ToolState::Missing))
            .unwrap_or(true);

        if rust_is_missing && (tool == Tool::RustAnalyzer || tool == Tool::WebTarget) {
            return None;
        }

        let host = Host::current().unwrap_or(Host::WindowsX64);
        let step = tool_install_step(tool, &self.statuses, host)?;

        match step {
            InstallStep::OpenUrl { .. } | InstallStep::CopyCommand { .. } => Some(step),
            InstallStep::Manual { .. } => {
                self.toggle_how(tool);
                None
            }
            InstallStep::DownloadAndRun { .. } | InstallStep::Run { .. } => {
                self.run_install_plan(vec![(tool, step)]);
                None
            }
        }
    }

    /// Installs all missing tools capable of automated execution.
    pub fn install_all(&mut self) {
        if self.install_cancel.is_some() {
            return;
        }

        let host = Host::current().unwrap_or(Host::WindowsX64);
        let mut plan_pairs = Vec::new();
        for &tool in &TOOL_ORDER {
            if let Some(step) = tool_install_step(tool, &self.statuses, host)
                && matches!(
                    step,
                    InstallStep::DownloadAndRun { .. } | InstallStep::Run { .. }
                )
            {
                plan_pairs.push((tool, step));
            }
        }

        if plan_pairs.is_empty() {
            return;
        }

        self.run_install_plan(plan_pairs);
    }

    fn run_install_plan(&mut self, plan_pairs: Vec<(Tool, InstallStep)>) {
        if self.install_cancel.is_some() || plan_pairs.is_empty() {
            return;
        }

        let cancel = Arc::new(AtomicBool::new(false));
        self.install_cancel = Some(Arc::clone(&cancel));
        self.running_tool = plan_pairs.first().map(|(t, _)| *t);
        self.running_step = Some(0);
        self.active_plan = plan_pairs.clone();
        self.last_output_line.clear();
        self.progress_bytes = None;

        let (event_tx, event_rx) = channel();
        let (done_tx, done_rx) = channel();
        self.install_rx = Some(event_rx);
        self.install_done_rx = Some(done_rx);

        let cancel_thread = Arc::clone(&cancel);
        let plan: Vec<InstallStep> = plan_pairs.into_iter().map(|(_, s)| s).collect();
        std::thread::Builder::new()
            .name("environment-install".into())
            .spawn(move || {
                let work_dir = std::env::temp_dir().join("crawrad-env-install");
                let result = run_plan(&plan, &work_dir, &cancel_thread, &mut |ev| {
                    let _ = event_tx.send(ev);
                    crawrad_os::tasks::wake();
                });
                let _ = done_tx.send(result);
                crawrad_os::tasks::wake();
            })
            .ok();
    }

    /// Cancels active installation.
    pub fn stop_install(&mut self) {
        if let Some(cancel) = &self.install_cancel {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    /// Records that the copy command for `tool` was placed on clipboard.
    pub fn mark_copied(&mut self, tool: Tool) {
        self.copied_tool = Some(tool);
    }

    /// Toggles the instructional manual note for `tool`.
    pub fn toggle_how(&mut self, tool: Tool) {
        self.show_how_tool = if self.show_how_tool == Some(tool) {
            None
        } else {
            Some(tool)
        };
    }

    /// Returns `true` if an active probe or install operation is running.
    pub fn is_busy(&self) -> bool {
        self.is_probing || self.install_cancel.is_some()
    }

    /// Returns `true` if any required tool is missing or outdated.
    pub fn needs_dialog(&self) -> bool {
        needs_dialog(&self.statuses)
    }

    /// Produces presentation-ready head and row parts for the UI widget.
    pub fn view(&self, host: Host) -> (EnvironmentHeadParts, Vec<EnvironmentRowParts>) {
        let required_missing = self
            .statuses
            .iter()
            .filter(|s| s.tool.required() && !matches!(s.state, ToolState::Installed { .. }))
            .count();
        // Nothing measured yet: saying "ready" here would be a guess.
        let heading = if self.statuses.is_empty() {
            "Checking your tools\u{2026}".to_string()
        } else {
            heading_text(required_missing)
        };

        let is_installing = self.install_cancel.is_some();
        let plan = install_plan(&self.statuses, host);
        let missing_count = install_all_count(&plan);
        // Nothing to install: no button, and no sentence about missing tools.
        let install_all_label = if missing_count == 0 {
            String::new()
        } else {
            format!("Install all missing ({missing_count})")
        };
        let intro = if required_missing == 0 {
            String::new()
        } else {
            INTRO.to_string()
        };

        let head: EnvironmentHeadParts = (heading, intro, install_all_label, is_installing);

        let rust_is_missing = self
            .statuses
            .iter()
            .find(|s| s.tool == Tool::RustToolchain)
            .map(|s| matches!(s.state, ToolState::Missing))
            .unwrap_or(true);

        let mut rows = Vec::with_capacity(TOOL_ORDER.len());
        for &tool in &TOOL_ORDER {
            let state = self
                .statuses
                .iter()
                .find(|s| s.tool == tool)
                .map(|s| &s.state)
                .unwrap_or(&ToolState::Missing);

            let is_this_running = self.running_tool == Some(tool);
            let color = status_color(state, tool, is_this_running);
            let word = state_word(state, tool, is_this_running);

            let detail_text = if self.copied_tool == Some(tool) {
                "Copied: run it in a terminal".to_string()
            } else if self.show_how_tool == Some(tool) {
                "Install lldb-dap via your system package manager or LLVM distribution.".to_string()
            } else if tool == Tool::BuildTools {
                tool.detail(host).to_string()
            } else {
                String::new()
            };

            let (progress, output_line) = if is_this_running {
                let ratio = match self.progress_bytes {
                    Some((done, Some(total))) if total > 0 => {
                        (done as f32 / total as f32).clamp(0.0, 1.0)
                    }
                    Some(_) => 0.5,
                    None => 0.0,
                };
                (Some(ratio), self.last_output_line.clone())
            } else {
                (None, String::new())
            };

            let btn_label = if matches!(state, ToolState::Installed { .. }) {
                None
            } else if rust_is_missing && (tool == Tool::RustAnalyzer || tool == Tool::WebTarget) {
                Some("After Rust".to_string())
            } else {
                let step = tool_install_step(tool, &self.statuses, host);
                let label = step.as_ref().map(button_label).unwrap_or("Install");
                Some(label.to_string())
            };

            rows.push((
                color,
                tool.title().to_string(),
                tool_mono_tag(tool).to_string(),
                word,
                tool.purpose().to_string(),
                detail_text,
                progress,
                output_line,
                btn_label,
            ));
        }

        (head, rows)
    }
}

#[cfg(test)]
#[path = "test/environment_session/test.rs"]
mod test;
