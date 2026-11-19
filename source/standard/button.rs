//! # `Button` — Canonical Reference Implementation
//!
//! Standard command button with vector 2D `Canvas` drawing,
//! rounded border, semantic theme colors, and full event handling.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A standard push button responding to user clicks, keyboard, and mouse events.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "btn",
    size = (96.0, 32.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
#[event(name = "on_key_down")]
#[event(name = "on_key_press")]
#[event(name = "on_key_up")]
#[event(name = "on_mouse_activate")]
#[event(name = "on_mouse_down")]
#[event(name = "on_mouse_enter")]
#[event(name = "on_mouse_leave")]
#[event(name = "on_mouse_move")]
#[event(name = "on_mouse_up")]
pub struct Button {
    /// Caption text displayed on the button surface.
    #[prop(display = "Caption", category = "appearance", default = "Button")]
    pub text: String,

    /// Controls button visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Optional hint displayed when hovering over the control.
    #[prop(display = "Hint", category = "behavior", default = "")]
    pub hint: String,

    /// Tab order index within the parent container.
    #[prop(display = "TabOrder", category = "behavior", default = 0)]
    pub tab_order: i64,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            text: String::from("Button"),
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Button {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Background surface with rounded corners (4px radius)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 96.0, 32.0, 4.0);

        // 2. Outer border
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 96.0, 32.0, 4.0);

        // 3. Centered caption text
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 0.0, 0.0, 96.0, 32.0, TextAlign::Center);
    }
}
