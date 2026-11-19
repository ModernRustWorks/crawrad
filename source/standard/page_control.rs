//! # `PageControl` — Canonical Reference Implementation
//!
//! Standard multi-page tabbed container with header tabs,
//! active page switching, and hosting of child pages (`children = "area"`).

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A multi-page container displaying tabbed sheets that host other controls.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "pgc",
    size = (320.0, 200.0),
    children = "area",
    targets = ["*"]
)]
#[event(name = "on_change")]
pub struct PageControl {
    /// Zero-based index of the currently active page tab.
    #[prop(display = "ActivePageIndex", category = "behavior", default = 0)]
    pub active_page: i64,

    /// Controls visibility of the page control and all hosted pages.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction with the page control.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for PageControl {
    fn default() -> Self {
        Self {
            active_page: 0,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for PageControl {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Tab headers at the top (height 24px)
        // Tab 1 (Active)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 80.0, 24.0, 3.0);
        canvas.stroke_round_rect(1.0, 1.0, 80.0, 24.0, 3.0);

        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("TabSheet1", 6.0, 4.0, 70.0, 18.0, TextAlign::Center);

        // Tab 2 (Inactive)
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_round_rect(82.0, 3.0, 80.0, 22.0, 3.0);
        canvas.stroke_round_rect(82.0, 3.0, 80.0, 22.0, 3.0);

        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("TabSheet2", 87.0, 5.0, 70.0, 18.0, TextAlign::Center);

        // 2. Main content area frame below the tabs
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 24.0, 320.0, 176.0, 2.0);
        canvas.stroke_round_rect(0.0, 24.0, 320.0, 176.0, 2.0);
    }
}
