//! Utilities, geometry, and shared color palette for Makepad components.
//!
//! # The palette is by ROLE, and the role is that of RXP
//!
//! The visual contract here is `crawrad-runtime-egui` (the RXP), and behind
//! it a classic RAD IDE. However, RXP paints on a LIGHT paper (`PAPEL`,
//! `#F0F0F0`) and this canvas paints on a DARK paper
//! ([`MAKEPAD_PAPER`], `#252538`) — copying the hex from there would yield black
//! text on near-black. ADR 0013 already answers this: the shared
//! drawing *"does not freeze any color"*.
//!
//! So what is mirrored is the **role the color plays**, and the mirror is
//! measured: each token here points to the RXP token of which it is the dark
//! equivalent, and `src/test/theme/test.rs` enforces the SAME contrast ratios that
//! `o_contorno_diz_onde_o_controle_esta` of RXP enforces there
//! (`crawrad-runtime-egui/src/tests.rs:2535`).
//!
//! # The value hierarchy, which is what the owner rejected
//!
//! The form read as *"a collection of holes"* because button, field,
//! panel, and group were the same dark borderless rectangle. The convention dates to
//! 1995 and a classic RAD IDE uses it: **the button RISES** (face lighter than paper) and **the
//! field SINKS** (background darker than paper), and both have borders. The
//! order `MAKEPAD_WIDGET > MAKEPAD_PAPER > MAKEPAD_WIDGET_INPUT` is what expresses
//! this, and a test fails if it flattens again.

use crawrad_ir::Rect;
use crawrad_note::note;

#[note("components-comum-rgb")]
pub const fn rgb(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
}

// --------------------------------------------------------------- the canvas

#[note("components-comum-makepad-bg")]
pub const MAKEPAD_BG: [f32; 4] = rgb(0x1D, 0x1F, 0x20);

#[note("components-comum-makepad-paper")]
pub const MAKEPAD_PAPER: [f32; 4] = rgb(0x25, 0x25, 0x38);

#[note("components-comum-makepad-grid")]
pub const MAKEPAD_GRID: [f32; 4] = rgb(0x7A, 0x7F, 0x9A);

#[note("components-comum-makepad-grid-faint")]
pub const MAKEPAD_GRID_FAINT: [f32; 4] = rgb(0x52, 0x55, 0x6B);

#[note("components-comum-makepad-snap-pointer")]
pub const MAKEPAD_SNAP_POINTER: [f32; 4] = rgb(0x9A, 0x9E, 0xB5);

#[note("components-comum-makepad-snap")]
pub const MAKEPAD_SNAP: [f32; 4] = rgb(0xDE, 0x5A, 0x2B);

/// The form checker's marks, fill then ink, as in the approved mockup.
pub const MAKEPAD_PIN_ERROR: [[f32; 4]; 2] = [rgb(0xE5, 0x53, 0x4B), rgb(0xFF, 0xFF, 0xFF)];
pub const MAKEPAD_PIN_WARNING: [[f32; 4]; 2] = [rgb(0xD2, 0x99, 0x22), rgb(0x1B, 0x1D, 0x1F)];
pub const MAKEPAD_TAB_BADGE: [[f32; 4]; 2] = [rgb(0x2F, 0x4D, 0x6E), rgb(0xCF, 0xE3, 0xFA)];
pub const MAKEPAD_TAB_CLASH: [[f32; 4]; 2] = [rgb(0x6E, 0x3A, 0x1C), rgb(0xFB, 0xD5, 0xB5)];

/// The corner badge of a component that comes from the base form.
pub const MAKEPAD_INHERITED: [f32; 4] = MAKEPAD_SNAP;

#[note("components-comum-snap-button-look")]
pub const fn snap_button_look(on: bool) -> [[f32; 4]; 3] {
    if on {
        [rgb(0x3B, 0x2A, 0x22), MAKEPAD_SNAP, rgb(0xFF, 0xB3, 0x8A)]
    } else {
        [
            rgb(0x2B, 0x2B, 0x2D),
            rgb(0x4D, 0x4F, 0x52),
            rgb(0xE7, 0xE7, 0xEA),
        ]
    }
}

// ------------------------------------------- the controls INSIDE the paper

#[note("components-comum-makepad-widget")]
pub const MAKEPAD_WIDGET: [f32; 4] = crate::shared_theme::ROLE_SURFACE;

#[note("components-comum-makepad-widget-input")]
pub const MAKEPAD_WIDGET_INPUT: [f32; 4] = rgb(0x18, 0x18, 0x25);

#[note("components-comum-makepad-border")]
pub const MAKEPAD_BORDER: [f32; 4] = crate::shared_theme::ROLE_BORDER;

#[note("components-comum-makepad-panel")]
pub const MAKEPAD_PANEL: [f32; 4] = rgb(0x2F, 0x2F, 0x45);

#[note("components-comum-makepad-panel-border")]
pub const MAKEPAD_PANEL_BORDER: [f32; 4] = rgb(0x6E, 0x72, 0x90);

#[note("components-comum-makepad-selection")]
pub const MAKEPAD_SELECTION: [f32; 4] = rgb(0x3A, 0x5A, 0x8C);

#[note("components-comum-makepad-text")]
pub const MAKEPAD_TEXT: [f32; 4] = crate::shared_theme::ROLE_TEXT;

#[note("components-comum-makepad-text-dim")]
pub const MAKEPAD_TEXT_DIM: [f32; 4] = crate::shared_theme::ROLE_TEXT_DIM;

// ------------------------------------------------------------- the ruler
//
// It is canvas **chrome**, not part of the form: light enough so
// numbers can be read over it, dark enough not to compete with the paper —
// what should catch the eye is the drawing, not its frame. This is the rationale
// RXP documented (`crawrad-runtime-egui/src/lib.rs:191-194`), and the four tokens
// below mirror the four there by ROLE, not by hex: there the band
// is one step LIGHTER than its dark surface, and here as well.

#[note("components-comum-makepad-regua-fundo")]
pub const MAKEPAD_REGUA_FUNDO: [f32; 4] = rgb(0x2B, 0x2B, 0x3E);

#[note("components-comum-makepad-regua-traco")]
pub const MAKEPAD_REGUA_TRACO: [f32; 4] = rgb(0x8A, 0x8F, 0xA8);

#[note("components-comum-makepad-regua-texto")]
pub const MAKEPAD_REGUA_TEXTO: [f32; 4] = rgb(0xC8, 0xCE, 0xE4);

#[note("components-comum-makepad-regua-ponteiro")]
pub const MAKEPAD_REGUA_PONTEIRO: [f32; 4] = MAKEPAD_ACCENT;

// ------------------------------------------------- the designer ornaments

#[note("components-comum-makepad-accent")]
pub const MAKEPAD_ACCENT: [f32; 4] = crate::shared_theme::ROLE_ACCENT;

#[note("components-comum-makepad-handle")]
pub const MAKEPAD_HANDLE: [f32; 4] = MAKEPAD_ACCENT;

// The designer's alignment guides. They come from here, not from drawing, because colors
// on the canvas are always tokens: the shell has tests that reject hardcoded colors.
pub const MAKEPAD_GUIA_COMPONENTE: [f32; 4] = [0.98, 0.72, 0.28, 0.9]; // Component guide (amber)
pub const MAKEPAD_GUIA_TEXTO: [f32; 4] = [0.95, 0.45, 0.75, 0.9]; // Text baseline guide (pink)
pub const MAKEPAD_GUIA_ESPACO: [f32; 4] = [0.35, 0.65, 0.98, 0.5]; // Equal spacing guide (blue)

// Legacy aliases for backward compatibility during refactoring
pub const MAKEPAD_FUNDO: [f32; 4] = MAKEPAD_BG;
pub const MAKEPAD_PAPEL: [f32; 4] = MAKEPAD_PAPER;
pub const MAKEPAD_BORDA: [f32; 4] = MAKEPAD_BORDER;
pub const MAKEPAD_GRADE: [f32; 4] = MAKEPAD_GRID;
pub const MAKEPAD_TEXTO: [f32; 4] = MAKEPAD_TEXT;
pub const MAKEPAD_TEXTO_DIM: [f32; 4] = MAKEPAD_TEXT_DIM;
pub const MAKEPAD_ACENTO: [f32; 4] = MAKEPAD_ACCENT;
pub const MAKEPAD_ALCA: [f32; 4] = MAKEPAD_HANDLE;

/// THE LIGHT PAGE (a web project, or a form whose `Theme` is `thLight`):
/// each role of the dark palette above, taken to its light twin, so the canvas
/// paints what the generated app paints with makepad's light theme on a white
/// page. A colour that is not a role of the form (the ruler, the handles, the
/// accent, the marks of the designer) comes back as it went.
pub fn light_page_ink(c: [f32; 4]) -> [f32; 4] {
    const PAIRS: &[([f32; 4], [f32; 4])] = &[
        (MAKEPAD_PAPER, rgb(0xFF, 0xFF, 0xFF)),
        (MAKEPAD_WIDGET, rgb(0xE6, 0xE6, 0xEB)),
        (MAKEPAD_WIDGET_INPUT, rgb(0xFF, 0xFF, 0xFF)),
        (MAKEPAD_BORDER, rgb(0x9A, 0x9C, 0xA8)),
        (MAKEPAD_PANEL, rgb(0xF3, 0xF3, 0xF6)),
        (MAKEPAD_PANEL_BORDER, rgb(0xC4, 0xC6, 0xD0)),
        (MAKEPAD_TEXT, rgb(0x1E, 0x1F, 0x26)),
        (MAKEPAD_TEXT_DIM, rgb(0x5A, 0x5C, 0x68)),
        (MAKEPAD_SELECTION, rgb(0xCC, 0xE0, 0xFF)),
    ];
    PAIRS
        .iter()
        .find(|(dark, _)| *dark == c)
        .map_or(c, |(_, light)| *light)
}

#[note("components-comum-rect-min-max")]
pub fn rect_min_max(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}

#[note("components-comum-safe-shrink")]
pub fn safe_shrink(r: Rect, d: f32) -> Rect {
    if r.w <= d * 2.0 || r.h <= d * 2.0 {
        r
    } else {
        Rect::new(r.x + d, r.y + d, r.w - d * 2.0, r.h - d * 2.0)
    }
}

#[note("components-comum-entre")]
pub fn entre(v: f64, piso: f64, teto: f64) -> f64 {
    v.max(piso).min(teto)
}

#[note("components-comum-sem-acelerador")]
pub fn sem_acelerador(t: &str) -> String {
    let mut saida = String::with_capacity(t.len());
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                saida.push('&');
            }
            continue;
        }
        saida.push(c);
    }
    saida
}

#[note("components-comum-indice-do-acelerador")]
pub fn indice_do_acelerador(t: &str) -> Option<usize> {
    let mut i = 0usize;
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                i += 1;
                continue;
            }
            return chars.peek().map(|_| i);
        }
        i += c.len_utf8();
    }
    None
}

// The non-visual badge (`ImageList`, `ActionList`, `Timer`, `Action`). The
// four values are those of RXP (`crawrad-runtime-egui/src/lib.rs:182-185`), and the
// test `o_selo_do_nao_visual_espelha_as_cores_do_rxp` rejects divergence: the
// same document opened in both runtimes must look the same, otherwise the
// owner sees two different CrawRADs.
//
// Note that the badge is LIGHT on the dark canvas, unlike the rest
// of the palette: it is not a form control, it is a designer
// tag, and in a classic RAD IDE it also stands out from what is drawn around it.
pub const MAKEPAD_NAO_VISUAL_FUNDO: [f32; 4] = rgb(0xD0, 0xE0, 0xE8);
pub const MAKEPAD_NAO_VISUAL_BORDA: [f32; 4] = rgb(0x40, 0x60, 0x80);
pub const MAKEPAD_NAO_VISUAL_TEXTO: [f32; 4] = rgb(0x20, 0x40, 0x60);

// The instance NAME is the only one of the four colors that does NOT copy RXP, and the
// reason is fundamental, not taste: the top three paint the BADGE, which carries its
// own light background and therefore reads over any canvas; the name is written
// directly onto the form's PAPER. RXP's paper is light and this one is dark
// (`MAKEPAD_PAPER`, #252538) — #404040 from there would end up black on near-black,
// unreadable. What the name IS in both is "secondary text", and the token serving
// that role in this theme is `MAKEPAD_TEXT_DIM`.
pub const MAKEPAD_NOME_NAO_VISUAL: [f32; 4] = MAKEPAD_TEXT_DIM;

#[cfg(test)]
#[path = "../test/theme/test.rs"]
mod tests;
