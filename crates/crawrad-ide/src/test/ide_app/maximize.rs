//! The gate over maximize — the window rectangle the taskbar is compared to.
//!
//! # What is left to test after the arithmetic went away
//!
//! The first version of this file measured a conversion: physical to logical,
//! client rect to window rect, frame added back. All of it existed because the
//! placing went through the framework's setters, and all of it went away with
//! them — [`crawrad_os::place_main_window`] takes the window rectangle in the
//! same physical pixels a work area is expressed in.
//!
//! What is left is the one reading that is still ours, and it is still a trap:
//! [`on_screen`] builds the window rectangle out of a `WindowGeom` whose
//! `position` is **physical** and whose `outer_size` is **logical**
//! (`win32_window.rs:906-912` against `:920-927`). Getting that wrong on a
//! scaled screen picks the wrong monitor and hands back the wrong rectangle on
//! restore.
//!
//! # The numbers are the owner's
//!
//! A 1920x1080 screen with a 1920x1032 work area — the taskbar is 48, measured
//! on his machine on 17/09/2026 — and, before that, a 1024x768 screen with a
//! 1024x720 work area and a window that came up 1044x788.

use crate::ide_app::maximize::{centre_in_physical_pixels, on_screen};
use crawrad_os::ScreenRect;
use makepad_widgets::makepad_platform::event::WindowGeom;
use makepad_widgets::*;

/// The sizing border, in physical pixels: 7 on each side, as measured.
const FRAME: f64 = 14.0;

/// The owner's work area: the screen minus the 48-pixel taskbar.
fn work_area() -> ScreenRect {
    ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1032.0,
    }
}

/// The whole monitor, taskbar included — what `SW_MAXIMIZE` fills.
fn whole_monitor() -> ScreenRect {
    ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
    }
}

/// A window on that screen, carrying the framework's own unit asymmetry.
///
/// `position` is physical and `outer_size` is logical, because that is what
/// `get_window_geom` produces (`win32_window.rs:872-884`). Reproducing the
/// asymmetry is the point: a reader that ignored it would pass a test that
/// ignored it too.
fn geom(dpi: f64) -> WindowGeom {
    let inner = dvec2(800.0, 600.0);
    WindowGeom {
        dpi_factor: dpi,
        position: dvec2(100.0, 80.0),
        inner_size: inner,
        outer_size: inner + dvec2(FRAME / dpi, FRAME / dpi),
        ..Default::default()
    }
}

/// **The window rectangle is physical, frame included, at any scale.**
///
/// The mutation this refuses is reading `outer_size` as if it were already
/// physical. At 100% it is indistinguishable; at 150% the window comes out two
/// thirds of its size, and the monitor lookup below starts answering for the
/// wrong screen.
#[test]
fn the_window_rectangle_is_physical_at_any_scale() {
    for dpi in [1.0, 1.25, 1.5, 2.0] {
        let g = geom(dpi);
        let r = on_screen(&g);
        assert_eq!(
            (r.x, r.y),
            (g.position.x, g.position.y),
            "at dpi {dpi} the position was converted — it is ALREADY physical"
        );
        assert_eq!(
            (r.width, r.height),
            (800.0 * dpi + FRAME, 600.0 * dpi + FRAME),
            "at dpi {dpi} the size is wrong: `outer_size` is logical and the \
             frame counts"
        );
    }
}

/// **A geometry with no DPI yet does not collapse the window.**
///
/// `WindowGeom::default()` carries `dpi_factor: 0.0`, and it is reachable — it
/// is the geometry of a window the platform has not described yet. Multiplying
/// by it would hand a zero-sized rectangle to `MoveWindow`.
#[test]
fn a_zero_dpi_is_treated_as_one_hundred_per_cent() {
    let g = WindowGeom {
        position: dvec2(10.0, 20.0),
        inner_size: dvec2(400.0, 300.0),
        outer_size: dvec2(414.0, 314.0),
        ..Default::default()
    };
    assert_eq!(
        g.dpi_factor, 0.0,
        "this is the default the framework derives"
    );
    let r = on_screen(&g);
    assert_eq!((r.x, r.y), (10.0, 20.0));
    assert_eq!(
        (r.width, r.height),
        (414.0, 314.0),
        "a zero DPI collapsed the window: {r:?}"
    );
}

/// **The centre names the monitor, and it is the centre of the FRAME.**
///
/// The corner would send a window dragged so its top-left sits on the
/// left-hand screen — while nearly all of it is on the right-hand one — to
/// maximize onto the wrong monitor.
#[test]
fn the_centre_is_the_middle_of_the_window() {
    let g = geom(1.0);
    let r = on_screen(&g);
    assert_eq!(
        centre_in_physical_pixels(&g),
        (r.x + r.width / 2.0, r.y + r.height / 2.0)
    );
    assert_ne!(
        centre_in_physical_pixels(&g),
        (g.position.x, g.position.y),
        "the centre became the corner"
    );
}

/// **A window that already fits the work area is left alone.**
///
/// The mutation this refuses is fitting unconditionally on startup, which
/// would shrink a deliberately sized window to full screen on every machine
/// with room to spare — and make the IDE impossible to open small.
#[test]
fn a_window_that_fits_is_not_touched() {
    let g = geom(1.0);
    assert!(
        on_screen(&g).fits_in(&work_area()),
        "an 814x614 window fits a 1920x1032 work area, so nothing should happen"
    );
}

/// **The window the owner photographed does NOT fit, by the pixels he counted.**
///
/// This is the startup half of the defect, measured: the DSL asks for
/// 1440x900, the window came up 1044x788 from the origin on a 1024x720 work
/// area, and 68 pixels of it — the status bar included — were below the usable
/// bottom.
#[test]
fn the_window_the_owner_photographed_does_not_fit_his_screen() {
    let pequena = ScreenRect {
        x: 0.0,
        y: 0.0,
        width: 1024.0,
        height: 720.0,
    };
    let g = WindowGeom {
        dpi_factor: 1.0,
        position: dvec2(0.0, 0.0),
        inner_size: dvec2(1044.0 - FRAME, 788.0 - FRAME),
        outer_size: dvec2(1044.0, 788.0),
        ..Default::default()
    };
    let grande = on_screen(&g);
    assert_eq!((grande.width, grande.height), (1044.0, 788.0));
    assert!(
        !grande.fits_in(&pequena),
        "this is the window he photographed; it MUST be seen as not fitting"
    );
    assert_eq!(
        grande.bottom() - pequena.bottom(),
        68.0,
        "the 68 pixels he counted"
    );
}

/// **The rectangle the framework's own maximize produces is the defect.**
///
/// `SW_MAXIMIZE` on a `WS_POPUP` fills the whole MONITOR. The assertion is not
/// that ours differs — any wrong answer differs — it is that the framework's
/// leaves the work area, and by exactly the taskbar.
#[test]
fn the_framework_maximize_is_the_one_that_covers_the_taskbar() {
    let monitor_inteiro = whole_monitor();
    assert!(
        !monitor_inteiro.fits_in(&work_area()),
        "the whole monitor does not fit the work area — that IS the defect"
    );
    assert_eq!(
        monitor_inteiro.bottom() - work_area().bottom(),
        48.0,
        "and the difference is the taskbar"
    );
    assert!(
        work_area().fits_in(&work_area()),
        "and what we ask for fits, by definition"
    );
}

/// **The Makepad shell responds to WindowDragQuery to allow dragging the window.**
///
/// Because the IDE hides the native Makepad `caption_bar` to draw the
/// custom `top_titlebar` with integrated menus and buttons, `handle_event`
/// intercepts `Event::WindowDragQuery` and responds with `WindowDragQueryResponse::Caption`.
#[test]
fn a_casca_responde_a_window_drag_query_no_handle_event() {
    let casca = super::casca::CASCA;
    assert!(
        casca.contains("Event::WindowDragQuery(dq)"),
        "handle_event MUST handle Event::WindowDragQuery"
    );
    assert!(
        casca.contains("WindowDragQueryResponse::Caption"),
        "the shell MUST respond with WindowDragQueryResponse::Caption for HTCAPTION on Windows"
    );
    assert!(
        casca.contains("point_in_window_drag"),
        "the shell MUST check ponto_no_arrasto_da_janela to isolate the bar buttons"
    );
}
