//! # `RadioGroup` — Canonical Reference Implementation
//!
//! A framed group of mutually exclusive options as ONE control: its
//! `items` are the options, `item_index` the chosen one (-1 for none), laid
//! out in `columns` columns, each filled top down.

use crawrad_component::Strings;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A group box holding exclusive options, read and set as one index.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "rgp",
    size = (185.0, 105.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_change")]
pub struct RadioGroup {
    /// Caption shown in the frame's top edge.
    #[prop(display = "Caption", category = "appearance", default = "RadioGroup")]
    pub text: String,

    /// The options, one radio each.
    #[prop(display = "Items", category = "data")]
    pub items: Strings,

    /// Zero-based index of the chosen option, or -1 for none.
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

    /// How many columns the options are laid out in.
    #[prop(display = "Columns", category = "appearance", default = 1)]
    pub columns: i64,

    /// Controls visibility of the group.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self {
            text: String::from("RadioGroup"),
            items: Strings::default(),
            item_index: -1,
            columns: 1,
            visible: true,
            enabled: true,
        }
    }
}

/// The group's size, as the palette drops it.
const W: f32 = 185.0;
const H: f32 = 105.0;

impl Paint for RadioGroup {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The frame, its top edge at the caption's middle
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.5, 7.0, W - 1.0, H - 7.5, 3.0);

        // 2. The caption over the frame's top edge
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(8.0, 0.0, 84.0, 14.0);
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 10.0, 0.0, 80.0, 14.0, TextAlign::Left);

        // 3. One ring per option, in `columns` columns filled top down; the
        //    chosen one gets its dot
        let columns = self.columns.max(1) as usize;
        let rows = self.items.len().div_ceil(columns).max(1);
        let cell_w = (W - 16.0) / columns as f32;
        let cell_h = ((H - 26.0) / rows as f32).min(22.0);
        for (i, item) in self.items.iter().enumerate() {
            let x = 8.0 + (i / rows) as f32 * cell_w;
            let y = 18.0 + (i % rows) as f32 * cell_h;
            let cy = y + (cell_h - 12.0) / 2.0;
            canvas.set_fill_style(ThemeRole::Surface);
            canvas.fill_round_rect(x, cy, 12.0, 12.0, 6.0);
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.stroke_round_rect(x, cy, 12.0, 12.0, 6.0);
            if self.item_index == i as i64 {
                canvas.set_fill_style(ThemeRole::Accent);
                canvas.fill_round_rect(x + 3.0, cy + 3.0, 6.0, 6.0, 3.0);
            }
            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_in_rect(item, x + 18.0, y, cell_w - 20.0, cell_h, TextAlign::Left);
        }
    }
}
