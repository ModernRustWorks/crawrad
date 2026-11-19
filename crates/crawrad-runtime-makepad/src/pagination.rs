//! **A Pagination component**: a row of page buttons.

use makepad_widgets::*;

use crate::list::ListLook;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    // A page cell: a rounded box filled with `color` and outlined with
    // `border_color` (a plain DrawQuad has no colour and paints red)
    pub DrawPageCell = {{DrawPageCell}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 3.0);
            sdf.fill_keep(self.color);
            sdf.stroke(self.border_color, 1.0);
            return sdf.result;
        }
    }

    pub CrawradPaginationBase = {{CrawradPagination}} {}

    pub CrawradPagination = <CrawradPaginationBase> {
        width: Fill, height: Fill,
        draw_cell: {
            color: #00000000,
            border_color: #45475a
        }
        draw_cell_current: {
            color: #3a598c,
            border_color: #3a598c
        }
        draw_cell_hover: {
            color: #00000000,
            border_color: #89b4fa
        }
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_current: {
            color: #ffffff,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_disabled: {
            color: #6c7086,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
    }
}

pub const PAGINATION_CELL: f32 = 28.0;
pub const PAGINATION_GAP: f32 = 4.0;

#[crawrad_note::note("pagination-pageslot")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageSlot {
    Prev,
    Page(usize),
    Gap,
    Next,
}

#[crawrad_note::note("pagination-pagination-slots")]
pub fn pagination_slots(
    page_count: usize,
    page_index: usize,
    max_buttons: usize,
    arrows: bool,
) -> Vec<PageSlot> {
    if page_count == 0 {
        return Vec::new();
    }
    let page_index = page_index.min(page_count - 1);
    let max_b = max_buttons.max(5);

    if page_count <= max_b {
        let mut slots = Vec::with_capacity(page_count + if arrows { 2 } else { 0 });
        if arrows {
            slots.push(PageSlot::Prev);
        }
        for i in 0..page_count {
            slots.push(PageSlot::Page(i));
        }
        if arrows {
            slots.push(PageSlot::Next);
        }
        return slots;
    }

    let mid_count = max_b - 4;
    let half_left = mid_count / 2;
    let half_right = mid_count - half_left - 1;

    let mut page_slots = Vec::with_capacity(max_b);
    if page_index <= half_left + 2 {
        for i in 0..(max_b - 2) {
            page_slots.push(PageSlot::Page(i));
        }
        page_slots.push(PageSlot::Gap);
        page_slots.push(PageSlot::Page(page_count - 1));
    } else if page_index + half_right >= page_count - 3 {
        page_slots.push(PageSlot::Page(0));
        page_slots.push(PageSlot::Gap);
        for i in (page_count - (max_b - 2))..page_count {
            page_slots.push(PageSlot::Page(i));
        }
    } else {
        page_slots.push(PageSlot::Page(0));
        page_slots.push(PageSlot::Gap);
        let start_mid = page_index - half_left;
        let end_mid = page_index + half_right;
        for i in start_mid..=end_mid {
            page_slots.push(PageSlot::Page(i));
        }
        page_slots.push(PageSlot::Gap);
        page_slots.push(PageSlot::Page(page_count - 1));
    }

    let mut slots = Vec::with_capacity(page_slots.len() + if arrows { 2 } else { 0 });
    if arrows {
        slots.push(PageSlot::Prev);
    }
    slots.extend(page_slots);
    if arrows {
        slots.push(PageSlot::Next);
    }
    slots
}

#[crawrad_note::note("pagination-drawpagecell")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPageCell {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub border_color: Vec4,
}

#[crawrad_note::note("pagination-crawradpaginationaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradPaginationAction {
    Changed(usize),
    None,
}

#[crawrad_note::note("pagination-crawradpagination")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradPagination {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_cell: DrawPageCell,
    #[live]
    draw_cell_current: DrawPageCell,
    #[live]
    draw_cell_hover: DrawPageCell,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_current: DrawText,
    #[live]
    draw_disabled: DrawText,
    #[rust]
    page_count: usize,
    #[rust]
    page_index: usize,
    #[rust]
    max_buttons: usize,
    #[rust]
    show_arrows: bool,
    #[rust]
    hovered: Option<usize>,
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
    /// Font size, text colour and background colour from the Inspector;
    /// zero values keep the theme's.
    #[rust]
    look: ListLook,
}

impl Widget for CrawradPagination {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.enabled || (self.visible_set && !self.visible) {
            return;
        }
        let page_count = self.effective_page_count();
        let max_buttons = self.effective_max_buttons();
        let slots = pagination_slots(page_count, self.page_index, max_buttons, self.show_arrows);

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                let r = self.area.rect(cx);
                let rel_x = (fe.abs.x - r.pos.x) as f32;
                let rel_y = (fe.abs.y - r.pos.y) as f32;
                let h = r.size.y as f32;

                let mut hovered_slot = None;
                if rel_y >= 0.0 && rel_y <= h {
                    let mut x = 0.0;
                    for (slot_idx, slot) in slots.iter().enumerate() {
                        let label = match slot {
                            PageSlot::Prev => "‹".to_string(),
                            PageSlot::Next => "›".to_string(),
                            PageSlot::Gap => "…".to_string(),
                            PageSlot::Page(p) => (p + 1).to_string(),
                        };
                        let text_w = self.measure_text_width(cx, &label);
                        let cell_w = h.max(text_w + 8.0);
                        if rel_x >= x && rel_x < x + cell_w {
                            let is_clickable = match slot {
                                PageSlot::Prev => self.page_index > 0,
                                PageSlot::Next => self.page_index + 1 < page_count,
                                PageSlot::Page(_) => true,
                                PageSlot::Gap => false,
                            };
                            if is_clickable {
                                hovered_slot = Some(slot_idx);
                            }
                            break;
                        }
                        x += cell_w + PAGINATION_GAP;
                    }
                }
                if self.hovered != hovered_slot {
                    self.hovered = hovered_slot;
                    self.redraw(cx);
                }
            }
            Hit::FingerHoverOut(_) if self.hovered.is_some() => {
                self.hovered = None;
                self.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if !fe.is_over || !fe.is_primary_hit() {
                    return;
                }
                let r = self.area.rect(cx);
                let rel_x = (fe.abs.x - r.pos.x) as f32;
                let rel_y = (fe.abs.y - r.pos.y) as f32;
                let h = r.size.y as f32;

                if rel_y >= 0.0 && rel_y <= h {
                    let mut x = 0.0;
                    for slot in &slots {
                        let label = match slot {
                            PageSlot::Prev => "‹".to_string(),
                            PageSlot::Next => "›".to_string(),
                            PageSlot::Gap => "…".to_string(),
                            PageSlot::Page(p) => (p + 1).to_string(),
                        };
                        let text_w = self.measure_text_width(cx, &label);
                        let cell_w = h.max(text_w + 8.0);
                        if rel_x >= x && rel_x < x + cell_w {
                            let new_index = match slot {
                                PageSlot::Prev if self.page_index > 0 => Some(self.page_index - 1),
                                PageSlot::Next if self.page_index + 1 < page_count => {
                                    Some(self.page_index + 1)
                                }
                                PageSlot::Page(p) => Some(*p),
                                _ => None,
                            };
                            if let Some(idx) = new_index
                                && idx != self.page_index
                            {
                                self.page_index = idx;
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    CrawradPaginationAction::Changed(idx),
                                );
                                self.redraw(cx);
                            }
                            return;
                        }
                        x += cell_w + PAGINATION_GAP;
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible_set && !self.visible {
            return DrawStep::done();
        }
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        let page_count = self.effective_page_count();
        let max_buttons = self.effective_max_buttons();
        let slots = pagination_slots(page_count, self.page_index, max_buttons, self.show_arrows);

        let alpha = if self.enabled { 1.0 } else { 0.5 };

        let look = self.look;
        let font_size = if look.font_size > 0.0 {
            look.font_size
        } else {
            9.0
        };
        let font_px = crawrad_component::text_line::font_px(font_size);
        let height = rect.size.y as f32;
        let text_top = f64::from(crawrad_component::text_line::cap_centred_top(
            height, font_px,
        ));

        if look.font_size > 0.0 {
            self.draw_text.text_style.font_size = look.font_size;
            self.draw_current.text_style.font_size = look.font_size;
            self.draw_disabled.text_style.font_size = look.font_size;
        }

        let mut current_fg = vec4(1.0, 1.0, 1.0, alpha);
        let mut normal_fg = vec4(0.804, 0.839, 0.957, alpha); // #cdd6f4
        let disabled_fg = vec4(0.424, 0.439, 0.525, alpha); // #6c7086

        if look.text[3] > 0 {
            let custom_fg = vec4(
                f32::from(look.text[0]) / 255.0,
                f32::from(look.text[1]) / 255.0,
                f32::from(look.text[2]) / 255.0,
                f32::from(look.text[3]) / 255.0 * alpha,
            );
            normal_fg = custom_fg;
            current_fg = custom_fg;
        }

        self.draw_text.color = normal_fg;
        self.draw_current.color = current_fg;
        self.draw_disabled.color = disabled_fg;

        let mut x = 0.0;
        for (slot_idx, slot) in slots.iter().enumerate() {
            if x as f64 >= rect.size.x {
                break;
            }
            let label = match slot {
                PageSlot::Prev => "‹".to_string(),
                PageSlot::Next => "›".to_string(),
                PageSlot::Gap => "…".to_string(),
                PageSlot::Page(p) => (p + 1).to_string(),
            };
            let text_w = self.measure_text_width(cx, &label);
            let cell_w = height.max(text_w + 8.0);
            let cell_rect = Rect {
                pos: dvec2(rect.pos.x + x as f64, rect.pos.y),
                size: dvec2(cell_w as f64, rect.size.y),
            };

            let text_x = cell_rect.pos.x + (cell_rect.size.x - text_w as f64) * 0.5;
            let text_pos = dvec2(text_x, cell_rect.pos.y + text_top);

            match slot {
                PageSlot::Page(p) if *p == self.page_index => {
                    self.draw_cell_current.draw_abs(cx, cell_rect);
                    self.draw_current.draw_abs(cx, text_pos, &label);
                    self.draw_current
                        .draw_abs(cx, dvec2(text_pos.x + 0.5, text_pos.y), &label);
                }
                PageSlot::Gap => {
                    self.draw_disabled.draw_abs(cx, text_pos, &label);
                }
                PageSlot::Prev => {
                    let is_disabled = self.page_index == 0;
                    if is_disabled {
                        self.draw_cell.draw_abs(cx, cell_rect);
                        self.draw_disabled.draw_abs(cx, text_pos, &label);
                    } else if self.hovered == Some(slot_idx) {
                        self.draw_cell_hover.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    } else {
                        self.draw_cell.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    }
                }
                PageSlot::Next => {
                    let is_disabled = self.page_index + 1 >= page_count;
                    if is_disabled {
                        self.draw_cell.draw_abs(cx, cell_rect);
                        self.draw_disabled.draw_abs(cx, text_pos, &label);
                    } else if self.hovered == Some(slot_idx) {
                        self.draw_cell_hover.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    } else {
                        self.draw_cell.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    }
                }
                PageSlot::Page(_) => {
                    if self.hovered == Some(slot_idx) {
                        self.draw_cell_hover.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    } else {
                        self.draw_cell.draw_abs(cx, cell_rect);
                        self.draw_text.draw_abs(cx, text_pos, &label);
                    }
                }
            }

            x += cell_w + PAGINATION_GAP;
        }

        DrawStep::done()
    }
}

impl CrawradPagination {
    fn effective_page_count(&self) -> usize {
        if self.page_count == 0 {
            10
        } else {
            self.page_count
        }
    }

    fn effective_max_buttons(&self) -> usize {
        if self.max_buttons == 0 {
            7
        } else {
            self.max_buttons.max(5)
        }
    }

    fn measure_text_width(&self, cx: &mut Cx, text: &str) -> f32 {
        self.draw_text
            .layout(cx, 0.0, 0.0, None, false, Align::default(), text)
            .size_in_lpxs
            .width
    }
}

crate::palette::palette_glue!(CrawradPaginationRef, enabled);

impl crate::palette::TakesLook for CrawradPagination {
    fn take_look(&mut self, _cx: &mut Cx, look: ListLook) {
        if look.font_size > 0.0 {
            self.draw_text.text_style.font_size = look.font_size;
            self.draw_current.text_style.font_size = look.font_size;
            self.draw_disabled.text_style.font_size = look.font_size;
        }
    }
}

impl CrawradPaginationRef {
    pub fn set_page_count(&self, cx: &mut Cx, count: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            let count = (count.max(1)) as usize;
            if inner.page_count != count {
                inner.page_count = count;
                inner.redraw(cx);
            }
        }
    }

    pub fn set_page_index(&self, cx: &mut Cx, index: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            let index = (index.max(0)) as usize;
            if inner.page_index != index {
                inner.page_index = index;
                inner.redraw(cx);
            }
        }
    }

    pub fn set_max_buttons(&self, cx: &mut Cx, max: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            let max = (max.max(5)) as usize;
            if inner.max_buttons != max {
                inner.max_buttons = max;
                inner.redraw(cx);
            }
        }
    }

    pub fn set_show_arrows(&self, cx: &mut Cx, show: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.show_arrows != show
        {
            inner.show_arrows = show;
            inner.redraw(cx);
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

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradPaginationAction::Changed(i) => Some(i),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/pagination/test.rs"]
mod test;
