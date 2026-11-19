//! The gate over the zoom of the whole interface.
//!
//! # What it can and cannot prove without a window
//!
//! The application half — `dpi_override`, the nudge, the pointer remap — needs
//! a `Cx` and a live window, and there is none here. What IS pure, and what
//! carries the behaviour the owner will judge, is the **ladder**: which step
//! comes next, where it stops, and that `100%` is a step you can land on
//! exactly.
//!
//! The wiring is charged by the wire gate in `fios.rs`, which reads the call.

use super::{DEGRAUS, UM, degrau_vizinho};

/// **`100%` is a step of the ladder, not a value between two.**
///
/// If it were not, `Ctrl 0` would land somewhere the `Ctrl +`/`Ctrl -` walk
/// could never reach, and the two keys would disagree about what *the system
/// size* is.
#[test]
fn one_hundred_per_cent_is_a_step() {
    assert!(
        DEGRAUS.contains(&UM),
        "`{UM}` is not in the ladder, so `Ctrl 0` lands off it: {DEGRAUS:?}"
    );
}

/// **The ladder climbs, and every step is distinct.**
///
/// A repeated step is a key press that does nothing, and an out-of-order one
/// makes `Ctrl +` shrink.
#[test]
fn the_ladder_only_climbs() {
    for par in DEGRAUS.windows(2) {
        assert!(
            par[1] > par[0],
            "the ladder went backwards at {par:?}: `Ctrl +` would shrink"
        );
    }
    assert!(DEGRAUS.len() >= 5, "a ladder of {} steps", DEGRAUS.len());
}

/// **Each step walks exactly one place, in the direction asked for.**
#[test]
fn a_step_walks_one_place() {
    for (i, degrau) in DEGRAUS.iter().enumerate() {
        if i + 1 < DEGRAUS.len() {
            assert_eq!(
                degrau_vizinho(*degrau, true),
                DEGRAUS[i + 1],
                "zoom in from {degrau} skipped a step"
            );
        }
        if i > 0 {
            assert_eq!(
                degrau_vizinho(*degrau, false),
                DEGRAUS[i - 1],
                "zoom out from {degrau} skipped a step"
            );
        }
    }
}

/// **The ends CLAMP, they do not wrap.**
///
/// The mutation this refuses is the modulo: someone holding `Ctrl -` at the
/// smallest step would suddenly get the biggest, which on a 200% jump is the
/// IDE appearing to break.
#[test]
fn the_ends_clamp_and_never_wrap() {
    let menor = DEGRAUS[0];
    let maior = DEGRAUS[DEGRAUS.len() - 1];
    assert_eq!(degrau_vizinho(menor, false), menor, "the small end wrapped");
    assert_eq!(degrau_vizinho(maior, true), maior, "the big end wrapped");
}

/// **A factor that is not on the ladder still walks it, from the nearest step.**
///
/// It is reachable: a session restored from a file, or a value a Settings
/// slider writes one day. Without the nearest-step search the walk would have
/// no index to start from, and the obvious fallback — start at zero — would
/// throw the interface to 50% on the first key press.
#[test]
fn a_factor_off_the_ladder_starts_from_the_nearest_step() {
    // 1.24 sits between 1.2 and 1.3, closer to 1.2.
    assert_eq!(
        degrau_vizinho(1.24, true),
        1.3,
        "1.24 should climb from 1.2"
    );
    assert_eq!(
        degrau_vizinho(1.24, false),
        1.1,
        "1.24 should fall from 1.2"
    );
    // And something wildly off the scale still answers with an end.
    assert_eq!(degrau_vizinho(99.0, true), DEGRAUS[DEGRAUS.len() - 1]);
    assert_eq!(degrau_vizinho(0.01, false), DEGRAUS[0]);
}

/// **Going in and straight back out returns to where it started.**
///
/// This is the property a user checks without meaning to: one `Ctrl +`, one
/// `Ctrl -`, and the IDE has to look exactly as before. A ladder built by
/// multiplying would fail it by rounding.
#[test]
fn in_and_out_comes_back_to_the_same_step() {
    for degrau in DEGRAUS.iter().skip(1).take(DEGRAUS.len() - 2) {
        assert_eq!(
            degrau_vizinho(degrau_vizinho(*degrau, true), false),
            *degrau,
            "in and out did not come back to {degrau}"
        );
    }
}

// ---------------------------------------------------------------------------
// THE POINTER — the half the framework does not do on this platform
// ---------------------------------------------------------------------------

/// **The pointer scale is the INVERSE of the factor, not the framework's.**
///
/// # The defect this test locks, and it was seen on screen
///
/// The owner, with the interface at 110%: *"side effect when zooming, the mouse
/// click loses precision, I click in one place and it selects another"*. Clicking
/// on `ListBox` lit up `PageControl`.
///
/// The first draft asked the framework's `remap_dpi_override` for the scale
/// (`window.rs:264-271`) — the same function macOS uses. However, it **only
/// answers when `os_dpi_factor` is set**, and that field is assigned
/// in only one place across the entire framework: `os/apple/macos/macos.rs:310`.
/// Measured: `os/windows/windows.rs` and `os/linux/x11/linux_x11.rs` mention it
/// **zero** times.
///
/// On Windows it is `None` forever, the function returns the position untouched, and
/// the correction **never happened** — the drawing grew and the click stayed where the
/// 100% drawing was.
///
/// # Why the correct calculation does not depend on that field
///
/// The `dpi_override` this shell writes is `base * fator`, and the position arrives
/// from win32 divided by `base` (`win32_window.rs:778-783`). What remains is
/// dividing by FACTOR — and the factor is ours, not the framework's.
#[test]
fn pointer_scale_is_inverse_of_factor() {
    let fonte = include_str!("../../ide_app/ui_zoom.rs");
    assert!(
        fonte.contains("let escala = 1.0 / self.zoom_da_ide;"),
        "pointer scale started coming from elsewhere again. If from \
         `remap_dpi_override`, it is a NO-OP on Windows and click misses \
         target at any scale other than 100%"
    );
    // THE NEEDLES ARE ASSEMBLED, and the reason has already bitten twice here: the
    // read module CITES both things in its own doc, to explain why they were
    // removed. A literal with the full name would cause the gate to fail due to
    // its own explanation.
    //
    // `concat!` resolves at compile time, and the form with PARENTHESIS only
    // appears in a call — never in prose.
    let chamada_do_framework = concat!("remap_dpi_", "override(");
    assert!(
        !fonte.contains(chamada_do_framework),
        "framework `remap` was called again. It only responds with the \
         macOS field populated, and on Windows is a no-op — click misses \
         target at any scale other than 100%"
    );
    // And desktop DPI also cannot come from that field.
    let campo_do_macos = concat!(".os_dpi_", "factor");
    assert!(
        !fonte.contains(campo_do_macos),
        "macOS DPI field was read again. It is `None` forever on \
         this platform, and reading it silently yields default"
    );
}

/// **The factor and pointer correction cancel out.**
///
/// The property a user verifies without meaning to: at 150% they click where they see.
/// A click at physical position `p` arrives as `p / base`, is multiplied by
/// the scale, and must land on the layout unit `p / (base * factor)`.
#[test]
fn correction_brings_click_back_to_design_unit() {
    for fator in [0.5, 0.9, 1.1, 1.25, 1.5, 2.0] {
        for base in [1.0, 1.25, 2.0] {
            let fisico = 900.0_f64;
            // What win32 delivers (`win32_window.rs:778-783`).
            let chega = fisico / base;
            // What this shell does with it.
            let corrigido = chega * (1.0 / fator);
            // Where the drawing actually is, with `dpi_override = base * fator`.
            let esperado = fisico / (base * fator);
            assert!(
                (corrigido - esperado).abs() < 1e-9,
                "at {fator} on a screen of {base}, click landed at \
                 {corrigido} and drawing is at {esperado}"
            );
        }
    }
}

/// **Every event family carrying position is scaled — including the ones
/// that do not look like pointer events.**
///
/// # The gap this gate closes, and it escaped once
///
/// The first list had six variants, all from `event/finger.rs`. The owner
/// found the missing one dropping a component from the Toolbox: *"look at the arrow,
/// where I dropped it and where the component went"* — `Event::Drop` lives in another file
/// (`event/drag_drop.rs`) and was not accounted for.
///
/// # The measurement, for whoever needs to redo it
///
/// `grep "pub abs: DVec2" event/*.rs` in `makepad-platform-1.0.0` answers
/// four files: `finger.rs` (13), `drag_drop.rs` (4), `window.rs` (1), and
/// `designer.rs` (1).
///
/// - from `finger.rs` only the SIX of `Event` are included; the other seven are `Hit::*`,
///   which `event.hits()` **derives** from the already corrected `Event`;
/// - from `drag_drop.rs`, the two of `Event` — the other two are `*HitEvent`,
///   derived the same way;
/// - from `window.rs`, the window drag query;
/// - `designer.rs` is Makepad's own designer tool, which this
///   shell does not use (ADR 0027: the canvas is our custom widget).
#[test]
fn every_event_family_with_position_is_scaled() {
    let fonte = include_str!("../../ide_app/ui_zoom.rs");
    const COM_POSICAO: &[&str] = &[
        // event/finger.rs
        "MouseDown",
        "MouseMove",
        "MouseUp",
        "MouseLeave",
        "Scroll",
        "LongPress",
        // event/drag_drop.rs — the family that escaped
        "Drag",
        "Drop",
        // event/window.rs
        "WindowDragQuery",
    ];
    for variante in COM_POSICAO {
        let braco = format!("Event::{variante}(e) => {{");
        assert!(
            fonte.contains(&braco),
            "`Event::{variante}` carries `abs` and is NOT scaled. With \
             magnified interface, this gesture lands `fator` times away from where \
             hand is — that is how Toolbox drop missed target"
        );
    }
    // AND EACH ONE SCALES, instead of just copying: an arm that cloned without modifying
    // `abs` would pass the test above and fail on screen.
    assert_eq!(
        fonte.matches("e.abs = escalar(e.abs);").count(),
        COM_POSICAO.len(),
        "number of scaling arms no longer matches number of \
         families with position"
    );
}
