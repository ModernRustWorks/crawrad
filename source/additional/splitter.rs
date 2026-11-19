//! # `Splitter` — Canonical Reference Implementation
//!
//! Visual sash/splitter control for dynamic runtime resizing of adjacent controls.
//! Typically aligned alongside adjacent panels (e.g. `Align = alLeft` or `Align = alTop`).

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A visual sash control that allows runtime resizing of adjacent docked or aligned controls.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "spl",
    size = (5.0, 100.0),
    targets = ["*"]
)]
#[event(name = "on_can_resize")]
#[event(name = "on_moved")]
pub struct Splitter {
    /// Minimum size in pixels that adjacent controls may be resized to.
    #[prop(display = "MinSize", category = "behavior", default = 30, min = 1)]
    pub min_size: i64,

    /// Whether the splitter snaps to the parent container edges when dragged close.
    #[prop(display = "AutoSnap", category = "behavior", default = true)]
    pub auto_snap: bool,

    /// Whether the splitter displays a 3D bevel or subtle grip marks.
    #[prop(display = "Beveled", category = "appearance", default = true)]
    pub beveled: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Splitter {
    fn default() -> Self {
        Self {
            min_size: 30,
            auto_snap: true,
            beveled: true,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Splitter {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = canvas.width().max(5.0);
        let h = canvas.height().max(5.0);

        // 1. Splitter bar background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        if self.beveled {
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.set_line_width(1.0);
            canvas.stroke_rect(0.0, 0.0, w, h);

            // 2. Centered grip handle: 3 subtle dots along the longer axis
            canvas.set_fill_style(ThemeRole::TextDim);
            let mid_x = w * 0.5;
            let mid_y = h * 0.5;
            let dot_radius = 1.5;

            if h >= w {
                // Vertical splitter: 3 dots stacked vertically
                canvas.fill_round_rect(
                    mid_x - dot_radius,
                    mid_y - 6.0 - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
                canvas.fill_round_rect(
                    mid_x - dot_radius,
                    mid_y - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
                canvas.fill_round_rect(
                    mid_x - dot_radius,
                    mid_y + 6.0 - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
            } else {
                // Horizontal splitter: 3 dots aligned horizontally
                canvas.fill_round_rect(
                    mid_x - 6.0 - dot_radius,
                    mid_y - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
                canvas.fill_round_rect(
                    mid_x - dot_radius,
                    mid_y - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
                canvas.fill_round_rect(
                    mid_x + 6.0 - dot_radius,
                    mid_y - dot_radius,
                    dot_radius * 2.0,
                    dot_radius * 2.0,
                    dot_radius,
                );
            }
        }
    }
}
