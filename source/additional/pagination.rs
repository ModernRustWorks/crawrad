//! # `Pagination` — Canonical Reference Implementation
//!
//! A row of page buttons: `‹ 1 … 4 [5] 6 … 20 ›`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A row of page buttons for navigation.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "pag",
    size = (280.0, 28.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
pub struct Pagination {
    /// Total number of pages.
    #[prop(display = "PageCount", category = "data", default = 10)]
    pub page_count: i64,

    /// The current 0-based page index.
    #[prop(display = "PageIndex", category = "data", default = 0)]
    pub page_index: i64,

    /// Maximum button slots displayed (values below 5 behave as 5).
    #[prop(display = "MaxButtons", category = "appearance", default = 7)]
    pub max_buttons: i64,

    /// Whether previous and next arrows are displayed.
    #[prop(display = "ShowArrows", category = "appearance", default = true)]
    pub show_arrows: bool,

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

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page_count: 10,
            page_index: 0,
            max_buttons: 7,
            show_arrows: true,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Pagination {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let cell_size = 28.0;
        let gap = 4.0;
        let count = (self.page_count.max(1)) as usize;
        let max_b = (self.max_buttons.max(5)) as usize;
        let current = ((self.page_index.max(0)) as usize).min(count.saturating_sub(1));
        let max_w = 280.0;

        let mut x = 0.0;

        if self.show_arrows {
            canvas.set_fill_style(ThemeRole::TextDim);
            canvas.fill_text_in_rect("‹", x, 0.0, cell_size, cell_size, TextAlign::Center);
            x += cell_size + gap;
        }

        let pages_to_show = if count <= max_b {
            (0..count).map(Some).collect::<Vec<_>>()
        } else {
            vec![Some(0), None, Some(current), None, Some(count - 1)]
        };

        for page in pages_to_show {
            if x + cell_size > max_w {
                break;
            }
            match page {
                Some(idx) => {
                    let is_current = idx == current;
                    if is_current {
                        canvas.set_fill_style(ThemeRole::Accent);
                        canvas.fill_rect(x, 0.0, cell_size, cell_size);
                        canvas.set_fill_style(ThemeRole::Text);
                    } else {
                        canvas.set_stroke_style(ThemeRole::Border);
                        canvas.stroke_rect(x, 0.0, cell_size, cell_size);
                        canvas.set_fill_style(ThemeRole::Text);
                    }
                    let label = format!("{}", idx + 1);
                    canvas.fill_text_in_rect(
                        &label,
                        x,
                        0.0,
                        cell_size,
                        cell_size,
                        TextAlign::Center,
                    );
                }
                None => {
                    canvas.set_fill_style(ThemeRole::TextDim);
                    canvas.fill_text_in_rect("…", x, 0.0, cell_size, cell_size, TextAlign::Center);
                }
            }
            x += cell_size + gap;
        }

        if self.show_arrows && x + cell_size <= max_w {
            canvas.set_fill_style(ThemeRole::TextDim);
            canvas.fill_text_in_rect("›", x, 0.0, cell_size, cell_size, TextAlign::Center);
        }
    }
}
