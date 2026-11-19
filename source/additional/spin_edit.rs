//! # `SpinEdit` — Canonical Reference Implementation
//!
//! A whole number typed in a field or stepped with the up and down buttons at
//! its right. `min_value` and `max_value` bound it unless both are 0;
//! `editor_enabled` off leaves only the buttons to change it.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A numeric field with step buttons.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "spn",
    size = (121.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct SpinEdit {
    /// The number shown.
    #[prop(display = "Value", category = "behavior", default = 0)]
    pub value: i64,

    /// The smallest it goes (with `max_value` both 0: no range).
    #[prop(display = "MinValue", category = "behavior", default = 0)]
    pub min_value: i64,

    /// The largest it goes.
    #[prop(display = "MaxValue", category = "behavior", default = 0)]
    pub max_value: i64,

    /// The step of the buttons.
    #[prop(display = "Increment", category = "behavior", default = 1)]
    pub increment: i64,

    /// Whether the number can be typed.
    #[prop(display = "EditorEnabled", category = "behavior", default = true)]
    pub editor_enabled: bool,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for SpinEdit {
    fn default() -> Self {
        Self {
            value: 0,
            min_value: 0,
            max_value: 0,
            increment: 1,
            editor_enabled: true,
            visible: true,
            enabled: true,
        }
    }
}

impl SpinEdit {
    /// One step up (`up`) or down, kept in the range when there is one.
    pub fn step(&mut self, up: bool) {
        let step = self.increment.max(1);
        self.value = if up {
            self.value.saturating_add(step)
        } else {
            self.value.saturating_sub(step)
        };
        if self.min_value != 0 || self.max_value != 0 {
            self.value = self
                .value
                .clamp(self.min_value, self.max_value.max(self.min_value));
        }
    }
}

impl Paint for SpinEdit {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The field
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_round_rect(0.0, 0.0, 121.0, 24.0, 2.0);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 121.0, 24.0, 2.0);

        // 2. The number
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect(
            &self.value.to_string(),
            4.0,
            0.0,
            97.0,
            24.0,
            TextAlign::Left,
        );

        // 3. The step buttons at the right
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.stroke_round_rect(105.0, 0.0, 16.0, 12.0, 1.0);
        canvas.stroke_round_rect(105.0, 12.0, 16.0, 12.0, 1.0);
    }
}
