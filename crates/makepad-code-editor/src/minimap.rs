//! Code editor minimap.
//!
//! Displays a high-fidelity panoramic overview of the entire source file to the right
//! of the editor, reproducing actual syntactic micro-density, proportional indentation, and a
//! translucent viewport slider synchronized in real time with Makepad's native
//! `CodeEditor` scrolling. Allows direct navigation by clicking or dragging the slider.

pub use crate::minimap_model::{MinimapLine, MinimapMetrics, TokenKind};
use makepad_widgets::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

live_design! {
    link crawrad_minimap;

    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub CrawradMinimap = {{CrawradMinimap}} {
        width: 80.0, height: Fill,
        draw_bg: {
            color: #1e1e2e
        }
        draw_border: {
            color: #313244
        }
        draw_code: {
            color: #9090aa
        }
        draw_slider: {
            color: #ffffff
        }
        draw_slider_border: {
            color: #ffffff
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CrawradMinimap {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_border: DrawColor,
    #[live]
    draw_code: DrawColor,
    #[live]
    draw_slider: DrawColor,
    #[live]
    draw_slider_border: DrawColor,

    #[area]
    #[rust]
    area: Area,
    #[rust]
    rect: Rect,

    #[rust]
    lines: Vec<MinimapLine>,
    #[rust]
    text_hash: u64,

    // Actual scroll state synchronized with the CodeEditor
    #[rust]
    pub editor_scroll_y: f64,
    #[rust]
    pub editor_total_h: f64,
    #[rust]
    pub editor_visible_h: f64,
    #[rust]
    pub cell_h: f64,
    #[rust]
    pub line_start: usize,
    #[rust]
    pub line_end: usize,
    #[rust]
    pub total_lines: usize,

    #[rust]
    current_line_s: f64,
    #[rust]
    current_minimap_scroll_y: f64,

    #[rust]
    is_hovered: bool,
    #[rust]
    is_dragging: bool,

    // Navigation signal requested by user interaction on the minimap
    #[rust]
    pub target_scroll_y: Option<f64>,
    #[rust]
    pub target_line: Option<usize>,
}

impl Widget for CrawradMinimap {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, Layout::default());
        let rect = cx.turtle().rect();
        self.rect = rect;

        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            cx.end_turtle_with_area(&mut self.area);
            return DrawStep::done();
        }

        // 1. Minimap background
        self.draw_bg.draw_abs(cx, rect);

        // 2. Left divider line (1px)
        let border_rect = Rect {
            pos: rect.pos,
            size: dvec2(1.0, rect.size.y),
        };
        self.draw_border.draw_abs(cx, border_rect);

        // 3. Code lines in micro-rendering
        let total_lines = self.lines.len().max(self.total_lines).max(1);
        if total_lines > 0 {
            // In a modern editor, each line occupies about 2.0px in the minimap
            let line_s = 2.0;
            let line_h = 1.3;
            let total_minimap_h = total_lines as f64 * line_s;

            // Proportional scrolling of the minimap when the file exceeds visible height
            let cell_h = if self.cell_h > 0.0 { self.cell_h } else { 19.0 };
            let max_editor_scroll =
                if self.editor_total_h > self.editor_visible_h && self.editor_total_h > 0.0 {
                    self.editor_total_h - self.editor_visible_h
                } else {
                    ((total_lines as f64 * cell_h) - self.editor_visible_h).max(1.0)
                };

            let scroll_ratio = (self.editor_scroll_y / max_editor_scroll).clamp(0.0, 1.0);

            let minimap_scroll_y = if total_minimap_h > rect.size.y {
                scroll_ratio * (total_minimap_h - rect.size.y)
            } else {
                0.0
            };

            self.current_line_s = line_s;
            self.current_minimap_scroll_y = minimap_scroll_y;

            // Frustum culling: only render lines intersecting the minimap window
            let first_visible = (minimap_scroll_y / line_s).floor().max(0.0) as usize;
            let last_visible = ((minimap_scroll_y + rect.size.y) / line_s).ceil() as usize;
            let last_visible = last_visible.min(self.lines.len());

            let base_x = rect.pos.x + 5.0;
            let max_x = rect.pos.x + rect.size.x - 5.0;

            for idx in first_visible..last_visible {
                let line_data = &self.lines[idx];
                let line_y = rect.pos.y + (idx as f64 * line_s) - minimap_scroll_y;

                // Proportional code indentation: 1.2px per space
                let indent_x = (line_data.indent as f64 * 1.2).min(32.0);
                let mut token_x = base_x + indent_x;

                for token in &line_data.tokens {
                    if token_x >= max_x {
                        break;
                    }
                    // Realistic proportional character width (0.72px/char)
                    let token_w = (token.len as f64 * 0.72).clamp(1.5, 45.0);
                    let available_w = (max_x - token_x).max(0.0);
                    let final_w = token_w.min(available_w);

                    if final_w > 0.0 {
                        let token_rect = Rect {
                            pos: dvec2(token_x, line_y),
                            size: dvec2(final_w, line_h),
                        };

                        // Soft and refined palette with transparency identical to a modern editor
                        self.draw_code.color = match token.kind {
                            TokenKind::Comment => vec4(0.38, 0.58, 0.35, 0.70),
                            TokenKind::Keyword => vec4(0.75, 0.48, 0.68, 0.85),
                            TokenKind::Type => vec4(0.32, 0.75, 0.70, 0.85),
                            TokenKind::Function => vec4(0.85, 0.82, 0.52, 0.85),
                            TokenKind::String => vec4(0.80, 0.58, 0.42, 0.80),
                            TokenKind::Text => vec4(0.65, 0.68, 0.75, 0.65),
                        };
                        self.draw_code.draw_abs(cx, token_rect);
                    }

                    token_x += final_w + 1.2;
                }
            }

            // 4. Viewport slider (visible editor window synchronized in real time)
            let metrics = MinimapMetrics::new(
                line_s,
                minimap_scroll_y,
                self.editor_visible_h,
                cell_h,
                total_lines,
            );
            let (rel_slider_y, slider_h) =
                metrics.calcular_slider_y_e_altura(self.editor_scroll_y, rect.size.y);
            let slider_x = rect.pos.x + 2.0;
            let slider_w = (rect.size.x - 4.0).max(8.0);
            let slider_rect = Rect {
                pos: dvec2(slider_x, rect.pos.y + rel_slider_y),
                size: dvec2(slider_w, slider_h),
            };

            // Translucent fill with focus highlight on hover or drag
            let fill_color = if self.is_dragging {
                vec4(1.0, 1.0, 1.0, 0.18)
            } else if self.is_hovered {
                vec4(1.0, 1.0, 1.0, 0.12)
            } else {
                vec4(1.0, 1.0, 1.0, 0.08)
            };
            self.draw_slider.color = fill_color;
            self.draw_slider.draw_abs(cx, slider_rect);

            // Subtle 1px border around the visible area
            let border_color = if self.is_dragging {
                vec4(1.0, 1.0, 1.0, 0.38)
            } else if self.is_hovered {
                vec4(1.0, 1.0, 1.0, 0.25)
            } else {
                vec4(1.0, 1.0, 1.0, 0.16)
            };
            self.draw_slider_border.color = border_color;
            // Top border
            self.draw_slider_border.draw_abs(
                cx,
                Rect {
                    pos: slider_rect.pos,
                    size: dvec2(slider_rect.size.x, 1.0),
                },
            );
            // Bottom border
            self.draw_slider_border.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        slider_rect.pos.x,
                        slider_rect.pos.y + slider_rect.size.y - 1.0,
                    ),
                    size: dvec2(slider_rect.size.x, 1.0),
                },
            );
            // Left border
            self.draw_slider_border.draw_abs(
                cx,
                Rect {
                    pos: slider_rect.pos,
                    size: dvec2(1.0, slider_rect.size.y),
                },
            );
            // Right border
            self.draw_slider_border.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        slider_rect.pos.x + slider_rect.size.x - 1.0,
                        slider_rect.pos.y,
                    ),
                    size: dvec2(1.0, slider_rect.size.y),
                },
            );
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                self.is_hovered = true;
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.is_hovered = false;
                self.redraw(cx);
            }
            Hit::FingerDown(fe) => {
                self.is_dragging = true;
                let rel_y = fe.abs.y - self.rect.pos.y;
                self.calcular_scroll_para_y(rel_y);
                self.redraw(cx);
            }
            Hit::FingerMove(fe) if self.is_dragging => {
                let rel_y = fe.abs.y - self.rect.pos.y;
                self.calcular_scroll_para_y(rel_y);
                self.redraw(cx);
            }
            Hit::FingerUp(_) => {
                self.is_dragging = false;
                self.redraw(cx);
            }
            _ => {}
        }
    }
}

impl CrawradMinimap {
    pub fn update_lines(&mut self, texto: &str) -> bool {
        let mut hasher = DefaultHasher::new();
        texto.hash(&mut hasher);
        let novo_hash = hasher.finish();
        if novo_hash == self.text_hash && !self.lines.is_empty() {
            return false;
        }
        self.text_hash = novo_hash;
        self.lines = texto.lines().map(MinimapLine::parse).collect();
        true
    }

    /// Updates the actual scroll state and geometry coming natively from CodeEditor/CodeView.
    pub fn update_scroll_state(
        &mut self,
        scroll_y: f64,
        total_h: f64,
        visible_h: f64,
        cell_h: f64,
        line_start: usize,
        line_end: usize,
        total_lines: usize,
    ) -> bool {
        let mudou = (self.editor_scroll_y - scroll_y).abs() > 0.001
            || (self.editor_total_h - total_h).abs() > 0.001
            || (self.editor_visible_h - visible_h).abs() > 0.001
            || self.total_lines != total_lines;

        self.editor_scroll_y = scroll_y;
        self.editor_total_h = total_h;
        self.editor_visible_h = visible_h;
        self.cell_h = if cell_h > 0.0 { cell_h } else { 19.0 };
        self.line_start = line_start;
        self.line_end = line_end;
        self.total_lines = total_lines.max(1);
        mudou
    }

    /// Converts the vertical coordinate of a minimap click or drag into editor scroll position.
    pub fn calcular_scroll_para_y(&mut self, rel_y: f64) {
        if self.current_line_s <= 0.0 {
            return;
        }
        let total_lines = self.lines.len().max(self.total_lines).max(1);
        let metrics = MinimapMetrics::new(
            self.current_line_s,
            self.current_minimap_scroll_y,
            self.editor_visible_h,
            self.cell_h,
            total_lines,
        );
        let (target_scroll_y, clicked_line) = metrics.calcular_scroll_para_y(rel_y);
        self.editor_scroll_y = target_scroll_y;
        self.target_scroll_y = Some(target_scroll_y);
        self.target_line = Some(clicked_line);
    }
}
