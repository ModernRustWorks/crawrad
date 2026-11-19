//! # `MainMenu` — Canonical Reference Implementation
//!
//! Non-visual component managing the top-level window menu bar and drop-down submenus.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual component defining the primary horizontal menu bar of a window.
#[derive(Default, Component, Clone, Debug, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "mnu",
    size = (28.0, 28.0),
    children = "items",
    targets = ["*"]
)]
pub struct MainMenu {
    /// Optional reference to an `ImageList` providing icons for menu items.
    #[prop(display = "Images", category = "appearance", default = "")]
    pub images: String,
}

impl Paint for MainMenu {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    ///
    /// Renders the classic RAD MainMenu visual metaphor: a top menu bar with drop-down sheets.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Menu bar strip at top (y=5 to y=10)
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(4.0, 5.0, 20.0, 5.0, 1.0);

        // 3. Drop-down menu preview sheet
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.stroke_round_rect(5.0, 12.0, 12.0, 11.0, 1.0);

        // 4. Menu item lines inside sheet
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_rect(7.0, 14.0, 8.0, 1.0);
        canvas.fill_rect(7.0, 17.0, 8.0, 1.0);
        canvas.fill_rect(7.0, 20.0, 8.0, 1.0);
    }
}
