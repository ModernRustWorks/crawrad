//! The `build.*` and `run.*` tools' answers: a cargo run as the tool says it,
//! and the one program `run.start` keeps until `run.stop`. Pure of the window,
//! so it lives here; the shell only finds the folder and writes the audit line.

use std::path::{Path, PathBuf};
use std::process::Child;

use serde_json::{Value, json};

use crate::cargo_tool::{self, Ran};
use crate::protocol::ToolCallResult;
use crawrad_note::note;

#[note("run-tool-answer")]
pub type Answer = (ToolCallResult, &'static str);

#[note("run-tool-no-project")]
pub fn no_project() -> Answer {
    let said = "No active project root directory.";
    (ToolCallResult::error_text("no-project", said), "error")
}

#[note("run-tool-in-project")]
pub fn in_project(root: Option<PathBuf>, run: impl FnOnce(&Path) -> Ran) -> Answer {
    root.map_or_else(no_project, |root| answer(run(&root)))
}

#[note("run-tool-answer-2")]
pub fn answer(ran: Ran) -> Answer {
    match ran {
        Ok((answer, said, success)) => (
            ToolCallResult::success_json(answer, said),
            if success { "applied" } else { "error" },
        ),
        Err(e) => (ToolCallResult::error_text("spawn-error", e), "error"),
    }
}

#[note("run-tool-runslot")]
#[derive(Default)]
pub struct RunSlot(Option<Child>);

impl RunSlot {
    #[note("run-tool-runslot-start")]
    pub fn start(&mut self, root: Option<PathBuf>, args: &Value) -> Answer {
        let Some(root) = root else {
            return no_project();
        };
        if let Some(pid) = self.running() {
            let said = format!("Application is already running with PID {pid}.");
            let data = json!({ "status": "already_running", "pid": pid });
            return (ToolCallResult::success_json(data, said), "rejected");
        }
        self.spawn(cargo_tool::run_command(&root, args))
    }

    #[note("run-tool-runslot-spawn")]
    fn spawn(&mut self, mut cmd: std::process::Command) -> Answer {
        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                self.0 = Some(child);
                let said = format!("Application started with PID {pid}.");
                let data = json!({ "status": "started", "pid": pid });
                (ToolCallResult::success_json(data, said), "applied")
            }
            Err(e) => {
                let said = format!("Failed to spawn cargo run: {e}");
                (ToolCallResult::error_text("spawn-error", said), "error")
            }
        }
    }

    #[note("run-tool-runslot-running")]
    pub fn running(&mut self) -> Option<u32> {
        let child = self.0.as_mut()?;
        matches!(child.try_wait(), Ok(None)).then(|| child.id())
    }

    #[note("run-tool-runslot-stop")]
    pub fn stop(&mut self) -> Answer {
        let Some(mut child) = self.0.take() else {
            let data = json!({ "status": "not_running" });
            let said = "No application process was running.";
            return (ToolCallResult::success_json(data, said), "applied");
        };
        let pid = child.id();
        let _ = child.kill();
        let _ = child.wait();
        let said = format!("Application process with PID {pid} stopped.");
        let data = json!({ "status": "stopped", "pid": pid });
        (ToolCallResult::success_json(data, said), "applied")
    }
}

/// Runs `cargo check` in the workspace root.
pub fn check_in_project(root: Option<PathBuf>) -> Answer {
    in_project(root, cargo_tool::check)
}

/// Runs `cargo build` in the workspace root.
pub fn build_in_project(root: Option<PathBuf>, args: &Value) -> Answer {
    in_project(root, |d| cargo_tool::build(d, args))
}

/// Extracts diagnostics from the workspace root.
pub fn diagnostics_in_project(root: Option<PathBuf>) -> Answer {
    in_project(root, cargo_tool::diagnostics)
}

#[cfg(test)]
#[path = "test/run_tool/test.rs"]
mod tests;
