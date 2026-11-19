//! # `Chips` — Canonical Reference Implementation
//!
//! A group of small rounded chips laid out left to right, wrapping to the
//! next line when the row is full.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A group of rounded chips / tag pickers.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "chp",
    size = (220.0, 56.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_click")]
pub struct Chips {
    /// The string items / tags.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// The selected chip index (-1 = none).
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

    /// Whether each chip shows a small remove button.
    #[prop(display = "Removable", category = "behavior", default = false)]
    pub removable: bool,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Chips {
    fn default() -> Self {
        Self {
            items: Strings::default(),
            item_index: -1,
            removable: false,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Chips {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        if self.items.is_empty() {
            return;
        }
        let chip_h = 24.0;
        let mut x = 0.0;
        let mut y = 0.0;
        let gap = 6.0;
        let max_w = 220.0;

        for (idx, text) in self.items.iter().enumerate() {
            let w = (text.len() as f32 * 7.0 + 16.0 + if self.removable { 14.0 } else { 0.0 })
                .max(36.0);
            if x > 0.0 && x + w > max_w {
                x = 0.0;
                y += chip_h + gap;
            }
            if y + chip_h > 56.0 {
                break;
            }

            let is_sel = self.item_index == idx as i64;
            if is_sel {
                canvas.set_fill_style(ThemeRole::Accent);
            } else {
                canvas.set_fill_style(ThemeRole::Surface);
            }
            canvas.fill_rect(x, y, w, chip_h);

            if is_sel {
                canvas.set_fill_style(ThemeRole::Text);
            } else {
                canvas.set_fill_style(ThemeRole::TextDim);
            }
            canvas.fill_text_prop("items", x, y, w, chip_h, TextAlign::Center);

            x += w + gap;
        }
    }
}
