//! **The Glyph of a button**: a picture left of the caption, the
//! first of `NumGlyphs` side by side in it.
//!
//! A makepad `Button` draws no bitmap of its own, so the generated button
//! has a sibling view `<field>_glyph` over its left edge, clipped to one
//! glyph, with the picture's `img` inside — the button keeps every click.
//! This shows the picture there (decoded once per picture by
//! [`crate::picture::show`]) and makes room for it in the caption.

use crawrad_note::note;
use makepad_widgets::*;

#[note("glyph-side")]
pub const SIDE: f64 = 16.0;

#[note("glyph-shown")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Shown {
    picture: crate::picture::Shown,
    slot: Option<(bool, i64)>,
}

#[note("glyph-show")]
pub fn show(
    cx: &mut Cx,
    ui: &WidgetRef,
    path: &[LiveId],
    button: &WidgetRef,
    bytes: &[u8],
    num_glyphs: i64,
    visible: bool,
    shown: &mut Shown,
) {
    crate::picture::show(cx, ui, path, bytes, &mut shown.picture);
    let has = visible && crate::picture::kind_of(bytes).is_some();
    let n = num_glyphs.max(1);
    if shown.slot == Some((has, n)) {
        return;
    }
    shown.slot = Some((has, n));
    let glyph = ui.widget(path);
    glyph.set_visible(cx, has);
    let strip = SIDE * n as f64;
    glyph
        .image(id!(img))
        .apply_over(cx, live! { width: (strip), height: (SIDE) });
    // the whole padding: a partial one leaves the other sides at zero and
    // the caption slides up out of the box
    let left = if has { SIDE + 10.0 } else { 6.0 };
    button.apply_over(
        cx,
        live! { padding: { left: (left), right: 6.0, top: 0.0, bottom: 0.0 } },
    );
}
