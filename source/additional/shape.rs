//! # `Shape` — Canonical Reference Implementation
//!
//! Decorative 2D geometric shape control rendering rectangles, rounded rectangles,
//! squares, ellipses, and circles with configurable stroke pen and fill brush.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-windowed graphic control rendering geometric shapes.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "shp",
    size = (64.0, 64.0),
    targets = ["*"]
)]
pub struct Shape {
    /// Geometric shape kind ("Rectangle", "Square", "RoundRect", "RoundSquare", "Ellipse", "Circle").
    #[prop(display = "Shape", category = "appearance", default = "Rectangle")]
    pub shape: String,

    /// Width of the outline pen in pixels.
    #[prop(display = "PenWidth", category = "appearance", default = 1, min = 0)]
    pub pen_width: i64,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the control.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            shape: String::from("Rectangle"),
            pen_width: 1,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Shape {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(self.pen_width.max(1) as f32);

        let w = 64.0;
        let h = 64.0;
        let p = 1.0;

        match self.shape.as_str() {
            "Square" | "Rectangle" => {
                canvas.fill_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, 0.0);
                canvas.stroke_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, 0.0);
            }
            "RoundSquare" | "RoundRect" => {
                let radius = 8.0;
                canvas.fill_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, radius);
                canvas.stroke_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, radius);
            }
            "Circle" | "Ellipse" => {
                let radius = (w - 2.0 * p).min(h - 2.0 * p) / 2.0;
                canvas.fill_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, radius);
                canvas.stroke_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, radius);
            }
            _ => {
                canvas.fill_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, 0.0);
                canvas.stroke_round_rect(p, p, w - 2.0 * p, h - 2.0 * p, 0.0);
            }
        }
    }
}
