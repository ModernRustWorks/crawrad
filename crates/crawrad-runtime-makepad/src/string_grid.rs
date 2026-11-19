//! **A string grid widget**: displays cells in rows and columns with fixed headers,
//! cell selection, and grid lines.

use crawrad_runtime::string_grid::{Cells, GridLayout};
use makepad_widgets::*;

use crate::canvas::rgba;
use crate::components::comum::{
    MAKEPAD_PANEL, MAKEPAD_PANEL_BORDER, MAKEPAD_SELECTION, MAKEPAD_TEXT, MAKEPAD_TEXT_DIM,
    MAKEPAD_WIDGET_INPUT,
};
use crate::events::ClickRecord;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;

    pub CrawradStringGridBase = {{CrawradStringGrid}} {}

    pub CrawradStringGrid = <CrawradStringGridBase> {
        width: Fill, height: Fill,
        draw_color: {
            color: #181825
        }
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        editor: <TextInput> {
            padding: {left: 4.0, top: 2.0, right: 4.0, bottom: 2.0}
            empty_text: ""
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
            }
        }
    }
}

#[crawrad_note::note("string-grid-crawradstringgridaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradStringGridAction {
    CellSelected(usize, usize),
    CellEdited(usize, usize, String),
    Clicked,
    DoubleClicked,
    None,
}

#[crawrad_note::note("string-grid-crawradstringgrid")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradStringGrid {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_color: DrawColor,
    #[live]
    draw_text: DrawText,
    #[live]
    editor: TextInput,
    #[rust]
    area: Area,
    #[rust]
    layout: GridLayout,
    #[rust]
    cells: Cells,
    #[rust]
    selected: Option<(usize, usize)>,
    #[rust(true)]
    grid_lines: bool,
    #[rust]
    editing: bool,
    #[rust]
    edited: Option<(usize, usize)>,
    /// The editor takes the key focus once it has been drawn: before that it
    /// has no area to hold the focus.
    #[rust]
    focus_editor: bool,
    #[rust]
    last_click: ClickRecord,
}

impl CrawradStringGrid {
    fn open_editor(&mut self, cx: &mut Cx, col: usize, row: usize) {
        self.edited = Some((col, row));
        self.editor.set_text(cx, self.cells.get(col, row));
        self.focus_editor = true;
        self.redraw(cx);
    }
}

impl Widget for CrawradStringGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some((col, row)) = self.edited {
            let actions = cx.capture_actions(|cx| self.editor.handle_event(cx, event, scope));
            for action in
                actions.filter_widget_actions_cast::<TextInputAction>(self.editor.widget_uid())
            {
                match action {
                    TextInputAction::Returned(..) | TextInputAction::KeyFocusLost => {
                        let text = self.editor.text();
                        self.edited = None;
                        self.redraw(cx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            CrawradStringGridAction::CellEdited(col, row, text),
                        );
                        break;
                    }
                    TextInputAction::Escaped => {
                        self.edited = None;
                        self.redraw(cx);
                        break;
                    }
                    _ => {}
                }
            }
        }

        match event.hits(cx, self.area) {
            Hit::FingerDown(fe) => {
                let rel = fe.abs - self.area.rect(cx).pos;
                if self.edited.is_some() && self.edited == self.layout.cell_at(rel.x, rel.y) {
                    return;
                }
                cx.set_key_focus(self.area);
                if let Some((col, row)) = self.layout.selectable_at(rel.x, rel.y)
                    && self.selected != Some((col, row))
                {
                    self.selected = Some((col, row));
                    self.redraw(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        CrawradStringGridAction::CellSelected(col, row),
                    );
                }
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    CrawradStringGridAction::Clicked,
                );
                let key = self
                    .layout
                    .cell_at(rel.x, rel.y)
                    .map_or(usize::MAX, |(col, row)| row * self.layout.col_count + col);
                let now = cx.seconds_since_app_start();
                if crate::events::second_click(&mut self.last_click, key, now) {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        CrawradStringGridAction::DoubleClicked,
                    );
                    if self.editing
                        && let Some((col, row)) = self.layout.selectable_at(rel.x, rel.y)
                    {
                        self.open_editor(cx, col, row);
                    }
                }
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::F2,
                ..
            }) => {
                if self.editing
                    && let Some((col, row)) = self.selected
                {
                    self.open_editor(cx, col, row);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            return DrawStep::done();
        }

        cx.begin_turtle(
            Walk::abs_rect(rect),
            Layout {
                clip_x: true,
                clip_y: true,
                ..Layout::default()
            },
        );

        // 1. Background
        self.draw_color.color = rgba(MAKEPAD_WIDGET_INPUT);
        self.draw_color.draw_abs(cx, rect);

        // 2. Cells (fixed shading, selection highlight, lines and text)
        for row in 0..self.layout.row_count {
            for col in 0..self.layout.col_count {
                let (cx_rel, cy_rel, cw, ch) = self.layout.cell_rect(col, row);
                let cell_r = Rect {
                    pos: dvec2(rect.pos.x + cx_rel, rect.pos.y + cy_rel),
                    size: dvec2(cw, ch),
                };

                let is_selected = self.selected == Some((col, row));
                let is_fixed = self.layout.is_fixed(col, row);

                if is_selected {
                    self.draw_color.color = rgba(MAKEPAD_SELECTION);
                    self.draw_color.draw_abs(cx, cell_r);
                } else if is_fixed {
                    self.draw_color.color = rgba(MAKEPAD_PANEL);
                    self.draw_color.draw_abs(cx, cell_r);
                }

                if self.grid_lines {
                    self.draw_color.color = rgba(MAKEPAD_PANEL_BORDER);
                    let line_v = Rect {
                        pos: dvec2(cell_r.pos.x + cw - 1.0, cell_r.pos.y),
                        size: dvec2(1.0, ch),
                    };
                    self.draw_color.draw_abs(cx, line_v);

                    let line_h = Rect {
                        pos: dvec2(cell_r.pos.x, cell_r.pos.y + ch - 1.0),
                        size: dvec2(cw, 1.0),
                    };
                    self.draw_color.draw_abs(cx, line_h);
                }

                // The cell being edited shows the editor's text alone
                let text = self.cells.get(col, row);
                if !text.is_empty() && self.edited != Some((col, row)) {
                    let text_color = if is_selected {
                        vec4(1.0, 1.0, 1.0, 1.0)
                    } else if is_fixed {
                        rgba(MAKEPAD_TEXT)
                    } else {
                        rgba(MAKEPAD_TEXT_DIM)
                    };
                    self.draw_text.color = text_color;
                    let font_px =
                        crawrad_component::text_line::font_px(self.draw_text.text_style.font_size);
                    let text_top = f64::from(crawrad_component::text_line::cap_centred_top(
                        ch as f32, font_px,
                    ));
                    let text_pos = dvec2(cell_r.pos.x + 4.0, cell_r.pos.y + text_top);
                    self.draw_text.draw_abs(cx, text_pos, text);
                }
            }
        }

        if let Some((col, row)) = self.edited {
            let (cx_rel, cy_rel, cw, ch) = self.layout.cell_rect(col, row);
            let cell_r = Rect {
                pos: dvec2(rect.pos.x + cx_rel, rect.pos.y + cy_rel),
                size: dvec2(cw, ch),
            };
            let _ = self.editor.draw_walk(cx, scope, Walk::abs_rect(cell_r));
            if std::mem::take(&mut self.focus_editor) {
                self.editor.set_key_focus(cx);
            }
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl CrawradStringGridRef {
    #[crawrad_note::note("string-grid-crawradstringgridref-set-grid")]
    pub fn set_grid(
        &self,
        cx: &mut Cx,
        layout: GridLayout,
        cells: &Cells,
        grid_lines: bool,
        editing: bool,
    ) {
        if let Some(mut inner) = self.borrow_mut()
            && (inner.layout != layout
                || inner.cells != *cells
                || inner.grid_lines != grid_lines
                || inner.editing != editing)
        {
            inner.layout = layout;
            inner.cells = cells.clone();
            inner.grid_lines = grid_lines;
            inner.editing = editing;
            inner.draw_color.redraw(cx);
        }
    }

    #[crawrad_note::note("string-grid-crawradstringgridref-cell-edited")]
    pub fn cell_edited(&self, actions: &Actions) -> Option<(usize, usize, String)> {
        actions
            .filter_widget_actions(self.widget_uid())
            .map(|a| a.cast())
            .find_map(|a| match a {
                CrawradStringGridAction::CellEdited(col, row, text) => Some((col, row, text)),
                _ => None,
            })
    }

    #[crawrad_note::note("string-grid-crawradstringgridref-cell-selected")]
    pub fn cell_selected(&self, actions: &Actions) -> Option<(usize, usize)> {
        actions
            .filter_widget_actions(self.widget_uid())
            .map(|a| a.cast())
            .find_map(|a| match a {
                CrawradStringGridAction::CellSelected(col, row) => Some((col, row)),
                _ => None,
            })
    }

    #[crawrad_note::note("string-grid-crawradstringgridref-clicked")]
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.raised(actions, |a| matches!(a, CrawradStringGridAction::Clicked))
    }

    #[crawrad_note::note("string-grid-crawradstringgridref-double-clicked")]
    pub fn double_clicked(&self, actions: &Actions) -> bool {
        self.raised(actions, |a| {
            matches!(a, CrawradStringGridAction::DoubleClicked)
        })
    }

    fn raised(&self, actions: &Actions, is: impl Fn(&CrawradStringGridAction) -> bool) -> bool {
        actions
            .filter_widget_actions(self.widget_uid())
            .any(|a| is(&a.cast()))
    }
}
