//! **A Chips component**: a group of rounded chips laid out left to right,
//! wrapping to the next line when the row is full.

use makepad_widgets::*;

use crate::list::ListLook;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub DrawChip = {{DrawChip}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            sdf.box(0.0, 0.0, w, h, self.radius);
            sdf.fill(self.color);
            return sdf.result;
        }
    }

    pub CrawradChipsBase = {{CrawradChips}} {}

    pub CrawradChips = <CrawradChipsBase> {
        width: Fill, height: Fill,
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_remove: {
            color: #6c7086,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
    }
}

pub const CHIP_HEIGHT: f32 = 24.0;
pub const CHIP_GAP: f32 = 6.0;
pub const CHIP_PAD_X: f32 = 8.0;
pub const CHIP_REMOVE_W: f32 = 14.0;
pub const CHIP_CHAR_W: f32 = 7.0;

#[crawrad_note::note("chips-chips-layout")]
pub fn chips_layout(
    widths: &[f32],
    area_width: f32,
    chip_height: f32,
    gap: f32,
) -> Vec<(f32, f32)> {
    let mut positions = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    let mut y = 0.0;
    for &w in widths {
        if x > 0.0 && x + w > area_width {
            x = 0.0;
            y += chip_height + gap;
        }
        positions.push((x, y));
        x += w + gap;
    }
    positions
}

#[crawrad_note::note("chips-chip-width")]
pub fn chip_width(text: &str, removable: bool, font_size: f32) -> f32 {
    // `CHIP_CHAR_W` is measured at the default 9 pt; a larger Font
    // widens every character in proportion (0 = the default)
    let scale = if font_size > 0.0 {
        font_size / 9.0
    } else {
        1.0
    };
    let text_w = text.chars().count() as f32 * CHIP_CHAR_W * scale;
    let extra = if removable { CHIP_REMOVE_W } else { 0.0 };
    text_w + CHIP_PAD_X * 2.0 + extra
}

#[crawrad_note::note("chips-chip-rects")]
pub fn chip_rects(
    items: &[String],
    removable: bool,
    font_size: f32,
    area: Rect,
) -> Vec<(String, Rect, Option<Rect>)> {
    let widths: Vec<f32> = items
        .iter()
        .map(|it| chip_width(it, removable, font_size))
        .collect();
    let positions = chips_layout(&widths, area.size.x as f32, CHIP_HEIGHT, CHIP_GAP);
    let mut out = Vec::new();
    for (i, (&(x, y), &w)) in positions.iter().zip(widths.iter()).enumerate() {
        if y + CHIP_HEIGHT > area.size.y as f32 {
            break;
        }
        let chip = Rect {
            pos: dvec2(area.pos.x + x as f64, area.pos.y + y as f64),
            size: dvec2(w as f64, CHIP_HEIGHT as f64),
        };
        let remove = removable.then(|| {
            let from = (w - CHIP_REMOVE_W - CHIP_PAD_X) as f64;
            Rect {
                pos: dvec2(chip.pos.x + from, chip.pos.y),
                size: dvec2(w as f64 - from, CHIP_HEIGHT as f64),
            }
        });
        out.push((items[i].clone(), chip, remove));
    }
    out
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawChip {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub radius: f32,
}

#[crawrad_note::note("chips-crawradchipsaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradChipsAction {
    Selected(usize),
    Removed(usize),
    None,
}

#[crawrad_note::note("chips-crawradchips")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradChips {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_chip: DrawChip,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_remove: DrawText,
    #[rust]
    items: Vec<String>,
    #[rust]
    item_index: i64,
    #[rust]
    removable: bool,
    /// Born enabled, like a Button: a host that never calls `set_enabled`
    /// still gets clicks (the Build dialog's chips ignored every one).
    #[rust(true)]
    enabled: bool,
    #[rust]
    area: Area,
    /// Font size, text colour and chip colour from the Inspector;
    /// zero values keep the theme's.
    #[rust]
    look: ListLook,
}

impl Widget for CrawradChips {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.enabled {
            return;
        }
        if let Hit::FingerUp(fe) = event.hits(cx, self.area) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            let rects = chip_rects(
                &self.items,
                self.removable,
                self.look.font_size,
                self.area.rect(cx),
            );
            let inside = |r: &Rect| r.contains(fe.abs);
            for (i, (_, chip, remove)) in rects.iter().enumerate() {
                if remove.as_ref().is_some_and(inside) {
                    self.items.remove(i);
                    if self.item_index == i as i64 {
                        self.item_index = -1;
                    } else if self.item_index > i as i64 {
                        self.item_index -= 1;
                    }
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        CrawradChipsAction::Removed(i),
                    );
                    self.redraw(cx);
                    return;
                }
                if inside(chip) {
                    self.item_index = i as i64;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        CrawradChipsAction::Selected(i),
                    );
                    self.redraw(cx);
                    return;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        let widths: Vec<f32> = self
            .items
            .iter()
            .map(|it| chip_width(it, self.removable, self.look.font_size))
            .collect();
        let positions = chips_layout(&widths, rect.size.x as f32, CHIP_HEIGHT, CHIP_GAP);

        let sel_bg = vec4(0.227, 0.349, 0.549, 1.0); // #3a598c
        let unsel_bg = vec4(0.192, 0.192, 0.251, 1.0); // #313140
        let sel_fg = vec4(1.0, 1.0, 1.0, 1.0);
        let unsel_fg = vec4(0.804, 0.839, 0.957, 1.0); // #cdd6f4
        let remove_fg = vec4(0.424, 0.439, 0.525, 1.0); // #6c7086

        let alpha = if self.enabled { 1.0 } else { 0.5 };

        for (i, (&(x, y), it)) in positions.iter().zip(self.items.iter()).enumerate() {
            if y + CHIP_HEIGHT > rect.size.y as f32 {
                break;
            }
            let w = widths[i];
            let chip_r = Rect {
                pos: dvec2(rect.pos.x + x as f64, rect.pos.y + y as f64),
                size: dvec2(w as f64, CHIP_HEIGHT as f64),
            };

            let is_sel = self.item_index >= 0 && self.item_index as usize == i;
            let look = self.look;
            let own_bg = vec4(
                look.background[0],
                look.background[1],
                look.background[2],
                look.background[3],
            );
            let own_fg = vec4(
                f32::from(look.text[0]) / 255.0,
                f32::from(look.text[1]) / 255.0,
                f32::from(look.text[2]) / 255.0,
                f32::from(look.text[3]) / 255.0,
            );
            let bg = if is_sel {
                sel_bg
            } else if look.background[3] > 0.0 {
                own_bg
            } else {
                unsel_bg
            };
            let fg = if is_sel {
                sel_fg
            } else if look.text[3] > 0 {
                own_fg
            } else {
                unsel_fg
            };

            // Pill radius = height * 0.25 (Makepad rounds with 2x the radius)
            self.draw_chip.radius = CHIP_HEIGHT * 0.25;
            self.draw_chip.color = vec4(bg.x, bg.y, bg.z, bg.w * alpha);
            self.draw_chip.draw_abs(cx, chip_r);

            if look.font_size > 0.0 {
                self.draw_text.text_style.font_size = look.font_size;
            }
            self.draw_text.color = vec4(fg.x, fg.y, fg.z, fg.w * alpha);
            let font_size = if look.font_size > 0.0 {
                look.font_size
            } else {
                9.0
            };
            // centred on the CAPITALS, not on the line box: Makepad places text
            // by its line box, which keeps room for descenders, so the letters
            // sat ~1 px low. Measured at 11 pt (14.7 px): cap height 0.68 of the
            // pixel size, cap top 0.227 below the box top.
            let font_px = crawrad_component::text_line::font_px(font_size);
            let text_top = f64::from(crawrad_component::text_line::cap_centred_top(
                CHIP_HEIGHT,
                font_px,
            ));
            let text_pos = dvec2(chip_r.pos.x + CHIP_PAD_X as f64, chip_r.pos.y + text_top);
            self.draw_text.draw_abs(cx, text_pos, it);

            if self.removable {
                self.draw_remove.color =
                    vec4(remove_fg.x, remove_fg.y, remove_fg.z, remove_fg.w * alpha);
                let remove_pos = dvec2(
                    chip_r.pos.x + w as f64 - CHIP_PAD_X as f64 - 8.0,
                    chip_r.pos.y + text_top,
                );
                self.draw_remove.draw_abs(cx, remove_pos, "×");
            }
        }

        DrawStep::done()
    }
}

crate::palette::palette_glue!(CrawradChipsRef, enabled);

impl crate::palette::TakesLook for CrawradChips {
    // the chips read `self.look` while drawing: nothing to take here
    fn take_look(&mut self, _cx: &mut Cx, _look: ListLook) {}
}

impl CrawradChipsRef {
    #[crawrad_note::note("chips-crawradchipsref-drawn-chips")]
    pub fn drawn_chips(&self, cx: &Cx) -> Vec<(String, Rect, Option<Rect>)> {
        let Some(inner) = self.borrow() else {
            return Vec::new();
        };
        chip_rects(
            &inner.items,
            inner.removable,
            inner.look.font_size,
            inner.area.rect(cx),
        )
    }

    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
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

    pub fn set_removable(&self, cx: &mut Cx, removable: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.removable != removable
        {
            inner.removable = removable;
            inner.redraw(cx);
        }
    }

    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradChipsAction::Selected(i) => Some(i),
            _ => None,
        }
    }

    pub fn removed(&self, actions: &Actions) -> Option<usize> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradChipsAction::Removed(i) => Some(i),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/chips/test.rs"]
mod test;
