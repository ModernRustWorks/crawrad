//! **The minimap's model** — a line as indentation and coloured token runs,
//! and the slider's geometry. Pure, beside the tokenizer whose keywords it
//! uses; the IDE's minimap widget draws it.

use crate::is_keyword;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Comment,
    Keyword,
    Type,
    Function,
    String,
    Text,
}

#[derive(Clone, Debug)]
pub struct MinimapToken {
    pub len: u16,
    pub kind: TokenKind,
}

#[derive(Clone, Debug)]
pub struct MinimapLine {
    pub indent: u16,
    pub tokens: Vec<MinimapToken>,
}

/// The minimap's geometry: where the slider is and which scroll a click means.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimapMetrics {
    pub current_line_s: f64,
    pub current_minimap_scroll_y: f64,
    pub editor_visible_h: f64,
    pub total_lines: usize,
    pub cell_h: f64,
}

impl MinimapMetrics {
    pub fn new(
        line_s: f64,
        minimap_scroll_y: f64,
        visible_h: f64,
        cell_h: f64,
        total_lines: usize,
    ) -> Self {
        Self {
            current_line_s: if line_s > 0.0 { line_s } else { 2.0 },
            current_minimap_scroll_y: minimap_scroll_y.max(0.0),
            editor_visible_h: visible_h.max(10.0),
            total_lines: total_lines.max(1),
            cell_h: if cell_h > 0.0 { cell_h } else { 19.0 },
        }
    }

    /// Converts the relative Y coordinate of a minimap click or drag into editor scroll position.
    pub fn calcular_scroll_para_y(&self, rel_y: f64) -> (f64, usize) {
        let visible_lines = (self.editor_visible_h / self.cell_h).max(5.0);
        let doc_y = rel_y + self.current_minimap_scroll_y;
        let clicked_line = (doc_y / self.current_line_s).max(0.0);

        let target_top_line = (clicked_line - (visible_lines * 0.5)).max(0.0);
        let max_top_line = (self.total_lines as f64 - visible_lines).max(0.0);
        let target_top_line = target_top_line.min(max_top_line);

        let target_scroll_y = target_top_line * self.cell_h;
        (target_scroll_y, clicked_line as usize)
    }

    /// Calculates the Y position and height of the viewport slider on screen.
    pub fn calcular_slider_y_e_altura(
        &self,
        editor_scroll_y: f64,
        minimap_height: f64,
    ) -> (f64, f64) {
        let top_line_fractional = (editor_scroll_y / self.cell_h).max(0.0);
        let visible_lines_count = (self.editor_visible_h / self.cell_h).max(5.0);

        let slider_top_y =
            (top_line_fractional * self.current_line_s) - self.current_minimap_scroll_y;
        let slider_h = (visible_lines_count * self.current_line_s).clamp(18.0, minimap_height);
        let clamped_y = slider_top_y.clamp(0.0, (minimap_height - slider_h).max(0.0));
        (clamped_y, slider_h)
    }
}

impl MinimapLine {
    pub fn parse(line: &str) -> Self {
        let trimmed_start = line.trim_start();
        let indent = (line.len() - trimmed_start.len()).min(40) as u16;
        if trimmed_start.is_empty() {
            return Self {
                indent,
                tokens: Vec::new(),
            };
        }

        // If it is a comment line
        if trimmed_start.starts_with("//")
            || trimmed_start.starts_with("/*")
            || trimmed_start.starts_with('*')
            || trimmed_start.starts_with('#')
        {
            return Self {
                indent,
                tokens: vec![MinimapToken {
                    len: trimmed_start.len().min(120) as u16,
                    kind: TokenKind::Comment,
                }],
            };
        }

        let mut tokens = Vec::new();
        let mut chars = trimmed_start.char_indices().peekable();

        while let Some(&(i, c)) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }

            if c == '#' || (c == '/' && trimmed_start[i..].starts_with("//")) {
                tokens.push(MinimapToken {
                    len: (trimmed_start.len() - i).min(120) as u16,
                    kind: TokenKind::Comment,
                });
                break;
            }

            if c == '"' {
                chars.next();
                let mut end_byte = i + c.len_utf8();
                while let Some(&(si, sc)) = chars.peek() {
                    chars.next();
                    end_byte = si + sc.len_utf8();
                    if sc == '"' && !trimmed_start[..si].ends_with('\\') {
                        break;
                    }
                }
                tokens.push(MinimapToken {
                    len: (end_byte - i).min(80) as u16,
                    kind: TokenKind::String,
                });
                continue;
            }

            if c.is_alphanumeric() || c == '_' {
                let start = i;
                let mut end = i;
                while let Some(&(ci, wc)) = chars.peek() {
                    if wc.is_alphanumeric() || wc == '_' {
                        end = ci + wc.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                let word = &trimmed_start[start..end];

                // Identifies whether the word is followed by a parenthesis (function call or declaration)
                let mut is_func = false;
                let mut probe = chars.clone();
                while let Some(&(_, next_c)) = probe.peek() {
                    if next_c == '(' {
                        is_func = true;
                        break;
                    } else if next_c.is_whitespace() {
                        probe.next();
                    } else {
                        break;
                    }
                }

                let kind = if is_keyword(word) {
                    TokenKind::Keyword
                } else if word
                    .chars()
                    .next()
                    .is_some_and(|first| first.is_uppercase())
                {
                    TokenKind::Type
                } else if is_func {
                    TokenKind::Function
                } else {
                    TokenKind::Text
                };
                tokens.push(MinimapToken {
                    len: (end - start).min(60) as u16,
                    kind,
                });
                continue;
            }

            chars.next();
        }

        Self { indent, tokens }
    }
}
