use std::path::PathBuf;
use std::time::Duration;

use crate::build_dialog::*;
use crate::build_progress::{BuildProgress, Finish};

#[test]
fn title_shows_spinner_and_action_when_running() {
    let progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    let title_0 = dialog_title(&progress, Duration::from_millis(0));
    let title_1 = dialog_title(&progress, Duration::from_millis(120));

    assert_eq!(title_0, "◐  Building clientes");
    assert_eq!(title_1, "◓  Building clientes");
}

#[test]
fn current_action_displays_compiling_crate_when_running() {
    let mut progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    assert_eq!(dialog_current_action(&progress), None);

    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "   Compiling slint v1.9.2 (https://github.com/slint-ui/slint#12345)",
    );
    assert_eq!(
        dialog_current_action(&progress),
        Some("Compiling slint v1.9.2".to_string())
    );

    progress.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(10),
    });
    assert_eq!(dialog_current_action(&progress), None);
}

#[test]
fn counter_tracks_done_and_total_while_running_and_never_shows_errors() {
    let mut progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "Building [==================>                  ] 42/118: slint",
    );
    assert_eq!(dialog_counter(&progress), Some("42 / 118".to_string()));

    // When build finishes with errors and warnings, dialog_counter must be None:
    // errors belong in the Problems panel, NEVER in this dialog.
    progress.finish(Finish {
        success: false,
        cancelled: false,
        errors: 5,
        warnings: 3,
        elapsed: Duration::from_secs(15),
    });
    assert_eq!(dialog_counter(&progress), None);
}

#[test]
fn fraction_reflects_build_completion() {
    let mut progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    assert_eq!(dialog_fraction(&progress), None);

    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "Building [====================>                ] 50/100: foo",
    );
    assert_eq!(dialog_fraction(&progress), Some(0.5));

    progress.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(20),
    });
    assert_eq!(dialog_fraction(&progress), Some(1.0));
}

#[test]
fn output_path_shortens_long_paths_with_middle_ellipsis() {
    let fallback = dialog_output_path(None);
    assert!(fallback.contains("target"));
    assert!(fallback.contains("debug"));

    let short = PathBuf::from("C:/proj/target/debug");
    assert_eq!(dialog_output_path(Some(&short)), "C:/proj/target/debug");

    let long = PathBuf::from(
        "C:/a_very_long_directory_structure_that_exceeds_the_maximum_allowed_length/target/debug",
    );
    let formatted = dialog_output_path(Some(&long));
    assert!(formatted.contains('…'));
    assert!(formatted.chars().count() <= OUTPUT_PATH_MAX_CHARS);
}

#[test]
fn button_label_is_always_cancel() {
    let mut progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    assert_eq!(dialog_button_label(&progress), "Cancel");

    progress.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(5),
    });
    assert_eq!(dialog_button_label(&progress), "Cancel");
}

#[test]
fn dialog_closes_itself_always_on_success_failure_and_cancellation() {
    let progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");

    // While running: stays open
    assert!(should_dialog_remain_open(&progress));

    // On success: closes itself automatically
    let mut prog_success = progress.clone();
    prog_success.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(47),
    });
    assert!(
        !should_dialog_remain_open(&prog_success),
        "dialog must close on success"
    );

    // On failure: closes itself automatically (errors go to Problems panel)
    let mut prog_failed = progress.clone();
    prog_failed.finish(Finish {
        success: false,
        cancelled: false,
        errors: 2,
        warnings: 1,
        elapsed: Duration::from_secs(12),
    });
    assert!(
        !should_dialog_remain_open(&prog_failed),
        "dialog must close on failure"
    );

    // On cancellation: closes itself automatically
    let mut prog_cancel = progress.clone();
    prog_cancel.finish(Finish {
        success: false,
        cancelled: true,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(3),
    });
    assert!(
        !should_dialog_remain_open(&prog_cancel),
        "dialog must close on cancel"
    );
}

#[test]
fn dialog_content_aggregates_snapshot_correctly() {
    let mut progress = BuildProgress::new("Running", "clientes", "cargo build -p clientes");
    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "   Compiling slint v1.9.2 (foo)",
    );
    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "Building [==================>                  ] 42/118: slint",
    );

    let path = PathBuf::from("C:/clientes/target/debug");
    let content = dialog_content(&progress, Some(&path), Duration::from_secs(47));

    assert_eq!(content.title, "◒  Building clientes");
    assert_eq!(
        content.current_action,
        Some("Compiling slint v1.9.2".to_string())
    );
    assert_eq!(content.counter, Some("42 / 118".to_string()));
    assert_eq!(content.fraction, Some(42.0 / 118.0));
    assert_eq!(content.output_path, "C:/clientes/target/debug");
    assert_eq!(content.elapsed, "00:47");
    assert_eq!(content.button_label, "Cancel");
    assert!(content.should_stay_open);
    assert!(content.is_running);
}
