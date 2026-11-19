// THE FIVE COLOUR ROLES — one file, two readers.
//
// This file is compiled HERE, as a module of `crawrad-runtime-makepad`, which is
// what paints the designer canvas. And it is compiled AGAIN inside
// `crawrad-codegen`, by an `include!` on the path, which turns each value into the
// `#RRGGBB` the generated application's `live_design!` carries.
//
// It is the sibling of `shared_geometry.rs`, and it exists for the same reason:
// what the designer shows has to be what the application shows. There the
// divergence was a row that measured 18 on the canvas and 20 in the
// application; here it would be a third-party component painted in one colour
// on the canvas and another on screen.
//
// ## Why ROLES and not colours
//
// A declared drawing (ADR 0036) cannot name a colour. It names a ROLE —
// *surface*, *border*, *text*, *text dim*, *accent* — and whoever paints
// resolves it. That is what lets a component published on crates.io follow the
// theme of whoever uses it instead of fighting it, and it is why there is no
// colour field in a `manifest.json`.
//
// ## The authoring rules
//
// The same three `shared_geometry.rs` carries:
//
// 1. **No `use`.** The file is included in two crates whose imports differ.
// 2. **No type of ours.** `f32` and nothing else.
// 3. **Pure.** Everything here is a constant or arithmetic over arguments.

/// An opaque 8-bit-per-channel colour as the `[f32; 4]` a shader wants.
///
/// The token is written with the SAME number the egui runtime writes, because
/// comparing `0xD0` against `0.816` by hand is an invitation to drift.
pub const fn role_rgb(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
}

/// The face of a component — its background.
pub const ROLE_SURFACE: [f32; 4] = role_rgb(0x3B, 0x3B, 0x54);

/// Its outline.
pub const ROLE_BORDER: [f32; 4] = role_rgb(0x92, 0x97, 0xB5);

/// The main text.
pub const ROLE_TEXT: [f32; 4] = role_rgb(0xCD, 0xD6, 0xF4);

/// Secondary text — a caption, a hint, whatever matters less.
pub const ROLE_TEXT_DIM: [f32; 4] = role_rgb(0xA6, 0xAD, 0xC8);

/// The house accent: selection, focus, whatever asks to be looked at.
pub const ROLE_ACCENT: [f32; 4] = role_rgb(0x89, 0xB4, 0xFA);
