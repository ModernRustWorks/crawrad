//! **The rust-analyzer status pill and progress cell**.
//!
//! A fixed pill (`cell_lsp`) showing server status with an icon and its name,
//! beside a fixed-width progress cell (`cell_lsp_progress`).

use crawrad_note::note;
#[note("lsp-cell-server")]
pub const SERVER: &str = "rust-analyzer";

#[note("lsp-cell-lspphase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LspPhase {
    Busy,
    #[default]
    Ready,
    Failed,
}

impl LspPhase {
    #[note("lsp-cell-lspphase-code")]
    pub fn code(self) -> f64 {
        match self {
            Self::Busy => 0.0,
            Self::Ready => 1.0,
            Self::Failed => 2.0,
        }
    }

    #[note("lsp-cell-lspphase-name")]
    pub fn name(self) -> &'static str {
        match self {
            Self::Busy => "busy",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }
}

#[note("lsp-cell-lspcell")]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LspCell {
    pub phase: LspPhase,
    pub name: String,
    pub detail: String,
}

impl LspCell {
    #[note("lsp-cell-lspcell-running")]
    pub fn running(name: &str, progress: Option<&str>, idle_note: Option<&str>) -> Self {
        match progress {
            Some(line) => Self::new(LspPhase::Busy, name, line),
            None => Self::new(LspPhase::Ready, name, idle_note.unwrap_or_default()),
        }
    }

    #[note("lsp-cell-lspcell-starting")]
    pub fn starting(name: &str) -> Self {
        Self::new(LspPhase::Busy, name, "starting…")
    }

    #[note("lsp-cell-lspcell-down")]
    pub fn down(name: &str, why: &str) -> Self {
        Self::new(LspPhase::Failed, name, why)
    }

    /// The LSP cell for the client's current status and progress.
    pub fn for_client_status(
        fallback: &Self,
        client: Option<(&str, bool, Option<&str>)>,
        idle_note: Option<&str>,
    ) -> Self {
        match client {
            None => fallback.clone(),
            Some((name, false, _)) => Self::down(name, "exited"),
            Some((name, true, prog)) => Self::running(name, prog, idle_note),
        }
    }

    fn new(phase: LspPhase, name: &str, detail: &str) -> Self {
        let (name, detail) = (name.to_string(), detail.to_string());
        Self {
            phase,
            name,
            detail,
        }
    }
}

#[note("lsp-cell-turn")]
pub fn turn(seconds: f64) -> f64 {
    (seconds / 0.9).rem_euclid(1.0)
}

#[cfg(test)]
#[path = "test/lsp_cell/test.rs"]
mod tests;
