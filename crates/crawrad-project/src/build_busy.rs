//! Busy indicators, corner card, and page turn math for the Build window.

use std::time::Duration;

use crawrad_component::pieces::BusyTiming;

use crate::build_progress::{BuildProgress, ProgressState};
use crate::build_settings::BuildMode;
use crawrad_note::note;

pub const SPINNER: [&str; 4] = ["◐", "◓", "◑", "◒"];

#[note("build-busy-spinner-frame")]
pub fn spinner_frame(since_start: Duration) -> &'static str {
    let step = (since_start.as_millis() / 120) % 4;
    SPINNER[step as usize]
}

#[note("build-busy-busy-label")]
pub fn busy_label(
    mode: BuildMode,
    timing: &BusyTiming,
    since_start: Duration,
    ended: Option<Duration>,
) -> Option<String> {
    if timing.opacity(since_start, ended) <= 0.0 {
        return None;
    }
    let verb = match mode {
        BuildMode::Build => "Building",
        BuildMode::Run => "Running",
        BuildMode::Check => "Checking",
        BuildMode::Clean => "Cleaning",
    };
    Some(format!("{} {}", spinner_frame(since_start), verb))
}

#[note("build-busy-card")]
#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub icon: String,
    pub headline: String,
    pub detail: String,
    pub clock: String,
    pub fraction: Option<f64>,
    pub running: bool,
}

pub const CARD_LINGER: Duration = Duration::from_secs(4);

#[note("build-busy-card-2")]
pub fn card(
    progress: &BuildProgress,
    since_start: Duration,
    since_end: Option<Duration>,
) -> Option<Card> {
    if let Some(end) = since_end {
        if end >= CARD_LINGER {
            return None;
        }
        let icon = match progress.state() {
            ProgressState::Succeeded => "✓",
            _ => "✕",
        }
        .to_string();
        let clock = progress
            .finished()
            .map(|f| BuildProgress::clock(f.elapsed))
            .unwrap_or_else(|| BuildProgress::clock(since_start));
        Some(Card {
            icon,
            headline: progress.headline(),
            detail: progress.counter().unwrap_or_default(),
            clock,
            fraction: progress.fraction(),
            running: false,
        })
    } else {
        Some(Card {
            icon: spinner_frame(since_start).to_string(),
            headline: progress.headline(),
            detail: progress.counter().unwrap_or_default(),
            clock: BuildProgress::clock(since_start),
            fraction: progress.fraction(),
            running: true,
        })
    }
}

#[note("build-busy-should-open-on-finish")]
pub fn should_open_on_finish(mode: BuildMode, progress: &BuildProgress, window_open: bool) -> bool {
    mode != BuildMode::Run && !window_open && progress.state() == ProgressState::Failed
}

pub const PAGE_TURN_SHIFT: f64 = 24.0;
pub const PAGE_TURN: Duration = Duration::from_millis(150);

#[note("build-busy-page-turn-offset")]
pub fn page_turn_offset(since_switch: Duration) -> f64 {
    if since_switch >= PAGE_TURN {
        return 0.0;
    }
    let t = since_switch.as_secs_f64() / PAGE_TURN.as_secs_f64();
    let remain = (1.0 - t).clamp(0.0, 1.0);
    PAGE_TURN_SHIFT * remain * remain
}

#[cfg(test)]
#[path = "test/build_busy/test.rs"]
mod tests;
