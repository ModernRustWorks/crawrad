//! **A form's `Timer` ticks on Makepad**.
//!
//! The Makepad target used to emit only the `Timer` struct: `OnTimer` never
//! fired. Makepad 1.0.0 has the clock — `Cx::start_interval(seconds)`,
//! `Cx::stop_timer` (`makepad-platform-1.0.0/src/cx_api.rs:350`, `:360`) and
//! `Timer::is_event` (`src/event/event.rs:563`). A [`Ticker`] lives in the
//! generated `App`, one per form `Timer`: the sync keeps it in step with the
//! form's `enabled` and `interval_ms`, and `handle_event` asks it whether an
//! event is its tick.

use crawrad_note::note;
use makepad_widgets::*;

#[note("timer-ticker")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ticker {
    timer: Timer,
    /// The interval running now, in ms; `None` when stopped.
    running: Option<u64>,
}

impl Ticker {
    #[note("timer-ticker-sync")]
    pub fn sync(&mut self, cx: &mut Cx, enabled: bool, interval_ms: u64) {
        let wanted = (enabled && interval_ms > 0).then_some(interval_ms);
        if wanted == self.running {
            return;
        }
        if self.running.is_some() {
            cx.stop_timer(self.timer);
        }
        self.running = wanted;
        if let Some(ms) = wanted {
            self.timer = cx.start_interval(ms as f64 / 1000.0);
        }
    }

    #[note("timer-ticker-fired")]
    pub fn fired(&self, event: &Event) -> bool {
        self.running.is_some() && self.timer.is_event(event).is_some()
    }

    #[note("timer-ticker-running")]
    pub fn running(&self) -> Option<u64> {
        self.running
    }
}
