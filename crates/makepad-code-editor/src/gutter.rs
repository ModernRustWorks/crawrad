use {
    crate::{document::LineChange, layout::BlockElement, session::CodeSession, str::StrExt},
    makepad_widgets::*,
    std::fmt::Write,
};

/// Computes the total gutter width: glyph margin, digit column and right padding.
pub fn calculate_gutter_width(
    session: &CodeSession,
    glyph_margin: f64,
    gutter_pad: usize,
    cell_size_x: f64,
    show_gutter: bool,
) -> (f64, usize) {
    if !show_gutter {
        return (0.0, 0);
    }
    let gutter_chars = session
        .document()
        .as_text()
        .as_lines()
        .len()
        .to_string()
        .column_count()
        + 1;
    let width =
        glyph_margin + (gutter_chars as f64 * cell_size_x) + (gutter_pad as f64 * cell_size_x);
    (width, gutter_chars)
}

/// Draws the gutter layer: glyph margin and right-aligned line numbers.
/// Line numbers for caret lines in `highlighted_lines` are drawn with
/// `draw_gutter_current_line` (brighter and bold).
/// Where and how the line numbers of the visible lines are drawn.
#[derive(Clone, Copy)]
pub struct GutterLines<'a> {
    pub line_start: usize,
    pub line_end: usize,
    pub gutter_rect: Rect,
    pub glyph_margin: f64,
    pub gutter_chars: usize,
    pub cell_size: DVec2,
    pub cell_offset_y: f64,
    /// The caret's lines: their numbers take the current-line style.
    pub highlighted_lines: &'a [usize],
    /// The lines with a diagnostic message: their numbers take its colour.
    pub message_numbers: &'a [(usize, Vec4)],
    /// Line numbers during diff review (None for Removed rows; empty slice when no review).
    pub review_numbers: Option<&'a [Option<usize>]>,
}

pub fn draw_gutter_layer(
    cx: &mut Cx2d,
    session: &CodeSession,
    lines: GutterLines,
    draw_gutter: &mut DrawText,
    draw_gutter_current_line: &mut DrawText,
) {
    let GutterLines {
        line_start,
        line_end,
        gutter_rect,
        glyph_margin,
        gutter_chars,
        cell_size,
        cell_offset_y,
        highlighted_lines,
        message_numbers,
        review_numbers,
    } = lines;
    let total_lines = session.document().as_text().as_lines().len();
    if total_lines == 0 || line_start >= total_lines || line_start >= line_end {
        return;
    }
    let mut line_index = line_start;
    let mut origin_y = session.layout().top_of(line_start);
    let mut buf = String::new();
    let num_col_width = gutter_chars as f64 * cell_size.x;

    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                // A collapsed note link has no height and draws nothing.
                if line.is_collapsed() {
                    line_index += 1;
                    continue;
                }
                let is_current = highlighted_lines.binary_search(&line_index).is_ok();
                let drawer = if is_current {
                    &mut *draw_gutter_current_line
                } else {
                    &mut *draw_gutter
                };
                drawer.font_scale = line.scale() as f32;
                let plain = drawer.color;
                if let Some((_, color)) = message_numbers.iter().find(|(l, _)| *l == line_index) {
                    drawer.color = *color;
                }
                buf.clear();
                let has_num = match review_numbers {
                    Some(nums) => match nums.get(line_index) {
                        Some(Some(n)) => {
                            let _ = write!(buf, "{}", n);
                            true
                        }
                        _ => false,
                    },
                    None => {
                        let _ = write!(buf, "{}", line_index + 1);
                        true
                    }
                };
                if has_num {
                    let digits_width = buf.len() as f64 * cell_size.x * line.scale();
                    let num_x = gutter_rect.pos.x
                        + glyph_margin
                        + (num_col_width * line.scale() - digits_width);
                    drawer.draw_abs(
                        cx,
                        DVec2 {
                            x: num_x,
                            y: origin_y * cell_size.y + gutter_rect.pos.y + cell_offset_y,
                        },
                        &buf,
                    );
                }
                drawer.color = plain;
                line_index += 1;
                origin_y += line.height();
            }
            other => origin_y += other.height(),
        }
    }
}

/// Draws the gutter marks (breakpoint dots) of the visible lines in the glyph
/// margin, centred on each line.
pub fn draw_gutter_marks(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    gutter_rect: Rect,
    glyph_margin: f64,
    cell_size: DVec2,
    draw_mark: &mut DrawColor,
    hover: Option<(usize, &mut DrawColor)>,
) {
    let marks = session.document().gutter_marks();
    let breakpoints = marks.lines(crate::gutter_mark::GutterMarkKind::Breakpoint);
    let (hover_line, mut draw_hint) = match hover {
        Some((line, draw)) if !breakpoints.contains(&line) => (Some(line), Some(draw)),
        _ => (None, None),
    };
    if breakpoints.is_empty() && hover_line.is_none() {
        return;
    }
    let total_lines = session.document().as_text().as_lines().len();
    if total_lines == 0 || line_start >= total_lines || line_start >= line_end {
        return;
    }
    let mut line_index = line_start;
    let mut origin_y = session.layout().line(line_start).y();
    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                // A collapsed note link has no height and draws nothing.
                if line.is_collapsed() {
                    line_index += 1;
                    continue;
                }
                let height = line.height() * cell_size.y;
                let size = (cell_size.y * line.scale() * 0.8).min(glyph_margin * 0.6);
                let dot = Rect {
                    pos: DVec2 {
                        x: gutter_rect.pos.x + (glyph_margin - size) * 0.5,
                        y: origin_y * cell_size.y + gutter_rect.pos.y + (height - size) * 0.5,
                    },
                    size: DVec2 { x: size, y: size },
                };
                if breakpoints.contains(&line_index) {
                    draw_mark.draw_abs(cx, dot);
                } else if hover_line == Some(line_index)
                    && let Some(draw) = draw_hint.as_deref_mut()
                {
                    draw.draw_abs(cx, dot);
                }
                line_index += 1;
                origin_y += line.height();
            }
            other => origin_y += other.height(),
        }
    }
}

/// The width of a change bar, in pixels.
const CHANGE_BAR_WIDTH: f64 = 3.0;

/// Draws the change bars of the visible lines: a thin full-height bar at the
/// right edge of the gutter, between the numbers and the text — green for a
/// line changed and saved in this session, yellow for one not saved yet.
pub fn draw_change_bars(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    gutter_rect: Rect,
    cell_size: DVec2,
    (draw_saved, draw_unsaved): (&mut DrawColor, &mut DrawColor),
) {
    let changes = session.document().line_changes();
    if changes.is_empty() || line_start >= line_end {
        return;
    }
    let x = gutter_rect.pos.x + gutter_rect.size.x - CHANGE_BAR_WIDTH - cell_size.x * 0.5;
    let mut line_index = line_start;
    let mut origin_y = session.layout().line(line_start).y();
    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                // A collapsed note link has no height and draws nothing.
                if line.is_collapsed() {
                    line_index += 1;
                    continue;
                }
                if let Ok(i) = changes.binary_search_by_key(&line_index, |(l, _)| *l) {
                    let draw = match changes[i].1 {
                        LineChange::Saved => &mut *draw_saved,
                        LineChange::Unsaved => &mut *draw_unsaved,
                    };
                    draw.draw_abs(
                        cx,
                        Rect {
                            pos: DVec2 {
                                x,
                                y: origin_y * cell_size.y + gutter_rect.pos.y,
                            },
                            size: DVec2 {
                                x: CHANGE_BAR_WIDTH,
                                y: line.height() * cell_size.y,
                            },
                        },
                    );
                }
                line_index += 1;
                origin_y += line.height();
            }
            other => origin_y += other.height(),
        }
    }
}

/// Draws the numbered bookmarks: a small red flag at the left of the glyph
/// margin with its number on it, as classic RAD IDEs show them.
pub fn draw_bookmarks(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    gutter_rect: Rect,
    cell_size: DVec2,
    (draw_flag, draw_digit): (&mut DrawColor, &mut DrawText),
) {
    let bookmarks = session.document().bookmarks();
    if bookmarks.is_empty() {
        return;
    }
    let mut line_index = line_start;
    let mut origin_y = session.layout().line(line_start).y();
    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                // A collapsed note link has no height and draws nothing.
                if line.is_collapsed() {
                    line_index += 1;
                    continue;
                }
                if let Some((n, _)) = bookmarks.iter().find(|(_, l)| *l == line_index) {
                    let height = line.height() * cell_size.y;
                    let h = (height * 0.86).max(8.0);
                    let w = (h * 0.72).max(8.0);
                    let pos = DVec2 {
                        x: gutter_rect.pos.x + 1.0,
                        y: origin_y * cell_size.y + gutter_rect.pos.y + (height - h) * 0.5,
                    };
                    draw_flag.draw_abs(
                        cx,
                        Rect {
                            pos,
                            size: DVec2 { x: w, y: h },
                        },
                    );
                    let digit = n.to_string();
                    draw_digit.draw_abs(
                        cx,
                        pos + DVec2 {
                            x: w * 0.24,
                            y: h * 0.08,
                        },
                        &digit,
                    );
                }
                line_index += 1;
                origin_y += line.height();
            }
            other => origin_y += other.height(),
        }
    }
}

/// Draws where the program under the debugger stands: a band across the line
/// and the yellow arrow in the glyph margin, over the breakpoint dot.
pub fn draw_execution_point(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    gutter_rect: Rect,
    glyph_margin: f64,
    cell_size: DVec2,
    band_width: f64,
    draw_arrow: &mut DrawColor,
    draw_band: &mut DrawColor,
) {
    let Some(target) = session
        .document()
        .gutter_mark_lines(crate::gutter_mark::GutterMarkKind::ExecutionPoint)
        .first()
        .copied()
    else {
        return;
    };
    if target < line_start || target >= line_end {
        return;
    }
    let mut line_index = line_start;
    let mut origin_y = session.layout().line(line_start).y();
    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                // A collapsed note link has no height and draws nothing.
                if line.is_collapsed() {
                    line_index += 1;
                    continue;
                }
                if line_index == target {
                    let height = line.height() * cell_size.y;
                    let y = origin_y * cell_size.y + gutter_rect.pos.y;
                    draw_band.draw_abs(
                        cx,
                        Rect {
                            pos: DVec2 {
                                x: gutter_rect.pos.x,
                                y,
                            },
                            size: DVec2 {
                                x: band_width + gutter_rect.size.x,
                                y: height,
                            },
                        },
                    );
                    let w = glyph_margin * 0.62;
                    let h = (height * 0.7).min(w);
                    draw_arrow.draw_abs(
                        cx,
                        Rect {
                            pos: DVec2 {
                                x: gutter_rect.pos.x + (glyph_margin - w) * 0.5,
                                y: y + (height - h) * 0.5,
                            },
                            size: DVec2 { x: w, y: h },
                        },
                    );
                    return;
                }
                line_index += 1;
                origin_y += line.height();
            }
            other => origin_y += other.height(),
        }
    }
}
