// Anti-flicker timing and opacity calculations for busy/working indicators.
//
// # Why this module exists
//
// Visual indicators for asynchronous background work (spinners, progress bars,
// pulsing icons) easily flicker if shown unconditionally:
// - An operation that finishes in 20 ms causes an annoying flash if shown immediately.
// - An operation that finishes in 160 ms (just after appearing) causes a 10 ms flash
//   if dismissed immediately.
//
// To prevent flicker across all shells without depending on any UI framework,
// every indicator follows the house rule:
// 1. Appear only after [`BusyTiming::show_after`] (~150 ms).
// 2. Once shown, stay visible for at least [`BusyTiming::min_visible`] (~400 ms).
// 3. Fade out smoothly over [`BusyTiming::fade_out`] (~150 ms).

use std::time::Duration;

/// Anti-flicker timing parameters for progress and busy indicators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BusyTiming {
    /// Delay before the indicator becomes visible. Fast operations ending before
    /// this threshold never show the indicator.
    pub show_after: Duration,
    /// Minimum duration the indicator stays visible once displayed, preventing
    /// perceptual micro-flashes.
    pub min_visible: Duration,
    /// Duration of the linear alpha fade-out transition after work completes.
    pub fade_out: Duration,
}

impl Default for BusyTiming {
    /// Standard defaults approved for IDE operations: 150 ms threshold,
    /// 400 ms minimum visibility, 150 ms fade out.
    fn default() -> Self {
        Self {
            show_after: Duration::from_millis(150),
            min_visible: Duration::from_millis(400),
            fade_out: Duration::from_millis(150),
        }
    }
}

impl BusyTiming {
    /// Opacity 0.0..=1.0 of the indicator `since_start` after the work began;
    /// `ended` = how long after the start the work ended (`None` = still running).
    ///
    /// - If work ended before `show_after`, returns 0.0 for all points in time (never shown).
    /// - While running (`ended = None`), returns 0.0 before `show_after` and 1.0 thereafter.
    /// - After ending at or after `show_after`, holds 1.0 until `max(ended, show_after + min_visible)`,
    ///   then fades linearly to 0.0 over `fade_out`.
    pub fn opacity(&self, since_start: Duration, ended: Option<Duration>) -> f64 {
        match ended {
            Some(end) => {
                if end < self.show_after {
                    return 0.0;
                }
                if since_start < self.show_after {
                    return 0.0;
                }
                let hold_until = end.max(self.show_after + self.min_visible);
                if since_start <= hold_until {
                    return 1.0;
                }
                let fade_end = hold_until + self.fade_out;
                if since_start >= fade_end {
                    return 0.0;
                }
                let fade_out_secs = self.fade_out.as_secs_f64();
                if fade_out_secs == 0.0 {
                    return 0.0;
                }
                let elapsed_in_fade = since_start - hold_until;
                let progress = elapsed_in_fade.as_secs_f64() / fade_out_secs;
                (1.0 - progress).clamp(0.0, 1.0)
            }
            None => {
                if since_start >= self.show_after {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    /// Whether the indicator still needs animation frames (to appear, stay or fade).
    ///
    /// Returns `true` while the operation is still running, or while fading out after
    /// having been shown. Returns `false` once the fade out has completed, or if the
    /// work ended before `show_after` and was never shown.
    pub fn animating(&self, since_start: Duration, ended: Option<Duration>) -> bool {
        match ended {
            Some(end) => {
                if end < self.show_after {
                    false
                } else {
                    let hold_until = end.max(self.show_after + self.min_visible);
                    let fade_end = hold_until + self.fade_out;
                    since_start < fade_end
                }
            }
            None => true,
        }
    }
}
