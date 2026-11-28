//! Timer management utilities for Slint applications.

use std::time::Duration;

/// Represents a timer configuration with interval and enabled state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerState {
    pub interval_ms: u64,
    pub enabled: bool,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            interval_ms: 1000,
            enabled: true,
        }
    }
}

impl TimerState {
    pub fn new(interval_ms: u64, enabled: bool) -> Self {
        Self {
            interval_ms,
            enabled,
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.interval_ms)
    }
}
