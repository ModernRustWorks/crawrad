use {
    crate::{
        layout::{BlockElement, LENS_HEIGHT},
        session::CodeSession,
    },
    makepad_widgets::*,
};

/// A syntactic block delimited by braces `{ ... }`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeBlock {
    pub start_line: usize,
    pub end_line: usize,
    pub indent_col: usize,
    pub depth: usize,
    /// Column of the opening `{` on `start_line` — where the active scope's
    /// header underline ends.
    pub open_col: usize,
}

/// Six-colour rainbow palette for increasing nesting levels.
pub const SCOPE_COLORS: [Vec4; 6] = [
    Vec4 {
        x: 0.88,
        y: 0.88,
        z: 0.92,
        w: 0.40,
    }, // 0: Soft white / light grey
    Vec4 {
        x: 0.98,
        y: 0.56,
        z: 0.22,
        w: 0.70,
    }, // 1: Orange / coral
    Vec4 {
        x: 0.78,
        y: 0.52,
        z: 0.95,
        w: 0.70,
    }, // 2: Violet / lilac
    Vec4 {
        x: 0.95,
        y: 0.42,
        z: 0.62,
        w: 0.70,
    }, // 3: Pink / salmon
    Vec4 {
        x: 0.32,
        y: 0.78,
        z: 0.98,
        w: 0.70,
    }, // 4: Cyan / sky blue
    Vec4 {
        x: 0.96,
        y: 0.82,
        z: 0.26,
        w: 0.70,
    }, // 5: Gold / amber
];

/// Extracts blocks delimited by `{` and `}`, skipping strings and comments.
pub fn compute_scope_blocks(lines: &[String], tab_column_count: usize) -> Vec<ScopeBlock> {
    struct ScopeOpen {
        start_line: usize,
        indent_col: usize,
        open_col: usize,
    }

    let mut blocks = Vec::new();
    let mut stack: Vec<ScopeOpen> = Vec::new();
    let mut in_multiline_comment = false;
    let tab_cols = tab_column_count.max(1);

    for (line_idx, line_str) in lines.iter().enumerate() {
        let chars: Vec<char> = line_str.chars().collect();
        let len = chars.len();
        let mut i = 0;

        let mut leading_indent = 0;
        let mut found_non_ws = false;
        for &ch in &chars {
            if ch == ' ' {
                leading_indent += 1;
            } else if ch == '\t' {
                leading_indent = (leading_indent / tab_cols + 1) * tab_cols;
            } else {
                found_non_ws = true;
                break;
            }
        }
        if !found_non_ws {
            leading_indent = 0;
        }

        let mut in_string = false;
        let mut string_delim = '"';

        while i < len {
            let ch = chars[i];

            if in_multiline_comment {
                if ch == '*' && i + 1 < len && chars[i + 1] == '/' {
                    in_multiline_comment = false;
                    i += 2;
                    continue;
                }
                i += 1;
                continue;
            }

            if in_string {
                if ch == '\\' {
                    i += 2; // skip the escape
                    continue;
                }
                if ch == string_delim {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            if ch == '/' && i + 1 < len {
                if chars[i + 1] == '/' {
                    break; // the rest of the line is a comment
                } else if chars[i + 1] == '*' {
                    in_multiline_comment = true;
                    i += 2;
                    continue;
                }
            }

            if ch == '"' || ch == '\'' {
                if ch == '\'' {
                    if i + 2 < len && chars[i + 2] == '\'' {
                        i += 3;
                        continue;
                    } else if i + 3 < len && chars[i + 1] == '\\' && chars[i + 3] == '\'' {
                        i += 4;
                        continue;
                    }
                } else {
                    in_string = true;
                    string_delim = ch;
                    i += 1;
                    continue;
                }
            }

            if ch == '{' {
                stack.push(ScopeOpen {
                    start_line: line_idx,
                    indent_col: leading_indent,
                    open_col: i,
                });
            } else if ch == '}'
                && let Some(open) = stack.pop()
            {
                let depth = stack.len();
                blocks.push(ScopeBlock {
                    start_line: open.start_line,
                    end_line: line_idx,
                    indent_col: open.indent_col,
                    depth,
                    open_col: open.open_col,
                });
            }

            i += 1;
        }
    }

    blocks
}

/// Draws the highlight of the header of the function or block that contains the cursor.
pub fn draw_scope_header(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    viewport_rect: Rect,
    cell_size: DVec2,
    draw_scope_header: &mut DrawColor,
) {
    let text = session.document().as_text();
    let lines = text.as_lines();
    if lines.is_empty() || line_start >= lines.len() || line_start >= line_end {
        return;
    }
    let tab_column_count = session.settings().tab_column_count;
    let blocks = compute_scope_blocks(lines, tab_column_count);
    let selections = session.selections();
    let cursor_line = session
        .last_added_selection_index()
        .and_then(|idx| selections.get(idx))
        .map(|sel| sel.cursor.position.line_index)
        .unwrap_or(usize::MAX);

    let active_scope = blocks
        .iter()
        .filter(|b| b.start_line <= cursor_line && cursor_line <= b.end_line)
        .max_by_key(|b| b.depth);

    if let Some(active) = active_scope
        && active.start_line >= line_start
        && active.start_line <= line_end
    {
        let mut line_index = line_start;
        let mut origin_y = session.layout().top_of(line_start);
        for element in session.layout().block_elements(line_start, line_end) {
            match element {
                BlockElement::Line { line, .. } => {
                    if line_index == active.start_line {
                        let line_text = lines.get(line_index).map(|s| s.as_str()).unwrap_or("");
                        let trimmed_len = line_text.trim_end().len();
                        let (x_start, y) = line.grid_to_normalized_position(0, active.indent_col);
                        let (x_end, _) = line.grid_to_normalized_position(0, trimmed_len);
                        let width = ((x_end - x_start) * cell_size.x + 10.0).max(cell_size.x * 4.0);

                        draw_scope_header.color = Vec4 {
                            x: 0.12,
                            y: 0.32,
                            z: 0.62,
                            w: 0.65,
                        };
                        draw_scope_header.draw_abs(
                            cx,
                            Rect {
                                pos: DVec2 {
                                    x: x_start * cell_size.x + viewport_rect.pos.x - 4.0,
                                    y: (origin_y + y) * cell_size.y + viewport_rect.pos.y,
                                },
                                size: DVec2 {
                                    x: width,
                                    y: line.scale() * cell_size.y,
                                },
                            },
                        );
                        break;
                    }
                    line_index += 1;
                    origin_y += line.height();
                }
                BlockElement::Widget(widget) => {
                    origin_y += widget.height;
                }
                BlockElement::Lens { .. } => {
                    origin_y += LENS_HEIGHT;
                }
            }
        }
    }
}

/// Draws the vertical guides linking an opening brace to its matching closing brace.
pub fn draw_indent_guides(
    cx: &mut Cx2d,
    session: &CodeSession,
    line_start: usize,
    line_end: usize,
    viewport_rect: Rect,
    cell_size: DVec2,
    draw_indent_guide: &mut DrawColor,
) {
    let text = session.document().as_text();
    let lines = text.as_lines();
    if lines.is_empty() || line_start >= lines.len() || line_start >= line_end {
        return;
    }
    let tab_column_count = session.settings().tab_column_count;
    let blocks = compute_scope_blocks(lines, tab_column_count);
    let selections = session.selections();
    let cursor_line = session
        .last_added_selection_index()
        .and_then(|idx| selections.get(idx))
        .map(|sel| sel.cursor.position.line_index)
        .unwrap_or(usize::MAX);

    let active_scope_idx = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.start_line <= cursor_line && cursor_line <= b.end_line)
        .max_by_key(|(_, b)| b.depth)
        .map(|(idx, _)| idx);

    let mut line_index = line_start;
    let mut origin_y = session.layout().top_of(line_start);

    for element in session.layout().block_elements(line_start, line_end) {
        match element {
            BlockElement::Line { line, .. } => {
                for (scope_idx, b) in blocks.iter().enumerate() {
                    let is_active = Some(scope_idx) == active_scope_idx;

                    if line_index >= b.start_line && line_index <= b.end_line {
                        // The ACTIVE scope closes an "L" at its header: an
                        // underline from the guide to the `{`, drawn on the
                        // bottom edge of the header line, and the vertical
                        // guide starts below it (a modern editor's horizontal
                        // bracket-pair guide; owner's mockup, 30/09/2026).
                        if is_active && line_index == b.start_line {
                            let (x0, y0) = line.grid_to_normalized_position(0, b.indent_col);
                            let (x1, _) = line.grid_to_normalized_position(0, b.open_col + 1);
                            let mut color = SCOPE_COLORS[b.depth % SCOPE_COLORS.len()];
                            // A hairline, as the owner asked on 01/10/2026: it
                            // shows the block, it must not shout.
                            color.w = 0.75;
                            draw_indent_guide.color = color;
                            draw_indent_guide.draw_abs(
                                cx,
                                Rect {
                                    pos: DVec2 {
                                        x: x0 * cell_size.x + viewport_rect.pos.x,
                                        y: (origin_y + y0 + line.scale()) * cell_size.y
                                            + viewport_rect.pos.y
                                            - 1.0,
                                    },
                                    size: DVec2 {
                                        x: ((x1 - x0) * cell_size.x).max(2.0),
                                        y: 1.0,
                                    },
                                },
                            );
                            continue;
                        }
                        let (y_rel_start, height_factor) = if line_index == b.start_line {
                            (0.5, 0.5)
                        } else if line_index == b.end_line {
                            (0.0, 0.65)
                        } else {
                            (0.0, 1.0)
                        };

                        let (x, y) = line.grid_to_normalized_position(0, b.indent_col);
                        let mut color = SCOPE_COLORS[b.depth % SCOPE_COLORS.len()];
                        if is_active {
                            color.w = 0.85;
                            color.x = (color.x * 1.15).min(1.0);
                            color.y = (color.y * 1.15).min(1.0);
                            color.z = (color.z * 1.15).min(1.0);
                        } else {
                            color.w *= 0.65;
                        }

                        draw_indent_guide.color = color;
                        draw_indent_guide.draw_abs(
                            cx,
                            Rect {
                                pos: DVec2 {
                                    x: x * cell_size.x + viewport_rect.pos.x,
                                    y: (origin_y + y + y_rel_start * line.scale()) * cell_size.y
                                        + viewport_rect.pos.y,
                                },
                                size: DVec2 {
                                    x: 1.0,
                                    y: height_factor * line.scale() * cell_size.y,
                                },
                            },
                        );
                    }
                }
                line_index += 1;
                origin_y += line.height();
            }
            BlockElement::Widget(widget) => {
                origin_y += widget.height;
            }
            BlockElement::Lens { .. } => {
                origin_y += LENS_HEIGHT;
            }
        }
    }
}
