use crate::BusyTiming;
use std::time::Duration;

#[test]
fn default_values() {
    let timing = BusyTiming::default();
    assert_eq!(timing.show_after, Duration::from_millis(150));
    assert_eq!(timing.min_visible, Duration::from_millis(400));
    assert_eq!(timing.fade_out, Duration::from_millis(150));
}

#[test]
fn running_work_appears_after_show_after() {
    let timing = BusyTiming::default();

    // Before show_after: hidden, but still animating
    assert_eq!(timing.opacity(Duration::from_millis(0), None), 0.0);
    assert!(timing.animating(Duration::from_millis(0), None));
    assert_eq!(timing.opacity(Duration::from_millis(50), None), 0.0);
    assert!(timing.animating(Duration::from_millis(50), None));
    assert_eq!(timing.opacity(Duration::from_millis(149), None), 0.0);
    assert!(timing.animating(Duration::from_millis(149), None));

    // At and after show_after: fully visible and animating
    assert_eq!(timing.opacity(Duration::from_millis(150), None), 1.0);
    assert!(timing.animating(Duration::from_millis(150), None));
    assert_eq!(timing.opacity(Duration::from_millis(300), None), 1.0);
    assert!(timing.animating(Duration::from_millis(300), None));
    assert_eq!(timing.opacity(Duration::from_secs(5), None), 1.0);
    assert!(timing.animating(Duration::from_secs(5), None));
}

#[test]
fn ended_at_100ms_is_never_shown() {
    let timing = BusyTiming::default();
    let ended = Some(Duration::from_millis(100));

    // Finished before 150 ms threshold: opacity is always 0.0, not animating
    for ms in [0, 50, 100, 150, 200, 550, 700, 1000] {
        let t = Duration::from_millis(ms);
        assert_eq!(
            timing.opacity(t, ended),
            0.0,
            "opacity at {ms} ms should be 0.0"
        );
        assert!(
            !timing.animating(t, ended),
            "animating at {ms} ms should be false"
        );
    }
}

#[test]
fn ended_at_200ms_stays_until_550_and_fades_at_700() {
    let timing = BusyTiming::default();
    let ended = Some(Duration::from_millis(200));

    // Before show_after: hidden
    assert_eq!(timing.opacity(Duration::from_millis(100), ended), 0.0);
    assert!(timing.animating(Duration::from_millis(100), ended));

    // From show_after (150 ms) through max(200, 150 + 400 = 550 ms): stays 1.0
    assert_eq!(timing.opacity(Duration::from_millis(150), ended), 1.0);
    assert!(timing.animating(Duration::from_millis(150), ended));
    assert_eq!(timing.opacity(Duration::from_millis(200), ended), 1.0);
    assert_eq!(timing.opacity(Duration::from_millis(400), ended), 1.0);
    assert_eq!(timing.opacity(Duration::from_millis(550), ended), 1.0);
    assert!(timing.animating(Duration::from_millis(550), ended));

    // Linear fade between 550 ms and 700 ms (midpoint at 625 ms = 0.5)
    let mid_opacity = timing.opacity(Duration::from_millis(625), ended);
    assert!(
        (mid_opacity - 0.5).abs() < 1e-6,
        "expected 0.5 at midpoint, got {mid_opacity}"
    );
    assert!(timing.animating(Duration::from_millis(625), ended));

    // Fade completed at 700 ms
    assert_eq!(timing.opacity(Duration::from_millis(700), ended), 0.0);
    assert!(!timing.animating(Duration::from_millis(700), ended));

    // After 700 ms: remains 0.0 and stops animating
    assert_eq!(timing.opacity(Duration::from_millis(800), ended), 0.0);
    assert!(!timing.animating(Duration::from_millis(800), ended));
}

#[test]
fn ended_at_2s_fades_over_150ms() {
    let timing = BusyTiming::default();
    let ended = Some(Duration::from_secs(2));

    // Stays 1.0 until max(2000, 150 + 400 = 550) = 2000 ms
    assert_eq!(timing.opacity(Duration::from_millis(1999), ended), 1.0);
    assert_eq!(timing.opacity(Duration::from_millis(2000), ended), 1.0);
    assert!(timing.animating(Duration::from_millis(2000), ended));

    // Fades linearly from 2000 ms to 2150 ms
    let mid_opacity = timing.opacity(Duration::from_millis(2075), ended);
    assert!(
        (mid_opacity - 0.5).abs() < 1e-6,
        "expected 0.5 at 2075 ms, got {mid_opacity}"
    );
    assert!(timing.animating(Duration::from_millis(2075), ended));

    // At and after 2150 ms (2.15 s): fully faded and no longer animating
    assert_eq!(timing.opacity(Duration::from_millis(2150), ended), 0.0);
    assert!(!timing.animating(Duration::from_millis(2150), ended));
    assert_eq!(timing.opacity(Duration::from_millis(2200), ended), 0.0);
    assert!(!timing.animating(Duration::from_millis(2200), ended));
}
