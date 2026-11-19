//! # `ListBox` — Canonical Reference Implementation
//!
//! Standard scrollable list control with item collection,
//! row selection highlighting, and change event notification.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A vertical list box displaying a collection of selectable text items.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "lst",
    size = (120.0, 96.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct ListBox {
    /// Collection of text items in the list.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// Zero-based index of the currently selected item, or -1.
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

    /// Controls visibility of the list box control.
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

impl Default for ListBox {
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

impl Paint for ListBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Surface background with subtle rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 120.0, 96.0, 2.0);

        // 2. Outer border
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 120.0, 96.0, 2.0);

        // 3. Render items or sample rows
        let row_height = 18.0;
        let visible_rows = 5.min(self.items.len().max(3));

        for i in 0..visible_rows {
            let y = 3.0 + (i as f32) * row_height;
            let is_selected = self.item_index == i as i64;

            if is_selected {
                canvas.set_fill_style(ThemeRole::Accent);
                canvas.fill_rect(2.0, y, 116.0, row_height);
            }

            let text = if i < self.items.len() {
                &self.items[i]
            } else {
                "Item"
            };

            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_in_rect(text, 6.0, y + 2.0, 108.0, row_height - 2.0, TextAlign::Left);
        }
    }
}
