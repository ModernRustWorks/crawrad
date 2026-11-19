//! Which control gets each pointer event, by the classic RAD rules.

use super::{Focus, FocusRouter, Pointer, PointerHit, PointerRouter, Raw, SizeWatch};
use makepad_widgets::{DVec2, Event, KeyModifiers, Rect, dvec2};

fn rect(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect {
        pos: dvec2(x, y),
        size: dvec2(w, h),
    }
}

/// A panel (0) with a button (1) on it, and a label (2) apart.
fn form() -> Vec<(usize, Rect)> {
    vec![
        (0, rect(10.0, 10.0, 200.0, 100.0)),
        (1, rect(20.0, 20.0, 80.0, 24.0)),
        (2, rect(300.0, 10.0, 60.0, 20.0)),
    ]
}

#[test]
fn the_topmost_control_gets_the_press_in_its_own_coordinates() {
    let mut r = PointerRouter::default();
    let hits = r.route_at(Raw::Down, dvec2(30.0, 25.0), 0.0, &form());
    assert_eq!(
        hits,
        vec![(1, Pointer::Down, dvec2(10.0, 5.0))],
        "the button, not the panel under it"
    );
    let mut r = PointerRouter::default();
    assert_eq!(
        r.route_at(Raw::Down, dvec2(150.0, 90.0), 0.0, &form())[0].0,
        0,
        "the panel where no button is"
    );
    assert!(
        PointerRouter::default()
            .route_at(Raw::Down, dvec2(250.0, 250.0), 0.0, &form())
            .is_empty()
    );
}

#[test]
fn the_release_goes_to_the_control_pressed_even_outside_it() {
    let mut r = PointerRouter::default();
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 0.0, &form());
    let up = r.route_at(Raw::Up, dvec2(320.0, 15.0), 0.0, &form());
    assert_eq!(
        up,
        vec![(1, Pointer::Up, dvec2(300.0, -5.0))],
        "the capture: the button, not the label"
    );
    let up = r.route_at(Raw::Up, dvec2(320.0, 15.0), 0.0, &form());
    assert_eq!(
        up[0].0, 2,
        "with no press pending, the control under the pointer"
    );
}

#[test]
fn enter_and_leave_are_the_change_of_the_control_under_the_pointer() {
    let mut r = PointerRouter::default();
    let kinds =
        |h: Vec<(usize, Pointer, _)>| h.into_iter().map(|(i, k, _)| (i, k)).collect::<Vec<_>>();
    assert_eq!(
        kinds(r.route_at(Raw::Move, dvec2(150.0, 90.0), 0.0, &form())),
        [(0, Pointer::Enter), (0, Pointer::Move)]
    );
    assert_eq!(
        kinds(r.route_at(Raw::Move, dvec2(151.0, 90.0), 0.0, &form())),
        [(0, Pointer::Move)],
        "no enter twice"
    );
    assert_eq!(
        kinds(r.route_at(Raw::Move, dvec2(30.0, 25.0), 0.0, &form())),
        [(0, Pointer::Leave), (1, Pointer::Enter), (1, Pointer::Move)],
        "onto the button: the panel is left"
    );
    assert_eq!(
        kinds(r.route_at(Raw::Move, dvec2(250.0, 250.0), 0.0, &form())),
        [(1, Pointer::Leave)]
    );
    r.route_at(Raw::Move, dvec2(320.0, 15.0), 0.0, &form());
    assert_eq!(
        kinds(r.route_at(Raw::Out, dvec2(0.0, 0.0), 0.0, &form())),
        [(2, Pointer::Leave)],
        "leaving the window"
    );
}

#[test]
fn a_click_is_a_press_and_a_release_on_the_same_control() {
    let mut r = PointerRouter::default();
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 0.0, &form());
    let up: Vec<_> = r
        .route_at(Raw::Up, dvec2(31.0, 26.0), 0.0, &form())
        .into_iter()
        .map(|(i, k, _)| (i, k))
        .collect();
    assert_eq!(up, [(1, Pointer::Up), (1, Pointer::Click)]);
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 0.0, &form());
    let up: Vec<_> = r
        .route_at(Raw::Up, dvec2(320.0, 15.0), 0.0, &form())
        .into_iter()
        .map(|(i, k, _)| (i, k))
        .collect();
    assert_eq!(up, [(1, Pointer::Up)], "released elsewhere: no click");
}

#[test]
fn the_focus_moves_with_an_exit_and_an_enter() {
    let mut f = FocusRouter::default();
    assert_eq!(f.focus(1), [(1, Focus::Enter)]);
    assert_eq!(f.focus(1), [], "the same control again: nothing");
    assert_eq!(f.focus(2), [(1, Focus::Exit), (2, Focus::Enter)]);
    assert_eq!(f.focused(), Some(2));
}

#[test]
fn a_press_on_a_control_that_cannot_take_the_focus_leaves_it() {
    let mut f = FocusRouter::default();
    f.focus(1);
    let down = PointerHit {
        control: 2,
        kind: Pointer::Down,
        x: 0.0,
        y: 0.0,
        at: dvec2(0.0, 0.0),
        button: None,
        modifiers: KeyModifiers::default(),
    };
    assert!(f.route(&Event::Pause, &[down], &[1], None).is_empty());
    assert_eq!(f.focused(), Some(1));
}

/// **Tab moves the focus as a press does**: the keyboard focus moving
/// into a control fires `Exit` and `Enter`; only a change of it counts, so a
/// press elsewhere is not undone by the widget that keeps the keyboard.
#[test]
fn the_keyboard_focus_moving_moves_the_focus() {
    let kinds = |v: Vec<super::FocusHit>| {
        v.into_iter()
            .map(|h| (h.control, h.kind))
            .collect::<Vec<_>>()
    };
    let mut f = FocusRouter::default();
    assert_eq!(
        kinds(f.route(&Event::Pause, &[], &[1, 2], Some(1))),
        [(1, Focus::Enter)]
    );
    assert_eq!(kinds(f.route(&Event::Pause, &[], &[1, 2], Some(1))), []);
    let down = PointerHit {
        control: 2,
        kind: Pointer::Down,
        x: 0.0,
        y: 0.0,
        at: dvec2(0.0, 0.0),
        button: None,
        modifiers: KeyModifiers::default(),
    };
    assert_eq!(
        kinds(f.route(&Event::Pause, &[down], &[1, 2], Some(1))),
        [(1, Focus::Exit), (2, Focus::Enter)],
        "the press moves it; the keyboard still on 1 is no change"
    );
    assert_eq!(
        kinds(f.route(&Event::Pause, &[], &[1, 2], Some(2))),
        [],
        "the widget pressed takes the keyboard: it has the focus already"
    );
    assert_eq!(
        kinds(f.route(&Event::Pause, &[], &[1, 2], Some(1))),
        [(2, Focus::Exit), (1, Focus::Enter)],
        "Shift+Tab back to 1"
    );
    // the keyboard leaving every control, or going to one that cannot take
    // the focus, leaves it where it is
    assert_eq!(kinds(f.route(&Event::Pause, &[], &[1, 2], None)), []);
    assert_eq!(kinds(f.route(&Event::Pause, &[], &[1, 2], Some(3))), []);
    assert_eq!(f.focused(), Some(1));
}

#[test]
fn two_clicks_on_the_same_control_soon_enough_are_a_double_click() {
    let mut r = PointerRouter::default();
    let kinds =
        |v: Vec<(usize, Pointer, _)>| v.into_iter().map(|(i, k, _)| (i, k)).collect::<Vec<_>>();
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 1.0, &form());
    assert_eq!(
        kinds(r.route_at(Raw::Up, dvec2(30.0, 25.0), 1.1, &form())),
        [(1, Pointer::Up), (1, Pointer::Click)]
    );
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 1.2, &form());
    assert_eq!(
        kinds(r.route_at(Raw::Up, dvec2(30.0, 25.0), 1.3, &form())),
        [
            (1, Pointer::Up),
            (1, Pointer::Click),
            (1, Pointer::DblClick)
        ]
    );
    // a third click starts over, and a slow one is not a double click
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 1.4, &form());
    r.route_at(Raw::Up, dvec2(30.0, 25.0), 1.5, &form());
    r.route_at(Raw::Down, dvec2(30.0, 25.0), 3.0, &form());
    assert_eq!(
        kinds(r.route_at(Raw::Up, dvec2(30.0, 25.0), 3.1, &form())),
        [(1, Pointer::Up), (1, Pointer::Click)]
    );
}

#[test]
fn the_wheel_goes_to_the_topmost_control_under_the_pointer() {
    let mut r = PointerRouter::default();
    let rects = form();
    let (_, top) = rects.last().copied().expect("a control");
    let at = top.pos + dvec2(1.0, 1.0);
    let hits = r.route_at(Raw::Wheel, at, 0.0, &rects);
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].1, Pointer::Wheel);
    assert!(
        r.route_at(Raw::Wheel, dvec2(-50.0, -50.0), 0.0, &rects)
            .is_empty()
    );
}

#[test]
fn a_control_is_resized_only_when_its_drawn_size_changes() {
    let mut w = SizeWatch::default();
    let small = dvec2(100.0, 50.0);
    assert!(w.resized(&[(0, DVec2::default())]).is_empty());
    assert!(w.resized(&[(0, small)]).is_empty());
    assert!(w.resized(&[(0, small)]).is_empty());
    assert_eq!(w.resized(&[(0, dvec2(120.0, 50.0))]), vec![0]);
    assert!(w.resized(&[(0, dvec2(120.0, 50.0))]).is_empty());
}

#[test]
fn a_second_click_on_the_same_row_in_time_is_a_double_click() {
    use super::{ClickRecord, DOUBLE_CLICK_SECONDS, second_click};
    let mut last: ClickRecord = None;
    assert!(
        !second_click(&mut last, 3, 10.0),
        "a first click is not double"
    );
    assert!(
        second_click(&mut last, 3, 10.0 + DOUBLE_CLICK_SECONDS),
        "the same row, in time"
    );
    assert!(
        !second_click(&mut last, 3, 10.6),
        "a third click starts over"
    );
    assert!(
        !second_click(&mut last, 4, 10.7),
        "another row is not a double click"
    );
    assert!(!second_click(&mut last, 4, 11.3), "too late");
}
