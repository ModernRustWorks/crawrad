//! # `Edit` — Canonical Reference Implementation
//!
//! Standard single-line text input control with border, focus support,
//! password masking, and change event notification.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A single-line text input field allowing user data entry.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "edt",
    size = (200.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
#[event(name = "on_key_down")]
#[event(name = "on_key_press")]
#[event(name = "on_key_up")]
pub struct Edit {
    /// Text content entered in the field.
    #[prop(display = "Text", category = "text", default = "")]
    pub text: String,

    /// Character used to mask passwords, or empty for plain text.
    #[prop(display = "PasswordChar", category = "behavior", default = "")]
    pub password_char: String,

    /// When true, prevents the user from editing text.
    #[prop(display = "ReadOnly", category = "behavior", default = false)]
    pub readonly: bool,

    /// Controls visibility of the edit control.
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

impl Default for Edit {
    fn default() -> Self {
        Self {
            text: String::new(),
            password_char: String::new(),
            readonly: false,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Edit {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. White / surface background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);

        // 2. Subtle outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);

        // 3. Text content or password mask
        canvas.set_fill_style(ThemeRole::Text);
        if !self.password_char.is_empty() && !self.text.is_empty() {
            let mask = self.password_char.repeat(self.text.chars().count().min(24));
            canvas.fill_text_in_rect(&mask, 6.0, 4.0, 188.0, 16.0, TextAlign::Left);
        } else if !self.text.is_empty() {
            canvas.fill_text_prop("text", 6.0, 4.0, 188.0, 16.0, TextAlign::Left);
        }
    }
}
