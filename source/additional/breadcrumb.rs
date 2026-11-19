//! # `Breadcrumb` — Canonical Reference Implementation
//!
//! A navigation trail on one line: `Home › Projects › CrawRAD`.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A navigation trail on one line.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "brc",
    size = (260.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_click")]
pub struct Breadcrumb {
    /// The string items / segments.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// The separator string between segments.
    #[prop(display = "Separator", category = "appearance", default = "›")]
    pub separator: String,

    /// The segment last clicked (-1 = none).
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

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

impl Default for Breadcrumb {
    fn default() -> Self {
        Self {
            items: Strings::default(),
            separator: "›".to_string(),
            item_index: -1,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Breadcrumb {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        if self.items.is_empty() {
            return;
        }
        let height = 24.0;
        let mut x = 0.0;
        let gap = 6.0;
        let sep_w = (self.separator.len() as f32 * 7.0).max(8.0);
        let max_w = 260.0;
        let count = self.items.len();

        for (idx, text) in self.items.iter().enumerate() {
            let is_last = idx + 1 == count;
            let w = (text.len() as f32 * 7.0).max(12.0);
            if x + w > max_w {
                break;
            }
            if is_last {
                canvas.set_fill_style(ThemeRole::Text);
            } else {
                canvas.set_fill_style(ThemeRole::Accent);
            }
            canvas.fill_text_in_rect(text, x, 0.0, w, height, TextAlign::Left);
            x += w + gap;

            if !is_last {
                if x + sep_w > max_w {
                    break;
                }
                canvas.set_fill_style(ThemeRole::TextDim);
                canvas.fill_text_in_rect(&self.separator, x, 0.0, sep_w, height, TextAlign::Left);
                x += sep_w + gap;
            }
        }
    }
}
