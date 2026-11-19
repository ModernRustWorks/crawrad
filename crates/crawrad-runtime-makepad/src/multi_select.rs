//! **A MultiSelect component**: a drop-down field whose list has a check box per item.

use makepad_widgets::*;

use crate::list::ListLook;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    // A cell / box: rounded box filled with `color` and outlined with `border_color`
    pub DrawMultiSelectCell = {{DrawMultiSelectCell}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.radius);
            sdf.fill_keep(self.color);
            sdf.stroke(self.border_color, 1.0);
            return sdf.result;
        }
    }

    // The "▾" as a filled triangle: the theme font has no glyph for it
    pub DrawMultiSelectCaret = {{DrawMultiSelectCaret}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            sdf.move_to(0.0, 0.0);
            sdf.line_to(w, 0.0);
            sdf.line_to(w * 0.5, h);
            sdf.close_path();
            sdf.fill(self.color);
            return sdf.result;
        }
    }

    pub CrawradMultiSelectBase = {{CrawradMultiSelect}} {}

    pub CrawradMultiSelect = <CrawradMultiSelectBase> {
        width: Fill, height: Fill,
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
        draw_check_box: {
            color: #181825,
            border_color: #6c7086,
            radius: 2.0
        }
        draw_check_box_checked: {
            color: #89b4fa,
            border_color: #89b4fa,
            radius: 2.0
        }
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_chevron: {
            color: #cdd6f4
        }
    }
}

#[crawrad_note::note("multi-select-drawmultiselectcell")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawMultiSelectCell {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub radius: f32,
}

#[crawrad_note::note("multi-select-drawmultiselectcaret")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawMultiSelectCaret {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
}

#[crawrad_note::note("multi-select-crawradmultiselectaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradMultiSelectAction {
    Changed,
    None,
}

#[crawrad_note::note("multi-select-multi-select-summary")]
pub fn multi_select_summary(
    items: &[String],
    selected: &[String],
    hint: &str,
    mut fits: impl FnMut(&str) -> bool,
) -> String {
    let checked: Vec<&str> = items
        .iter()
        .filter(|it| selected.iter().any(|s| s == *it))
        .map(|s| s.as_str())
        .collect();

    if checked.is_empty() {
        return hint.to_string();
    }

    let joined = checked.join(", ");
    if fits(&joined) {
        joined
    } else {
        format!("{} selected", checked.len())
    }
}

#[crawrad_note::note("multi-select-toggle-selected")]
pub fn toggle_selected(items: &[String], selected: &mut Vec<String>, index: usize) {
    if index >= items.len() {
        return;
    }
    let target = &items[index];
    if let Some(pos) = selected.iter().position(|s| s == target) {
        selected.remove(pos);
    } else {
        selected.push(target.clone());
        selected.sort_by_key(|s| items.iter().position(|it| it == s).unwrap_or(usize::MAX));
    }
}

#[crawrad_note::note("multi-select-crawradmultiselect")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradMultiSelect {
    #[walk]
    walk: Walk,
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
    draw_check_box: DrawMultiSelectCell,
    #[live]
    draw_check_box_checked: DrawMultiSelectCell,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_chevron: DrawMultiSelectCaret,
    #[rust]
    items: Vec<String>,
    #[rust]
    selected_items: Vec<String>,
    #[rust]
    text_hint: String,
    #[rust]
    drop_down_count: usize,
    #[rust]
    is_open: bool,
    #[rust]
    hovered_row: Option<usize>,
    #[rust]
    scroll_offset: usize,
    /// Born enabled, like a Button: a host that never calls `set_enabled`
    /// still gets clicks (the Build dialog's chips ignored every one).
    #[rust(true)]
    enabled: bool,
    #[rust]
    visible: bool,
    #[rust]
    visible_set: bool,
    #[rust]
    area: Area,
    /// Font size, text colour and background colour from the Inspector.
    #[rust]
    look: ListLook,
}

impl Widget for CrawradMultiSelect {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.enabled || (self.visible_set && !self.visible) {
            return;
        }

        let field_rect = self.area.rect(cx);
        let row_h = (field_rect.size.y as f32).max(20.0);
        let max_visible = if self.drop_down_count == 0 {
            6
        } else {
            self.drop_down_count
        };
        let visible_count = self.items.len().min(max_visible);
        let list_h = row_h * visible_count as f32 + 8.0;
        let list_rect = Rect {
            pos: dvec2(field_rect.pos.x, field_rect.pos.y + field_rect.size.y + 2.0),
            size: dvec2(field_rect.size.x, list_h as f64),
        };

        if self.is_open {
            if let Event::Scroll(se) = event
                && list_rect.contains(se.abs)
                && self.items.len() > max_visible
            {
                if se.scroll.y > 0.0 {
                    if self.scroll_offset + max_visible < self.items.len() {
                        self.scroll_offset += 1;
                        self.redraw(cx);
                    }
                } else if se.scroll.y < 0.0 && self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.redraw(cx);
                }
            }

            // While open, the list lives in an overlay outside the field's
            // area, so hits on the field never see it: read the raw mouse
            // position and test it against the field and the list.
            let row_at = |abs: DVec2| -> Option<usize> {
                if !list_rect.contains(abs) {
                    return None;
                }
                let rel_y = (abs.y - (list_rect.pos.y + 4.0)) as f32;
                if rel_y < 0.0 {
                    return None;
                }
                let idx_on_screen = (rel_y / row_h) as usize;
                (idx_on_screen < visible_count)
                    .then_some(self.scroll_offset + idx_on_screen)
                    .filter(|i| *i < self.items.len())
            };
            match event {
                Event::MouseMove(me) => {
                    let hovered = row_at(me.abs);
                    if self.hovered_row != hovered {
                        self.hovered_row = hovered;
                        self.redraw(cx);
                    }
                }
                Event::MouseUp(me) => {
                    if let Some(item_idx) = row_at(me.abs) {
                        toggle_selected(&self.items, &mut self.selected_items, item_idx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            CrawradMultiSelectAction::Changed,
                        );
                        self.redraw(cx);
                    } else if !list_rect.contains(me.abs) && !field_rect.contains(me.abs) {
                        // anywhere else closes it (the field's own click is below)
                        self.close_drop_down(cx);
                    }
                }
                _ => {}
            }
            if matches!(event.hits(cx, self.area), Hit::FingerUp(fe) if field_rect.contains(fe.abs))
            {
                self.close_drop_down(cx);
            }
        } else if matches!(event.hits(cx, self.area), Hit::FingerUp(fe) if field_rect.contains(fe.abs))
        {
            self.open_drop_down(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible_set && !self.visible {
            return DrawStep::done();
        }

        self.draw_field.begin(cx, walk, Layout::default());
        let field_rect = cx.turtle().rect();
        self.draw_field.end(cx);
        self.area = self.draw_field.area();

        // Calculate font size in px
        let font_size = if self.look.font_size > 0.0 {
            self.look.font_size
        } else {
            9.0
        };
        let font_px = crawrad_component::text_line::font_px(font_size);
        let summary_text = self.summary_text(cx, field_rect.size.x as f32);

        // Center text on capitals: top = row_h / 2.0 - 0.567 * font_px
        let field_h = field_rect.size.y as f32;
        let text_top = field_rect.pos.y as f32
            + crawrad_component::text_line::cap_centred_top(field_h, font_px);

        // Draw summary text
        self.draw_text.draw_abs(
            cx,
            dvec2(field_rect.pos.x + 8.0, text_top as f64),
            &summary_text,
        );

        // Draw the "▾" at the right, centred on the field's height
        let caret = Rect {
            pos: dvec2(
                field_rect.pos.x + field_rect.size.x - 18.0,
                field_rect.pos.y + field_rect.size.y * 0.5 - 3.0,
            ),
            size: dvec2(9.0, 6.0),
        };
        self.draw_chevron.draw_abs(cx, caret);

        // Draw open drop-down overlay if active
        if self.is_open && !self.items.is_empty() {
            let row_h = field_h.max(20.0);
            let max_visible = if self.drop_down_count == 0 {
                6
            } else {
                self.drop_down_count
            };
            let visible_count = self.items.len().min(max_visible);
            let list_h = row_h * visible_count as f32 + 8.0;
            let list_rect = Rect {
                pos: dvec2(field_rect.pos.x, field_rect.pos.y + field_rect.size.y + 2.0),
                size: dvec2(field_rect.size.x, list_h as f64),
            };

            self.draw_list.begin_overlay_reuse(cx);

            // Draw list background
            self.draw_list_bg.draw_abs(cx, list_rect);

            // The hovered row's background BEFORE any row: Makepad batches each
            // draw object into one call per draw list, in order of first use,
            // so a highlight first drawn after row 1's text would cover all text
            if let Some(hovered) = self.hovered_row
                && hovered >= self.scroll_offset
                && hovered < self.scroll_offset + visible_count
            {
                let i = hovered - self.scroll_offset;
                let row_y = list_rect.pos.y as f32 + 4.0 + (i as f32) * row_h;
                let row_rect = Rect {
                    pos: dvec2(list_rect.pos.x + 4.0, row_y as f64),
                    size: dvec2((list_rect.size.x - 8.0).max(1.0), row_h as f64),
                };
                self.draw_item_hover.draw_abs(cx, row_rect);
            }

            // Draw visible rows
            let box_size = 14.0f32;
            let check_inner_size = 8.0f32;
            for i in 0..visible_count {
                let item_idx = self.scroll_offset + i;
                if item_idx >= self.items.len() {
                    break;
                }
                let item_text = &self.items[item_idx];
                let is_checked = self.selected_items.iter().any(|s| s == item_text);
                let row_y = list_rect.pos.y as f32 + 4.0 + (i as f32) * row_h;

                // Check box rect
                let box_y = row_y + (row_h - box_size) * 0.5;
                let box_rect = Rect {
                    pos: dvec2(list_rect.pos.x + 10.0, box_y as f64),
                    size: dvec2(box_size as f64, box_size as f64),
                };

                if is_checked {
                    self.draw_check_box_checked.draw_abs(cx, box_rect);
                    let inner_offset = (box_size - check_inner_size) * 0.5;
                    let inner_rect = Rect {
                        pos: dvec2(
                            box_rect.pos.x + inner_offset as f64,
                            box_rect.pos.y + inner_offset as f64,
                        ),
                        size: dvec2(check_inner_size as f64, check_inner_size as f64),
                    };
                    self.draw_list_bg.draw_abs(cx, inner_rect);
                } else {
                    self.draw_check_box.draw_abs(cx, box_rect);
                }

                // Item label
                let item_text_top =
                    row_y + crawrad_component::text_line::cap_centred_top(row_h, font_px);
                self.draw_text.draw_abs(
                    cx,
                    dvec2(list_rect.pos.x + 32.0, item_text_top as f64),
                    item_text,
                );
            }

            self.draw_list.end(cx);
        }

        DrawStep::done()
    }
}

impl CrawradMultiSelect {
    fn open_drop_down(&mut self, cx: &mut Cx) {
        self.is_open = true;
        self.hovered_row = None;
        self.scroll_offset = 0;
        cx.sweep_lock(self.draw_field.area());
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

    fn summary_text(&self, cx: &mut Cx, field_width: f32) -> String {
        let hint = if self.text_hint.is_empty() {
            "Select…"
        } else {
            &self.text_hint
        };
        let available_w = (field_width - 28.0).max(0.0);
        multi_select_summary(&self.items, &self.selected_items, hint, |s| {
            let measured = self
                .draw_text
                .layout(cx, 0.0, 0.0, None, false, Align::default(), s)
                .size_in_lpxs
                .width;
            measured <= available_w
        })
    }
}

crate::palette::palette_glue!(CrawradMultiSelectRef, enabled);

impl crate::palette::TakesLook for CrawradMultiSelect {
    fn take_look(&mut self, _cx: &mut Cx, look: ListLook) {
        if look.font_size > 0.0 {
            self.draw_text.text_style.font_size = look.font_size;
        }
    }
}

impl CrawradMultiSelectRef {
    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
            inner.redraw(cx);
        }
    }

    pub fn set_selected_items(&self, cx: &mut Cx, selected: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.selected_items != selected
        {
            inner.selected_items = selected.to_vec();
            inner.redraw(cx);
        }
    }

    pub fn selected_items(&self) -> Vec<String> {
        self.borrow()
            .map(|inner| inner.selected_items.clone())
            .unwrap_or_default()
    }

    pub fn set_text_hint(&self, cx: &mut Cx, hint: &str) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.text_hint != hint
        {
            inner.text_hint = hint.to_string();
            inner.redraw(cx);
        }
    }

    pub fn set_drop_down_count(&self, cx: &mut Cx, count: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            let count = count.max(1) as usize;
            if inner.drop_down_count != count {
                inner.drop_down_count = count;
                inner.redraw(cx);
            }
        }
    }

    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && (!inner.visible_set || inner.visible != visible)
        {
            inner.visible = visible;
            inner.visible_set = true;
            inner.redraw(cx);
        }
    }

    pub fn changed(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(action.cast(), CrawradMultiSelectAction::Changed)
        } else {
            false
        }
    }
}

#[cfg(test)]
#[path = "test/multi_select/test.rs"]
mod test;
