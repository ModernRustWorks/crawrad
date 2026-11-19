//! **How a control paints its text and background**: the `Font` size
//! and the `Color`, read at run time.
//!
//! The generator knows, per class, where a control's text and background live
//! (the widget itself, or the label it wraps) — an unknown live field in
//! `apply_over` is logged as an error by makepad 1.0.0
//! (`live_traits.rs::apply_value_unknown`), so nothing is applied blind. A
//! size of 0 means *the theme's own*, and a colour with alpha 0 the parent's: nothing is
//! applied, and an unstyled form looks as it always did.
//!
//! Makepad 1.0.0 has no font loading by name, and bold or italic are fixed
//! `text_style` resources, so the size and the text colour are what reach
//! the screen here.
//!
//! # The text colour, per kind of widget
//!
//! Each widget's `draw_text` mixes the colour of its states (hover, focus,
//! pressed, checked), and they are different fields per widget. A colour
//! the user chose is the text's colour in every one of them except
//! *disabled*, which keeps the theme's: a disabled control has to look
//! disabled whatever its font says.

use crawrad_note::note;
use makepad_widgets::*;

#[note("look-font-size")]
pub fn font_size(cx: &mut Cx, widget: &WidgetRef, size: f32) {
    if size > 0.0 {
        let s = size as f64;
        widget.apply_over(cx, live! { draw_text: { text_style: { font_size: (s) } } });
    }
}

#[note("look-background")]
pub fn background(cx: &mut Cx, widget: &WidgetRef, rgba: [f32; 4], show_bg: bool) {
    if rgba[3] > 0.0 {
        let c = vec4(rgba[0], rgba[1], rgba[2], rgba[3]);
        if show_bg {
            widget.apply_over(cx, live! { show_bg: true, draw_bg: { color: (c) } });
        } else {
            widget.apply_over(cx, live! { draw_bg: { color: (c) } });
        }
    } else if show_bg {
        widget.apply_over(cx, live! { show_bg: false });
    }
}

/// The browser page: a form with no colour of its own, whose theme
/// is `thSystem`, is a white page when the app runs as WebAssembly — the
/// generator links makepad's light theme there, and this is its background.
pub fn light_page(cx: &mut Cx, ui: &WidgetRef) {
    let white = vec4(1.0, 1.0, 1.0, 1.0);
    ui.apply_over(cx, live! { pass: { clear_color: (white) } });
    // the body is a child of the window, reached by name and not as a field
    ui.widget(id!(body))
        .apply_over(cx, live! { draw_bg: { color: (white) } });
}

#[note("look-textstates")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextStates {
    /// A label: one colour.
    Plain,
    /// A button or a text box: normal, hover, focus and pressed.
    Pressable,
    /// A check box or a radio button: those, and checked.
    Toggle,
}

#[note("look-text-color")]
pub fn text_color(cx: &mut Cx, widget: &WidgetRef, rgba: [u8; 4], states: TextStates) {
    if rgba[3] == 0 {
        return;
    }
    let c = vec4(
        f32::from(rgba[0]) / 255.0,
        f32::from(rgba[1]) / 255.0,
        f32::from(rgba[2]) / 255.0,
        f32::from(rgba[3]) / 255.0,
    );
    match states {
        TextStates::Plain => widget.apply_over(cx, live! { draw_text: { color: (c) } }),
        TextStates::Pressable => widget.apply_over(
            cx,
            live! { draw_text: { color: (c), color_hover: (c), color_focus: (c), color_down: (c) } },
        ),
        TextStates::Toggle => widget.apply_over(
            cx,
            live! { draw_text: { color: (c), color_hover: (c), color_focus: (c), color_down: (c), color_active: (c) } },
        ),
    }
}
