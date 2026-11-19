//! # `ListView` — Canonical Reference Implementation
//!
//! A report-style list: column headers on top, one row per item, each row
//! showing the item's caption in the first column and its sub-items in the
//! next ones.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A report-style list view with column headers and virtualized rows.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "lvw",
    size = (240.0, 150.0),
    children = "none",
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_click")]
#[event(name = "on_dbl_click")]
pub struct ListView {
    /// Zero-based selected item index, or -1 for none.
    #[prop(display = "ItemIndex", category = "behavior", default = -1)]
    pub item_index: i64,

    /// Whether full rows are highlighted on selection.
    #[prop(display = "RowSelect", category = "behavior", default = true)]
    pub row_select: bool,

    /// Grid lines between rows and columns.
    #[prop(display = "GridLines", category = "appearance", default = false)]
    pub grid_lines: bool,

    /// Whether column headers are visible.
    #[prop(display = "ShowColumnHeaders", category = "appearance", default = true)]
    pub show_column_headers: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ListView {
    fn default() -> Self {
        Self {
            item_index: -1,
            row_select: true,
            grid_lines: false,
            show_column_headers: true,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ListView {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let (w, h, header_h) = (240.0, 150.0, 24.0);

        // 1. Background and border
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.5, 0.5, w - 1.0, h - 1.0);

        // 2. Column headers
        if self.show_column_headers {
            canvas.fill_rect(0.0, 0.0, w, header_h);
            canvas.stroke_rect(0.5, 0.5, w - 1.0, header_h - 1.0);
            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_in_rect("Column 1", 6.0, 0.0, 80.0, header_h, TextAlign::Left);
            canvas.begin_path();
            canvas.move_to(80.0, 0.0);
            canvas.line_to(80.0, header_h);
            canvas.stroke();
            canvas.fill_text_in_rect("Column 2", 86.0, 0.0, 80.0, header_h, TextAlign::Left);
        }

        // 3. Grid lines if enabled
        if self.grid_lines {
            let row_h = 20.0;
            let mut y = if self.show_column_headers {
                header_h
            } else {
                0.0
            } + row_h;
            while y < h {
                canvas.begin_path();
                canvas.move_to(0.0, y);
                canvas.line_to(w, y);
                canvas.stroke();
                y += row_h;
            }
            let top_y = if self.show_column_headers {
                header_h
            } else {
                0.0
            };
            canvas.begin_path();
            canvas.move_to(80.0, top_y);
            canvas.line_to(80.0, h);
            canvas.stroke();
        }
    }
}
