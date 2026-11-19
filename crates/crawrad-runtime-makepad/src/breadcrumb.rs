//! **A Breadcrumb component**: a navigation trail on one line.

use makepad_widgets::*;

use crate::list::ListLook;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub DrawChevronBlock = {{DrawChevronBlock}} {
        // The block as two slanted edges, computed per pixel: the right edge
        // is the arrow tip, the left edge the notch (straight on the first
        // block). Sdf2d's move_to/line_to path did not fill this shape.
        fn pixel(self) -> vec4 {
            let p = self.pos * self.rect_size;
            let w = self.rect_size.x;
            let half_h = self.rect_size.y * 0.5;
            let slope = abs(p.y - half_h) / half_h;
            let right = (w - self.tip * slope) - p.x;
            let left = p.x - self.notched * self.tip * (1.0 - slope);
            let a = clamp(min(right, left) + 0.5, 0.0, 1.0) * self.color.w;
            return vec4(self.color.xyz * a, a);
        }
    }

    pub CrawradBreadcrumbBase = {{CrawradBreadcrumb}} {}

    pub CrawradBreadcrumb = <CrawradBreadcrumbBase> {
        width: Fill, height: Fill,
        style: "bsText",
        segment_color: #45464b,
        current_color: #e0612b,
        chevron_text: #cdd6f4,
        chevron_current_text: #ffffff,
        draw_block: <DrawChevronBlock> {}
        draw_text: {
            color: #89b4fa,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_hover: {
            color: #b4befe,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_current: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_separator: {
            color: #6c7086,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
    }
}

pub use crawrad_component::text_line::{
    CHEVRON_OVERLAP, CHEVRON_PADS, CHEVRON_TIP, ChevronPads, chevron_layout,
};
use crawrad_note::note;

pub const BREADCRUMB_GAP: f32 = 6.0;

#[note("breadcrumb-breadcrumb-layout")]
pub fn breadcrumb_layout(widths: &[f32], separator_width: f32, gap: f32) -> Vec<f32> {
    let mut positions = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    for &w in widths {
        positions.push(x);
        x += w + separator_width + 2.0 * gap;
    }
    positions
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawChevronBlock {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub tip: f32,
    #[live]
    pub notched: f32,
}

#[note("breadcrumb-crawradbreadcrumbaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradBreadcrumbAction {
    Clicked(usize),
    None,
}

#[note("breadcrumb-crawradbreadcrumb")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradBreadcrumb {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_block: DrawChevronBlock,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_hover: DrawText,
    #[live]
    draw_current: DrawText,
    #[live]
    draw_separator: DrawText,
    #[live]
    style: String,
    #[live]
    segment_color: Vec4,
    #[live]
    current_color: Vec4,
    /// Chevron text colours: the segments' and the current block's.
    #[live]
    chevron_text: Vec4,
    #[live]
    chevron_current_text: Vec4,
    #[rust]
    items: Vec<String>,
    #[rust]
    separator: String,
    #[rust]
    item_index: i64,
    #[rust]
    hovered: Option<usize>,
    /// Born enabled, like a Button: a host that never calls `set_enabled`
    /// still gets clicks (the Build dialog's chips ignored every one).
    #[rust(true)]
    enabled: bool,
    #[rust]
    area: Area,
    /// Font size, text colour and background colour from the Inspector;
    /// zero values keep the theme's.
    #[rust]
    look: ListLook,
}

impl Widget for CrawradBreadcrumb {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.enabled {
            return;
        }
        let count = self.items.len();
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                let r = self.area.rect(cx);
                let (rel_x, rel_y) = ((fe.abs.x - r.pos.x) as f32, (fe.abs.y - r.pos.y) as f32);
                let (widths, sep_w) = self.measured_widths(cx);

                let mut hovered_seg = None;
                if count > 1 && rel_y >= 0.0 && rel_y <= r.size.y as f32 {
                    if self.style == "bsChevron" {
                        let positions =
                            chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
                        for (i, &(x, w)) in positions.iter().enumerate().take(count - 1) {
                            if rel_x >= x && rel_x < x + w {
                                hovered_seg = Some(i);
                                break;
                            }
                        }
                    } else {
                        let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);
                        for (i, (&x, &w)) in positions
                            .iter()
                            .zip(widths.iter())
                            .enumerate()
                            .take(count - 1)
                        {
                            if rel_x >= x && rel_x < x + w {
                                hovered_seg = Some(i);
                                break;
                            }
                        }
                    }
                }
                if self.hovered != hovered_seg {
                    self.hovered = hovered_seg;
                    self.redraw(cx);
                }
            }
            Hit::FingerHoverOut(_) if self.hovered.is_some() => {
                self.hovered = None;
                self.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if !fe.is_over || !fe.is_primary_hit() || count <= 1 {
                    return;
                }
                let r = self.area.rect(cx);
                let (rel_x, rel_y) = ((fe.abs.x - r.pos.x) as f32, (fe.abs.y - r.pos.y) as f32);
                let (widths, sep_w) = self.measured_widths(cx);

                if rel_y >= 0.0 && rel_y <= r.size.y as f32 {
                    if self.style == "bsChevron" {
                        let positions =
                            chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
                        for (i, &(x, w)) in positions.iter().enumerate().take(count - 1) {
                            if rel_x >= x && rel_x < x + w {
                                self.item_index = i as i64;
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    CrawradBreadcrumbAction::Clicked(i),
                                );
                                self.redraw(cx);
                                return;
                            }
                        }
                    } else {
                        let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);
                        for (i, (&x, &w)) in positions
                            .iter()
                            .zip(widths.iter())
                            .enumerate()
                            .take(count - 1)
                        {
                            if rel_x >= x && rel_x < x + w {
                                self.item_index = i as i64;
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    CrawradBreadcrumbAction::Clicked(i),
                                );
                                self.redraw(cx);
                                return;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        let (widths, sep_w) = self.measured_widths(cx);
        let sep_str = self.separator_text().to_string();

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

        let count = self.items.len();

        if self.style == "bsChevron" {
            let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
            let current_idx = if self.item_index >= 0 && (self.item_index as usize) < count {
                self.item_index as usize
            } else {
                count.saturating_sub(1)
            };

            for (i, (&(x, w), it)) in positions.iter().zip(self.items.iter()).enumerate() {
                if x as f64 >= rect.size.x {
                    break;
                }
                let is_current = i == current_idx;
                let bg_color = if is_current {
                    vec4(
                        self.current_color.x,
                        self.current_color.y,
                        self.current_color.z,
                        self.current_color.w * alpha,
                    )
                } else {
                    vec4(
                        self.segment_color.x,
                        self.segment_color.y,
                        self.segment_color.z,
                        self.segment_color.w * alpha,
                    )
                };

                let block_pos = dvec2(rect.pos.x + x as f64, rect.pos.y);
                let block_size = dvec2(w as f64, height as f64);
                self.draw_block.color = bg_color;
                self.draw_block.tip = CHEVRON_TIP;
                self.draw_block.notched = if i == 0 { 0.0 } else { 1.0 };
                self.draw_block.draw_abs(
                    cx,
                    Rect {
                        pos: block_pos,
                        size: block_size,
                    },
                );

                let left_pad = if i == 0 {
                    CHEVRON_PADS.0
                } else {
                    CHEVRON_PADS.1
                };
                let text_pos = dvec2(
                    rect.pos.x + x as f64 + left_pad as f64,
                    rect.pos.y + text_top,
                );

                if is_current {
                    if look.font_size > 0.0 {
                        self.draw_current.text_style.font_size = look.font_size;
                    }
                    let c = self.chevron_current_text;
                    let mut color = vec4(c.x, c.y, c.z, c.w * alpha);
                    if look.text[3] > 0 {
                        color = vec4(
                            f32::from(look.text[0]) / 255.0,
                            f32::from(look.text[1]) / 255.0,
                            f32::from(look.text[2]) / 255.0,
                            f32::from(look.text[3]) / 255.0 * alpha,
                        );
                    }
                    self.draw_current.color = color;
                    self.draw_current.draw_abs(cx, text_pos, it);
                    self.draw_current
                        .draw_abs(cx, dvec2(text_pos.x + 0.5, text_pos.y), it);
                } else {
                    let is_hovered = self.hovered == Some(i);
                    let painter = if is_hovered {
                        &mut self.draw_hover
                    } else {
                        &mut self.draw_text
                    };
                    if look.font_size > 0.0 {
                        painter.text_style.font_size = look.font_size;
                    }
                    let c = self.chevron_text;
                    let default_color = if is_hovered {
                        vec4(0.95, 0.95, 0.95, alpha)
                    } else {
                        vec4(c.x, c.y, c.z, c.w * alpha)
                    };
                    painter.color = default_color;
                    painter.draw_abs(cx, text_pos, it);
                }
            }
            return DrawStep::done();
        }

        let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);

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

        let count = self.items.len();
        for (i, (&x, it)) in positions.iter().zip(self.items.iter()).enumerate() {
            if x as f64 >= rect.size.x {
                break;
            }
            let is_last = i + 1 == count;
            let w = widths[i];
            let text_pos = dvec2(rect.pos.x + x as f64, rect.pos.y + text_top);

            if is_last {
                if look.font_size > 0.0 {
                    self.draw_current.text_style.font_size = look.font_size;
                }
                let mut color = vec4(0.804, 0.839, 0.957, alpha); // #cdd6f4
                if look.text[3] > 0 {
                    color = vec4(
                        f32::from(look.text[0]) / 255.0,
                        f32::from(look.text[1]) / 255.0,
                        f32::from(look.text[2]) / 255.0,
                        f32::from(look.text[3]) / 255.0 * alpha,
                    );
                }
                self.draw_current.color = color;
                // Draw current segment bold (with 0.5px offset for faux-bold)
                self.draw_current.draw_abs(cx, text_pos, it);
                self.draw_current
                    .draw_abs(cx, dvec2(text_pos.x + 0.5, text_pos.y), it);
            } else {
                let is_hovered = self.hovered == Some(i);
                let painter = if is_hovered {
                    &mut self.draw_hover
                } else {
                    &mut self.draw_text
                };
                if look.font_size > 0.0 {
                    painter.text_style.font_size = look.font_size;
                }
                let default_color = if is_hovered {
                    vec4(0.706, 0.745, 0.996, alpha) // #b4befe
                } else {
                    vec4(0.537, 0.706, 0.980, alpha) // #89b4fa
                };
                painter.color = default_color;
                painter.draw_abs(cx, text_pos, it);

                let sep_x = x + w + BREADCRUMB_GAP;
                if (sep_x as f64) < rect.size.x {
                    let sep_pos = dvec2(rect.pos.x + sep_x as f64, rect.pos.y + text_top);
                    if look.font_size > 0.0 {
                        self.draw_separator.text_style.font_size = look.font_size;
                    }
                    self.draw_separator.color = vec4(0.424, 0.439, 0.525, alpha); // #6c7086
                    self.draw_separator.draw_abs(cx, sep_pos, &sep_str);
                }
            }
        }

        DrawStep::done()
    }
}

impl CrawradBreadcrumb {
    #[note("breadcrumb-crawradbreadcrumb-separator-text")]
    fn separator_text(&self) -> &str {
        if self.separator.is_empty() {
            "›"
        } else {
            &self.separator
        }
    }

    #[note("breadcrumb-crawradbreadcrumb-measured-widths")]
    fn measured_widths(&self, cx: &mut Cx) -> (Vec<f32>, f32) {
        let widths = self
            .items
            .iter()
            .map(|it| {
                self.draw_text
                    .layout(cx, 0.0, 0.0, None, false, Align::default(), it)
                    .size_in_lpxs
                    .width
            })
            .collect();
        let separator = self.separator_text().to_string();
        let sep_w = self
            .draw_separator
            .layout(cx, 0.0, 0.0, None, false, Align::default(), &separator)
            .size_in_lpxs
            .width;
        (widths, sep_w)
    }
}

crate::palette::palette_glue!(CrawradBreadcrumbRef, enabled);

impl crate::palette::TakesLook for CrawradBreadcrumb {
    fn take_look(&mut self, _cx: &mut Cx, look: ListLook) {
        if look.font_size > 0.0 {
            self.draw_text.text_style.font_size = look.font_size;
            self.draw_hover.text_style.font_size = look.font_size;
            self.draw_current.text_style.font_size = look.font_size;
            self.draw_separator.text_style.font_size = look.font_size;
        }
    }
}

impl CrawradBreadcrumbRef {
    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
            inner.redraw(cx);
        }
    }

    #[note("breadcrumb-crawradbreadcrumbref-items")]
    pub fn items(&self) -> Vec<String> {
        self.borrow()
            .map(|inner| inner.items.clone())
            .unwrap_or_default()
    }

    pub fn set_separator(&self, cx: &mut Cx, separator: &str) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.separator != separator
        {
            inner.separator = separator.to_string();
            inner.redraw(cx);
        }
    }

    pub fn set_item_index(&self, cx: &mut Cx, index: i64) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.item_index != index
        {
            inner.item_index = index;
            inner.redraw(cx);
        }
    }

    pub fn set_style(&self, cx: &mut Cx, style: &str) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.style != style
        {
            inner.style = style.to_string();
            inner.redraw(cx);
        }
    }

    pub fn style(&self) -> String {
        self.borrow()
            .map(|inner| inner.style.clone())
            .unwrap_or_else(|| "bsText".to_string())
    }

    pub fn set_segment_color(&self, cx: &mut Cx, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.segment_color != color
        {
            inner.segment_color = color;
            inner.redraw(cx);
        }
    }

    pub fn segment_color(&self) -> Vec4 {
        self.borrow()
            .map(|inner| inner.segment_color)
            .unwrap_or_else(|| vec4(0.271, 0.275, 0.294, 1.0))
    }

    pub fn set_current_color(&self, cx: &mut Cx, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.current_color != color
        {
            inner.current_color = color;
            inner.redraw(cx);
        }
    }

    pub fn current_color(&self) -> Vec4 {
        self.borrow()
            .map(|inner| inner.current_color)
            .unwrap_or_else(|| vec4(0.878, 0.380, 0.169, 1.0))
    }

    pub fn clicked(&self, actions: &Actions) -> Option<usize> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradBreadcrumbAction::Clicked(i) => Some(i),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/breadcrumb/test.rs"]
mod test;
