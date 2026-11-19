//! Makepad Grid component definition and canvas drawing.

use crawrad_ir::design_rows::GridRows;
use crawrad_runtime::grid::{
    CellRenderer, ColumnAlign, ColumnVAlign, GridColumn, Marks, cell_text, share_widths_clamped,
    wrap_text,
};
use crawrad_runtime::string_grid::{Cells, GridLayout};
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::{Brush, ContentLine, shrink};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-grid-classe")]
pub const CLASSE: &str = "Grid";

fn int_prop(c: &MakepadCtl, key: &str, default: i64) -> i64 {
    c.props
        .get(key)
        .and_then(|v| match v {
            crawrad_ir::Value::Int(n) => Some(*n),
            _ => None,
        })
        .unwrap_or(default)
}

#[note("components-grid-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn lines(&self, c: &MakepadCtl) -> Vec<ContentLine> {
        let col_count = int_prop(c, "col_count", 5);
        let row_count = int_prop(c, "row_count", 5);
        vec![ContentLine::leaf(
            0,
            format!("Grid: {} cols x {} rows", col_count, row_count),
        )]
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);

        // The same numbers the app's `CrawradStringGrid` uses, scaled to the canvas.
        let layout = GridLayout {
            col_width: p.medida(int_prop(c, "default_col_width", 80).max(1) as f32),
            row_height: p.medida(int_prop(c, "default_row_height", 24).max(1) as f32),
            ..GridLayout::from_props(
                int_prop(c, "col_count", 5),
                int_prop(c, "row_count", 5),
                int_prop(c, "fixed_cols", 0),
                int_prop(c, "fixed_rows", 1),
                1,
                1,
            )
        };

        if let Some(rows) = &c.design_rows {
            p.with_clip(r, |p| draw_design_rows(p, c, r, rows, layout.row_height));
            return;
        }

        let grid_lines = c
            .props
            .get("grid_lines")
            .and_then(|v| match v {
                crawrad_ir::Value::Bool(b) => Some(*b),
                _ => None,
            })
            .unwrap_or(true);

        let cell_text = c
            .props
            .get("cell_text")
            .and_then(|v| match v {
                crawrad_ir::Value::Str(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("");
        let cells = Cells::from_text(cell_text);

        p.with_clip(r, |p| {
            for row in 0..layout.row_count {
                for col in 0..layout.col_count {
                    let (cx, cy, cw, ch) = layout.cell_rect(col, row);
                    let cell_r = Rect {
                        pos: dvec2(r.pos.x + cx, r.pos.y + cy),
                        size: dvec2(cw, ch),
                    };

                    if layout.is_fixed(col, row) {
                        // on the box layer: the text goes over it
                        p.rounded_box(cell_r, MAKEPAD_PANEL, MAKEPAD_PANEL, 0.0);
                    }

                    if grid_lines {
                        let line_v = Rect {
                            pos: dvec2(cell_r.pos.x + cw - 1.0, cell_r.pos.y),
                            size: dvec2(1.0, ch),
                        };
                        p.stroke(line_v, MAKEPAD_PANEL_BORDER);

                        let line_h = Rect {
                            pos: dvec2(cell_r.pos.x, cell_r.pos.y + ch - 1.0),
                            size: dvec2(cw, 1.0),
                        };
                        p.stroke(line_h, MAKEPAD_PANEL_BORDER);
                    }

                    let text = cells.get(col, row);
                    if !text.is_empty() {
                        let inset = 4.0;
                        let text_r = Rect {
                            pos: dvec2(cell_r.pos.x + inset, cell_r.pos.y),
                            size: dvec2((cell_r.size.x - inset * 2.0).max(0.0), cell_r.size.y),
                        };
                        p.write(text_r, text, MAKEPAD_TEXT, false);
                    }
                }
            }
        });
    }
}

/// The dot of a badge whose rows are here.
const ROWS_READY: [f32; 4] = rgb(0x4C, 0xAF, 0x7D);
/// The dot of a badge that is reading, or has no rows.
const ROWS_WAITING: [f32; 4] = rgb(0xC9, 0xA2, 0x4A);
/// The words of the box that says why there are no rows.
const ROWS_WARNING_TEXT: [f32; 4] = rgb(0xE6, 0xC2, 0x7A);
/// The edge of that box.
const ROWS_WARNING_EDGE: [f32; 4] = rgb(0x5A, 0x4A, 0x2A);
/// The strip under the rows, where the badge sits.
const ROWS_FOOTER: [f32; 4] = rgb(0x26, 0x28, 0x2C);

/// A bound grid at design time: the real header, the first rows of the
/// table (read-only) and, at its foot, where they came from — or, without a
/// connection, the columns of the last good read and why there are no rows.
fn draw_design_rows(p: &mut Brush, c: &MakepadCtl, r: Rect, rows: &GridRows, row_h: f64) {
    let empty = Vec::new();
    let data = match rows {
        GridRows::Ready { sample, .. } => &sample.rows,
        _ => &empty,
    };
    let fallback: Vec<String> = (0..int_prop(c, "col_count", 5).max(1))
        .map(|_| String::new())
        .collect();
    let header = if rows.columns().is_empty() {
        &fallback
    } else {
        rows.columns()
    };
    let numeric: Vec<bool> = (0..header.len())
        .map(|i| {
            matches!(
                rows.kinds().get(i).map(String::as_str),
                Some("INTEGER" | "REAL")
            )
        })
        .collect();
    let cols = GridColumn::from_design(header, &numeric, &c.columns);
    // Without columns the grid looks as it did before: widths by how long the words are.
    let widths = if c.columns.is_empty() {
        column_widths(header, data, r.size.x)
    } else {
        let asked: Vec<Option<f64>> = cols
            .iter()
            .enumerate()
            .map(|(col_idx, col)| {
                if col.auto_size {
                    let src = col.source.unwrap_or(col_idx);
                    let cells: Vec<&str> = data
                        .iter()
                        .filter_map(|row| row.get(src).and_then(|opt| opt.as_deref()))
                        .collect();
                    let pad = if let Some(pad) = col.padding {
                        p.medida(pad.left + pad.right)
                    } else {
                        12.0
                    };
                    let min = if col.min_width > 0.0 {
                        p.medida(col.min_width)
                    } else {
                        0.0
                    };
                    let max = col.max_width.map(|w| p.medida(w));
                    let font_size = (col.font.size > 0.0).then_some(col.font.size);
                    let full_title = col.design_title(&Marks::default());
                    let mut auto_w = 0.0;
                    p.with_font(font_size, None, |p| {
                        auto_w = crawrad_runtime::grid::auto_width(
                            &full_title,
                            &cells,
                            pad,
                            min,
                            max,
                            |s| p.measure(s).x,
                        );
                    });
                    Some(auto_w)
                } else {
                    col.width.map(|w| p.medida(w))
                }
            })
            .collect();
        let bounds: Vec<(f64, Option<f64>)> = cols
            .iter()
            .map(|col| {
                let min = if col.min_width > 0.0 {
                    p.medida(col.min_width)
                } else {
                    0.0
                };
                let max = col.max_width.map(|w| p.medida(w));
                (min, max)
            })
            .collect();
        share_widths_clamped(r.size.x, &asked, &bounds)
    };
    let footer = Rect {
        pos: dvec2(r.pos.x, r.pos.y + r.size.y - row_h),
        size: dvec2(r.size.x, row_h),
    };
    let has_summary = cols
        .iter()
        .any(|c| c.summary != "smNone" && !c.summary.is_empty());
    let summary_h = if has_summary { row_h } else { 0.0 };
    let summary_row = if has_summary {
        Some(Rect {
            pos: dvec2(r.pos.x, footer.pos.y - summary_h),
            size: dvec2(r.size.x, summary_h),
        })
    } else {
        None
    };
    let has_filter_row = cols.iter().any(|c| c.filterable);
    let filter_h = if has_filter_row { row_h } else { 0.0 };
    let mut header_h = row_h;
    for (col_idx, w) in widths.iter().enumerate() {
        if let Some(col) = cols.get(col_idx)
            && col.title_word_wrap
        {
            let pad_l = 6.0;
            let pad_r = 6.0;
            let avail_w = (w - pad_l - pad_r).max(0.0);
            let font_size = (col.title_font.size > 0.0).then_some(col.title_font.size);
            let full_title = col.design_title(&Marks::default());
            p.with_font(font_size, None, |p| {
                let line_h = p.measure("Ag").y.max(1.0);
                let lines = wrap_text(&full_title, avail_w, 3, col.ellipsis, |s| p.measure(s).x);
                if lines.len() > 1 {
                    let needed = lines.len() as f64 * line_h + 8.0;
                    if needed > header_h {
                        header_h = needed;
                    }
                }
            });
        }
    }
    let row_y = |i: usize| {
        if i == 0 {
            r.pos.y
        } else {
            r.pos.y + header_h + filter_h + row_h * (i - 1) as f64
        }
    };
    let row_height_for = |i: usize| if i == 0 { header_h } else { row_h };
    // the header, on the panel colour, like the fixed row of the app's grid
    p.rounded_box(
        Rect {
            pos: dvec2(r.pos.x, r.pos.y),
            size: dvec2(r.size.x, header_h),
        },
        MAKEPAD_PANEL,
        MAKEPAD_PANEL,
        0.0,
    );
    if has_filter_row {
        let filter_y = r.pos.y + header_h;
        let filter_bar = Rect {
            pos: dvec2(r.pos.x, filter_y),
            size: dvec2(r.size.x, filter_h),
        };
        p.rounded_box(filter_bar, MAKEPAD_PANEL, MAKEPAD_PANEL, 0.0);
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, filter_y + filter_h - 1.0),
                size: dvec2(r.size.x, 1.0),
            },
            MAKEPAD_PANEL_BORDER,
        );
        let mut fx = r.pos.x;
        for (col_idx, w) in widths.iter().enumerate() {
            let col = &cols[col_idx];
            let cell = Rect {
                pos: dvec2(fx, filter_y),
                size: dvec2(*w, filter_h),
            };
            p.stroke(
                Rect {
                    pos: dvec2(cell.pos.x + cell.size.x - 1.0, cell.pos.y),
                    size: dvec2(1.0, cell.size.y),
                },
                MAKEPAD_PANEL_BORDER,
            );
            if col.filterable {
                let kind = crawrad_runtime::grid::effective_filter_kind(col);
                let inner = shrink(cell, 2.0);
                match kind {
                    "fkText" => p.rounded_box(inner, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0),
                    "fkList" => {
                        p.rounded_box(inner, MAKEPAD_PANEL, MAKEPAD_BORDER, 2.0);
                        let area = Rect {
                            pos: dvec2(inner.pos.x + 4.0, inner.pos.y),
                            size: dvec2((inner.size.x - 8.0).max(0.0), inner.size.y),
                        };
                        let caption =
                            format!("All{}", crawrad_runtime::grid::Marks::default().drop_down);
                        p.write_aligned(area, &caption, MAKEPAD_TEXT_DIM, "taLeftJustify");
                    }
                    "fkRange" => draw_filter_pair(p, inner, ("min", "max")),
                    "fkDate" => draw_filter_pair(p, inner, ("from", "to")),
                    _ => {}
                }
            } else {
                p.rounded_box(cell, MAKEPAD_PANEL, MAKEPAD_PANEL, 0.0);
            }
            fx += *w;
        }
    }
    let body_space = (footer.pos.y - summary_h - (r.pos.y + header_h + filter_h)).max(0.0);
    let body_rows = 1 + (body_space / row_h).floor().max(0.0) as usize;
    for i in 0..body_rows {
        let cur_h = row_height_for(i);
        let cur_y = row_y(i);
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, cur_y + cur_h - 1.0),
                size: dvec2(r.size.x, 1.0),
            },
            MAKEPAD_PANEL_BORDER,
        );
        let mut x = r.pos.x;
        for (col_idx, w) in widths.iter().enumerate() {
            let col = &cols[col_idx];
            let cell = Rect {
                pos: dvec2(x, cur_y),
                size: dvec2(*w, cur_h),
            };
            let sample_row = if i > 0 { data.get(i - 1) } else { None };
            let raw_cell = if i == 0 {
                None
            } else {
                sample_row.and_then(|row| {
                    let src = col.source.unwrap_or(col_idx);
                    row.get(src).and_then(|opt| opt.as_deref())
                })
            };
            let base_text = match i {
                0 => Some(col.design_title(&Marks::default())),
                _ => {
                    let src = col.source.unwrap_or(col_idx);
                    let kind = rows.kinds().get(src).map(String::as_str);
                    Some(cell_text(col, kind, raw_cell))
                }
            };
            let style = if i > 0
                && let Some(sample) = sample_row
            {
                crawrad_runtime::grid::design_cell_style(
                    col,
                    header,
                    sample,
                    base_text.as_deref().unwrap_or(""),
                )
            } else {
                None
            };
            if i == 0 && col.title_color.a > 0 {
                let col_color = [
                    col.title_color.r as f32 / 255.0,
                    col.title_color.g as f32 / 255.0,
                    col.title_color.b as f32 / 255.0,
                    col.title_color.a as f32 / 255.0,
                ];
                p.rounded_box(cell, col_color, col_color, 0.0);
            } else if i > 0 {
                if let Some(c) = style.as_ref().and_then(|st| st.color) {
                    let col_color = [
                        c.r as f32 / 255.0,
                        c.g as f32 / 255.0,
                        c.b as f32 / 255.0,
                        c.a as f32 / 255.0,
                    ];
                    p.rounded_box(cell, col_color, col_color, 0.0);
                } else if col.color.a > 0 {
                    let col_color = [
                        col.color.r as f32 / 255.0,
                        col.color.g as f32 / 255.0,
                        col.color.b as f32 / 255.0,
                        col.color.a as f32 / 255.0,
                    ];
                    p.rounded_box(cell, col_color, col_color, 0.0);
                }
            }
            let text = match &style {
                Some(st) if !st.text.is_empty() => Some(st.text.clone()),
                _ => base_text,
            };
            if let Some(t) = text.filter(|t| !t.is_empty()) {
                let (pad_l, pad_t, pad_r, pad_b) = if i > 0
                    && let Some(pad) = col.padding
                {
                    (
                        p.medida(pad.left),
                        p.medida(pad.top),
                        p.medida(pad.right),
                        p.medida(pad.bottom),
                    )
                } else {
                    (6.0, 0.0, 6.0, 0.0)
                };
                let align = if i == 0 { col.title_align } else { col.align };
                let align_str = match align {
                    ColumnAlign::Left => "taLeftJustify",
                    ColumnAlign::Center => "taCenter",
                    ColumnAlign::Right => "taRightJustify",
                };
                let avail_w = (cell.size.x - pad_l - pad_r).max(0.0);
                let avail_h = (cell.size.y - pad_t - pad_b).max(0.0);
                let font_size = if i == 0 {
                    (col.title_font.size > 0.0).then_some(col.title_font.size)
                } else {
                    (col.font.size > 0.0).then_some(col.font.size)
                };
                let font_color = if i == 0 {
                    (col.title_font.color.a > 0).then(|| {
                        [
                            col.title_font.color.r as f32 / 255.0,
                            col.title_font.color.g as f32 / 255.0,
                            col.title_font.color.b as f32 / 255.0,
                            col.title_font.color.a as f32 / 255.0,
                        ]
                    })
                } else if let Some(tc) = style.as_ref().and_then(|st| st.text_color) {
                    Some([
                        tc.r as f32 / 255.0,
                        tc.g as f32 / 255.0,
                        tc.b as f32 / 255.0,
                        tc.a as f32 / 255.0,
                    ])
                } else if col.font.color.a > 0 {
                    Some([
                        col.font.color.r as f32 / 255.0,
                        col.font.color.g as f32 / 255.0,
                        col.font.color.b as f32 / 255.0,
                        col.font.color.a as f32 / 255.0,
                    ])
                } else if col.text_color.a > 0 {
                    Some([
                        col.text_color.r as f32 / 255.0,
                        col.text_color.g as f32 / 255.0,
                        col.text_color.b as f32 / 255.0,
                        col.text_color.a as f32 / 255.0,
                    ])
                } else {
                    None
                };
                p.with_font(font_size, font_color, |p| {
                    let line_h = p.measure("Ag").y.max(1.0);
                    if i == 0 {
                        if col.title_word_wrap {
                            let lines = wrap_text(&t, avail_w, 3, col.ellipsis, |s| p.measure(s).x);
                            let total_text_h = lines.len() as f64 * line_h;
                            let start_y =
                                cell.pos.y + ((cell.size.y - total_text_h) * 0.5).max(0.0);
                            for (l_idx, line_str) in lines.iter().enumerate() {
                                let line_y = start_y + l_idx as f64 * line_h;
                                let line_area = Rect {
                                    pos: dvec2(cell.pos.x + pad_l, line_y),
                                    size: dvec2(avail_w, line_h),
                                };
                                p.write_aligned(line_area, line_str, MAKEPAD_TEXT, align_str);
                            }
                        } else {
                            let area = Rect {
                                pos: dvec2(cell.pos.x + pad_l, cell.pos.y),
                                size: dvec2(avail_w, cell.size.y),
                            };
                            p.write_aligned(area, &t, MAKEPAD_TEXT, align_str);
                        }
                    } else {
                        match col.renderer {
                            CellRenderer::CheckBox => {
                                if let Some(checked) =
                                    raw_cell.and_then(crawrad_runtime::grid::checkbox_state)
                                {
                                    let box_size = 14.0_f64.min(avail_h);
                                    let box_x = match align {
                                        ColumnAlign::Left => cell.pos.x + pad_l,
                                        ColumnAlign::Center => {
                                            cell.pos.x + pad_l + (avail_w - box_size) * 0.5
                                        }
                                        ColumnAlign::Right => {
                                            cell.pos.x + pad_l + avail_w - box_size
                                        }
                                    };
                                    let box_y = match col.valign {
                                        ColumnVAlign::Top => cell.pos.y + pad_t,
                                        ColumnVAlign::Center => {
                                            cell.pos.y + pad_t + (avail_h - box_size) * 0.5
                                        }
                                        ColumnVAlign::Bottom => {
                                            cell.pos.y + cell.size.y - pad_b - box_size
                                        }
                                    };
                                    let box_rect = Rect {
                                        pos: dvec2(box_x, box_y),
                                        size: dvec2(box_size, box_size),
                                    };
                                    p.rounded_box(
                                        box_rect,
                                        MAKEPAD_WIDGET_INPUT,
                                        MAKEPAD_PANEL_BORDER,
                                        2.0,
                                    );
                                    if checked {
                                        let inner = Rect {
                                            pos: dvec2(box_x + 3.0, box_y + 3.0),
                                            size: dvec2(box_size - 6.0, box_size - 6.0),
                                        };
                                        p.rounded_box(inner, MAKEPAD_ACCENT, MAKEPAD_ACCENT, 1.0);
                                    }
                                    return;
                                }
                            }
                            CellRenderer::ProgressBar => {
                                if let Some(frac) =
                                    raw_cell.and_then(crawrad_runtime::grid::progress_fraction)
                                {
                                    let bar_h = 16.0_f64.min(avail_h);
                                    let bar_rect = Rect {
                                        pos: dvec2(
                                            cell.pos.x + pad_l,
                                            cell.pos.y + pad_t + (avail_h - bar_h) * 0.5,
                                        ),
                                        size: dvec2(avail_w, bar_h),
                                    };
                                    p.rounded_box(
                                        bar_rect,
                                        MAKEPAD_WIDGET_INPUT,
                                        MAKEPAD_PANEL_BORDER,
                                        2.0,
                                    );
                                    let fill_w = avail_w * frac;
                                    if fill_w > 0.0 {
                                        p.rounded_box(
                                            Rect {
                                                pos: bar_rect.pos,
                                                size: dvec2(fill_w, bar_h),
                                            },
                                            MAKEPAD_ACCENT,
                                            MAKEPAD_ACCENT,
                                            2.0,
                                        );
                                    }
                                    p.write_aligned(bar_rect, &t, MAKEPAD_TEXT, "taCenter");
                                    return;
                                }
                            }
                            CellRenderer::ColourSwatch => {
                                if let Some(c) =
                                    raw_cell.and_then(crawrad_runtime::grid::parse_hex_color)
                                {
                                    let swatch_size = 12.0_f64.min(avail_h);
                                    let swatch_y =
                                        cell.pos.y + pad_t + (avail_h - swatch_size) * 0.5;
                                    let swatch_rect = Rect {
                                        pos: dvec2(cell.pos.x + pad_l, swatch_y),
                                        size: dvec2(swatch_size, swatch_size),
                                    };
                                    let c_col = [
                                        c.r as f32 / 255.0,
                                        c.g as f32 / 255.0,
                                        c.b as f32 / 255.0,
                                        c.a as f32 / 255.0,
                                    ];
                                    p.rounded_box(swatch_rect, c_col, MAKEPAD_PANEL_BORDER, 1.0);
                                    let rem_w = (avail_w - swatch_size - 6.0).max(0.0);
                                    let fitted = crawrad_runtime::grid::fit_text(
                                        &t,
                                        rem_w,
                                        col.ellipsis,
                                        |s| p.measure(s).x,
                                    );
                                    let text_h = p.measure(&fitted).y;
                                    let y_pos = match col.valign {
                                        ColumnVAlign::Top => cell.pos.y + pad_t,
                                        ColumnVAlign::Bottom => {
                                            cell.pos.y + cell.size.y - pad_b - text_h
                                        }
                                        ColumnVAlign::Center => {
                                            cell.pos.y + pad_t + ((avail_h - text_h) * 0.5).max(0.0)
                                        }
                                    };
                                    let text_area = Rect {
                                        pos: dvec2(cell.pos.x + pad_l + swatch_size + 6.0, y_pos),
                                        size: dvec2(rem_w, text_h),
                                    };
                                    p.write_aligned(
                                        text_area,
                                        &fitted,
                                        MAKEPAD_TEXT,
                                        "taLeftJustify",
                                    );
                                    return;
                                }
                            }
                            CellRenderer::Link => {
                                let fitted = crawrad_runtime::grid::fit_text(
                                    &t,
                                    avail_w,
                                    col.ellipsis,
                                    |s| p.measure(s).x,
                                );
                                let text_size = p.measure(&fitted);
                                let y_pos = match col.valign {
                                    ColumnVAlign::Top => cell.pos.y + pad_t,
                                    ColumnVAlign::Bottom => {
                                        cell.pos.y + cell.size.y - pad_b - text_size.y
                                    }
                                    ColumnVAlign::Center => {
                                        cell.pos.y
                                            + pad_t
                                            + ((avail_h - text_size.y) * 0.5).max(0.0)
                                    }
                                };
                                let area = Rect {
                                    pos: dvec2(cell.pos.x + pad_l, y_pos),
                                    size: dvec2(avail_w, text_size.y),
                                };
                                p.write_aligned(area, &fitted, MAKEPAD_ACCENT, align_str);
                                let line_x = match align {
                                    ColumnAlign::Left => area.pos.x,
                                    ColumnAlign::Center => {
                                        area.pos.x + (avail_w - text_size.x) * 0.5
                                    }
                                    ColumnAlign::Right => area.pos.x + avail_w - text_size.x,
                                };
                                let underline_y = y_pos + text_size.y - 1.0;
                                p.stroke(
                                    Rect {
                                        pos: dvec2(line_x, underline_y),
                                        size: dvec2(text_size.x.min(avail_w), 1.0),
                                    },
                                    MAKEPAD_ACCENT,
                                );
                                return;
                            }
                            CellRenderer::Text => {}
                        }

                        if col.word_wrap {
                            let max_lines = (avail_h / line_h).floor().max(1.0) as usize;
                            let lines =
                                wrap_text(&t, avail_w, max_lines, col.ellipsis, |s| p.measure(s).x);
                            let total_text_h = lines.len() as f64 * line_h;
                            let start_y = match col.valign {
                                ColumnVAlign::Top => cell.pos.y + pad_t,
                                ColumnVAlign::Bottom => {
                                    cell.pos.y + cell.size.y - pad_b - total_text_h
                                }
                                ColumnVAlign::Center => {
                                    cell.pos.y + pad_t + ((avail_h - total_text_h) * 0.5).max(0.0)
                                }
                            };
                            for (l_idx, line_str) in lines.iter().enumerate() {
                                let line_y = start_y + l_idx as f64 * line_h;
                                let line_area = Rect {
                                    pos: dvec2(cell.pos.x + pad_l, line_y),
                                    size: dvec2(avail_w, line_h),
                                };
                                p.write_aligned(line_area, line_str, MAKEPAD_TEXT, align_str);
                            }
                        } else {
                            let fitted =
                                crawrad_runtime::grid::fit_text(&t, avail_w, col.ellipsis, |s| {
                                    p.measure(s).x
                                });
                            let text_h = p.measure(&fitted).y;
                            let y_pos = match col.valign {
                                ColumnVAlign::Top => cell.pos.y + pad_t,
                                ColumnVAlign::Bottom => cell.pos.y + cell.size.y - pad_b - text_h,
                                ColumnVAlign::Center => {
                                    cell.pos.y + pad_t + ((avail_h - text_h) * 0.5).max(0.0)
                                }
                            };
                            let area = Rect {
                                pos: dvec2(cell.pos.x + pad_l, y_pos),
                                size: dvec2(avail_w, text_h),
                            };
                            p.write_aligned(area, &fitted, MAKEPAD_TEXT, align_str);
                            if style.as_ref().is_some_and(|st| st.bold) {
                                let bold_area = Rect {
                                    pos: dvec2(area.pos.x + 0.5, area.pos.y),
                                    ..area
                                };
                                p.write_aligned(bold_area, &fitted, MAKEPAD_TEXT, align_str);
                            }
                        }
                    }
                });
            }
            if col_idx + 1 < widths.len() {
                p.stroke(
                    Rect {
                        pos: dvec2(x + w - 1.0, cell.pos.y),
                        size: dvec2(1.0, cur_h),
                    },
                    MAKEPAD_PANEL_BORDER,
                );
            }
            x += w;
        }
    }
    if let Some(sum_rect) = summary_row {
        p.rounded_box(sum_rect, MAKEPAD_PANEL, MAKEPAD_PANEL, 0.0);
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, sum_rect.pos.y),
                size: dvec2(r.size.x, 1.0),
            },
            MAKEPAD_PANEL_BORDER,
        );
        let mut x = r.pos.x;
        for (col_idx, w) in widths.iter().enumerate() {
            let col = &cols[col_idx];
            let cell = Rect {
                pos: dvec2(x, sum_rect.pos.y),
                size: dvec2(*w, sum_rect.size.y),
            };
            if col.summary != "smNone" && !col.summary.is_empty() {
                let src = col.source.unwrap_or(col_idx);
                let col_values: Vec<Option<&str>> = data
                    .iter()
                    .map(|row| row.get(src).and_then(|opt| opt.as_deref()))
                    .collect();
                let agg_val = crawrad_runtime::grid::summarize(&col_values, &col.summary);
                let sum_str = crawrad_runtime::grid::summary_text(col, agg_val);
                if !sum_str.is_empty() {
                    let pad_l = 6.0;
                    let pad_r = 6.0;
                    let avail_w = (cell.size.x - pad_l - pad_r).max(0.0);
                    let align_str = match col.align {
                        ColumnAlign::Left => "taLeftJustify",
                        ColumnAlign::Center => "taCenter",
                        ColumnAlign::Right => "taRightJustify",
                    };
                    let area = Rect {
                        pos: dvec2(cell.pos.x + pad_l, cell.pos.y),
                        size: dvec2(avail_w, cell.size.y),
                    };
                    p.write_aligned(area, &sum_str, MAKEPAD_TEXT, align_str);
                }
            }
            if col_idx + 1 < widths.len() {
                p.stroke(
                    Rect {
                        pos: dvec2(x + w - 1.0, cell.pos.y),
                        size: dvec2(1.0, sum_rect.size.y),
                    },
                    MAKEPAD_PANEL_BORDER,
                );
            }
            x += w;
        }
    }
    if let GridRows::Offline { reason, .. } = rows {
        let body = Rect {
            pos: dvec2(r.pos.x, r.pos.y + header_h),
            size: dvec2(
                r.size.x,
                (footer.pos.y - summary_h - (r.pos.y + header_h)).max(0.0),
            ),
        };
        let words = if rows.columns().is_empty() {
            format!("No rows at design time: {reason}.")
        } else {
            format!("No rows at design time: {reason}. Columns come from the last successful read.")
        };
        let w = (p.measure(&words).x + 24.0)
            .min(body.size.x - 16.0)
            .max(0.0);
        let h = row_h * 1.4;
        let warning = Rect {
            pos: dvec2(
                body.pos.x + (body.size.x - w) * 0.5,
                body.pos.y + ((body.size.y - h) * 0.5).max(0.0),
            ),
            size: dvec2(w, h),
        };
        p.rounded_box(warning, ROWS_FOOTER, ROWS_WARNING_EDGE, 4.0);
        p.write_aligned(shrink(warning, 12.0), &words, ROWS_WARNING_TEXT, "taCenter");
    }
    p.rounded_box(footer, ROWS_FOOTER, ROWS_FOOTER, 0.0);
    let words = rows.footer();
    let text_w = p.measure(&words).x.min(footer.size.x - 28.0).max(0.0);
    let text = Rect {
        pos: dvec2(footer.pos.x + footer.size.x - 8.0 - text_w, footer.pos.y),
        size: dvec2(text_w, footer.size.y),
    };
    let dot = 7.0;
    let dot_r = Rect {
        pos: dvec2(text.pos.x - dot - 6.0, footer.pos.y + (row_h - dot) * 0.5),
        size: dvec2(dot, dot),
    };
    let colour = match rows {
        GridRows::Ready { .. } => ROWS_READY,
        _ => ROWS_WAITING,
    };
    p.rounded_box(dot_r, colour, colour, dot * 0.5);
    p.write_aligned(text, &words, MAKEPAD_TEXT_DIM, "taLeftJustify");
}

/// The columns share the grid's width by how long their words are.
fn column_widths(header: &[String], rows: &[Vec<Option<String>>], width: f64) -> Vec<f64> {
    let titles: Vec<&str> = header.iter().map(String::as_str).collect();
    crawrad_runtime::grid::text_share_widths(
        &titles,
        |col| {
            rows.iter()
                .filter_map(|row| row.get(col).and_then(|opt| opt.as_deref()))
                .collect()
        },
        width,
    )
}

/// Two input boxes with "–" between: the design look of an fkRange/fkDate filter.
fn draw_filter_pair(p: &mut Brush, inner: Rect, hints: (&str, &str)) {
    let sep_w = 14.0;
    let box_w = ((inner.size.x - sep_w) / 2.0).max(10.0);
    let boxes = [
        Rect {
            pos: inner.pos,
            size: dvec2(box_w, inner.size.y),
        },
        Rect {
            pos: dvec2(inner.pos.x + box_w + sep_w, inner.pos.y),
            size: dvec2(box_w, inner.size.y),
        },
    ];
    let dash = Rect {
        pos: dvec2(inner.pos.x + box_w, inner.pos.y),
        size: dvec2(sep_w, inner.size.y),
    };
    p.write_aligned(dash, "–", MAKEPAD_TEXT_DIM, "taCenter");
    for (b, hint) in boxes.into_iter().zip([hints.0, hints.1]) {
        p.rounded_box(b, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let text = Rect {
            pos: dvec2(b.pos.x + 3.0, b.pos.y),
            size: dvec2((b.size.x - 6.0).max(0.0), b.size.y),
        };
        p.write_aligned(text, hint, MAKEPAD_TEXT_DIM, "taLeftJustify");
    }
}
