//! # `ToolBar` — Canonical Reference Implementation
//!
//! Window header toolbar container providing button docking, separators, and command layout.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A toolbar container that arranges buttons, drop-downs, and separators in rows.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "tbr",
    size = (320.0, 32.0),
    children = "area",
    targets = ["*"]
)]
pub struct ToolBar {
    /// Whether buttons in the toolbar have a flat appearance.
    #[prop(display = "Flat", category = "appearance", default = true)]
    pub flat: bool,

    /// Whether button captions are shown alongside or under icons.
    #[prop(display = "ShowCaptions", category = "appearance", default = false)]
    pub show_captions: bool,

    /// Width of individual tool buttons in pixels.
    #[prop(
        display = "ButtonWidth",
        category = "appearance",
        default = 28,
        min = 16
    )]
    pub button_width: i64,

    /// Height of individual tool buttons in pixels.
    #[prop(
        display = "ButtonHeight",
        category = "appearance",
        default = 28,
        min = 16
    )]
    pub button_height: i64,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the toolbar.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ToolBar {
    fn default() -> Self {
        Self {
            flat: true,
            show_captions: false,
            button_width: 28,
            button_height: 28,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ToolBar {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 320.0;
        let h = 32.0;

        // 1. Toolbar background surface & bottom border line
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(0.0, h - 1.0, w, 1.0);

        // 2. Toolbar grip handle on the left
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(4.0, 4.0, 2.0, h - 8.0);
        canvas.fill_rect(7.0, 4.0, 2.0, h - 8.0);

        // 3. Sample tool button slots (visual design-time preview)
        let bw = self.button_width as f32;
        let bh = self.button_height as f32;

        let icons = ["📄", "📂", "💾"];
        let mut x = 14.0;
        for icon in icons {
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.set_line_width(1.0);
            canvas.stroke_round_rect(x, 2.0, bw, bh, 2.0);

            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_in_rect(icon, x + 2.0, 4.0, bw - 4.0, bh - 6.0, TextAlign::Center);

            x += bw + 2.0;
        }

        // Separator bar
        x += 4.0;
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(x, 4.0, 1.0, h - 8.0);
    }
}
