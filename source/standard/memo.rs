//! # `Memo` — Canonical Reference Implementation
//!
//! Standard multiline text editor control with border, focus support,
//! scrolling configuration, and text change notification.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A multiline text area allowing entry and display of multiple lines of text.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "mmo",
    size = (200.0, 90.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
#[event(name = "on_key_down")]
#[event(name = "on_key_press")]
#[event(name = "on_key_up")]
pub struct Memo {
    /// Multiline text content.
    #[prop(display = "Lines", category = "text", default = "Line1\nLine2\nLine3")]
    pub text: String,

    /// When true, prevents the user from modifying text.
    #[prop(display = "ReadOnly", category = "behavior", default = false)]
    pub readonly: bool,

    /// Controls visibility of the memo control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Optional tooltip or help text.
    #[prop(display = "Hint", category = "behavior", default = "")]
    pub hint: String,

    /// Tab order index within the parent container.
    #[prop(display = "TabOrder", category = "behavior", default = 0)]
    pub tab_order: i64,
}

impl Default for Memo {
    fn default() -> Self {
        Self {
            text: String::from("Line1\nLine2\nLine3"),
            readonly: false,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Memo {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Surface background with subtle rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 200.0, 90.0, 2.0);

        // 2. Outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 200.0, 90.0, 2.0);

        // 3. Text lines
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 6.0, 6.0, 188.0, 78.0, TextAlign::Left);
    }
}
