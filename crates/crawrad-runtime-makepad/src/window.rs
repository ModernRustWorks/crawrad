//! **A form whose `Position` is `poScreenCenter` opens centred**.
//!
//! Makepad 1.0 opens a window where the system puts it (`CW_USEDEFAULT`,
//! `makepad-platform-1.0.0/src/os/windows/win32_window.rs:336`) and publishes
//! no screen size: `DisplayContext.screen_size` is the window's, and
//! `WindowGeom` carries the window's position and sizes only
//! (`src/event/window.rs:12-21`). The monitor comes from `crawrad-os`, the one
//! crate that may ask the system, taken without its default features — the
//! generated app compiles `monitor_at` and nothing else of it.
//!
//! The units are Makepad's own: `WindowGeom.position` is in physical pixels
//! (`GetWindowRect`, `win32_window.rs:906-911`), `outer_size` is logical
//! (`:923-929`), and `RepositionWindow` takes a logical point that it
//! multiplies by the DPI factor (`:932-946`).

use crawrad_note::note;
use crawrad_os::ScreenRect;
use makepad_widgets::*;

#[note("window-centred-in")]
pub fn centred_in(work: ScreenRect, outer: DVec2, dpi: f64) -> DVec2 {
    let dpi = if dpi > 0.0 { dpi } else { 1.0 };
    let (w, h) = (outer.x * dpi, outer.y * dpi);
    let x = work.x + ((work.width - w) / 2.0).max(0.0);
    let y = work.y + ((work.height - h) / 2.0).max(0.0);
    dvec2(x / dpi, y / dpi)
}

#[note("window-centre")]
pub fn centre(cx: &mut Cx, e: &WindowGeomChangeEvent) -> bool {
    let g = &e.new_geom;
    let Some(monitor) = crawrad_os::monitor_at((g.position.x, g.position.y)) else {
        return false;
    };
    let at = centred_in(monitor.work, g.outer_size, g.dpi_factor);
    cx.push_unique_platform_op(CxOsOp::RepositionWindow(e.window_id, at));
    true
}

pub fn on_screen(geom: &makepad_widgets::makepad_platform::event::WindowGeom) -> ScreenRect {
    let (p, s) = (geom.position, geom.outer_size);
    ScreenRect::of_window([p.x, p.y], [s.x, s.y], geom.dpi_factor)
}

pub fn centre_in_physical_pixels(
    geom: &makepad_widgets::makepad_platform::event::WindowGeom,
) -> (f64, f64) {
    let r = on_screen(geom);
    (r.x + r.width / 2.0, r.y + r.height / 2.0)
}

#[cfg(test)]
#[path = "test/window/test.rs"]
mod tests;
