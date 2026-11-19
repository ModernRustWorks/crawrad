//! **A ComboBox component**: a drop-down field with our own list and `DropDownCount`.

use crawrad_runtime::list_selection;
use makepad_widgets::*;

use crate::list::ListLook;
// the boxes and the caret are the MultiSelect's: one look for both lists
use crate::multi_select::{DrawMultiSelectCaret, DrawMultiSelectCell};
use crawrad_note::note;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;
    use crate::multi_select::*;

    // A check mark drawn with Sdf2d at the selected row's right
    pub DrawComboBoxCheck = {{DrawComboBoxCheck}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            sdf.move_to(w * 0.15, h * 0.50);
            sdf.line_to(w * 0.40, h * 0.80);
            sdf.line_to(w * 0.85, h * 0.20);
            sdf.stroke(self.color, 1.6);
            return sdf.result;
        }
    }

    pub CrawradComboBoxBase = {{CrawradComboBox}} {}

    pub CrawradComboBox = <CrawradComboBoxBase> {
        width: Fill, height: Fill,
        text_input: <TextInput> {
            padding: {left: 8.0, top: 4.0, right: 4.0, bottom: 4.0}
            empty_text: ""
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
            }
        }
        draw_field: {
            color: #181825,
            border_color: #45475a,
            radius: 2.0
        }
        draw_list_bg: {
            color: #1e1e2e,
            border_color: #585b70,
            radius: 4.0
        }
        draw_item_hover: {
            color: #313244,
            border_color: #00000000,
            radius: 2.0
        }
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_text_selected: {
            color: #89b4fa,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_check: {
            color: #89b4fa
        }
        draw_chevron: {
            color: #cdd6f4
        }
        draw_scroll_track: {
            color: #313244,
            border_color: #00000000,
            radius: 2.0
        }
        draw_scroll_thumb: {
            color: #6c7086,
            border_color: #00000000,
            radius: 2.0
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawComboBoxCheck {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradComboBoxAction {
    Selected(usize),
    TextChanged(String),
    None,
}

#[note("combo-box-visible-rows")]
pub fn visible_rows(count: usize, len: usize) -> usize {
    if count == 0 { len } else { len.min(count) }
}

#[note("combo-box-offset-showing")]
pub fn offset_showing(row: usize, offset: usize, rows: usize, len: usize) -> usize {
    if len == 0 || rows == 0 {
        return 0;
    }
    let max_offset = len.saturating_sub(rows);
    let offset = offset.min(max_offset);
    let target_row = row.min(len.saturating_sub(1));
    let adjusted = if target_row < offset {
        target_row
    } else if target_row >= offset + rows {
        target_row + 1 - rows
    } else {
        offset
    };
    adjusted.min(max_offset)
}

#[note("combo-box-thumb")]
pub fn thumb(offset: usize, rows: usize, len: usize, track_h: f64) -> Option<(f64, f64)> {
    if len <= rows || rows == 0 || len == 0 || track_h <= 0.0 {
        return None;
    }
    let thumb_h = track_h * (rows as f64 / len as f64);
    let max_offset = len.saturating_sub(rows);
    let y = if max_offset == 0 {
        0.0
    } else {
        (track_h - thumb_h) * (offset.min(max_offset) as f64 / max_offset as f64)
    };
    Some((y, thumb_h))
}

#[note("combo-box-step")]
pub fn step(hovered: Option<usize>, key: KeyCode, rows: usize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let last = len.saturating_sub(1);
    let step_size = rows.max(1);
    match key {
        KeyCode::ArrowUp => match hovered {
            Some(curr) => Some(curr.min(last).saturating_sub(1)),
            None => Some(last),
        },
        KeyCode::ArrowDown => match hovered {
            Some(curr) => Some((curr.min(last) + 1).min(last)),
            None => Some(0),
        },
        KeyCode::PageUp => match hovered {
            Some(curr) => Some(curr.min(last).saturating_sub(step_size)),
            None => Some(0),
        },
        KeyCode::PageDown => match hovered {
            Some(curr) => Some((curr.min(last) + step_size).min(last)),
            None => Some((step_size - 1).min(last)),
        },
        KeyCode::Home => Some(0),
        KeyCode::End => Some(last),
        _ => hovered.map(|curr| curr.min(last)),
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CrawradComboBox {
    #[walk]
    walk: Walk,
    #[live]
    text_input: TextInput,
    #[live]
    draw_list: DrawList2d,
    #[redraw]
    #[live]
    draw_field: DrawMultiSelectCell,
    #[live]
    draw_list_bg: DrawMultiSelectCell,
    #[live]
    draw_item_hover: DrawMultiSelectCell,
    #[live]
    draw_scroll_track: DrawMultiSelectCell,
    #[live]
    draw_scroll_thumb: DrawMultiSelectCell,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_text_selected: DrawText,
    #[live]
    draw_check: DrawComboBoxCheck,
    #[live]
    draw_chevron: DrawMultiSelectCaret,
    #[rust]
    items: Vec<String>,
    #[rust]
    selected: Option<usize>,
    #[rust(8usize)]
    drop_down_count: usize,
    #[rust]
    is_open: bool,
    #[rust]
    hovered_row: Option<usize>,
    #[rust]
    scroll_offset: usize,
    #[rust(true)]
    enabled: bool,
    #[visible]
    #[live(true)]
    visible: bool,
    #[rust]
    editable: bool,
    #[rust]
    filtered_indices: Vec<usize>,
    #[rust]
    area: Area,
    #[rust]
    look: ListLook,
}

impl Widget for CrawradComboBox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible || !self.enabled {
            // hidden or disabled while open: the list goes with it
            self.close_drop_down(cx);
            return;
        }

        let field_rect = self.area.rect(cx);
        let caret_rect = Rect {
            pos: dvec2(
                field_rect.pos.x + (field_rect.size.x - 22.0).max(0.0),
                field_rect.pos.y,
            ),
            size: dvec2(22.0, field_rect.size.y),
        };
        let (row_h, visible_count, list_rect) = self.list_geometry(field_rect);

        if self.is_open {
            if let Event::KeyDown(ke) = event {
                match ke.key_code {
                    KeyCode::Escape => {
                        self.close_drop_down(cx);
                        return;
                    }
                    KeyCode::ReturnKey => {
                        if let Some(row) = self.hovered_row {
                            self.pick_item(cx, scope, row);
                        } else {
                            if self.editable {
                                let typed = self.text_input.text();
                                self.selected = self.items.iter().position(|it| it == &typed);
                            }
                            self.close_drop_down(cx);
                        }
                        return;
                    }
                    key => {
                        if let Some(next) =
                            step(self.hovered_row, key, visible_count, self.rows_count())
                        {
                            if self.hovered_row != Some(next) {
                                self.hovered_row = Some(next);
                                self.scroll_offset = offset_showing(
                                    next,
                                    self.scroll_offset,
                                    visible_count,
                                    self.rows_count(),
                                );
                                self.redraw(cx);
                            }
                            return;
                        }
                    }
                }
            }

            if let Event::Scroll(se) = event
                && list_rect.contains(se.abs)
                && self.rows_count() > visible_count
            {
                if se.scroll.y > 0.0 {
                    if self.scroll_offset + visible_count < self.rows_count() {
                        self.scroll_offset += 1;
                        self.redraw(cx);
                    }
                } else if se.scroll.y < 0.0 && self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.redraw(cx);
                }
            }

            let row_at = |abs: DVec2| -> Option<usize> {
                if !list_rect.contains(abs) {
                    return None;
                }
                let rel_y = (abs.y - (list_rect.pos.y + 4.0)) as f32;
                if rel_y < 0.0 {
                    return None;
                }
                let idx_on_screen = (rel_y / row_h) as usize;
                if idx_on_screen < visible_count {
                    let index = self.scroll_offset + idx_on_screen;
                    if index < self.rows_count() {
                        return Some(index);
                    }
                }
                None
            };

            match event {
                // the mouse moves the highlight only over a row: leaving the list
                // keeps it, so the arrow keys go on from the row last shown
                Event::MouseMove(me) => {
                    if let Some(row) = row_at(me.abs)
                        && self.hovered_row != Some(row)
                    {
                        self.hovered_row = Some(row);
                        self.redraw(cx);
                    }
                }
                Event::MouseUp(me) => {
                    if let Some(row_idx) = row_at(me.abs) {
                        self.pick_item(cx, scope, row_idx);
                        return;
                    } else if caret_rect.contains(me.abs)
                        || (!list_rect.contains(me.abs) && !field_rect.contains(me.abs))
                    {
                        self.close_drop_down(cx);
                        return;
                    }
                }
                _ => {}
            }

            if !self.editable
                && matches!(event.hits(cx, self.area), Hit::FingerUp(fe) if field_rect.contains(fe.abs))
            {
                self.close_drop_down(cx);
                return;
            }
        } else if self.editable {
            if let Event::MouseUp(me) = event
                && caret_rect.contains(me.abs)
            {
                self.filtered_indices = (0..self.items.len()).collect();
                self.open_drop_down(cx);
                return;
            }
        } else if matches!(event.hits(cx, self.area), Hit::FingerUp(fe) if field_rect.contains(fe.abs))
        {
            self.open_drop_down(cx);
            return;
        }

        if self.editable {
            if let Event::MouseDown(me) = event
                && field_rect.contains(me.abs)
                && !caret_rect.contains(me.abs)
            {
                self.text_input.set_key_focus(cx);
            }
            let text_before = self.text_input.text();
            self.text_input.handle_event(cx, event, scope);
            let text_after = self.text_input.text();
            if text_after != text_before {
                self.filtered_indices = list_selection::matching(&self.items, &text_after);
                if !self.filtered_indices.is_empty() {
                    self.is_open = true;
                    self.hovered_row = Some(0);
                    self.scroll_offset = 0;
                } else {
                    self.is_open = false;
                    self.hovered_row = None;
                }
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    CrawradComboBoxAction::TextChanged(text_after),
                );
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let font_size = if self.look.font_size > 0.0 {
            self.look.font_size
        } else {
            9.0
        };
        let font_px = crawrad_component::text_line::font_px(font_size);

        let field_fill = if self.look.background[3] > 0.0 {
            let [r, g, b, a] = self.look.background;
            vec4(r, g, b, a)
        } else {
            vec4(0.09411765, 0.09411765, 0.14509805, 1.0)
        };

        let field_border = if self.is_open {
            vec4(0.5372549, 0.7058824, 0.98039216, 1.0)
        } else {
            vec4(0.27058825, 0.2784314, 0.3529412, 1.0)
        };

        let field_text_color = if self.look.text != [0, 0, 0, 0] {
            let [r, g, b, a] = self.look.text;
            vec4(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            )
        } else {
            vec4(0.8039216, 0.8392157, 0.95686275, 1.0)
        };

        let alpha_factor = if self.enabled { 1.0 } else { 0.5 };

        self.draw_field.color = vec4(
            field_fill.x,
            field_fill.y,
            field_fill.z,
            field_fill.w * alpha_factor,
        );
        self.draw_field.border_color = vec4(
            field_border.x,
            field_border.y,
            field_border.z,
            field_border.w * alpha_factor,
        );

        self.draw_field.begin(cx, walk, Layout::default());
        let field_rect = cx.turtle().rect();
        self.draw_field.end(cx);
        self.area = self.draw_field.area();

        let field_h = field_rect.size.y as f32;
        let text_top = field_rect.pos.y as f32
            + crawrad_component::text_line::cap_centred_top(field_h, font_px);

        if self.editable {
            let input_walk = Walk {
                abs_pos: Some(dvec2(field_rect.pos.x, field_rect.pos.y)),
                width: Size::Fixed((field_rect.size.x - 22.0).max(0.0)),
                height: Size::Fixed(field_rect.size.y),
                margin: Margin::default(),
            };
            let _ = self.text_input.draw_walk(cx, _scope, input_walk);
        } else {
            let selected_text = self
                .selected
                .and_then(|idx| self.items.get(idx))
                .map(|s| s.as_str())
                .unwrap_or("");

            self.draw_text.color = vec4(
                field_text_color.x,
                field_text_color.y,
                field_text_color.z,
                field_text_color.w * alpha_factor,
            );
            self.draw_text.draw_abs(
                cx,
                dvec2(field_rect.pos.x + 8.0, text_top as f64),
                selected_text,
            );
        }

        let caret = Rect {
            pos: dvec2(
                field_rect.pos.x + field_rect.size.x - 18.0,
                field_rect.pos.y + field_rect.size.y * 0.5 - 3.0,
            ),
            size: dvec2(9.0, 6.0),
        };
        self.draw_chevron.color = vec4(0.8039216, 0.8392157, 0.95686275, alpha_factor);
        self.draw_chevron.draw_abs(cx, caret);

        if self.is_open && self.rows_count() > 0 {
            let (row_h, visible_count, list_rect) = self.list_geometry(field_rect);

            self.draw_list.begin_overlay_reuse(cx);

            self.draw_list_bg.draw_abs(cx, list_rect);

            let has_scroll_bar = self.rows_count() > visible_count;
            let row_w = if has_scroll_bar {
                (list_rect.size.x - 16.0).max(1.0)
            } else {
                (list_rect.size.x - 8.0).max(1.0)
            };

            if let Some(hovered) = self.hovered_row
                && hovered >= self.scroll_offset
                && hovered < self.scroll_offset + visible_count
            {
                let i = hovered - self.scroll_offset;
                let row_y = list_rect.pos.y + 4.0 + (i as f64) * row_h as f64;
                let row_rect = Rect {
                    pos: dvec2(list_rect.pos.x + 4.0, row_y),
                    size: dvec2(row_w, row_h as f64),
                };
                self.draw_item_hover.draw_abs(cx, row_rect);
            }

            if has_scroll_bar {
                let track_x = list_rect.pos.x + list_rect.size.x - 8.0;
                let track_y = list_rect.pos.y + 4.0;
                let track_h = (list_rect.size.y - 8.0).max(1.0);
                let track_rect = Rect {
                    pos: dvec2(track_x, track_y),
                    size: dvec2(4.0, track_h),
                };
                self.draw_scroll_track.draw_abs(cx, track_rect);

                if let Some((thumb_y, thumb_h)) = thumb(
                    self.scroll_offset,
                    visible_count,
                    self.rows_count(),
                    track_h,
                ) {
                    let thumb_rect = Rect {
                        pos: dvec2(track_x, track_y + thumb_y),
                        size: dvec2(4.0, thumb_h),
                    };
                    self.draw_scroll_thumb.draw_abs(cx, thumb_rect);
                }
            }

            let check_size = 12.0f64;
            for i in 0..visible_count {
                let row_idx = self.scroll_offset + i;
                let Some(item_idx) = self.item_index_at(row_idx) else {
                    break;
                };
                if self.selected == Some(item_idx) {
                    let row_y = list_rect.pos.y + 4.0 + (i as f64) * row_h as f64;
                    let check_x = list_rect.pos.x + 4.0 + row_w - check_size - 4.0;
                    let check_y = row_y + (row_h as f64 - check_size) * 0.5;
                    let check_rect = Rect {
                        pos: dvec2(check_x, check_y),
                        size: dvec2(check_size, check_size),
                    };
                    self.draw_check.draw_abs(cx, check_rect);
                }
            }

            for i in 0..visible_count {
                let row_idx = self.scroll_offset + i;
                let Some(item_idx) = self.item_index_at(row_idx) else {
                    break;
                };
                let item_text = &self.items[item_idx];
                let row_y = list_rect.pos.y as f32 + 4.0 + (i as f32) * row_h;
                let item_text_top =
                    row_y + crawrad_component::text_line::cap_centred_top(row_h, font_px);

                if self.selected == Some(item_idx) {
                    self.draw_text_selected.draw_abs(
                        cx,
                        dvec2(list_rect.pos.x + 10.0, item_text_top as f64),
                        item_text,
                    );
                } else {
                    self.draw_text.color = vec4(0.8039216, 0.8392157, 0.95686275, 1.0);
                    self.draw_text.draw_abs(
                        cx,
                        dvec2(list_rect.pos.x + 10.0, item_text_top as f64),
                        item_text,
                    );
                }
            }

            self.draw_list.end(cx);
        }

        DrawStep::done()
    }
}

impl CrawradComboBox {
    fn rows_count(&self) -> usize {
        if self.editable {
            self.filtered_indices.len()
        } else {
            self.items.len()
        }
    }

    fn item_index_at(&self, row_idx: usize) -> Option<usize> {
        if self.editable {
            self.filtered_indices.get(row_idx).copied()
        } else if row_idx < self.items.len() {
            Some(row_idx)
        } else {
            None
        }
    }

    #[note("combo-box-crawradcombobox-list-geometry")]
    fn list_geometry(&self, field_rect: Rect) -> (f32, usize, Rect) {
        let row_h = (field_rect.size.y as f32).max(20.0);
        let rows = visible_rows(self.drop_down_count, self.rows_count());
        let list_rect = Rect {
            pos: dvec2(field_rect.pos.x, field_rect.pos.y + field_rect.size.y + 2.0),
            size: dvec2(field_rect.size.x, (row_h * rows as f32 + 8.0) as f64),
        };
        (row_h, rows, list_rect)
    }

    fn open_drop_down(&mut self, cx: &mut Cx) {
        self.is_open = true;
        let count = self.rows_count();
        let visible_count = visible_rows(self.drop_down_count, count);
        if self.editable {
            self.hovered_row = if count > 0 { Some(0) } else { None };
            self.scroll_offset = 0;
        } else {
            self.hovered_row = self.selected;
            self.scroll_offset = if let Some(sel) = self.selected {
                offset_showing(sel, 0, visible_count, count)
            } else {
                0
            };
        }
        cx.sweep_lock(self.draw_field.area());
        if !self.editable {
            cx.set_key_focus(self.area);
        }
        self.redraw(cx);
    }

    fn close_drop_down(&mut self, cx: &mut Cx) {
        if self.is_open {
            self.is_open = false;
            self.hovered_row = None;
            cx.sweep_unlock(self.draw_field.area());
            self.redraw(cx);
        }
    }

    fn pick_item(&mut self, cx: &mut Cx, scope: &mut Scope, row_idx: usize) {
        if let Some(item_idx) = self.item_index_at(row_idx) {
            self.selected = Some(item_idx);
            if self.editable {
                let picked_text = self.items[item_idx].clone();
                self.text_input.set_text(cx, &picked_text);
                self.filtered_indices = list_selection::matching(&self.items, &picked_text);
            }
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradComboBoxAction::Selected(item_idx),
            );
            self.close_drop_down(cx);
        }
    }
}

crate::palette::palette_glue!(CrawradComboBoxRef, enabled);

impl crate::palette::TakesLook for CrawradComboBox {
    // the colours are the look's at draw time (`draw_walk`); only the font is kept
    fn take_look(&mut self, cx: &mut Cx, look: ListLook) {
        if look.font_size > 0.0 {
            self.draw_text.text_style.font_size = look.font_size;
            self.draw_text_selected.text_style.font_size = look.font_size;
            self.text_input.apply_over(
                cx,
                live! {
                    draw_text: {
                        text_style: { font_size: (look.font_size) }
                    }
                },
            );
        }
    }
}

impl CrawradComboBoxRef {
    /// Sets the drop-down items, retaining valid selection index.
    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
            if inner.editable {
                let current_text = inner.text_input.text();
                inner.filtered_indices = list_selection::matching(&inner.items, &current_text);
            }
            if let Some(sel) = inner.selected
                && sel >= inner.items.len()
            {
                inner.selected = None;
            }
            inner.redraw(cx);
        }
    }

    /// Sets the selected item index.
    pub fn set_selected(&self, cx: &mut Cx, selected: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut() {
            let valid = selected.filter(|i| *i < inner.items.len());
            if inner.selected != valid {
                inner.selected = valid;
                if inner.editable
                    && let Some(idx) = valid
                {
                    let text = inner.items[idx].clone();
                    inner.text_input.set_text(cx, &text);
                    inner.filtered_indices = list_selection::matching(&inner.items, &text);
                }
                inner.redraw(cx);
            }
        }
    }

    /// Sets whether the drop-down is editable (DropDown) or pick-only.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// combo_box.set_editable(cx, true);
    /// ```
    pub fn set_editable(&self, cx: &mut Cx, editable: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.editable != editable
        {
            inner.editable = editable;
            if editable {
                let current_text = inner.text_input.text();
                inner.filtered_indices = list_selection::matching(&inner.items, &current_text);
            }
            inner.redraw(cx);
        }
    }

    /// Sets the text box text when different, avoiding cursor reset while typing.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// combo_box.set_text(cx, "hello");
    /// ```
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.text_input.text() != text
        {
            inner.text_input.set_text(cx, text);
            if inner.editable {
                inner.filtered_indices = list_selection::matching(&inner.items, text);
            }
            inner.redraw(cx);
        }
    }

    /// Returns the updated text from actions if text changed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if let Some(text) = combo_box.text_changed(actions) {
    ///     println!("{text}");
    /// }
    /// ```
    pub fn text_changed(&self, actions: &Actions) -> Option<String> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            match action.cast() {
                CrawradComboBoxAction::TextChanged(s) => Some(s),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns the selected item index from widget actions.
    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            match action.cast() {
                CrawradComboBoxAction::Selected(i) => Some(i),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns the currently selected item index.
    pub fn selected_item(&self) -> Option<usize> {
        self.borrow().and_then(|inner| inner.selected)
    }

    /// Sets maximum rows shown in drop-down; zero means no limit.
    pub fn set_drop_down_count(&self, cx: &mut Cx, count: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            let count = count.max(0) as usize;
            if inner.drop_down_count != count {
                inner.drop_down_count = count;
                inner.redraw(cx);
            }
        }
    }
}

#[cfg(test)]
#[path = "test/combo_box/test.rs"]
mod test;
