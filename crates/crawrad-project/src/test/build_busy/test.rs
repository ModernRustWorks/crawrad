use std::time::Duration;

use super::*;
use crate::build_progress::{BuildProgress, Finish};
use crate::build_settings::BuildMode;

#[test]
fn spinner_steps_at_boundaries() {
    assert_eq!(spinner_frame(Duration::from_millis(0)), "◐");
    assert_eq!(spinner_frame(Duration::from_millis(119)), "◐");
    assert_eq!(spinner_frame(Duration::from_millis(120)), "◓");
    assert_eq!(spinner_frame(Duration::from_millis(239)), "◓");
    assert_eq!(spinner_frame(Duration::from_millis(240)), "◑");
    assert_eq!(spinner_frame(Duration::from_millis(359)), "◑");
    assert_eq!(spinner_frame(Duration::from_millis(360)), "◒");
    assert_eq!(spinner_frame(Duration::from_millis(479)), "◒");
    assert_eq!(spinner_frame(Duration::from_millis(480)), "◐");
    assert_eq!(spinner_frame(Duration::from_millis(600)), "◓");
}

#[test]
fn busy_label_hidden_before_threshold() {
    let timing = BusyTiming::default(); // show_after = 150 ms
    assert_eq!(
        busy_label(BuildMode::Build, &timing, Duration::from_millis(0), None),
        None
    );
    assert_eq!(
        busy_label(BuildMode::Build, &timing, Duration::from_millis(149), None),
        None
    );

    // Visible at exactly 150 ms
    assert_eq!(
        busy_label(BuildMode::Build, &timing, Duration::from_millis(150), None),
        Some("◓ Building".to_string())
    );

    // Different modes
    assert_eq!(
        busy_label(BuildMode::Run, &timing, Duration::from_millis(240), None),
        Some("◑ Running".to_string())
    );
    assert_eq!(
        busy_label(BuildMode::Check, &timing, Duration::from_millis(360), None),
        Some("◒ Checking".to_string())
    );
    assert_eq!(
        busy_label(BuildMode::Clean, &timing, Duration::from_millis(480), None),
        Some("◐ Cleaning".to_string())
    );

    // Fast finish before show_after -> never shown
    assert_eq!(
        busy_label(
            BuildMode::Build,
            &timing,
            Duration::from_millis(200),
            Some(Duration::from_millis(50))
        ),
        None
    );
}

#[test]
fn card_running_state() {
    let mut progress = BuildProgress::new("Building", "my_pkg", "cargo build");
    progress.feed(
        crawrad_os::process::Pipe::Stderr,
        "    Building [====>     ] 10/20: my_pkg",
    );

    let card =
        card(&progress, Duration::from_millis(120), None).expect("running card should be Some");
    assert_eq!(card.icon, "◓");
    assert_eq!(card.headline, "Building my_pkg…");
    assert_eq!(card.detail, "10 / 20 crates");
    assert_eq!(card.clock, "00:00");
    assert_eq!(card.fraction, Some(0.5));
    assert!(card.running);
}

#[test]
fn card_lingers_then_none() {
    let mut progress = BuildProgress::new("Building", "my_pkg", "cargo build");
    progress.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(3),
    });

    let c_start = card(&progress, Duration::from_secs(3), Some(Duration::ZERO));
    assert!(c_start.is_some());
    let c = c_start.unwrap();
    assert_eq!(c.icon, "✓");
    assert_eq!(c.clock, "00:03");
    assert!(!c.running);

    // At 2 seconds (within 4 s CARD_LINGER)
    assert!(
        card(
            &progress,
            Duration::from_secs(5),
            Some(Duration::from_secs(2))
        )
        .is_some()
    );

    // At 3999 ms
    assert!(
        card(
            &progress,
            Duration::from_secs(7),
            Some(Duration::from_millis(3999))
        )
        .is_some()
    );

    // At 4 s (CARD_LINGER boundary) -> None
    assert!(
        card(
            &progress,
            Duration::from_secs(7),
            Some(Duration::from_secs(4))
        )
        .is_none()
    );

    // Past 4 s
    assert!(
        card(
            &progress,
            Duration::from_secs(8),
            Some(Duration::from_secs(5))
        )
        .is_none()
    );
}

#[test]
fn card_cancelled_and_failed_show_x() {
    let mut p_cancelled = BuildProgress::new("Building", "my_pkg", "cargo build");
    p_cancelled.finish(Finish {
        success: false,
        cancelled: true,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });

    let card_c = card(
        &p_cancelled,
        Duration::from_secs(1),
        Some(Duration::from_millis(100)),
    )
    .expect("lingering cancelled card should be Some");
    assert_eq!(card_c.icon, "✕");
    assert_eq!(card_c.headline, "Building my_pkg cancelled");

    let mut p_failed = BuildProgress::new("Building", "my_pkg", "cargo build");
    p_failed.finish(Finish {
        success: false,
        cancelled: false,
        errors: 1,
        warnings: 0,
        elapsed: Duration::from_secs(2),
    });

    let card_f = card(
        &p_failed,
        Duration::from_secs(2),
        Some(Duration::from_millis(100)),
    )
    .expect("lingering failed card should be Some");
    assert_eq!(card_f.icon, "✕");
    assert_eq!(card_f.headline, "Building my_pkg failed");
}

#[test]
fn should_open_on_finish_matrix() {
    let mut p_failed = BuildProgress::new("Building", "my_pkg", "cargo build");
    p_failed.finish(Finish {
        success: false,
        cancelled: false,
        errors: 1,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });

    // Failed + closed -> opens for Build, Check, Clean
    assert!(should_open_on_finish(BuildMode::Build, &p_failed, false));
    assert!(should_open_on_finish(BuildMode::Check, &p_failed, false));
    assert!(should_open_on_finish(BuildMode::Clean, &p_failed, false));
    // Failed + closed -> does NOT open for Run
    assert!(!should_open_on_finish(BuildMode::Run, &p_failed, false));
    // Failed + open -> does not need to open
    assert!(!should_open_on_finish(BuildMode::Build, &p_failed, true));

    let mut p_cancelled = BuildProgress::new("Building", "my_pkg", "cargo build");
    p_cancelled.finish(Finish {
        success: false,
        cancelled: true,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });
    // Cancelled -> never opens by itself
    assert!(!should_open_on_finish(
        BuildMode::Build,
        &p_cancelled,
        false
    ));
    assert!(!should_open_on_finish(BuildMode::Build, &p_cancelled, true));

    let mut p_success = BuildProgress::new("Building", "my_pkg", "cargo build");
    p_success.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });
    // Succeeded -> does not open by itself
    assert!(!should_open_on_finish(BuildMode::Build, &p_success, false));
    assert!(!should_open_on_finish(BuildMode::Build, &p_success, true));

    let p_running = BuildProgress::new("Building", "my_pkg", "cargo build");
    // Running -> does not open
    assert!(!should_open_on_finish(BuildMode::Build, &p_running, false));
}

#[test]
fn page_turn_offset_properties() {
    assert_eq!(page_turn_offset(Duration::ZERO), 24.0);
    assert_eq!(page_turn_offset(Duration::from_millis(150)), 0.0);
    assert_eq!(page_turn_offset(Duration::from_millis(200)), 0.0);

    // Ease-out: at midpoint 75 ms, offset is 24 * (0.5)^2 = 6.0
    let mid = page_turn_offset(Duration::from_millis(75));
    assert!((mid - 6.0).abs() < 1e-4);

    // Monotonic decreasing
    let mut last = 24.0;
    for ms in 1..=150 {
        let cur = page_turn_offset(Duration::from_millis(ms));
        assert!(cur <= last, "offset at {ms}ms ({cur}) > offset at {last}");
        last = cur;
    }
}
