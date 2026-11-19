use super::*;

/// The owner's 1920x1080 at 100%, with a 48-pixel taskbar at the bottom.
fn work() -> ScreenRect {
    ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1032.0,
    }
}

#[test]
fn a_window_opens_in_the_middle_of_the_work_area() {
    assert_eq!(
        centred_in(work(), dvec2(640.0, 420.0), 1.0),
        dvec2(640.0, 306.0)
    );
}

/// At 150% the window is 1.5x bigger on screen, and the point Makepad takes
/// is logical: it multiplies it back by the factor.
#[test]
fn the_point_is_logical_at_any_scale() {
    let at = centred_in(work(), dvec2(640.0, 420.0), 1.5);
    assert_eq!(
        at * 1.5,
        dvec2((1920.0 - 960.0) / 2.0, (1032.0 - 630.0) / 2.0)
    );
}

/// A secondary monitor left of the main one sits at a negative `x`.
#[test]
fn a_monitor_left_of_the_main_one_centres_on_itself() {
    let left = ScreenRect {
        x: -1280.0,
        y: 0.0,
        width: 1280.0,
        height: 984.0,
    };
    assert_eq!(
        centred_in(left, dvec2(640.0, 420.0), 1.0),
        dvec2(-960.0, 282.0)
    );
}

#[test]
fn a_window_bigger_than_the_area_keeps_its_caption_on_screen() {
    assert_eq!(
        centred_in(work(), dvec2(2400.0, 1200.0), 1.0),
        dvec2(0.0, 0.0)
    );
}
