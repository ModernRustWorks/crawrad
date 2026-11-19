//! # `ScrollBox` — Canonical Reference Implementation
//!
//! Scrollable client area container hosting visual children, providing automatic
//! scrolling bars (`children = "area"`), and configurable border framing.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A windowed container with horizontal and vertical scroll bars for large client areas.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "sbx",
    size = (200.0, 150.0),
    children = "area",
    targets = ["*"]
)]
pub struct ScrollBox {
    /// Enables automatic scroll bar activation when child controls exceed boundaries.
    #[prop(display = "AutoScroll", category = "behavior", default = true)]
    pub auto_scroll: bool,

    /// Shows or hides the horizontal scroll bar.
    #[prop(display = "HorzScrollBar", category = "appearance", default = true)]
    pub horz_scroll_bar: bool,

    /// Shows or hides the vertical scroll bar.
    #[prop(display = "VertScrollBar", category = "appearance", default = true)]
    pub vert_scroll_bar: bool,

    /// Border width around the client area in pixels.
    #[prop(display = "BorderWidth", category = "appearance", default = 1, min = 0)]
    pub border_width: i64,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ScrollBox {
    fn default() -> Self {
        Self {
            auto_scroll: true,
            horz_scroll_bar: true,
            vert_scroll_bar: true,
            border_width: 1,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ScrollBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 200.0;
        let h = 150.0;
        let sb_size = 14.0;

        // 1. Surface background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        // 2. Outer container border
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(self.border_width as f32);
        canvas.stroke_rect(0.0, 0.0, w, h);

        // 3. Vertical scroll bar along the right edge
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(w - sb_size, 0.0, sb_size, h - sb_size);

        // Vertical thumb indicator
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(w - sb_size + 2.0, 16.0, sb_size - 4.0, 36.0, 2.0);

        // 4. Horizontal scroll bar along the bottom edge
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(0.0, h - sb_size, w - sb_size, sb_size);

        // Horizontal thumb indicator
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(24.0, h - sb_size + 2.0, 48.0, sb_size - 4.0, 2.0);

        // 5. Corner junction box
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(w - sb_size, h - sb_size, sb_size, sb_size);
    }
}
