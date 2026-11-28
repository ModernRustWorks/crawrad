//! The build modal dialog content model: title, progress, current action, output and timing.
//!
//! # Why this module exists
//!
//! When compiling an application on **Run**, the IDE shows a centered modal dialog
//! (the classic RAD IDE experience) that keeps the developer informed about the
//! compilation progress. This module provides pure, testable functions that determine
//! WHAT the dialog displays without any dependency on UI shell widgets.
//!
//! Per owner decision (28/11/2026):
//! - The dialog is ONLY the processing: spinner, package, compiling crate, done/total,
//!   output folder, elapsed time, and Cancel.
//! - It closes itself when the build ends - ALWAYS, on failure as well as on success.
//! - It NEVER shows an error, a count of errors or warnings, or a compiler line.
//!   Errors belong in the Problems panel behind the dialog.

use std::path::Path;
use std::time::Duration;

use crate::build_busy::spinner_frame;
use crate::build_progress::BuildProgress;
use crate::workspace::shorten_middle;

/// Maximum characters displayed for the output directory before middle-shortening with ellipsis.
pub const OUTPUT_PATH_MAX_CHARS: usize = 36;

/// Pure snapshot of everything the centered build modal dialog needs to draw.
#[derive(Debug, Clone, PartialEq)]
pub struct BuildDialogContent {
    /// Dialog headline, e.g. "◐  Building clientes".
    pub title: String,
    /// Currently compiling or checking crate, e.g. "Compiling slint v1.9.2".
    pub current_action: Option<String>,
    /// Progress counter, e.g. "42 / 118" while running (None when finished or indeterminate).
    pub counter: Option<String>,
    /// Progress bar fraction from 0.0 to 1.0 (None if indeterminate).
    pub fraction: Option<f64>,
    /// Target output folder path string.
    pub output_path: String,
    /// Elapsed duration formatted as mm:ss or hh:mm:ss.
    pub elapsed: String,
    /// Button label: always "Cancel" during the processing dialog.
    pub button_label: &'static str,
    /// Whether the modal dialog should be displayed on screen (true while running, false when build ends).
    pub should_stay_open: bool,
    /// Whether the compiler process is actively running.
    pub is_running: bool,
}

/// Computes the title string for the build dialog.
pub fn dialog_title(progress: &BuildProgress, elapsed: Duration) -> String {
    let verb = progress.verb();
    let package = progress.package();
    let spinner = spinner_frame(elapsed);
    let display_verb = if verb == "Running" { "Building" } else { verb };
    format!("{spinner}  {display_verb} {package}")
}

/// Returns the current crate and action being processed (e.g. "Compiling slint v1.9.2").
pub fn dialog_current_action(progress: &BuildProgress) -> Option<String> {
    if !progress.is_running() {
        return None;
    }
    progress.current().map(|curr| {
        let action = progress.current_action().unwrap_or("Compiling");
        format!("{action} {curr}")
    })
}

/// Returns the progress counter text ("done / total" while running, None when finished).
///
/// Per owner decision: NEVER returns error or warning counts. Errors belong in the Problems panel.
pub fn dialog_counter(progress: &BuildProgress) -> Option<String> {
    if !progress.is_running() {
        return None;
    }
    if let Some(total) = progress.total() {
        Some(format!("{} / {total}", progress.done()))
    } else if progress.done() > 0 {
        Some(format!("{}", progress.done()))
    } else {
        None
    }
}

/// Returns the progress bar fraction (0.0 to 1.0).
pub fn dialog_fraction(progress: &BuildProgress) -> Option<f64> {
    progress.fraction()
}

/// Formats the target output folder path, shortening with middle ellipsis if needed.
pub fn dialog_output_path(output_dir: Option<&Path>) -> String {
    let default_sep = if cfg!(windows) { "\\" } else { "/" };
    let fallback = format!("…{default_sep}target{default_sep}debug");
    let Some(path) = output_dir else {
        return fallback;
    };
    let path_str = path.display().to_string();
    if path_str.chars().count() > OUTPUT_PATH_MAX_CHARS {
        shorten_middle(&path_str, OUTPUT_PATH_MAX_CHARS)
    } else {
        path_str
    }
}

/// Formats the elapsed time.
pub fn dialog_elapsed(elapsed: Duration) -> String {
    BuildProgress::clock(elapsed)
}

/// Returns the primary button label: always "Cancel".
pub fn dialog_button_label(_progress: &BuildProgress) -> &'static str {
    "Cancel"
}

/// Evaluates whether the modal dialog should still remain open on screen.
///
/// Per owner decision:
/// The dialog closes itself when the build ends - ALWAYS, on failure as well as on success.
/// It stays open ONLY while actively running.
pub fn should_dialog_remain_open(progress: &BuildProgress) -> bool {
    progress.is_running()
}

/// Produces the complete `BuildDialogContent` snapshot.
pub fn dialog_content(
    progress: &BuildProgress,
    output_dir: Option<&Path>,
    elapsed: Duration,
) -> BuildDialogContent {
    let is_running = progress.is_running();
    let effective_elapsed = progress.finished().map(|f| f.elapsed).unwrap_or(elapsed);

    BuildDialogContent {
        title: dialog_title(progress, elapsed),
        current_action: dialog_current_action(progress),
        counter: dialog_counter(progress),
        fraction: dialog_fraction(progress),
        output_path: dialog_output_path(output_dir),
        elapsed: dialog_elapsed(effective_elapsed),
        button_label: dialog_button_label(progress),
        should_stay_open: should_dialog_remain_open(progress),
        is_running,
    }
}

#[cfg(test)]
#[path = "test/build_dialog/test.rs"]
mod tests;
