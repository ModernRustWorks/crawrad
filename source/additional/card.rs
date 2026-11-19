//! # `Card` — Canonical Reference Implementation
//!
//! A modern container: a rounded surface, slightly raised, with an
//! optional title and subtitle on top and the user's controls inside.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A modern rounded card container with optional header.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "crd",
    size = (220.0, 140.0),
    children = "area",
    targets = ["*"]
)]
#[event(name = "on_click")]
pub struct Card {
    /// The card's title text.
    #[prop(display = "Caption", category = "appearance", default = "Card")]
    pub text: String,

    /// Secondary subtitle text below the title.
    #[prop(display = "Subtitle", category = "appearance", default = "")]
    pub subtitle: String,

    /// Corner radius in pixels.
    #[prop(display = "CornerRadius", category = "appearance", default = 8)]
    pub corner_radius: i64,

    /// Elevation depth (0..3) controlling shadow depth.
    #[prop(display = "Elevation", category = "appearance", default = 1)]
    pub elevation: i64,

    /// Whether to display the card header section.
    #[prop(display = "ShowHeader", category = "appearance", default = true)]
    pub show_header: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            text: String::from("Card"),
            subtitle: String::new(),
            corner_radius: 8,
            elevation: 1,
            show_header: true,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Card {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h) = (220.0, 140.0);
        let header_h = 40.0;

        // 1. Shadow if elevated
        if self.elevation > 0 {
            canvas.set_fill_style(ThemeRole::Border);
            canvas.fill_rect(2.0, 2.0, w, h);
        }

        // 2. Card background and outline
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.5, 0.5, w - 1.0, h - 1.0);

        // 3. Header section if shown
        if self.show_header {
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.stroke_rect(0.5, header_h - 0.5, w - 1.0, 0.0);

            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_prop("text", 12.0, 4.0, w - 24.0, 16.0, TextAlign::Left);

            if !self.subtitle.is_empty() {
                canvas.fill_text_prop("subtitle", 12.0, 20.0, w - 24.0, 16.0, TextAlign::Left);
            }
        }
    }
}
