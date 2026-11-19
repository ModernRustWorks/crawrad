//! # `Bevel` — Canonical Reference Implementation
//!
//! A sunk or raised box, frame or line that groups controls visually:
//! a shadow line and a light line, swapped by `style`.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A graphic control: no focus, no events — only the lines.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "bvl",
    size = (50.0, 50.0),
    targets = ["*"]
)]
pub struct Bevel {
    /// What is drawn ("Box", "Frame", "TopLine", "BottomLine", "LeftLine",
    /// "RightLine", "Spacer").
    #[prop(display = "Shape", category = "appearance", default = "Box")]
    pub bevel_shape: String,

    /// "Lowered" (shadow on the top and left) or "Raised".
    #[prop(display = "Style", category = "appearance", default = "Lowered")]
    pub bevel_style: String,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the control.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Bevel {
    fn default() -> Self {
        Self {
            bevel_shape: String::from("Box"),
            bevel_style: String::from("Lowered"),
            visible: true,
            enabled: true,
        }
    }
}

impl Bevel {
    /// One line from `(x0, y0)` to `(x1, y1)` in the role's colour.
    fn line(canvas: &mut Canvas, role: ThemeRole, x0: f32, y0: f32, x1: f32, y1: f32) {
        canvas.set_stroke_style(role);
        canvas.begin_path();
        canvas.move_to(x0, y0);
        canvas.line_to(x1, y1);
        canvas.stroke();
    }
}

impl Paint for Bevel {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h) = (50.0, 50.0);
        let (first, second) = if self.bevel_style == "Raised" {
            (ThemeRole::Border, ThemeRole::Surface)
        } else {
            (ThemeRole::Surface, ThemeRole::Border)
        };
        canvas.set_line_width(1.0);
        match self.bevel_shape.as_str() {
            "Box" | "Frame" => {
                Self::line(canvas, first, 0.5, 0.5, w - 0.5, 0.5);
                Self::line(canvas, first, 0.5, 0.5, 0.5, h - 0.5);
                Self::line(canvas, second, 0.5, h - 0.5, w - 0.5, h - 0.5);
                Self::line(canvas, second, w - 0.5, 0.5, w - 0.5, h - 0.5);
            }
            "TopLine" => {
                Self::line(canvas, first, 0.0, 0.5, w, 0.5);
                Self::line(canvas, second, 0.0, 1.5, w, 1.5);
            }
            "BottomLine" => {
                Self::line(canvas, first, 0.0, h - 1.5, w, h - 1.5);
                Self::line(canvas, second, 0.0, h - 0.5, w, h - 0.5);
            }
            "LeftLine" => {
                Self::line(canvas, first, 0.5, 0.0, 0.5, h);
                Self::line(canvas, second, 1.5, 0.0, 1.5, h);
            }
            "RightLine" => {
                Self::line(canvas, first, w - 1.5, 0.0, w - 1.5, h);
                Self::line(canvas, second, w - 0.5, 0.0, w - 0.5, h);
            }
            _ => {}
        }
    }
}
