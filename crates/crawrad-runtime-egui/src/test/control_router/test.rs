//! The control event router (`control_router.rs`): the classic
//! rules, one by one, and layer filtering through real egui.

use super::*;

const PANEL: usize = 0;
const BUTTON: usize = 1;
const EDIT: usize = 2;

/// A panel with a button on it (drawn after it, so on top), and an edit apart.
fn rects() -> Vec<(usize, egui::Rect)> {
    vec![
        (
            PANEL,
            egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(200.0, 100.0)),
        ),
        (
            BUTTON,
            egui::Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(80.0, 24.0)),
        ),
        (
            EDIT,
            egui::Rect::from_min_size(egui::pos2(300.0, 0.0), egui::vec2(120.0, 24.0)),
        ),
    ]
}

/// Runs `raws` at `time` through `r`, focusable `[BUTTON, EDIT]`.
fn run(
    r: &mut CrawradControlRouter,
    raws: &[CrawradRaw],
    time: f64,
) -> (CrawradPointerHits, CrawradFocusHits) {
    let (mut pointer, mut focus) = (Vec::new(), Vec::new());
    for raw in raws {
        r.step(
            *raw,
            time,
            &rects(),
            &[BUTTON, EDIT],
            &mut pointer,
            &mut focus,
        );
    }
    (pointer, focus)
}

const PRIMARY: egui::PointerButton = egui::PointerButton::Primary;

#[test]
fn a_press_goes_to_the_topmost_control_with_its_local_position() {
    let mut r = CrawradControlRouter::default();
    let (pointer, _) = run(
        &mut r,
        &[CrawradRaw::Press(egui::pos2(15.0, 12.0), PRIMARY)],
        0.0,
    );
    assert_eq!(
        pointer,
        vec![(BUTTON, CrawradPointer::Down(PRIMARY), egui::vec2(5.0, 2.0))],
        "the button is drawn over the panel, so it takes the press"
    );
}

#[test]
fn press_and_release_on_the_same_control_is_a_click_and_the_release_follows_the_capture() {
    let mut r = CrawradControlRouter::default();
    let (pointer, _) = run(
        &mut r,
        &[
            CrawradRaw::Press(egui::pos2(15.0, 12.0), PRIMARY),
            CrawradRaw::Release(egui::pos2(15.0, 12.0), PRIMARY, true),
        ],
        0.0,
    );
    let kinds: Vec<_> = pointer.iter().map(|(c, k, _)| (*c, *k)).collect();
    assert_eq!(
        kinds,
        vec![
            (BUTTON, CrawradPointer::Down(PRIMARY)),
            (BUTTON, CrawradPointer::Up(PRIMARY)),
            (BUTTON, CrawradPointer::Click),
        ]
    );

    // released elsewhere: the Up still goes to the button, and no click
    let (pointer, _) = run(
        &mut r,
        &[
            CrawradRaw::Press(egui::pos2(15.0, 12.0), PRIMARY),
            CrawradRaw::Release(egui::pos2(150.0, 80.0), PRIMARY, true),
        ],
        5.0,
    );
    let kinds: Vec<_> = pointer.iter().map(|(c, k, _)| (*c, *k)).collect();
    assert_eq!(
        kinds,
        vec![
            (BUTTON, CrawradPointer::Down(PRIMARY)),
            (BUTTON, CrawradPointer::Up(PRIMARY)),
        ]
    );
}

#[test]
fn a_second_click_soon_enough_is_a_double_click() {
    let mut r = CrawradControlRouter::default();
    let click = [
        CrawradRaw::Press(egui::pos2(100.0, 80.0), PRIMARY),
        CrawradRaw::Release(egui::pos2(100.0, 80.0), PRIMARY, true),
    ];
    run(&mut r, &click, 1.0);
    let (pointer, _) = run(&mut r, &click, 1.3);
    assert!(pointer.contains(&(PANEL, CrawradPointer::DblClick, egui::vec2(100.0, 80.0))));

    // too late: a click again, never a double one
    let (pointer, _) = run(&mut r, &click, 3.0);
    assert!(
        !pointer
            .iter()
            .any(|(_, k, _)| *k == CrawradPointer::DblClick)
    );
}

#[test]
fn enter_and_leave_are_the_change_of_the_control_under_the_pointer() {
    let mut r = CrawradControlRouter::default();
    let (pointer, _) = run(
        &mut r,
        &[
            CrawradRaw::Move(egui::pos2(150.0, 80.0)),
            CrawradRaw::Move(egui::pos2(151.0, 80.0)),
            CrawradRaw::Move(egui::pos2(15.0, 12.0)),
            CrawradRaw::Gone,
        ],
        0.0,
    );
    let kinds: Vec<_> = pointer.iter().map(|(c, k, _)| (*c, *k)).collect();
    assert_eq!(
        kinds,
        vec![
            (PANEL, CrawradPointer::Enter),
            (PANEL, CrawradPointer::Move),
            (PANEL, CrawradPointer::Move),
            (PANEL, CrawradPointer::Leave),
            (BUTTON, CrawradPointer::Enter),
            (BUTTON, CrawradPointer::Move),
            (BUTTON, CrawradPointer::Leave),
        ]
    );
}

#[test]
fn a_press_moves_the_focus_only_to_a_control_that_can_take_it_and_the_keys_follow_it() {
    let mut r = CrawradControlRouter::default();
    let (_, focus) = run(
        &mut r,
        &[
            CrawradRaw::Press(egui::pos2(15.0, 12.0), PRIMARY),
            CrawradRaw::Key(egui::Key::A, true),
            CrawradRaw::Char('a'),
            CrawradRaw::Key(egui::Key::A, false),
            // the panel cannot take the focus: the button keeps it
            CrawradRaw::Press(egui::pos2(150.0, 80.0), PRIMARY),
            CrawradRaw::Press(egui::pos2(310.0, 5.0), PRIMARY),
        ],
        0.0,
    );
    assert_eq!(
        focus,
        vec![
            (BUTTON, CrawradFocus::Enter),
            (BUTTON, CrawradFocus::KeyDown(egui::Key::A)),
            (BUTTON, CrawradFocus::Char('a')),
            (BUTTON, CrawradFocus::KeyUp(egui::Key::A)),
            (BUTTON, CrawradFocus::Exit),
            (EDIT, CrawradFocus::Enter),
        ]
    );
}

/// **Tab moves the control focus as a press does**: egui focusing a
/// widget inside a control fires `Exit` and `Enter`, and the keys follow.
#[test]
fn egui_moving_its_focus_moves_the_control_focus() {
    let mut r = CrawradControlRouter::default();
    let (pointer, focus) = run(
        &mut r,
        &[
            CrawradRaw::Focused(egui::pos2(15.0, 12.0)),
            // the same control again: nothing
            CrawradRaw::Focused(egui::pos2(20.0, 14.0)),
            CrawradRaw::Focused(egui::pos2(310.0, 5.0)),
            CrawradRaw::Char('a'),
            // no control that can take the focus starts there: it stays
            CrawradRaw::Focused(egui::pos2(150.0, 80.0)),
            CrawradRaw::Focused(egui::pos2(900.0, 900.0)),
        ],
        0.0,
    );
    assert!(pointer.is_empty(), "{pointer:?}");
    assert_eq!(
        focus,
        vec![
            (BUTTON, CrawradFocus::Enter),
            (BUTTON, CrawradFocus::Exit),
            (EDIT, CrawradFocus::Enter),
            (EDIT, CrawradFocus::Char('a')),
        ]
    );
}

/// **Through egui itself**: a widget that takes egui's focus moves the
/// control focus on the frame it happens, and only a change of it does — a
/// press on another control is not undone by the widget that keeps egui's
/// focus.
#[test]
fn route_follows_a_change_of_eguis_focus() {
    let ctx = egui::Context::default();
    let mut r = CrawradControlRouter::default();
    let edit_id = egui::Id::new("edit");
    let frame = |events: Vec<egui::Event>, focus_edit: bool, r: &mut CrawradControlRouter| {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        };
        let mut out = Vec::new();
        ctx.run_ui(input, |ui| {
            let rect = rects()[EDIT].1;
            let resp = ui.interact(rect, edit_id, egui::Sense::focusable_noninteractive());
            if focus_edit {
                resp.request_focus();
            }
            out = r.route(ui, &rects(), &[BUTTON, EDIT]).1;
        })
        .textures_delta
        .clear();
        out
    };
    assert!(frame(Vec::new(), false, &mut r).is_empty());
    assert_eq!(
        frame(Vec::new(), true, &mut r),
        vec![(EDIT, CrawradFocus::Enter)]
    );
    // a press on the button takes the control focus; egui's stays on the
    // edit, and that is no change
    let press = egui::Event::PointerButton {
        pos: egui::pos2(15.0, 12.0),
        button: PRIMARY,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    };
    assert_eq!(
        frame(vec![press], false, &mut r),
        vec![(EDIT, CrawradFocus::Exit), (BUTTON, CrawradFocus::Enter)]
    );
    assert!(frame(Vec::new(), false, &mut r).is_empty());
}

#[test]
fn with_no_focus_the_keys_go_nowhere() {
    let mut r = CrawradControlRouter::default();
    let (_, focus) = run(&mut r, &[CrawradRaw::Key(egui::Key::A, true)], 0.0);
    assert!(focus.is_empty());
}

/// The routing through egui itself: the frame's events, read from a real
/// `Context`.
#[test]
fn route_reads_the_frames_events_from_egui() {
    let ctx = egui::Context::default();
    let mut r = CrawradControlRouter::default();
    let frame = |events: Vec<egui::Event>, r: &mut CrawradControlRouter| {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        };
        let mut out = (Vec::new(), Vec::new());
        // the font atlas a first frame uploads is not this test's business
        ctx.run_ui(input, |ui| {
            out = r.route(ui, &rects(), &[BUTTON, EDIT]);
        })
        .textures_delta
        .clear();
        out
    };
    let press = |pressed| egui::Event::PointerButton {
        pos: egui::pos2(15.0, 12.0),
        button: PRIMARY,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let (pointer, focus) = frame(
        vec![
            egui::Event::PointerMoved(egui::pos2(15.0, 12.0)),
            press(true),
            press(false),
            egui::Event::Text("x".to_string()),
        ],
        &mut r,
    );
    let kinds: Vec<_> = pointer.iter().map(|(c, k, _)| (*c, *k)).collect();
    assert_eq!(
        kinds,
        vec![
            (BUTTON, CrawradPointer::Enter),
            (BUTTON, CrawradPointer::Move),
            (BUTTON, CrawradPointer::Down(PRIMARY)),
            (BUTTON, CrawradPointer::Up(PRIMARY)),
            (BUTTON, CrawradPointer::Click),
        ]
    );
    assert_eq!(
        focus,
        vec![
            (BUTTON, CrawradFocus::Enter),
            (BUTTON, CrawradFocus::Char('x'))
        ]
    );
}

#[test]
fn the_wheel_goes_to_the_topmost_control_under_the_pointer() {
    let mut r = CrawradControlRouter::default();
    let (pointer, _) = run(&mut r, &[CrawradRaw::Wheel(egui::pos2(20.0, 20.0))], 0.0);
    assert_eq!(
        pointer,
        vec![(BUTTON, CrawradPointer::Wheel, egui::vec2(10.0, 10.0))]
    );
    let (pointer, _) = run(&mut r, &[CrawradRaw::Wheel(egui::pos2(250.0, 50.0))], 0.0);
    assert!(pointer.is_empty(), "{pointer:?}");
}

#[test]
fn a_control_is_resized_only_when_its_drawn_size_changes() {
    let mut w = CrawradSizeWatch::default();
    let small = egui::vec2(100.0, 50.0);
    // not drawn yet, then drawn: remembered, not announced
    assert!(w.resized(&[(PANEL, egui::Vec2::ZERO)]).is_empty());
    assert!(w.resized(&[(PANEL, small)]).is_empty());
    assert!(w.resized(&[(PANEL, small)]).is_empty());
    assert_eq!(w.resized(&[(PANEL, egui::vec2(120.0, 50.0))]), vec![PANEL]);
    assert!(w.resized(&[(PANEL, egui::vec2(120.0, 50.0))]).is_empty());
}
