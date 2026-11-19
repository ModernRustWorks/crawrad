//! # `MultiSelect` — Canonical Reference Implementation
//!
//! A drop-down field whose list has a check box per item; any number of items
//! can be checked.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A drop-down multi-selection component allowing multiple item selection with checkboxes.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "mse",
    size = (180.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
pub struct MultiSelect {
    /// List of available items.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// List of selected item texts.
    #[prop(display = "SelectedItems", category = "data")]
    pub selected_items: Strings,

    /// Placeholder text when nothing is selected.
    #[prop(display = "TextHint", category = "appearance", default = "Select…")]
    pub text_hint: String,

    /// Number of rows visible before the drop-down list scrolls.
    #[prop(display = "DropDownCount", category = "appearance", default = 6)]
    pub drop_down_count: i64,

    /// Controls visibility of the control.
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

impl Default for MultiSelect {
    fn default() -> Self {
        Self {
            items: Strings::default(),
            selected_items: Strings::default(),
            text_hint: String::from("Select…"),
            drop_down_count: 6,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for MultiSelect {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h) = (180.0, 24.0);

        // 1. Surface background with rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, w, h, 2.0);

        // 2. Outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, w, h, 2.0);

        // 3. Dropdown chevron on the right
        canvas.stroke_rect(w - 22.0, 0.0, 22.0, h);
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_round_rect(w - 14.0, 11.0, 6.0, 3.0, 1.0);

        // 4. Summary text or hint
        let summary = if self.selected_items.is_empty() {
            &self.text_hint
        } else {
            &self.selected_items[0]
        };

        if !summary.is_empty() {
            canvas.fill_text_in_rect(summary, 6.0, 4.0, w - 30.0, 16.0, TextAlign::Left);
        }
    }
}
