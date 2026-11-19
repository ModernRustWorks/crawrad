//! # `Grid` — Canonical Reference Implementation
//!
//! Unified high-performance tabular grid component for CrawRAD (ADR 0041).
//! Replaces and unifies the classic RAD `TStringGrid`, `TDrawGrid`, and `TDBGrid` into a single
//! component capable of static data editing or reactive data binding via `crawrad-binding`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A 2D data grid control displaying tabular data with customizable columns, headers, and grid lines.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "grd",
    size = (320.0, 200.0),
    targets = ["*"]
)]
pub struct Grid {
    /// Number of columns in the grid.
    #[prop(display = "ColCount", category = "behavior", default = 5)]
    pub col_count: i64,

    /// Number of rows in the grid.
    #[prop(display = "RowCount", category = "behavior", default = 5)]
    pub row_count: i64,

    /// Number of fixed non-scrolling columns on the left.
    #[prop(display = "FixedCols", category = "behavior", default = 0)]
    pub fixed_cols: i64,

    /// Number of fixed non-scrolling header rows at the top.
    #[prop(display = "FixedRows", category = "behavior", default = 1)]
    pub fixed_rows: i64,

    /// Default width for columns in design units.
    #[prop(display = "DefaultColWidth", category = "appearance", default = 80)]
    pub default_col_width: i64,

    /// Default height for rows in design units.
    #[prop(display = "DefaultRowHeight", category = "appearance", default = 24)]
    pub default_row_height: i64,

    /// Whether subtle cell dividing grid lines are rendered.
    #[prop(display = "GridLines", category = "appearance", default = true)]
    pub grid_lines: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Optional tooltip or help text.
    #[prop(display = "Hint", category = "behavior", default = "")]
    pub hint: String,

    /// Tab navigation order.
    #[prop(display = "TabOrder", category = "behavior", default = 0)]
    pub tab_order: i64,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            col_count: 5,
            row_count: 5,
            fixed_cols: 0,
            fixed_rows: 1,
            default_col_width: 80,
            default_row_height: 24,
            grid_lines: true,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for Grid {
    /// Design-time 2D Canvas rendering on the IDE form surface.
    ///
    /// Renders outer rounded panel, header rows with contrast background, cell grid lines,
    /// and default header column titles (A, B, C, ...).
    fn paint(&self, canvas: &mut Canvas) {
        let w = 320.0;
        let h = 200.0;
        let col_w = (self.default_col_width as f32).max(20.0);
        let row_h = (self.default_row_height as f32).max(16.0);

        // 1. Outer background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.fill_round_rect(0.0, 0.0, w, h, 2.0);
        canvas.stroke_round_rect(0.0, 0.0, w, h, 2.0);

        // 2. Fixed header row (if fixed_rows > 0)
        let header_h = (self.fixed_rows as f32 * row_h).min(h);
        if header_h > 0.0 {
            canvas.set_fill_style(ThemeRole::Surface);
            canvas.fill_rect(1.0, 1.0, w - 2.0, header_h);
        }

        // 3. Vertical column dividers & headers
        let mut x = 0.0;
        for c in 0..self.col_count.max(1) {
            x += col_w;
            if x >= w {
                break;
            }
            if self.grid_lines {
                canvas.set_stroke_style(ThemeRole::Border);
                canvas.begin_path();
                canvas.move_to(x, 0.0);
                canvas.line_to(x, h);
                canvas.stroke();
            }

            // Draw column header letter (A, B, C...) in fixed row
            if self.fixed_rows > 0 {
                let col_letter = (b'A' + (c as u8 % 26)) as char;
                let col_title = format!("{col_letter}");
                canvas.set_fill_style(ThemeRole::TextDim);
                canvas.fill_text_in_rect(
                    &col_title,
                    x - col_w,
                    0.0,
                    col_w,
                    row_h,
                    TextAlign::Center,
                );
            }
        }

        // 4. Horizontal row dividers
        let mut y = 0.0;
        for _ in 0..self.row_count.max(1) {
            y += row_h;
            if y >= h {
                break;
            }
            if self.grid_lines {
                canvas.set_stroke_style(ThemeRole::Border);
                canvas.begin_path();
                canvas.move_to(0.0, y);
                canvas.line_to(w, y);
                canvas.stroke();
            }
        }
    }
}
