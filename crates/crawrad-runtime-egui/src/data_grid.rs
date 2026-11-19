//! Visual data grid component for egui rendering the shared `GridView` model.
//!
//! # Why this widget exists
//! The CrawRAD data grid is backed by a headless state machine (`GridState` in `crawrad-runtime::grid`)
//! and an off-UI-thread background worker (`LiveRows` in `datagrid`). Both Makepad and egui
//! render the identical screen model (`GridView`), maintaining identical layout semantics,
//! ordering indicators, grouping hierarchy, and anti-flicker loading cards without duplicating logic.

use std::sync::Arc;

use crawrad_component::BusyTiming;
use crawrad_note::note;
use crawrad_runtime::grid::{
    CellRenderer, ColumnAlign, CurrentRow, EditKey, EditOutcome, GridColumn, GridState, Marks,
    ViewLine, line_for_item, wrap_text,
};
use datagrid::{LiveRows, RowSource};
use egui::{
    Align, Align2, CornerRadius, FontId, Frame, Layout, Margin, Pos2, Rect, ScrollArea, Sense,
    Stroke, StrokeKind, Ui, Vec2, vec2,
};

#[note("data-grid-egui-marks")]
pub const EGUI_MARKS: Marks = Marks {
    ascending: " ⏶",
    descending: " ⏷",
    group_open: "⏷ ",
    drop_down: " ⏷",
    search: "🔍",
    // the egui fonts have no ✎ (U+270E); its emoji font has the pencil ✏
    edited_row: "✏",
    new_row: "*",
};

#[note("data-grid-page-size")]
pub const PAGE_SIZE: usize = 200;

#[note("data-grid-max-pages")]
pub const MAX_PAGES: usize = 8;

#[note("data-grid-row-height")]
pub const ROW_HEIGHT: f32 = 24.0;

#[note("data-grid-header-height")]
pub const HEADER_HEIGHT: f32 = 26.0;

#[note("data-grid-footer-height")]
pub const FOOTER_HEIGHT: f32 = 24.0;

#[note("data-grid-compute-column-widths")]
pub fn compute_column_widths(available_width: f32, widths: &[Option<f32>]) -> Vec<f32> {
    let asked: Vec<Option<f64>> = widths.iter().map(|w| w.map(f64::from)).collect();
    crawrad_runtime::grid::share_widths(f64::from(available_width), &asked)
        .into_iter()
        .map(|w| w as f32)
        .collect()
}

/// Computes column widths for a slice of `GridColumn`, clamping free columns.
pub fn compute_grid_column_widths(available_width: f32, columns: &[GridColumn]) -> Vec<f32> {
    let asked: Vec<Option<f64>> = columns.iter().map(|c| c.width.map(f64::from)).collect();
    let bounds: Vec<(f64, Option<f64>)> = columns
        .iter()
        .map(|c| (c.min_width as f64, c.max_width.map(f64::from)))
        .collect();
    crawrad_runtime::grid::share_widths_clamped(f64::from(available_width), &asked, &bounds)
        .into_iter()
        .map(|w| w as f32)
        .collect()
}

#[note("data-grid-compute-column-ranges")]
pub fn compute_column_ranges(available_width: f32, widths: &[Option<f32>]) -> Vec<(f32, f32)> {
    let col_widths = compute_column_widths(available_width, widths);
    let mut ranges = Vec::with_capacity(col_widths.len());
    let mut current_x = 0.0;
    for w in col_widths {
        ranges.push((current_x, current_x + w));
        current_x += w;
    }
    ranges
}

#[note("data-grid-compute-fetch-request")]
pub fn compute_fetch_request(
    visible_first: usize,
    requested_first: usize,
    page_size: usize,
) -> Option<(usize, usize)> {
    if visible_first != requested_first {
        Some((visible_first, page_size))
    } else {
        None
    }
}

#[note("data-grid-rows-to-show")]
pub fn rows_to_show(total: Option<usize>, shown_total: usize) -> usize {
    total.unwrap_or(shown_total)
}

#[note("data-grid-compute-group-indent")]
pub fn compute_group_indent(level: usize) -> f32 {
    (level as f32) * 16.0 + 8.0
}

#[note("data-grid-compute-loading-card-rect")]
pub fn compute_loading_card_rect(
    rows_rect: Rect,
    text_size: Vec2,
    spinner_size: f32,
    padding: Vec2,
    gap: f32,
) -> Rect {
    let card_w = padding.x * 2.0 + spinner_size + gap + text_size.x;
    let card_h = padding.y * 2.0 + spinner_size.max(text_size.y);
    Rect::from_center_size(rows_rect.center(), vec2(card_w, card_h))
}

#[note("data-grid-compute-spinner-arc")]
pub fn compute_spinner_arc(center: Pos2, radius: f32, time: f64, n_points: usize) -> Vec<Pos2> {
    let n = n_points.max(8);
    let start_angle = time * std::f64::consts::TAU;
    let sweep = std::f64::consts::PI * 1.5;
    (0..=n)
        .map(|i| {
            let frac = i as f64 / n as f64;
            let angle = start_angle + frac * sweep;
            let (sin, cos) = angle.sin_cos();
            center + radius * vec2(cos as f32, sin as f32)
        })
        .collect()
}

#[note("data-grid-datagrid")]
pub struct DataGrid {
    state: GridState,
    live: LiveRows,
    requested_first: usize,
    /// The column whose fkList drop-down is open.
    open_list: Option<usize>,
    was_animating: bool,
    /// The last row count the source answered. A new query forgets the total
    /// until the worker counts again; scrolling over that gap with zero rows
    /// would erase the old rows the veil is meant to dim.
    shown_total: usize,
    row_height: f32,
    group_bar: bool,
    resizing: Option<(usize, f32, f32)>,
    needs_auto_width: bool,
    /// Whether the pick list of the open edList editor is down.
    open_cell_list: bool,
    /// Whether the grid owns the keyboard: a click on it takes it, a click elsewhere
    /// gives it back, so typing in another field never edits the grid.
    keyboard: bool,
    /// The scroll position of the rows, kept to bring the row the user is on into view
    /// when the keyboard moves it.
    scroll_offset: f32,
    /// The height of the rows actually seen last frame.
    rows_height: f32,
    /// The width the rows had for their columns last frame (inside the scroll bar, past the
    /// gutter); header, filter and summary share it so every column lines up.
    rows_width: f32,
    follow_rec: bool,
    /// The row the user is on must be brought fully into view once it is drawn (the
    /// offset `follow_rec` sets is only near it when group headers or a write move rows).
    reveal_rec: bool,
}

/// This frame's keys, as the grid's editing names them.
fn edit_keys(ui: &Ui) -> Vec<EditKey> {
    use egui::{Event, Key};
    ui.input(|i| {
        i.events
            .iter()
            .filter_map(|event| match event {
                Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } => match key {
                    Key::Escape => Some(EditKey::Escape),
                    Key::Enter if modifiers.ctrl || modifiers.command => Some(EditKey::CtrlEnter),
                    Key::Enter => Some(EditKey::Enter),
                    Key::F2 => Some(EditKey::F2),
                    Key::Tab => Some(EditKey::Tab {
                        back: modifiers.shift,
                    }),
                    Key::Insert => Some(EditKey::Insert),
                    Key::ArrowLeft => Some(EditKey::Left),
                    Key::ArrowRight => Some(EditKey::Right),
                    Key::ArrowUp => Some(EditKey::Up),
                    Key::ArrowDown => Some(EditKey::Down),
                    _ => None,
                },
                Event::Text(text) if !text.chars().any(char::is_control) => {
                    Some(EditKey::Char(text.clone()))
                }
                _ => None,
            })
            .collect()
    })
}

/// The width of `text` in `font_id`, as the grid paints it.
fn text_width(ui: &egui::Ui, font_id: &FontId, text: &str) -> f64 {
    ui.painter()
        .layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::WHITE)
        .size()
        .x as f64
}

impl DataGrid {
    #[note("data-grid-datagrid-new")]
    pub fn new(source: Arc<dyn RowSource>, columns: Vec<GridColumn>) -> Self {
        let mut state = GridState::new(columns);
        state.marks = EGUI_MARKS;
        state.writes_through(source.as_ref());
        let mut live = LiveRows::spawn(source, state.query(), PAGE_SIZE, MAX_PAGES);
        live.show(0, PAGE_SIZE);
        Self {
            state,
            live,
            requested_first: 0,
            open_list: None,
            was_animating: false,
            shown_total: 0,
            row_height: ROW_HEIGHT,
            group_bar: false,
            resizing: None,
            needs_auto_width: true,
            open_cell_list: false,
            keyboard: false,
            scroll_offset: 0.0,
            rows_height: 0.0,
            rows_width: 0.0,
            follow_rec: false,
            reveal_rec: false,
        }
    }

    /// Sends the write or records the pending check an editing key or click produced.
    fn apply(&mut self, outcome: EditOutcome) {
        match outcome {
            EditOutcome::Write(write) => {
                let ticket = self.live.write(write);
                self.state.sent(ticket);
            }
            EditOutcome::Check(check) => {
                self.state.pending_check = Some(check);
            }
            EditOutcome::Button(req) => {
                self.state.pending_button = Some(req);
            }
            _ => {}
        }
    }

    /// Requests custom display texts for visible rows from columns with `get_text_hook`.
    pub fn text_requests(&mut self) -> Vec<crawrad_runtime::grid::TextRequest> {
        self.state.text_requests(self.live.screen())
    }

    /// Stores answered custom texts for cells.
    pub fn set_texts(&mut self, reqs: Vec<crawrad_runtime::grid::TextRequest>) {
        self.state.set_texts(reqs);
    }

    /// Requests custom styles for visible rows from columns with `draw_cell_hook` or style rules.
    pub fn style_requests(&mut self) -> Vec<crawrad_runtime::grid::StyleRequest> {
        self.state.style_requests(self.live.screen())
    }

    /// Stores answered custom styles for cells.
    pub fn set_styles(&mut self, reqs: Vec<crawrad_runtime::grid::StyleRequest>) {
        self.state.set_styles(reqs);
    }

    /// Returns the active row check waiting for OnSetText / OnValidate answers.
    pub fn pending_check(&self) -> Option<crawrad_runtime::grid::RowCheck> {
        self.state.pending_check()
    }

    /// Applies answered cell checks, setting errors or proceeding to write.
    pub fn resolve_check(&mut self, check: crawrad_runtime::grid::RowCheck) {
        let outcome = self.state.checked(check);
        self.apply(outcome);
    }

    /// Returns the active button request waiting for OnButtonClick.
    pub fn pending_button(&self) -> Option<crawrad_runtime::grid::ButtonRequest> {
        self.state.pending_button()
    }

    /// Completes the button request, applying the returned text to the active editor.
    pub fn button_done(&mut self, req: crawrad_runtime::grid::ButtonRequest) {
        self.state.button_done(req);
    }

    /// Sets the height of every data row, the grid's `DefaultRowHeight`.
    pub fn set_row_height(&mut self, row_height: f32) {
        self.row_height = row_height;
    }

    /// Shows or hides the "Group by" bar, the grid's `GroupBar`.
    pub fn set_group_bar(&mut self, visible: bool) {
        self.group_bar = visible;
    }

    /// The height of every data row.
    pub fn row_height(&self) -> f32 {
        self.row_height
    }

    #[note("data-grid-datagrid-set-timing")]
    pub fn set_timing(&mut self, timing: BusyTiming) {
        self.state.timing = timing;
    }

    #[note("data-grid-datagrid-set-dim")]
    pub fn set_dim(&mut self, dim: f64) {
        self.state.dim = dim;
    }

    #[note("data-grid-datagrid-rec-no")]
    pub fn rec_no(&self) -> usize {
        self.state.rec_no
    }

    /// Makes `rec_no` the current row (a bound dataset moved there) and brings it into view.
    pub fn set_rec_no(&mut self, rec_no: usize) {
        self.state.rec_no = rec_no;
        self.follow_rec = true;
        self.reveal_rec = true;
    }

    #[note("data-grid-datagrid-is-pending")]
    pub fn is_pending(&self) -> bool {
        self.live.screen().pending.is_some()
    }

    #[note("data-grid-datagrid-total")]
    pub fn total(&self) -> Option<usize> {
        self.live.screen().total
    }

    #[note("data-grid-datagrid-columns")]
    pub fn columns(&self) -> &[GridColumn] {
        &self.state.columns
    }

    #[note("data-grid-datagrid-show")]
    pub fn show(&mut self, ui: &mut Ui) {
        let changed = self.live.poll();
        if let Some(written) = self.live.screen().written.as_ref() {
            // The screen re-read after a write may move rows: keep the user's row in view.
            self.reveal_rec |= self.state.on_written(written);
        }
        if changed || self.needs_auto_width {
            let screen = self.live.screen();
            for (col_idx, col) in self.state.columns.iter_mut().enumerate() {
                if col.auto_size {
                    let source_idx = col.source.unwrap_or(col_idx);
                    let cell_texts: Vec<&str> = screen
                        .lines
                        .iter()
                        .filter_map(|line| match line {
                            datagrid::ScreenLine::Row { cells, .. } => {
                                cells.get(source_idx).and_then(|c| c.as_deref())
                            }
                            _ => None,
                        })
                        .collect();
                    let font_size = if col.font.size > 0.0 {
                        col.font.size
                    } else {
                        12.0
                    };
                    let font_id = FontId::proportional(font_size);
                    let min_w = (col.min_width as f64).max(24.0);
                    let max_w = col.max_width.map(f64::from);
                    let w = crawrad_runtime::grid::auto_width(
                        &col.title,
                        &cell_texts,
                        16.0,
                        min_w,
                        max_w,
                        |s| text_width(ui, &font_id, s),
                    );
                    col.width = Some(w as f32);
                }
            }
            self.needs_auto_width = false;
        }

        let now = std::time::Instant::now();
        self.state.update_lookups(self.live.screen());
        self.state.refresh_rule_styles(self.live.screen());
        let view = self.state.view(self.live.screen(), now);
        let pending = self.live.screen().pending.is_some();
        let animating = view.animating;

        if changed || animating || pending || self.was_animating {
            ui.ctx().request_repaint();
        }
        self.was_animating = animating || pending;

        if let Some((resizing_col, start_x, start_w)) = self.resizing {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
            if ui.input(|i| i.pointer.primary_down()) {
                let current_x = ui.input(|i| i.pointer.interact_pos().map_or(start_x, |p| p.x));
                let delta = current_x - start_x;
                let new_w = (start_w + delta) as f64;
                let min_w = self
                    .state
                    .column_order
                    .get(resizing_col)
                    .and_then(|&pi| self.state.columns.get(pi))
                    .map(|c| (c.min_width as f64).max(24.0))
                    .unwrap_or(24.0);
                let max_w = self
                    .state
                    .column_order
                    .get(resizing_col)
                    .and_then(|&pi| self.state.columns.get(pi))
                    .and_then(|c| c.max_width.map(f64::from));
                let clamped = crawrad_runtime::grid::clamp_column_width(new_w, min_w, max_w);
                self.state.set_column_width(resizing_col, clamped as f32);
                ui.ctx().request_repaint();
            } else {
                self.resizing = None;
            }
        }

        if self.group_bar {
            // a. Group-by bar: drop zone accepting column headers to add grouping levels
            let group_bar_frame = Frame::new()
                .fill(ui.visuals().faint_bg_color)
                .inner_margin(Margin::symmetric(8, 6));

            let mut ungroup_level = None;
            let (_, dropped_payload) = ui.dnd_drop_zone::<usize, _>(group_bar_frame, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Group by:")
                            .strong()
                            .color(ui.visuals().weak_text_color()),
                    );

                    for (level, chip_title) in view.chips.iter().enumerate() {
                        Frame::new()
                            .fill(ui.visuals().widgets.inactive.bg_fill)
                            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                            .corner_radius(CornerRadius::same(3))
                            .inner_margin(Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(chip_title)
                                            .color(ui.visuals().text_color()),
                                    );
                                    if ui.small_button("×").clicked() {
                                        ungroup_level = Some(level);
                                    }
                                });
                            });
                    }

                    ui.label(
                        egui::RichText::new(view.group_hint).color(ui.visuals().weak_text_color()),
                    );
                });
            });

            if let Some(col_idx) = dropped_payload {
                let q = self.state.group_by_column(*col_idx);
                self.live.set_query(q);
                self.requested_first = 0;
                ui.ctx().request_repaint();
            }
            if let Some(level) = ungroup_level {
                let q = self.state.ungroup(level);
                self.live.set_query(q);
                self.requested_first = 0;
                ui.ctx().request_repaint();
            }
        }

        // b. Header row: interactive sorting click and drag source for grouping
        // The grid's frame starts here: header, filter, rows (with the scroll bar) and totals.
        let frame_top_left = ui.cursor().left_top();
        let frame_width = ui.available_width();
        let gutter_w = if view.gutter { 22.0 } else { 0.0 };
        let available_width = ui.available_width();
        let content_width = if self.rows_width > 0.0 {
            self.rows_width.min(available_width - gutter_w).max(0.0)
        } else {
            (available_width - gutter_w).max(0.0)
        };
        let ordered_cols = self.state.ordered_columns();
        let widths = compute_grid_column_widths(content_width, &ordered_cols);

        let mut header_height = HEADER_HEIGHT;
        for (col_idx, cell_info) in view.header.iter().enumerate() {
            if cell_info.title_word_wrap {
                let font_size = if cell_info.title_font.size > 0.0 {
                    cell_info.title_font.size
                } else {
                    12.0
                };
                let font_id = FontId::proportional(font_size);
                let line_h = ui.ctx().fonts_mut(|f| f.row_height(&font_id)).max(12.0);
                let width = widths[col_idx];
                let avail_w = (width as f64 - 16.0).max(1.0);
                let title_text = format!("{}{}", cell_info.title, cell_info.arrow);
                let lines = wrap_text(&title_text, avail_w, 3, false, |s| {
                    text_width(ui, &font_id, s)
                });
                let h = (lines.len() as f32) * line_h + 8.0;
                header_height = header_height.max(h);
            }
        }

        let mut header_clicked_col = None;
        let mut move_col_request = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let sep_stroke = ui.visuals().widgets.noninteractive.bg_stroke;

            if view.gutter {
                let (g_rect, _) =
                    ui.allocate_exact_size(vec2(gutter_w, header_height), Sense::hover());
                ui.painter()
                    .rect_filled(g_rect, 0.0, ui.visuals().widgets.inactive.bg_fill);
                ui.painter()
                    .line_segment([g_rect.right_top(), g_rect.right_bottom()], sep_stroke);
                ui.painter()
                    .line_segment([g_rect.left_bottom(), g_rect.right_bottom()], sep_stroke);
            }

            for (col_idx, cell_info) in view.header.iter().enumerate() {
                let width = widths[col_idx];
                let is_resizable = ordered_cols.get(col_idx).is_none_or(|c| c.resizable);
                let is_movable = ordered_cols.get(col_idx).is_none_or(|c| c.movable);

                // One click-and-drag area rather than `dnd_drag_source`: the
                // drag source sits over its content and swallows the click,
                // and a header that cannot be clicked cannot sort.
                let (rect, mut resp) =
                    ui.allocate_exact_size(vec2(width, header_height), Sense::click_and_drag());
                if !cell_info.title_hint.is_empty() {
                    resp = resp.on_hover_text(&cell_info.title_hint);
                }

                let pointer_pos = ui.input(|i| i.pointer.hover_pos());
                let is_edge = pointer_pos.is_some_and(|p| {
                    p.x >= rect.right() - 6.0
                        && p.x <= rect.right() + 2.0
                        && p.y >= rect.top()
                        && p.y <= rect.bottom()
                });

                if is_resizable && (is_edge || self.resizing.is_some_and(|(c, _, _)| c == col_idx))
                {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
                }

                if resp.drag_started() {
                    if is_edge && is_resizable {
                        self.resizing = Some((col_idx, pointer_pos.map_or(0.0, |p| p.x), width));
                    } else if is_movable {
                        egui::DragAndDrop::set_payload(ui.ctx(), col_idx);
                    }
                }

                if let Some(source_idx) = egui::DragAndDrop::payload::<usize>(ui.ctx())
                    && ui.input(|i| i.pointer.any_released())
                    && resp.hovered()
                    && *source_idx != col_idx
                {
                    move_col_request = Some((*source_idx, col_idx));
                }

                if resp.clicked() && self.resizing.is_none() {
                    header_clicked_col = Some(col_idx);
                }
                {
                    if cell_info.title_color.a > 0 {
                        let bg = egui::Color32::from_rgba_unmultiplied(
                            cell_info.title_color.r,
                            cell_info.title_color.g,
                            cell_info.title_color.b,
                            cell_info.title_color.a,
                        );
                        ui.painter().rect_filled(rect, 0.0, bg);
                    } else if resp.hovered() {
                        ui.painter()
                            .rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
                    }
                    let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                    ui.painter()
                        .line_segment([rect.right_top(), rect.right_bottom()], stroke);
                    ui.painter()
                        .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);

                    let title_text = format!("{}{}", cell_info.title, cell_info.arrow);
                    let text_color = if !cell_info.arrow.is_empty() {
                        ui.visuals().selection.bg_fill
                    } else {
                        ui.visuals().text_color()
                    };

                    let align = match cell_info.align {
                        ColumnAlign::Left => Align2::LEFT_CENTER,
                        ColumnAlign::Center => Align2::CENTER_CENTER,
                        ColumnAlign::Right => Align2::RIGHT_CENTER,
                    };
                    let text_pos = match cell_info.align {
                        ColumnAlign::Left => rect.left_center() + vec2(8.0, 0.0),
                        ColumnAlign::Center => rect.center(),
                        ColumnAlign::Right => rect.right_center() - vec2(8.0, 0.0),
                    };
                    let font_size = if cell_info.title_font.size > 0.0 {
                        cell_info.title_font.size
                    } else {
                        12.0
                    };
                    let font_id = FontId::proportional(font_size);
                    // A sorted column keeps its own colour; the others take their TitleFont colour.
                    let title_color =
                        if cell_info.arrow.is_empty() && cell_info.title_font.color.a > 0 {
                            egui::Color32::from_rgba_unmultiplied(
                                cell_info.title_font.color.r,
                                cell_info.title_font.color.g,
                                cell_info.title_font.color.b,
                                cell_info.title_font.color.a,
                            )
                        } else {
                            text_color
                        };

                    if cell_info.title_word_wrap {
                        let line_h = ui.ctx().fonts_mut(|f| f.row_height(&font_id)).max(12.0);
                        let avail_w = (width as f64 - 16.0).max(1.0);
                        let lines = wrap_text(&title_text, avail_w, 3, false, |s| {
                            text_width(ui, &font_id, s)
                        });
                        let total_text_h = (lines.len() as f32) * line_h;
                        let start_y = rect.center().y - total_text_h * 0.5 + line_h * 0.5;
                        for (l_idx, line) in lines.into_iter().enumerate() {
                            let y = start_y + (l_idx as f32) * line_h;
                            let line_pos = match cell_info.align {
                                ColumnAlign::Left => egui::pos2(rect.left() + 8.0, y),
                                ColumnAlign::Center => egui::pos2(rect.center().x, y),
                                ColumnAlign::Right => egui::pos2(rect.right() - 8.0, y),
                            };
                            ui.painter()
                                .text(line_pos, align, line, font_id.clone(), title_color);
                        }
                    } else {
                        ui.painter()
                            .text(text_pos, align, title_text, font_id, title_color);
                    }
                }
            }
        });

        if let Some((from, to)) = move_col_request {
            self.state.move_column(from, to);
            egui::DragAndDrop::clear_payload(ui.ctx());
            ui.ctx().request_repaint();
        }

        if let Some(col_idx) = header_clicked_col {
            let q = self.state.click_header(col_idx);
            self.live.set_query(q);
            self.requested_first = 0;
            ui.ctx().request_repaint();
        }

        // b2. Filter row rendered below header when filter_row is not empty
        let filter_h = if !view.filter_row.is_empty() {
            26.0
        } else {
            0.0
        };

        let mut filter_changed = false;
        if !view.filter_row.is_empty() {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                if view.gutter {
                    let (g_rect, _) =
                        ui.allocate_exact_size(vec2(gutter_w, filter_h), Sense::hover());
                    ui.painter()
                        .rect_filled(g_rect, 0.0, ui.visuals().faint_bg_color);
                    ui.painter()
                        .line_segment([g_rect.right_top(), g_rect.right_bottom()], stroke);
                    ui.painter()
                        .line_segment([g_rect.left_bottom(), g_rect.right_bottom()], stroke);
                }
                for (col_idx, cell) in view.filter_row.iter().enumerate() {
                    let width = widths[col_idx];
                    let (rect, _) = ui.allocate_exact_size(vec2(width, filter_h), Sense::hover());
                    let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                    ui.painter()
                        .line_segment([rect.right_top(), rect.right_bottom()], stroke);
                    ui.painter()
                        .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);

                    if !cell.enabled {
                        ui.painter().rect_filled(rect, 0.0, ui.visuals().faint_bg_color);
                        continue;
                    }

                    let inner_rect = rect.shrink2(vec2(2.0, 2.0));
                    ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| match cell.kind {
                        "fkText" => {
                            let mut text = match self.state.filters.get(&cell.column) {
                                Some(crawrad_runtime::grid::FilterInput::Text(s)) => s.clone(),
                                _ => String::new(),
                            };
                            let resp = ui.add_sized(
                                vec2(inner_rect.width(), inner_rect.height()),
                                egui::TextEdit::singleline(&mut text).hint_text(EGUI_MARKS.search),
                            );
                            if resp.changed() {
                                if text.trim().is_empty() {
                                    self.state.filters.remove(&cell.column);
                                } else {
                                    self.state.filters.insert(
                                        cell.column,
                                        crawrad_runtime::grid::FilterInput::Text(text),
                                    );
                                }
                                filter_changed = true;
                            }
                        }
                        "fkRange" | "fkDate" => {
                            let range = cell.kind == "fkRange";
                            let (lo, hi) = match self.state.filters.get(&cell.column) {
                                Some(crawrad_runtime::grid::FilterInput::Range { min, max }) => {
                                    (min.clone(), max.clone())
                                }
                                Some(crawrad_runtime::grid::FilterInput::Date { from, to }) => {
                                    (from.clone(), to.clone())
                                }
                                _ => (String::new(), String::new()),
                            };
                            let hints = if range { ("min", "max") } else { ("from", "to") };
                            if let Some((lo, hi)) = pair_edit(ui, inner_rect, lo, hi, hints) {
                                if lo.trim().is_empty() && hi.trim().is_empty() {
                                    self.state.filters.remove(&cell.column);
                                } else {
                                    let input = if range {
                                        crawrad_runtime::grid::FilterInput::Range { min: lo, max: hi }
                                    } else {
                                        crawrad_runtime::grid::FilterInput::Date { from: lo, to: hi }
                                    };
                                    self.state.filters.insert(cell.column, input);
                                }
                                filter_changed = true;
                            }
                        }
                        "fkList" => {
                            let popup_id =
                                ui.make_persistent_id(format!("fk_list_popup_{}", cell.column));
                            let btn_text = cell.caption.as_str();
                            let btn_resp = ui.add_sized(
                                vec2(inner_rect.width(), inner_rect.height()),
                                egui::Button::new(btn_text),
                            );
                            if btn_resp.clicked() {
                                self.open_list =
                                    (self.open_list != Some(cell.column)).then_some(cell.column);
                                if self.open_list.is_some() {
                                    let q_without = self.state.query_without(cell.column);
                                    if let Some(col) = self.state.columns.get(cell.column) {
                                        self.live.distinct(col.name.clone(), q_without);
                                    }
                                }
                            }
                            if self.open_list == Some(cell.column) {
                                let area_resp = egui::Area::new(popup_id)
                                    .order(egui::Order::Foreground)
                                    .fixed_pos(btn_resp.rect.left_bottom())
                                    .show(ui.ctx(), |ui| {
                                        egui::Frame::popup(ui.style()).show(ui, |ui| {
                                            ui.set_max_height(250.0);
                                            ui.set_min_width(120.0);
                                            egui::ScrollArea::vertical().show(ui, |ui| {
                                                let col_name = self
                                                    .state
                                                    .columns
                                                    .get(cell.column)
                                                    .map(|c| c.name.as_str())
                                                    .unwrap_or("");
                                                let current_distinct = self
                                                    .live
                                                    .screen()
                                                    .distinct
                                                    .as_ref()
                                                    .filter(|(col, _)| col == col_name)
                                                    .map(|(_, vals)| vals.as_slice());

                                                let mut current_checked: Vec<Option<String>> =
                                                    match self.state.filters.get(&cell.column) {
                                                        Some(
                                                            crawrad_runtime::grid::FilterInput::List(
                                                                list,
                                                            ),
                                                        ) => list.clone(),
                                                        _ => Vec::new(),
                                                    };

                                                if let Some(distinct_vals) = current_distinct {
                                                    let mut list_changed = false;
                                                    for val_opt in distinct_vals {
                                                        let label = match val_opt {
                                                            Some(s) if !s.is_empty() => s.as_str(),
                                                            _ => "(empty)",
                                                        };
                                                        let mut is_checked =
                                                            current_checked.contains(val_opt);
                                                        if ui.checkbox(&mut is_checked, label).changed()
                                                        {
                                                            if is_checked {
                                                                if !current_checked.contains(val_opt) {
                                                                    current_checked.push(val_opt.clone());
                                                                }
                                                            } else {
                                                                current_checked
                                                                    .retain(|v| v != val_opt);
                                                            }
                                                            list_changed = true;
                                                        }
                                                    }
                                                    if list_changed {
                                                        if current_checked.is_empty() {
                                                            self.state.filters.remove(&cell.column);
                                                        } else {
                                                            self.state.filters.insert(
                                                                cell.column,
                                                                crawrad_runtime::grid::FilterInput::List(
                                                                    current_checked,
                                                                ),
                                                            );
                                                        }
                                                        filter_changed = true;
                                                    }
                                                } else {
                                                    ui.label("Loading…");
                                                }
                                            });
                                        });
                                    });

                                if ui.input(|i| i.pointer.any_pressed())
                                    && let Some(pos) = ui.input(|i| i.pointer.interact_pos())
                                    && !btn_resp.rect.contains(pos)
                                    && !area_resp.response.rect.contains(pos)
                                {
                                    self.open_list = None;
                                }
                            }
                        }
                        _ => {}
                    });
                }
            });
        }

        if filter_changed {
            let q = self.state.query();
            self.live.set_query(q);
            self.requested_first = 0;
            ui.ctx().request_repaint();
        }

        // c. Virtualized rows rendered via ScrollArea
        // A new row being typed is one more row, after the last.
        let new_row = self.state.edit.as_ref().is_some_and(|e| e.index.is_none());
        let total = rows_to_show(self.live.screen().total, self.shown_total) + usize::from(new_row);
        self.shown_total = total;
        let summary_h = if !view.summary.is_empty() {
            self.row_height
        } else {
            0.0
        };
        let scroll_height = (ui.available_height() - FOOTER_HEIGHT - summary_h - filter_h).max(0.0);

        let mut scroll = ScrollArea::vertical().max_height(scroll_height);
        if std::mem::take(&mut self.follow_rec) {
            // `show_rows` steps one row plus the item spacing of the ui it is given.
            let pitch = self.row_height + ui.spacing().item_spacing.y;
            let top = self.state.rec_no as f32 * pitch;
            let bottom = top + pitch;
            let offset = if top < self.scroll_offset {
                top
            } else {
                (bottom - self.rows_height).max(self.scroll_offset)
            };
            scroll = scroll.vertical_scroll_offset(offset);
        }
        let scroll_output = scroll.auto_shrink([false, false]).show_rows(
            ui,
            self.row_height,
            total,
            |ui, range| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                if let Some((first, count)) =
                    compute_fetch_request(range.start, self.requested_first, PAGE_SIZE)
                {
                    self.requested_first = first;
                    self.live.show(first, count);
                }

                let rows_width = (ui.available_width() - gutter_w).max(0.0);
                if (rows_width - self.rows_width).abs() > 0.5 {
                    self.rows_width = rows_width;
                    ui.ctx().request_repaint();
                }
                let col_widths = compute_grid_column_widths(rows_width, &ordered_cols);
                let mut clicked_rec = None;
                let mut clicked_cell = None;

                for item in range {
                    let item_lines = line_for_item(&view, self.requested_first, item);

                    // Render group summary headers preceding this row
                    for h in &item_lines.headers {
                        if let ViewLine::Header {
                            level,
                            title,
                            count,
                            totals,
                        } = h
                        {
                            let (h_rect, _) = ui.allocate_exact_size(
                                vec2(ui.available_width(), self.row_height),
                                Sense::hover(),
                            );
                            ui.painter()
                                .rect_filled(h_rect, 0.0, ui.visuals().faint_bg_color);
                            let sep_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                            ui.painter().line_segment(
                                [h_rect.left_bottom(), h_rect.right_bottom()],
                                sep_stroke,
                            );

                            let pad_left = compute_group_indent(*level);
                            let mut x = h_rect.left() + pad_left;
                            let y = h_rect.center().y;

                            let text_color = ui.visuals().text_color();
                            let weak_color = ui.visuals().weak_text_color();

                            let galley_title = ui.painter().layout_no_wrap(
                                title.clone(),
                                FontId::proportional(12.0),
                                text_color,
                            );
                            let title_w = galley_title.size().x;
                            ui.painter().galley(
                                egui::pos2(x, y - galley_title.size().y * 0.5),
                                galley_title,
                                text_color,
                            );
                            x += title_w + 12.0;

                            let galley_count = ui.painter().layout_no_wrap(
                                count.clone(),
                                FontId::proportional(11.0),
                                weak_color,
                            );
                            let count_w = galley_count.size().x;
                            ui.painter().galley(
                                egui::pos2(x, y - galley_count.size().y * 0.5),
                                galley_count,
                                weak_color,
                            );
                            x += count_w + 12.0;

                            if !totals.is_empty() {
                                let galley_totals = ui.painter().layout_no_wrap(
                                    totals.clone(),
                                    FontId::proportional(11.0),
                                    text_color,
                                );
                                ui.painter().galley(
                                    egui::pos2(x, y - galley_totals.size().y * 0.5),
                                    galley_totals,
                                    text_color,
                                );
                            }
                        }
                    }

                    // Render data row
                    if let Some(ViewLine::Row {
                        index,
                        id,
                        cells,
                        raw_cells,
                        odd,
                        selected,
                        marker,
                    }) = item_lines.row
                    {
                        let (row_rect, row_resp) = ui.allocate_exact_size(
                            vec2(ui.available_width(), self.row_height),
                            Sense::click(),
                        );
                        if *index == self.state.rec_no && std::mem::take(&mut self.reveal_rec) {
                            ui.scroll_to_rect(row_rect, None);
                        }
                        if row_resp.clicked() {
                            clicked_rec = Some((*index, *id));
                        }

                        if *selected {
                            ui.painter()
                                .rect_filled(row_rect, 0.0, ui.visuals().selection.bg_fill);
                        } else if *odd {
                            ui.painter()
                                .rect_filled(row_rect, 0.0, ui.visuals().faint_bg_color);
                        } else {
                            ui.painter()
                                .rect_filled(row_rect, 0.0, ui.visuals().panel_fill);
                        }

                        let sep_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                        ui.painter().line_segment(
                            [row_rect.left_bottom(), row_rect.right_bottom()],
                            sep_stroke,
                        );

                        if view.gutter {
                            let g_rect = Rect::from_min_size(
                                egui::pos2(row_rect.left(), row_rect.top()),
                                vec2(gutter_w, self.row_height),
                            );
                            ui.painter()
                                .rect_filled(g_rect, 0.0, ui.visuals().faint_bg_color);
                            ui.painter().line_segment(
                                [g_rect.right_top(), g_rect.right_bottom()],
                                sep_stroke,
                            );
                            if !marker.is_empty() {
                                ui.painter().text(
                                    g_rect.center(),
                                    Align2::CENTER_CENTER,
                                    *marker,
                                    FontId::proportional(12.0),
                                    ui.visuals().strong_text_color(),
                                );
                            }
                        }

                        let text_color = if *selected {
                            ui.visuals().strong_text_color()
                        } else {
                            ui.visuals().text_color()
                        };

                        // The view marks the edited row ("✎") and the new one ("*").
                        let is_editing_this_row = !marker.is_empty();

                        let mut col_x = row_rect.left() + gutter_w;
                        for (c_idx, col_w) in col_widths.iter().enumerate() {
                            let cell_rect = Rect::from_min_size(
                                egui::pos2(col_x, row_rect.top()),
                                vec2(*col_w, self.row_height),
                            );
                            let orig_col_idx = self.state.column_order.get(c_idx).copied();
                            let cell_style =
                                orig_col_idx.and_then(|ci| self.state.cell_style(*id, ci).cloned());
                            let col = ordered_cols.get(c_idx);
                            if !*selected {
                                if let Some(style_bg) = cell_style.as_ref().and_then(|cs| cs.color) {
                                    let bg = egui::Color32::from_rgba_unmultiplied(
                                        style_bg.r, style_bg.g, style_bg.b, style_bg.a,
                                    );
                                    ui.painter().rect_filled(cell_rect, 0.0, bg);
                                } else if let Some(c) = col
                                    && c.color.a > 0
                                {
                                    let bg = egui::Color32::from_rgba_unmultiplied(
                                        c.color.r, c.color.g, c.color.b, c.color.a,
                                    );
                                    ui.painter().rect_filled(cell_rect, 0.0, bg);
                                }
                            }
                            ui.painter().line_segment(
                                [cell_rect.right_top(), cell_rect.right_bottom()],
                                sep_stroke,
                            );

                            let default_text =
                                cells.get(c_idx).map(|s| s.as_str()).unwrap_or("");
                            let cell_text =
                                cell_style.as_ref().map(|cs| cs.text.as_str()).unwrap_or(default_text);
                            let col_align = col.map(|c| c.align).unwrap_or(ColumnAlign::Left);
                            let col_valign = col
                                .map(|c| c.valign)
                                .unwrap_or(crawrad_runtime::grid::ColumnVAlign::Center);
                            let pad = col.and_then(|c| c.padding).unwrap_or(crawrad_ir::Edges {
                                left: 8.0,
                                top: 0.0,
                                right: 8.0,
                                bottom: 0.0,
                            });
                            let inner_min =
                                egui::pos2(cell_rect.left() + pad.left, cell_rect.top() + pad.top);
                            let inner_max = egui::pos2(
                                (cell_rect.right() - pad.right).max(inner_min.x),
                                (cell_rect.bottom() - pad.bottom).max(inner_min.y),
                            );
                            let inner_rect = Rect::from_min_max(inner_min, inner_max);
                            let avail_w = inner_rect.width() as f64;
                            let font_size = col
                                .map(|c| c.font.size)
                                .filter(|&s| s > 0.0)
                                .unwrap_or(12.0);
                            let font_id = FontId::proportional(font_size);
                            let ellipsis = col.map(|c| c.ellipsis).unwrap_or(false);
                            let word_wrap = col.map(|c| c.word_wrap).unwrap_or(false);

                            let horiz_align = match col_align {
                                ColumnAlign::Left => Align::Min,
                                ColumnAlign::Center => Align::Center,
                                ColumnAlign::Right => Align::Max,
                            };
                            let vert_align = match col_valign {
                                crawrad_runtime::grid::ColumnVAlign::Top => Align::Min,
                                crawrad_runtime::grid::ColumnVAlign::Center => Align::Center,
                                crawrad_runtime::grid::ColumnVAlign::Bottom => Align::Max,
                            };
                            let text_x = match horiz_align {
                                Align::Min => inner_rect.left(),
                                Align::Center => inner_rect.center().x,
                                Align::Max => inner_rect.right(),
                            };
                            let text_y = match vert_align {
                                Align::Min => inner_rect.top(),
                                Align::Center => inner_rect.center().y,
                                Align::Max => inner_rect.bottom(),
                            };

                            if *selected && self.state.focus_col == c_idx {
                                ui.painter().rect_stroke(
                                    cell_rect,
                                    0.0,
                                    Stroke::new(1.0, ui.visuals().selection.stroke.color),
                                    StrokeKind::Inside,
                                );
                            }

                            let is_open_editor =
                                is_editing_this_row && view.open_editor == Some(c_idx);
                            let is_cell_edited = is_editing_this_row
                                && self
                                    .state
                                    .column_order
                                    .get(c_idx)
                                    .is_some_and(|&ci| self.state.edited_text(ci).is_some());

                            let effective_text_color = if is_cell_edited {
                                egui::Color32::from_rgb(0xff, 0xd2, 0x7a)
                            } else if !*selected
                                && let Some(tc) = cell_style.as_ref().and_then(|cs| cs.text_color)
                            {
                                egui::Color32::from_rgba_unmultiplied(tc.r, tc.g, tc.b, tc.a)
                            } else if !*selected
                                && let Some(c) = col
                                && c.font.color.a > 0
                            {
                                egui::Color32::from_rgba_unmultiplied(
                                    c.font.color.r,
                                    c.font.color.g,
                                    c.font.color.b,
                                    c.font.color.a,
                                )
                            } else if !*selected
                                && let Some(c) = col
                                && c.text_color.a > 0
                            {
                                egui::Color32::from_rgba_unmultiplied(
                                    c.text_color.r,
                                    c.text_color.g,
                                    c.text_color.b,
                                    c.text_color.a,
                                )
                            } else {
                                text_color
                            };

                            let renderer = col.map(|c| c.renderer).unwrap_or(CellRenderer::Text);
                            let raw_cell = raw_cells.get(c_idx).and_then(|opt| opt.as_deref());
                            // CellHint: the full text on hover when what is shown was cut.
                            let cell_hint = col.is_some_and(|c| c.cell_hint);
                            let hint = |ui: &mut egui::Ui, shortened: bool| {
                                if cell_hint && shortened {
                                    ui.interact(
                                        cell_rect,
                                        ui.id().with((item, c_idx)),
                                        Sense::hover(),
                                    )
                                    .on_hover_text(cell_text);
                                }
                            };
                            let mut handled = false;
                            let mut do_button = false;
                            let mut do_lookup_search: Option<String> = None;
                            let mut toggle_lookup = false;
                            let mut picked_lookup_row: Option<usize> = None;
                            if is_open_editor && let Some(edit) = self.state.edit.as_mut() {
                                let in_cell = egui::UiBuilder::new().max_rect(inner_rect);
                                let size = vec2(inner_rect.width(), inner_rect.height());
                                match col.map(crawrad_runtime::grid::effective_editor) {
                                    Some("edList") => {
                                        let caption = format!("{} ▼", edit.editor_text);
                                        let button = ui
                                            .scope_builder(in_cell, |ui| {
                                                ui.add_sized(size, egui::Button::new(caption))
                                            })
                                            .inner;
                                        if button.clicked() {
                                            self.open_cell_list = !self.open_cell_list;
                                        }
                                        let picked = self
                                            .open_cell_list
                                            .then(|| {
                                                let options = col
                                                    .map(|c| c.pick_list.as_slice())
                                                    .unwrap_or(&[]);
                                                egui::Area::new(ui.id().with("cell_pick_list"))
                                                    .order(egui::Order::Foreground)
                                                    .fixed_pos(button.rect.left_bottom())
                                                    .show(ui.ctx(), |ui| {
                                                        egui::Frame::popup(ui.style())
                                                            .show(ui, |ui| {
                                                                options
                                                                    .iter()
                                                                    .find(|o| {
                                                                        ui.button(o.as_str())
                                                                            .clicked()
                                                                    })
                                                                    .cloned()
                                                            })
                                                            .inner
                                                    })
                                                    .inner
                                            })
                                            .flatten();
                                        if let Some(option) = picked {
                                            edit.editor_text = option;
                                            self.open_cell_list = false;
                                            self.state.commit_editor();
                                        }
                                    }
                                    Some("edButton") => {
                                        let btn_w = 26.0_f32.min(inner_rect.width());
                                        let text_w = (inner_rect.width() - btn_w).max(0.0);
                                        let text_rect = Rect::from_min_size(
                                            inner_rect.min,
                                            vec2(text_w, inner_rect.height()),
                                        );
                                        let btn_rect = Rect::from_min_size(
                                            egui::pos2(inner_rect.right() - btn_w, inner_rect.top()),
                                            vec2(btn_w, inner_rect.height()),
                                        );

                                        let edit_id = ui.id().with((item, c_idx, "cell_text_edit"));
                                        let text_box = egui::TextEdit::singleline(&mut edit.editor_text)
                                            .id(edit_id);
                                        let resp = ui
                                            .scope_builder(egui::UiBuilder::new().max_rect(text_rect), |ui| {
                                                ui.add_sized(vec2(text_w, inner_rect.height()), text_box)
                                            })
                                            .inner;
                                        resp.request_focus();
                                        if let Some(col_ref) = col
                                            && !col_ref.edit_mask.is_empty()
                                            && resp.changed()
                                        {
                                            let mask = crawrad_runtime::mask::EditMask::parse(
                                                &col_ref.edit_mask,
                                            );
                                            let before = &edit.editor_from;
                                            crate::hold_text(
                                                ui.ctx(),
                                                edit_id,
                                                &mut edit.editor_text,
                                                |edited, cursor| {
                                                    mask.hold_edit(before, edited, cursor)
                                                },
                                            );
                                        }

                                        let btn_resp = ui.interact(
                                            btn_rect,
                                            ui.id().with((item, c_idx, "cell_button")),
                                            Sense::click(),
                                        );
                                        let btn_bg = if btn_resp.is_pointer_button_down_on() {
                                            egui::Color32::from_rgb(0x38, 0x3a, 0x40)
                                        } else if btn_resp.hovered() {
                                            egui::Color32::from_rgb(0x33, 0x35, 0x3a)
                                        } else {
                                            egui::Color32::from_rgb(0x2b, 0x2d, 0x30)
                                        };
                                        ui.painter().rect_filled(btn_rect, 0.0, btn_bg);
                                        ui.painter().line_segment(
                                            [btn_rect.left_top(), btn_rect.left_bottom()],
                                            Stroke::new(1.0, egui::Color32::from_rgb(0x39, 0x3b, 0x40)),
                                        );
                                        ui.painter().text(
                                            btn_rect.center(),
                                            Align2::CENTER_CENTER,
                                            "…",
                                            FontId::proportional(12.0),
                                            ui.visuals().text_color(),
                                        );
                                        if btn_resp.clicked() {
                                            do_button = true;
                                        }
                                    }
                                    Some("edLookup") => {
                                        let arrow_w = 20.0_f32.min(inner_rect.width());
                                        let text_w = (inner_rect.width() - arrow_w).max(0.0);
                                        let text_rect = Rect::from_min_size(
                                            inner_rect.min,
                                            vec2(text_w, inner_rect.height()),
                                        );
                                        let arrow_rect = Rect::from_min_size(
                                            egui::pos2(inner_rect.right() - arrow_w, inner_rect.top()),
                                            vec2(arrow_w, inner_rect.height()),
                                        );

                                        let edit_id = ui.id().with((item, c_idx, "cell_text_edit"));
                                        let text_box = egui::TextEdit::singleline(&mut edit.editor_text)
                                            .id(edit_id);
                                        let resp = ui
                                            .scope_builder(egui::UiBuilder::new().max_rect(text_rect), |ui| {
                                                ui.add_sized(vec2(text_w, inner_rect.height()), text_box)
                                            })
                                            .inner;
                                        resp.request_focus();
                                        if resp.changed() {
                                            do_lookup_search = Some(edit.editor_text.clone());
                                        }

                                        let arrow_resp = ui.interact(
                                            arrow_rect,
                                            ui.id().with((item, c_idx, "cell_lookup_arrow")),
                                            Sense::click(),
                                        );
                                        let arrow_bg = if arrow_resp.hovered() {
                                            ui.visuals().widgets.hovered.bg_fill
                                        } else {
                                            egui::Color32::TRANSPARENT
                                        };
                                        ui.painter().rect_filled(arrow_rect, 0.0, arrow_bg);
                                        ui.painter().text(
                                            arrow_rect.center(),
                                            Align2::CENTER_CENTER,
                                            "▼",
                                            FontId::proportional(10.0),
                                            ui.visuals().weak_text_color(),
                                        );
                                        if arrow_resp.clicked() {
                                            toggle_lookup = true;
                                        }

                                        if let Some(lookup_list) = &edit.lookup {
                                            let popup_pos = egui::pos2(inner_rect.left(), inner_rect.bottom() + 1.0);
                                            let popup_w = inner_rect.width().max(240.0);
                                            egui::Area::new(ui.id().with("cell_lookup_dropdown"))
                                                .order(egui::Order::Foreground)
                                                .fixed_pos(popup_pos)
                                                .show(ui.ctx(), |ui| {
                                                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                                                        ui.set_min_width(popup_w);
                                                        if let Some(col_ref) = col && !col_ref.lookup_display.is_empty() {
                                                            ui.horizontal(|ui| {
                                                                let num_cols = col_ref.lookup_display.len().max(1);
                                                                let part_w = (ui.available_width() - 8.0) / (num_cols as f32);
                                                                for name in &col_ref.lookup_display {
                                                                    ui.add_sized(
                                                                        vec2(part_w, 14.0),
                                                                        egui::Label::new(
                                                                            egui::RichText::new(name)
                                                                                .strong()
                                                                                .size(10.0)
                                                                                .color(ui.visuals().weak_text_color()),
                                                                        ),
                                                                    );
                                                                }
                                                            });
                                                            ui.separator();
                                                        }
                                                        let row_h = 20.0_f32;
                                                        let shown = col.map(|c| c.drop_down_count).unwrap_or(8).max(1);
                                                        let max_h = row_h * shown as f32;
                                                        egui::ScrollArea::vertical()
                                                            .max_height(max_h)
                                                            .show(ui, |ui| {
                                                                ui.spacing_mut().item_spacing = Vec2::ZERO;
                                                                for (r_idx, (_key, display_cols)) in lookup_list.rows.iter().enumerate() {
                                                                    let is_hl = r_idx == lookup_list.highlighted;
                                                                    let (row_rect, row_resp) = ui.allocate_exact_size(
                                                                        vec2(ui.available_width(), row_h),
                                                                        Sense::click(),
                                                                    );
                                                                    if is_hl {
                                                                        ui.painter().rect_filled(
                                                                            row_rect,
                                                                            0.0,
                                                                            egui::Color32::from_rgb(0x2e, 0x43, 0x6b),
                                                                        );
                                                                    } else if row_resp.hovered() {
                                                                        ui.painter().rect_filled(
                                                                            row_rect,
                                                                            0.0,
                                                                            ui.visuals().faint_bg_color,
                                                                        );
                                                                    }
                                                                    let num_cols = display_cols.len().max(1);
                                                                    let part_w = row_rect.width() / (num_cols as f32);
                                                                    for (d_idx, col_text) in display_cols.iter().enumerate() {
                                                                        let text_pos = egui::pos2(
                                                                            row_rect.left() + (d_idx as f32) * part_w + 4.0,
                                                                            row_rect.center().y,
                                                                        );
                                                                        let pfx = &lookup_list.prefix;
                                                                        // Byte length of the typed prefix inside the label: only
                                                                        // where it ends on a character (a lowercase may change size).
                                                                        let cut = pfx.len();
                                                                        if !pfx.is_empty()
                                                                            && col_text.is_char_boundary(cut)
                                                                            && col_text[..cut].to_lowercase() == pfx.to_lowercase()
                                                                        {
                                                                            let (matched, rest) = col_text.split_at(cut);
                                                                            let mut job = egui::text::LayoutJob::default();
                                                                            job.append(
                                                                                matched,
                                                                                0.0,
                                                                                egui::TextFormat {
                                                                                    font_id: FontId::proportional(11.0),
                                                                                    color: egui::Color32::WHITE,
                                                                                    ..Default::default()
                                                                                },
                                                                            );
                                                                            job.append(
                                                                                rest,
                                                                                0.0,
                                                                                egui::TextFormat {
                                                                                    font_id: FontId::proportional(11.0),
                                                                                    color: ui.visuals().text_color(),
                                                                                    ..Default::default()
                                                                                },
                                                                            );
                                                                            let galley = ui.painter().layout_job(job);
                                                                            ui.painter().galley(
                                                                                text_pos - vec2(0.0, galley.size().y * 0.5),
                                                                                galley,
                                                                                egui::Color32::WHITE,
                                                                            );
                                                                        } else {
                                                                            ui.painter().text(
                                                                                text_pos,
                                                                                Align2::LEFT_CENTER,
                                                                                col_text,
                                                                                FontId::proportional(11.0),
                                                                                ui.visuals().text_color(),
                                                                            );
                                                                        }
                                                                    }
                                                                    if row_resp.clicked() {
                                                                        picked_lookup_row = Some(r_idx);
                                                                    }
                                                                }
                                                            });
                                                        ui.separator();
                                                        let count_n = lookup_list.rows.len();
                                                        let footer = match lookup_list.total_hint {
                                                            Some(total_m) => format!(
                                                                "{} of {} · ↑↓ choose · Enter picks · Esc closes",
                                                                count_n, total_m
                                                            ),
                                                            None => format!(
                                                                "{} · ↑↓ choose · Enter picks · Esc closes",
                                                                count_n
                                                            ),
                                                        };
                                                        ui.label(
                                                            egui::RichText::new(footer)
                                                                .size(10.0)
                                                                .color(ui.visuals().weak_text_color()),
                                                        );
                                                    });
                                                });
                                        }
                                    }
                                    editor => {
                                        let edit_id = ui.id().with((item, c_idx, "cell_text_edit"));
                                        let mut text_box =
                                            egui::TextEdit::singleline(&mut edit.editor_text)
                                                .id(edit_id);
                                        if editor == Some("edNumber") {
                                            text_box = text_box.horizontal_align(Align::Max);
                                        }
                                        let resp = ui
                                            .scope_builder(in_cell, |ui| {
                                                ui.add_sized(size, text_box)
                                            })
                                            .inner;
                                        resp.request_focus();
                                        if let Some(col_ref) = col
                                            && !col_ref.edit_mask.is_empty()
                                            && resp.changed()
                                        {
                                            let mask = crawrad_runtime::mask::EditMask::parse(
                                                &col_ref.edit_mask,
                                            );
                                            let before = &edit.editor_from;
                                            crate::hold_text(
                                                ui.ctx(),
                                                edit_id,
                                                &mut edit.editor_text,
                                                |edited, cursor| {
                                                    mask.hold_edit(before, edited, cursor)
                                                },
                                            );
                                        }
                                    }
                                }
                                handled = true;
                            }
                            if do_button {
                                let outcome = self.state.trigger_button();
                                self.apply(outcome);
                            }
                            if let Some(prefix) = do_lookup_search {
                                self.state.update_lookup_search(&prefix);
                            }
                            if toggle_lookup
                                && let Some((listed, typed)) = self
                                    .state
                                    .edit
                                    .as_ref()
                                    .map(|e| (e.lookup.is_some(), e.editor_text.clone()))
                            {
                                if listed {
                                    if let Some(edit) = self.state.edit.as_mut() {
                                        edit.lookup = None;
                                    }
                                } else {
                                    self.state.update_lookup_search(&typed);
                                }
                            }
                            if let Some(r_idx) = picked_lookup_row {
                                self.state.pick_lookup(r_idx);
                            }
                            match renderer {
                                CellRenderer::CheckBox => {
                                    if let Some(checked) =
                                        raw_cell.and_then(crawrad_runtime::grid::checkbox_state)
                                    {
                                        let box_size = 14.0_f32.min(inner_rect.height());
                                        let box_x = match horiz_align {
                                            Align::Min => inner_rect.left(),
                                            Align::Center => {
                                                inner_rect.left()
                                                    + (inner_rect.width() - box_size) * 0.5
                                            }
                                            Align::Max => inner_rect.right() - box_size,
                                        };
                                        let box_y = match vert_align {
                                            Align::Min => inner_rect.top(),
                                            Align::Center => {
                                                inner_rect.top()
                                                    + (inner_rect.height() - box_size) * 0.5
                                            }
                                            Align::Max => inner_rect.bottom() - box_size,
                                        };
                                        let box_rect = Rect::from_min_size(
                                            egui::pos2(box_x, box_y),
                                            vec2(box_size, box_size),
                                        );
                                        ui.painter().rect_filled(box_rect, 2.0, RENDERER_BG);
                                        ui.painter().rect_stroke(
                                            box_rect,
                                            2.0,
                                            Stroke::new(1.0, RENDERER_BORDER),
                                            StrokeKind::Inside,
                                        );
                                        if checked {
                                            let inner = Rect::from_min_size(
                                                egui::pos2(box_x + 3.0, box_y + 3.0),
                                                vec2(
                                                    (box_size - 6.0).max(0.0),
                                                    (box_size - 6.0).max(0.0),
                                                ),
                                            );
                                            ui.painter().rect_filled(inner, 1.0, RENDERER_ACCENT);
                                        }
                                        handled = true;
                                    }
                                }
                                CellRenderer::ProgressBar => {
                                    if let Some(frac) =
                                        raw_cell.and_then(crawrad_runtime::grid::progress_fraction)
                                    {
                                        let bar_h = 16.0_f32.min(inner_rect.height());
                                        let bar_rect = Rect::from_min_size(
                                            egui::pos2(
                                                inner_rect.left(),
                                                inner_rect.top()
                                                    + (inner_rect.height() - bar_h) * 0.5,
                                            ),
                                            vec2(inner_rect.width(), bar_h),
                                        );
                                        ui.painter().rect_filled(bar_rect, 2.0, RENDERER_BG);
                                        ui.painter().rect_stroke(
                                            bar_rect,
                                            2.0,
                                            Stroke::new(1.0, RENDERER_BORDER),
                                            StrokeKind::Inside,
                                        );
                                        let fill_w = inner_rect.width() * frac as f32;
                                        if fill_w > 0.0 {
                                            let fill_rect = Rect::from_min_size(
                                                bar_rect.min,
                                                vec2(fill_w, bar_h),
                                            );
                                            ui.painter().rect_filled(
                                                fill_rect,
                                                2.0,
                                                RENDERER_ACCENT,
                                            );
                                        }
                                        let fitted_text = crawrad_runtime::grid::fit_text(
                                            cell_text,
                                            inner_rect.width() as f64,
                                            ellipsis,
                                            |s| text_width(ui, &font_id, s),
                                        );
                                        let text_pos = bar_rect.center();
                                        ui.painter().text(
                                            text_pos,
                                            Align2::CENTER_CENTER,
                                            fitted_text,
                                            font_id.clone(),
                                            effective_text_color,
                                        );
                                        handled = true;
                                    }
                                }
                                CellRenderer::ColourSwatch => {
                                    if let Some(c) =
                                        raw_cell.and_then(crawrad_runtime::grid::parse_hex_color)
                                    {
                                        let swatch_size = 12.0_f32.min(inner_rect.height());
                                        let swatch_y = inner_rect.top()
                                            + (inner_rect.height() - swatch_size) * 0.5;
                                        let swatch_rect = Rect::from_min_size(
                                            egui::pos2(inner_rect.left(), swatch_y),
                                            vec2(swatch_size, swatch_size),
                                        );
                                        let c_col = egui::Color32::from_rgba_unmultiplied(
                                            c.r, c.g, c.b, c.a,
                                        );
                                        ui.painter().rect_filled(swatch_rect, 1.0, c_col);
                                        ui.painter().rect_stroke(
                                            swatch_rect,
                                            1.0,
                                            Stroke::new(1.0, RENDERER_BORDER),
                                            StrokeKind::Inside,
                                        );
                                        let rem_w =
                                            (inner_rect.width() - swatch_size - 6.0).max(0.0);
                                        let fitted_text = crawrad_runtime::grid::fit_text(
                                            cell_text,
                                            rem_w as f64,
                                            ellipsis,
                                            |s| text_width(ui, &font_id, s),
                                        );
                                        let shortened = fitted_text != cell_text;
                                        hint(ui, shortened);
                                        let text_x = inner_rect.left() + swatch_size + 6.0;
                                        let text_pos = egui::pos2(text_x, text_y);
                                        ui.painter().text(
                                            text_pos,
                                            Align2([Align::Min, vert_align]),
                                            fitted_text,
                                            font_id.clone(),
                                            effective_text_color,
                                        );
                                        handled = true;
                                    }
                                }
                                CellRenderer::Link => {
                                    let link_color = egui::Color32::from_rgb(0x89, 0xB4, 0xFA);
                                    let fitted_text = crawrad_runtime::grid::fit_text(
                                        cell_text,
                                        avail_w,
                                        ellipsis,
                                        |s| text_width(ui, &font_id, s),
                                    );
                                    let shortened = fitted_text != cell_text;
                                    hint(ui, shortened);
                                    let t_width = text_width(ui, &font_id, &fitted_text) as f32;
                                    let line_h =
                                        ui.ctx().fonts_mut(|f| f.row_height(&font_id)).max(12.0);
                                    let text_pos = egui::pos2(text_x, text_y);
                                    ui.painter().text(
                                        text_pos,
                                        Align2([horiz_align, vert_align]),
                                        fitted_text,
                                        font_id.clone(),
                                        link_color,
                                    );
                                    let line_x = match horiz_align {
                                        Align::Min => inner_rect.left(),
                                        Align::Center => {
                                            inner_rect.left() + (inner_rect.width() - t_width) * 0.5
                                        }
                                        Align::Max => inner_rect.right() - t_width,
                                    };
                                    let underline_y = match vert_align {
                                        Align::Min => inner_rect.top() + line_h,
                                        Align::Center => inner_rect.center().y + line_h * 0.5,
                                        Align::Max => inner_rect.bottom(),
                                    };
                                    ui.painter().line_segment(
                                        [
                                            egui::pos2(line_x, underline_y),
                                            egui::pos2(
                                                line_x + t_width.min(inner_rect.width()),
                                                underline_y,
                                            ),
                                        ],
                                        Stroke::new(1.0, link_color),
                                    );
                                    let link_resp = ui.interact(
                                        cell_rect,
                                        ui.id().with((item, c_idx, "link")),
                                        Sense::click(),
                                    );
                                    if link_resp.hovered() {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }
                                    if link_resp.clicked() {
                                        clicked_rec = Some((*index, *id));
                                        if let Some(val) = raw_cell
                                            && crawrad_runtime::shell::openable(val)
                                        {
                                            crawrad_runtime::shell::open_url(val);
                                        }
                                    }
                                    handled = true;
                                }
                                CellRenderer::Text => {}
                            }

                            if !handled {
                                let has_font_style =
                                    cell_style.as_ref().is_some_and(|cs| cs.bold || cs.italic);
                                if word_wrap {
                                    let line_h =
                                        ui.ctx().fonts_mut(|f| f.row_height(&font_id)).max(12.0);
                                    let avail_h = (inner_rect.height() as f64).max(line_h as f64);
                                    let max_lines =
                                        ((avail_h / line_h as f64).floor() as usize).max(1);
                                    let lines =
                                        wrap_text(cell_text, avail_w, max_lines, ellipsis, |s| {
                                            text_width(ui, &font_id, s)
                                        });
                                    let shortened = lines.join(" ") != cell_text;
                                    hint(ui, shortened);

                                    let total_text_h = (lines.len() as f32) * line_h;
                                    let start_y = match vert_align {
                                        Align::Min => inner_rect.top() + line_h * 0.5,
                                        Align::Center => {
                                            inner_rect.center().y - total_text_h * 0.5
                                                + line_h * 0.5
                                        }
                                        Align::Max => {
                                            inner_rect.bottom() - total_text_h + line_h * 0.5
                                        }
                                    };
                                    for (l_idx, line) in lines.into_iter().enumerate() {
                                        let y = start_y + (l_idx as f32) * line_h;
                                        let line_pos = egui::pos2(text_x, y);
                                        if has_font_style {
                                            let mut rich = egui::RichText::new(&line)
                                                .color(effective_text_color)
                                                .size(font_size);
                                            if cell_style.as_ref().is_some_and(|cs| cs.bold) {
                                                rich = rich.strong();
                                            }
                                            if cell_style.as_ref().is_some_and(|cs| cs.italic) {
                                                rich = rich.italics();
                                            }
                                            let widget_text: egui::WidgetText = rich.into();
                                            let galley = widget_text.into_galley(
                                                ui,
                                                None,
                                                avail_w as f32,
                                                font_id.clone(),
                                            );
                                            let x = match horiz_align {
                                                Align::Min => text_x,
                                                Align::Center => text_x - galley.size().x * 0.5,
                                                Align::Max => text_x - galley.size().x,
                                            };
                                            let gy = y - galley.size().y * 0.5;
                                            ui.painter().galley(
                                                egui::pos2(x, gy),
                                                galley,
                                                effective_text_color,
                                            );
                                        } else {
                                            ui.painter().text(
                                                line_pos,
                                                Align2([horiz_align, Align::Center]),
                                                line,
                                                font_id.clone(),
                                                effective_text_color,
                                            );
                                        }
                                    }
                                } else {
                                    let fitted_text = crawrad_runtime::grid::fit_text(
                                        cell_text,
                                        avail_w,
                                        ellipsis,
                                        |s| text_width(ui, &font_id, s),
                                    );
                                    let shortened = fitted_text != cell_text;
                                    hint(ui, shortened);

                                    let align = Align2([horiz_align, vert_align]);
                                    let text_pos = egui::pos2(text_x, text_y);
                                    if has_font_style {
                                        let mut rich = egui::RichText::new(&fitted_text)
                                            .color(effective_text_color)
                                            .size(font_size);
                                        if cell_style.as_ref().is_some_and(|cs| cs.bold) {
                                            rich = rich.strong();
                                        }
                                        if cell_style.as_ref().is_some_and(|cs| cs.italic) {
                                            rich = rich.italics();
                                        }
                                        let widget_text: egui::WidgetText = rich.into();
                                        let galley = widget_text.into_galley(
                                            ui,
                                            None,
                                            avail_w as f32,
                                            font_id.clone(),
                                        );
                                        let x = match horiz_align {
                                            Align::Min => text_x,
                                            Align::Center => text_x - galley.size().x * 0.5,
                                            Align::Max => text_x - galley.size().x,
                                        };
                                        let y = match vert_align {
                                            Align::Min => text_y,
                                            Align::Center => text_y - galley.size().y * 0.5,
                                            Align::Max => text_y - galley.size().y,
                                        };
                                        ui.painter().galley(
                                            egui::pos2(x, y),
                                            galley,
                                            effective_text_color,
                                        );
                                    } else {
                                        ui.painter().text(
                                            text_pos,
                                            align,
                                            fitted_text,
                                            font_id,
                                            effective_text_color,
                                        );
                                    }
                                }
                            }

                            if is_editing_this_row
                                && let Some((err_col, err_msg)) = &view.error_cell
                                && *err_col == c_idx
                            {
                                ui.painter().rect_stroke(
                                    cell_rect,
                                    0.0,
                                    Stroke::new(2.0, egui::Color32::from_rgb(0xff, 0x4d, 0x4f)),
                                    StrokeKind::Inside,
                                );
                                // Over the rows below it, which are painted after this one.
                                let above = ui.ctx().layer_painter(egui::LayerId::new(
                                    egui::Order::Foreground,
                                    ui.id().with("grid_cell_error"),
                                ));
                                let err_rect = Rect::from_min_size(
                                    egui::pos2(cell_rect.left(), cell_rect.bottom() + 1.0),
                                    vec2(cell_rect.width().max(160.0), 20.0),
                                );
                                above.rect_filled(
                                    err_rect,
                                    CornerRadius::same(3),
                                    egui::Color32::from_rgb(0x4a, 0x14, 0x14),
                                );
                                above.rect_stroke(
                                    err_rect,
                                    CornerRadius::same(3),
                                    Stroke::new(1.0, egui::Color32::from_rgb(0xff, 0x4d, 0x4f)),
                                    StrokeKind::Inside,
                                );
                                above.text(
                                    err_rect.center(),
                                    Align2::CENTER_CENTER,
                                    err_msg,
                                    FontId::proportional(11.0),
                                    egui::Color32::from_rgb(0xff, 0xcc, 0xcc),
                                );
                            }

                            let cell_click_resp = ui.interact(
                                cell_rect,
                                ui.id().with((item, c_idx, "cell_click")),
                                Sense::click(),
                            );
                            if cell_click_resp.clicked() {
                                clicked_cell = Some((
                                    *index,
                                    *id,
                                    raw_cells.clone(),
                                    *marker == EGUI_MARKS.new_row,
                                    c_idx,
                                ));
                            }

                            col_x += *col_w;
                        }
                    }
                }

                // A click on a cell (or elsewhere on a row) is the grid's: it takes the focus.
                let clicked = clicked_cell.or_else(|| {
                    clicked_rec
                        .map(|(index, id)| (index, id, Vec::new(), false, self.state.focus_col))
                });
                if let Some((index, id, raw, new, position)) = clicked {
                    self.keyboard = true;
                    let row = CurrentRow {
                        index,
                        id,
                        raw: &raw,
                        new,
                    };
                    let outcome = self.state.click_cell(&row, position);
                    self.apply(outcome);
                }
            },
        );

        // The grid answers the keyboard only while it owns it (see `keyboard`).
        let outside = ui.input(|i| {
            i.pointer.any_pressed()
                && i.pointer
                    .press_origin()
                    .is_some_and(|p| !scroll_output.inner_rect.contains(p))
        });
        if outside && !self.open_cell_list {
            self.keyboard = false;
        }
        if self.keyboard {
            let total_rows = rows_to_show(self.live.screen().total, self.shown_total);
            let rec_before = self.state.rec_no;
            for key in edit_keys(ui) {
                let current = view.lines.iter().find_map(|l| match l {
                    ViewLine::Row {
                        index,
                        id,
                        raw_cells,
                        marker,
                        ..
                    } if *index == self.state.rec_no => Some(CurrentRow {
                        index: *index,
                        id: *id,
                        raw: raw_cells,
                        new: *marker == EGUI_MARKS.new_row,
                    }),
                    _ => None,
                });
                let outcome = self.state.edit_key(key, current.as_ref(), total_rows);
                if outcome != EditOutcome::Ignored {
                    self.open_cell_list = false;
                }
                self.apply(outcome);
            }
            if self.state.rec_no != rec_before {
                self.follow_rec = true;
                self.reveal_rec = true;
                ui.ctx().request_repaint();
            }
        }
        self.scroll_offset = scroll_output.state.offset.y;
        self.rows_height = scroll_output.inner_rect.height();

        let rows_rect = scroll_output.inner_rect;

        // d. Veil: dimming translucent curtain over rows during query loading
        if view.rows_alpha < 1.0 {
            let veil_alpha = ((1.0 - view.rows_alpha) as f32).clamp(0.0, 1.0);
            let veil_color = ui.visuals().panel_fill.gamma_multiply(veil_alpha);
            ui.painter().rect_filled(rows_rect, 0.0, veil_color);
        }

        // e. Centered loading card with animated spinner and telemetry text
        if let Some(ov) = &view.overlay {
            let opacity = (ov.opacity as f32).clamp(0.0, 1.0);
            let card_bg = ui.visuals().window_fill.gamma_multiply(opacity);
            let border_stroke = Stroke::new(
                1.0,
                ui.visuals()
                    .widgets
                    .noninteractive
                    .bg_stroke
                    .color
                    .gamma_multiply(opacity),
            );
            let title_color = ui.visuals().text_color().gamma_multiply(opacity);
            let detail_color = ui.visuals().weak_text_color().gamma_multiply(opacity);
            let spinner_color = ui.visuals().selection.bg_fill.gamma_multiply(opacity);

            let font_title = FontId::proportional(12.0);
            let font_detail = FontId::proportional(11.0);
            let galley_title =
                ui.painter()
                    .layout_no_wrap(ov.title.clone(), font_title, title_color);
            let galley_detail =
                ui.painter()
                    .layout_no_wrap(ov.detail.clone(), font_detail, detail_color);

            let text_w = galley_title.size().x.max(galley_detail.size().x);
            let text_h = galley_title.size().y + 4.0 + galley_detail.size().y;

            let spinner_radius = 10.0;
            let spinner_diam = spinner_radius * 2.0;
            let gap = 12.0;
            let padding = vec2(18.0, 12.0);

            let card_rect = compute_loading_card_rect(
                rows_rect,
                vec2(text_w, text_h),
                spinner_diam,
                padding,
                gap,
            );

            ui.painter().rect_filled(card_rect, 4.0, card_bg);
            ui.painter()
                .rect_stroke(card_rect, 4.0, border_stroke, StrokeKind::Outside);

            // Rotating spinner on the left
            let spinner_center = egui::pos2(
                card_rect.left() + padding.x + spinner_radius,
                card_rect.center().y,
            );
            let time = ui.input(|i| i.time);
            let arc_points = compute_spinner_arc(spinner_center, spinner_radius, time, 24);
            ui.painter().add(egui::Shape::line(
                arc_points,
                Stroke::new(2.5, spinner_color),
            ));

            // Title and detail labels on the right
            let text_left = card_rect.left() + padding.x + spinner_diam + gap;
            let text_top = card_rect.center().y - text_h * 0.5;

            let title_h = galley_title.size().y;
            ui.painter()
                .galley(egui::pos2(text_left, text_top), galley_title, title_color);
            ui.painter().galley(
                egui::pos2(text_left, text_top + title_h + 4.0),
                galley_detail,
                detail_color,
            );
        }

        // Summary footer row
        if !view.summary.is_empty() {
            let col_widths = compute_grid_column_widths(content_width, &ordered_cols);
            let (sum_rect, _) = ui.allocate_exact_size(
                vec2(gutter_w + content_width, self.row_height),
                Sense::hover(),
            );
            ui.painter()
                .rect_filled(sum_rect, 0.0, ui.visuals().widgets.noninteractive.bg_fill);
            let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            ui.painter()
                .line_segment([sum_rect.left_top(), sum_rect.right_top()], stroke);
            ui.painter()
                .line_segment([sum_rect.left_bottom(), sum_rect.right_bottom()], stroke);

            if view.gutter {
                let g_rect = Rect::from_min_size(
                    Pos2::new(sum_rect.left(), sum_rect.top()),
                    vec2(gutter_w, self.row_height),
                );
                ui.painter()
                    .line_segment([g_rect.right_top(), g_rect.right_bottom()], stroke);
            }

            let mut col_x = sum_rect.left() + gutter_w;
            for (c_idx, col_w) in col_widths.iter().enumerate() {
                let cell_rect = Rect::from_min_size(
                    Pos2::new(col_x, sum_rect.top()),
                    vec2(*col_w, self.row_height),
                );
                ui.painter()
                    .line_segment([cell_rect.right_top(), cell_rect.right_bottom()], stroke);

                if let Some(text) = view.summary.get(c_idx)
                    && !text.is_empty()
                {
                    let col_align = ordered_cols
                        .get(c_idx)
                        .map(|c| c.align)
                        .unwrap_or(ColumnAlign::Left);
                    let align = match col_align {
                        ColumnAlign::Left => Align2::LEFT_CENTER,
                        ColumnAlign::Center => Align2::CENTER_CENTER,
                        ColumnAlign::Right => Align2::RIGHT_CENTER,
                    };
                    let text_pos = match col_align {
                        ColumnAlign::Left => cell_rect.left_center() + vec2(8.0, 0.0),
                        ColumnAlign::Center => cell_rect.center(),
                        ColumnAlign::Right => cell_rect.right_center() - vec2(8.0, 0.0),
                    };
                    let font_id = FontId::proportional(12.0);
                    ui.painter().text(
                        text_pos,
                        align,
                        text.clone(),
                        font_id,
                        ui.visuals().text_color(),
                    );
                }
                col_x += *col_w;
            }
        }

        // One plain line around the grid, drawn last so no row or header covers it.
        let frame = Rect::from_min_max(
            frame_top_left,
            Pos2::new(frame_top_left.x + frame_width, ui.cursor().top()),
        );
        ui.painter().rect_stroke(
            frame,
            0.0,
            ui.visuals().widgets.noninteractive.bg_stroke,
            egui::StrokeKind::Inside,
        );

        // f. Status footer showing record index, fetch state, order description, timing, and errors
        ui.horizontal(|ui| {
            if !view.status_text.is_empty() {
                ui.label(
                    egui::RichText::new(&view.status_text)
                        .color(egui::Color32::from_rgb(0xff, 0xd2, 0x7a))
                        .strong(),
                );
            } else {
                ui.label(
                    egui::RichText::new(&view.footer.position).color(ui.visuals().text_color()),
                );
                if !view.footer.state.is_empty() {
                    ui.label(
                        egui::RichText::new(view.footer.state)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(err) = &view.error {
                    ui.label(egui::RichText::new(err).color(ui.visuals().warn_fg_color));
                }
                if !view.footer.timing.is_empty() {
                    ui.label(
                        egui::RichText::new(&view.footer.timing)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
                if !view.footer.order.is_empty() {
                    ui.label(
                        egui::RichText::new(&view.footer.order)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
            });
        });
    }
}

/// Renderer parts: the box/track fill, its border and the accent, as on the canvas.
const RENDERER_BG: egui::Color32 = egui::Color32::from_rgb(0x18, 0x18, 0x25);
const RENDERER_BORDER: egui::Color32 = egui::Color32::from_rgb(0x6E, 0x72, 0x90);
const RENDERER_ACCENT: egui::Color32 = egui::Color32::from_rgb(0x89, 0xB4, 0xFA);

/// Two small boxes with "–" between (the fkRange and fkDate filters); the new
/// pair when either changed.
fn pair_edit(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    mut lo: String,
    mut hi: String,
    hints: (&str, &str),
) -> Option<(String, String)> {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        let size = vec2(((rect.width() - 14.0) / 2.0).max(10.0), rect.height());
        let a = ui.add_sized(size, egui::TextEdit::singleline(&mut lo).hint_text(hints.0));
        ui.label("–");
        let b = ui.add_sized(size, egui::TextEdit::singleline(&mut hi).hint_text(hints.1));
        (a.changed() || b.changed()).then_some((lo, hi))
    })
    .inner
}
