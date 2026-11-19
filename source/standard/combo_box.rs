//! # `ComboBox` — Canonical Reference Implementation
//!
//! Standard drop-down list selector control with item collection,
//! index selection, chevron indicator, and change event notification.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A drop-down list component allowing selection from a list of strings.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "cbo",
    size = (145.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct ComboBox {
    /// List of available items.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// Zero-based index of the selected item, or -1 if none selected.
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

    /// Controls visibility of the combo box control.
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

impl Default for ComboBox {
    fn default() -> Self {
        Self {
            items: Strings::default(),
            item_index: -1,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for ComboBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. White/surface background with rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 145.0, 24.0, 2.0);

        // 2. Outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 145.0, 24.0, 2.0);

        // 3. Dropdown button divider and chevron indicator on the right
        canvas.stroke_rect(123.0, 0.0, 22.0, 24.0);
        canvas.set_fill_style(ThemeRole::Text);
        // Small downward chevron / triangle indicator
        canvas.fill_round_rect(131.0, 11.0, 6.0, 3.0, 1.0);

        // 4. Selected item text or preview
        let text = if self.item_index >= 0 && (self.item_index as usize) < self.items.len() {
            &self.items[self.item_index as usize]
        } else if !self.items.is_empty() {
            &self.items[0]
        } else {
            ""
        };

        if !text.is_empty() {
            canvas.fill_text_in_rect(text, 6.0, 4.0, 114.0, 16.0, TextAlign::Left);
        }
    }
}
