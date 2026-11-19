//! The gate over the one question this crate asks the operating system.
//!
//! # What can and cannot be proved without a screen
//!
//! [`crate::monitor_at`] reads the machine it runs on: its answer on a laptop
//! with an auto-hidden taskbar is not the answer on the owner's 1024x768 with
//! a 48-pixel bar. So there is no number to assert here, and inventing one
//! would produce a test that is green on this machine and red on the next.
//!
//! What IS assertable is the pair of invariants the call promises regardless
//! of the machine — the work area is real, and it is part of the monitor —
//! plus the arithmetic of [`crate::ScreenRect`], which is pure and carries the
//! measured numbers.
//!
//! The rest is proved one layer up, in `crawrad-ide`, where the maximize
//! arithmetic is a pure function fed the owner's measured screen.

use crate::ScreenRect;

/// A 1024x768 screen whose bottom 48 pixels are the taskbar.
///
/// These are the owner's numbers, measured on 17/09/2026 while chasing the
/// defect this crate was born for, in the owner's words (translated): *"the
/// maximized IDE (…) the footer is being cut off"*.
fn owners_screen() -> (ScreenRect, ScreenRect) {
    let full = ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1024.0,
        height: 768.0,
    };
    let work = ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1024.0,
        height: 720.0,
    };
    (full, work)
}

/// **The work area is part of the monitor, and the taskbar is the difference.**
#[test]
fn the_work_area_is_contained_in_the_monitor() {
    let (full, work) = owners_screen();
    assert!(
        work.fits_in(&full),
        "a work area that leaves the monitor is not a work area"
    );
    assert_eq!(
        full.bottom() - work.bottom(),
        48.0,
        "the difference between the two rectangles IS the taskbar"
    );
}

/// **`fits_in` catches every edge, and it is not `<` dressed up as `<=`.**
///
/// The mutation this refuses is the one that would quietly reintroduce the
/// defect: a rectangle that ends exactly on the work area's bottom edge FITS.
/// An off-by-one here would push the status bar one pixel under the taskbar
/// and no one would see it until a screenshot.
#[test]
fn touching_the_edge_still_fits() {
    let (_, work) = owners_screen();
    let exact = work;
    assert!(exact.fits_in(&work), "the work area fits in itself");

    let one_too_tall = ScreenRect {
        height: work.height + 1.0,
        ..work
    };
    assert!(
        !one_too_tall.fits_in(&work),
        "one pixel past the bottom edge is the defect this crate exists for"
    );

    let one_too_high = ScreenRect {
        y: work.y - 1.0,
        ..work
    };
    assert!(
        !one_too_high.fits_in(&work),
        "above the top edge is outside"
    );

    let one_too_far_left = ScreenRect {
        x: work.x - 1.0,
        ..work
    };
    assert!(
        !one_too_far_left.fits_in(&work),
        "left of the edge is outside"
    );

    let one_too_wide = ScreenRect {
        width: work.width + 1.0,
        ..work
    };
    assert!(!one_too_wide.fits_in(&work), "right of the edge is outside");
}

/// **The edges are computed and not stored, so they cannot drift apart.**
#[test]
fn the_edges_come_from_the_origin_and_the_size() {
    let r = ScreenRect {
        x: -1920.0,
        y: 120.0,
        width: 1600.0,
        height: 900.0,
    };
    assert_eq!(r.right(), -320.0, "a second monitor sits at a negative x");
    assert_eq!(r.bottom(), 1020.0);
}

/// **The system really answers, and the answer holds together.**
///
/// This is the only test that touches the FFI, and it asserts the two things
/// that are true on any Windows machine: the monitor has a positive size, and
/// the work area lives inside it. It would catch the mistake that costs the
/// most here — reading `rcMonitor` where `rcWork` was meant would still pass,
/// but swapping them would not, and neither would a `cbSize` that made the
/// system refuse to fill the struct.
#[cfg(target_os = "windows")]
#[test]
fn the_system_describes_the_monitor_under_the_origin() {
    let Some(area) = crate::monitor_at((0.0, 0.0)) else {
        panic!("GetMonitorInfoW refused to describe the primary monitor");
    };
    assert!(
        area.full.width > 0.0 && area.full.height > 0.0,
        "a monitor with no size means the struct was never filled: {:?}",
        area.full
    );
    assert!(
        area.work.width > 0.0 && area.work.height > 0.0,
        "a work area with no size means rcWork was never filled: {:?}",
        area.work
    );
    assert!(
        area.work.fits_in(&area.full),
        "the work area left the monitor — the two rectangles are swapped: {area:?}"
    );
}

/// **A point off the desktop still lands on a monitor.**
///
/// `MONITOR_DEFAULTTONEAREST` is what makes this true, and it is the flag that
/// keeps a window dragged half off the screen from getting `None` and falling
/// back to the maximize that covers the taskbar.
#[cfg(target_os = "windows")]
#[test]
fn a_point_far_off_the_desktop_gets_the_nearest_monitor() {
    let area = crate::monitor_at((-99_000.0, -99_000.0));
    assert!(
        area.is_some_and(|a| a.full.width > 0.0),
        "MONITOR_DEFAULTTONEAREST is what stops this from being None"
    );
}

/// **Off Windows the answer is `None`, and the caller is told so.**
///
/// The gate is here so the day someone writes a Linux arm, this test is what
/// tells them the contract they are changing.
#[cfg(not(target_os = "windows"))]
#[test]
fn off_windows_the_question_has_no_answer_yet() {
    assert_eq!(
        crate::monitor_at((0.0, 0.0)),
        None,
        "only Windows maximizes a popup over the taskbar, so only Windows is asked"
    );
}

/// The window rectangle: the size comes back logical and is scaled, the
/// position is already physical, and a zero DPI counts as one.
#[test]
fn a_window_rect_scales_only_the_size() {
    let r = ScreenRect::of_window([10.0, -8.0], [800.0, 600.0], 1.5);
    assert_eq!((r.x, r.y, r.width, r.height), (10.0, -8.0, 1200.0, 900.0));
    let unborn = ScreenRect::of_window([0.0, 0.0], [800.0, 600.0], 0.0);
    assert_eq!((unborn.width, unborn.height), (800.0, 600.0));
}
