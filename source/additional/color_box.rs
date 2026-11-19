//! # `ColorBox` — Canonical Reference Implementation
//!
//! A drop-down of the standard colours, each with its swatch:
//! `selected` is the chosen colour, shown at the left of the field.

use crawrad_component::Color;
use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A colour picker over the standard colours: `selected` is the choice.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "clb",
    size = (145.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct ColorBox {
    /// The chosen colour.
    #[prop(display = "Selected", category = "appearance")]
    pub selected: Color,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ColorBox {
    fn default() -> Self {
        Self {
            selected: Color::default(),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ColorBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The field
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, 145.0, 24.0);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.0, 0.0, 145.0, 24.0);

        // 2. The swatch of the chosen colour
        canvas.set_fill_style(self.selected);
        canvas.fill_rect(4.0, 5.0, 14.0, 14.0);
        canvas.stroke_rect(4.0, 5.0, 14.0, 14.0);

        // 3. The list's drop arrow
        canvas.begin_path();
        canvas.move_to(129.0, 10.0);
        canvas.line_to(133.0, 14.0);
        canvas.line_to(137.0, 10.0);
        canvas.stroke();
    }
}
