//! # `PopupMenu` — Canonical Reference Implementation
//!
//! Non-visual component managing contextual right-click popup menus.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual component defining a floating context menu displayed upon right-clicking.
#[derive(Default, Component, Clone, Debug, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "pop",
    size = (28.0, 28.0),
    children = "items",
    targets = ["*"]
)]
pub struct PopupMenu {
    /// Optional reference to an `ImageList` providing icons for menu items.
    #[prop(display = "Images", category = "appearance", default = "")]
    pub images: String,
}

impl Paint for PopupMenu {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    ///
    /// Renders the classic RAD PopupMenu visual metaphor: a floating contextual menu with mouse pointer.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Floating context menu sheet (14x16 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Accent);
        canvas.fill_round_rect(4.0, 4.0, 14.0, 18.0, 2.0);
        canvas.stroke_round_rect(4.0, 4.0, 14.0, 18.0, 2.0);

        // 3. Menu item stripes
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_rect(7.0, 8.0, 8.0, 2.0);
        canvas.fill_rect(7.0, 12.0, 8.0, 2.0);
        canvas.fill_rect(7.0, 16.0, 8.0, 2.0);

        // 4. Mouse pointer click indicator
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_rect(17.0, 13.0, 4.0, 4.0);
    }
}
