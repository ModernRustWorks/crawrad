//! # `Drawer` — Canonical Reference Implementation
//!
//! A side panel container: opens over the form with a header,
//! an optional close button, child controls, and a dimmed overlay (scrim).

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A side panel drawer container with optional overlay and header.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "drw",
    size = (240.0, 320.0),
    children = "area",
    targets = ["*"]
)]
#[event(name = "on_close")]
pub struct Drawer {
    /// The drawer's caption text in the header.
    #[prop(display = "Caption", category = "appearance", default = "Drawer")]
    pub text: String,

    /// Which side the drawer slides out from ("dsLeft" or "dsRight").
    #[prop(
        display = "Side",
        category = "appearance",
        options = ["dsLeft", "dsRight"],
        default = "dsLeft"
    )]
    pub side: String,

    /// Whether the drawer is open and visible.
    #[prop(display = "Opened", category = "behavior", default = false)]
    pub opened: bool,

    /// Whether to display the drawer header section with title and close button.
    #[prop(display = "ShowHeader", category = "appearance", default = true)]
    pub show_header: bool,

    /// Whether a dimmed scrim covers the rest of the form when open.
    #[prop(display = "Overlay", category = "appearance", default = true)]
    pub overlay: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Drawer {
    fn default() -> Self {
        Self {
            text: String::from("Drawer"),
            side: String::from("dsLeft"),
            opened: false,
            show_header: true,
            overlay: true,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Drawer {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h) = (240.0, 320.0);
        let header_h = 32.0;
        let is_right = self.side == "dsRight";

        // 1. Shadow opposite to Side
        canvas.set_fill_style(ThemeRole::Border);
        if is_right {
            canvas.fill_rect(-4.0, 0.0, 4.0, h);
        } else {
            canvas.fill_rect(w, 0.0, 4.0, h);
        }

        // 2. Drawer background and outline
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.5, 0.5, w - 1.0, h - 1.0);

        // 3. Header section if shown
        if self.show_header {
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.stroke_rect(0.5, header_h - 0.5, w - 1.0, 0.0);

            let (close_x, title_x) = if is_right {
                (8.0, 36.0)
            } else {
                (w - 24.0, 12.0)
            };

            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_prop("text", title_x, 8.0, w - 48.0, 16.0, TextAlign::Left);
            canvas.fill_text_in_rect("×", close_x, 8.0, 16.0, 16.0, TextAlign::Center);
        }
    }
}
