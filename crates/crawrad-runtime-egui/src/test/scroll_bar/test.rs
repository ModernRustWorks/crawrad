//! The `ScrollBar` of the generated application: the `scroll_bar` section of
//! `desenho_comum.rs`, measured without a window — geometry and gestures.

use super::*;

/// A horizontal bar of 200×16, with its corner at (10, 10).
fn wide() -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(200.0, 16.0))
}

/// A vertical bar of 16×120.
fn tall() -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(16.0, 120.0))
}

#[test]
fn a_tall_bar_has_its_minimum_at_the_top() {
    let at_min = scroll_bar_parts(tall(), 0.0);
    let at_max = scroll_bar_parts(tall(), 1.0);
    assert!(!at_min.horizontal);
    assert_eq!(at_min.start.min, tall().min, "start button is at the top");
    assert_eq!(
        at_min.thumb.min.y, at_min.start.max.y,
        "at minimum, thumb touches top button"
    );
    assert_eq!(
        at_max.thumb.max.y, at_max.end.min.y,
        "at maximum, touches bottom button"
    );
}

#[test]
fn the_measures_are_the_designers() {
    // Those of the canvas (`crawrad-runtime-makepad/src/components/bar.rs`): 16 button,
    // 30% track thumb, 2 clearance.
    let p = scroll_bar_parts(wide(), 0.5);
    assert_eq!(p.start.width(), 16.0);
    assert_eq!(p.end.width(), 16.0);
    let (_, track) = p.track();
    assert_eq!(track, 200.0 - 32.0);
    assert!((p.thumb.width() - track * 0.3).abs() < 0.01);
    assert_eq!(p.thumb.min.y, 12.0);
    assert_eq!(p.thumb.height(), 12.0);
}

/// The thumb never leaves the component at any size — the panic that
/// `clamp(8.0, pista)` previously caused (`a_barra_de_rolagem_aguenta_qualquer_tamanho`).
#[test]
fn nothing_of_the_bar_paints_outside_it() {
    for w in 0..=40 {
        for h in 0..=40 {
            let r = egui::Rect::from_min_size(egui::pos2(5.0, 5.0), egui::vec2(w as f32, h as f32));
            for f in [0.0, 0.5, 1.0] {
                let p = scroll_bar_parts(r, f);
                for part in [p.start, p.end, p.thumb] {
                    assert!(
                        part.min.x >= r.min.x - 0.01
                            && part.min.y >= r.min.y - 0.01
                            && part.max.x <= r.max.x + 0.01
                            && part.max.y <= r.max.y + 0.01,
                        "at {w}x{h} ({f}) {part:?} escapes {r:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_fraction_runs_from_min_to_max() {
    assert_eq!(scroll_bar_fraction(0, 0, 100), 0.0);
    assert_eq!(scroll_bar_fraction(100, 0, 100), 1.0);
    assert_eq!(scroll_bar_fraction(15, 10, 20), 0.5);
    assert_eq!(
        scroll_bar_fraction(-5, 0, 100),
        0.0,
        "out of range, at the boundary"
    );
    assert_eq!(
        scroll_bar_fraction(7, 7, 7),
        0.0,
        "empty range does not divide by zero"
    );
}

// ------------------------------------------------------------------ gestures

/// Runs a frame with the bar at `rect` and returns whether it changed.
fn frame(
    ctx: &egui::Context,
    rect: egui::Rect,
    events: Vec<egui::Event>,
    position: &mut i64,
    enabled: bool,
) -> bool {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 300.0),
        )),
        events,
        ..Default::default()
    };
    let mut changed = false;
    let mut output = ctx.run_ui(input, |ui| {
        changed = scroll_bar(ui, "sb", rect, position, (0, 100), (1, 10), enabled);
    });
    // No renderer here: the font atlas upload is dropped on purpose.
    output.textures_delta.clear();
    changed
}

fn press(p: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(p),
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

fn release(p: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: p,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    }]
}

/// Presses and releases at `p`, starting from `position`; returns the final position.
fn click_at(p: egui::Pos2, position: i64, enabled: bool) -> i64 {
    let ctx = egui::Context::default();
    let mut pos = position;
    frame(&ctx, wide(), vec![], &mut pos, enabled);
    frame(&ctx, wide(), press(p), &mut pos, enabled);
    frame(&ctx, wide(), release(p), &mut pos, enabled);
    pos
}

#[test]
fn the_buttons_step_one_small_change() {
    let start = wide().min + egui::vec2(8.0, 8.0);
    let end = egui::pos2(wide().max.x - 8.0, wide().center().y);
    assert_eq!(click_at(end, 50, true), 51);
    assert_eq!(click_at(start, 50, true), 49);
    assert_eq!(click_at(start, 0, true), 0, "does not go below minimum");
}

#[test]
fn the_track_jumps_one_large_change_towards_the_click() {
    let p = scroll_bar_parts(wide(), 0.5);
    let before = egui::pos2(p.thumb.min.x - 10.0, wide().center().y);
    let after = egui::pos2(p.thumb.max.x + 10.0, wide().center().y);
    assert_eq!(click_at(after, 50, true), 60);
    assert_eq!(click_at(before, 50, true), 40);
}

#[test]
fn the_thumb_drags() {
    let ctx = egui::Context::default();
    let mut pos = 0;
    let p = scroll_bar_parts(wide(), 0.0);
    let grab = p.thumb.center();
    frame(&ctx, wide(), vec![], &mut pos, true);
    frame(&ctx, wide(), press(grab), &mut pos, true);
    assert_eq!(pos, 0, "grabbing thumb does not move it");
    // Dragging to the end of the track reaches the maximum.
    let far = egui::pos2(wide().max.x + 50.0, grab.y);
    let changed = frame(
        &ctx,
        wide(),
        vec![egui::Event::PointerMoved(far)],
        &mut pos,
        true,
    );
    assert!(changed);
    assert_eq!(pos, 100);
    frame(&ctx, wide(), release(far), &mut pos, true);
}

#[test]
fn a_disabled_bar_does_not_move() {
    let end = egui::pos2(wide().max.x - 8.0, wide().center().y);
    assert_eq!(click_at(end, 50, false), 50);
}

#[test]
fn the_keyboard_moves_it_when_focused() {
    let ctx = egui::Context::default();
    let mut pos = 50;
    let end = egui::pos2(wide().max.x - 8.0, wide().center().y);
    frame(&ctx, wide(), vec![], &mut pos, true);
    frame(&ctx, wide(), press(end), &mut pos, true);
    frame(&ctx, wide(), release(end), &mut pos, true);
    assert_eq!(pos, 51);
    let key = |k: egui::Key| {
        vec![egui::Event::Key {
            key: k,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]
    };
    frame(&ctx, wide(), key(egui::Key::PageDown), &mut pos, true);
    assert_eq!(pos, 61);
    frame(&ctx, wide(), key(egui::Key::ArrowLeft), &mut pos, true);
    assert_eq!(pos, 60);
    frame(&ctx, wide(), key(egui::Key::End), &mut pos, true);
    assert_eq!(pos, 100);
    frame(&ctx, wide(), key(egui::Key::Home), &mut pos, true);
    assert_eq!(pos, 0);
}
