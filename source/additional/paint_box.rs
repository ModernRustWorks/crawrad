//! # `PaintBox` — Canonical Reference Implementation
//!
//! Visual surface control designed for custom 2D painting via the `on_paint` event.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A canvas area control providing an `on_paint` event for custom procedural rendering.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "pbx",
    size = (105.0, 105.0),
    targets = ["*"]
)]
#[event(name = "on_paint")]
#[event(name = "on_mouse_down")]
#[event(name = "on_mouse_move")]
#[event(name = "on_mouse_up")]
pub struct PaintBox {
    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for PaintBox {
    fn default() -> Self {
        Self {
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for PaintBox {
    /// Design-time visual rendering on the IDE form surface.
    ///
    /// Renders a dashed guide border and an artist palette/brush icon indicating a custom canvas area.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Subtle surface background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 105.0, 105.0, 2.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 105.0, 105.0, 2.0);

        // 2. Palette icon and guide label
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_text_in_rect("🎨", 37.0, 30.0, 30.0, 30.0, TextAlign::Center);

        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("PaintBox", 10.0, 68.0, 85.0, 18.0, TextAlign::Center);
    }
}
