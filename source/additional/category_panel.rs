//! # `CategoryPanel` — Canonical Reference Implementation
//!
//! A container under a header: a click on the header folds the panel
//! to the header alone, hiding its children, and runs `OnCollapsed`; another
//! click opens it and runs `OnExpanded`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A foldable container with a caption header.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "cat",
    size = (200.0, 160.0),
    children = "area",
    targets = ["*"]
)]
#[event(name = "on_collapsed")]
#[event(name = "on_expanded")]
pub struct CategoryPanel {
    /// The header's caption.
    #[prop(
        display = "Caption",
        category = "appearance",
        default = "CategoryPanel"
    )]
    pub text: String,

    /// Folded to the header, its children out of sight.
    #[prop(display = "Collapsed", category = "behavior", default = false)]
    pub collapsed: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for CategoryPanel {
    fn default() -> Self {
        Self {
            text: String::from("CategoryPanel"),
            collapsed: false,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for CategoryPanel {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h, header) = (200.0, 160.0, 24.0);

        // 1. The body's frame, when open
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        if !self.collapsed {
            canvas.stroke_rect(0.5, 0.5, w - 1.0, h - 1.0);
        }

        // 2. The header
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, header);
        canvas.stroke_rect(0.5, 0.5, w - 1.0, header - 1.0);

        // 3. The chevron: right when folded, down when open
        canvas.set_fill_style(ThemeRole::Text);
        canvas.begin_path();
        if self.collapsed {
            canvas.move_to(10.0, 8.0);
            canvas.line_to(15.0, 12.0);
            canvas.line_to(10.0, 16.0);
        } else {
            canvas.move_to(8.0, 10.0);
            canvas.line_to(16.0, 10.0);
            canvas.line_to(12.0, 15.0);
        }
        canvas.close_path();
        canvas.fill();

        // 4. The caption
        canvas.fill_text_prop("text", 24.0, 0.0, w - 28.0, header, TextAlign::Left);
    }
}
