//! **A RadioGroup**: a framed group of exclusive options, one widget.
//!
//! Its items are the form's at run time (a `Strings` field the code can
//! change), so it is drawn here and not as one makepad widget per
//! design-time item — the same reason as the `CrawradListBox`. The
//! drawing is a list of [`crate::paint::Command`]s (the frame with a gap for
//! the caption, a circle per item, the dot of the chosen one), painted by
//! [`crate::paint::paint`]; a click on an item's cell reports
//! [`CrawradRadioGroupAction::Picked`].

use crate::paint::{Command, DrawPaint, paint};
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;

    pub CrawradRadioGroupBase = {{CrawradRadioGroup}} {}

    pub CrawradRadioGroup = <CrawradRadioGroupBase> {
        width: Fill, height: Fill,
        draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }
    }
}

#[crawrad_note::note("radio-group-mark")]
const MARK: f32 = 12.0;
#[crawrad_note::note("radio-group-top")]
const TOP: f32 = 18.0;
const SIDE: f32 = 8.0;
#[crawrad_note::note("radio-group-row")]
const ROW: f32 = 22.0;

#[crawrad_note::note("radio-group-crawradradiogroupaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradRadioGroupAction {
    /// The user clicked item `.0`.
    Picked(usize),
    None,
}

#[crawrad_note::note("radio-group-groupstate")]
#[derive(Clone, Debug, PartialEq)]
pub struct GroupState {
    pub caption: String,
    pub items: Vec<String>,
    /// The chosen item; -1 (or out of range) is none.
    pub index: i64,
    pub columns: i64,
    pub enabled: bool,
    /// The text's size and colour; 0 and alpha 0 mean the theme's.
    pub font_size: f32,
    pub text: [u8; 4],
}

impl Default for GroupState {
    fn default() -> Self {
        Self {
            caption: String::new(),
            items: Vec::new(),
            index: -1,
            columns: 1,
            enabled: true,
            font_size: 0.0,
            text: [0; 4],
        }
    }
}

#[crawrad_note::note("radio-group-cells")]
pub fn cells(w: f32, h: f32, n: usize, columns: i64) -> Vec<(f32, f32, f32, f32)> {
    let columns = columns.max(1) as usize;
    let rows = n.div_ceil(columns).max(1);
    let inner_w = (w - SIDE * 2.0).max(0.0);
    let inner_h = (h - TOP - SIDE).max(0.0);
    let cw = inner_w / columns as f32;
    let ch = (inner_h / rows as f32).min(ROW);
    (0..n)
        .map(|i| {
            (
                SIDE + (i / rows) as f32 * cw,
                TOP + (i % rows) as f32 * ch,
                cw,
                ch,
            )
        })
        .collect()
}

#[crawrad_note::note("radio-group-commands")]
pub fn commands(state: &GroupState, w: f32, h: f32, caption_w: f32) -> Vec<Command> {
    let line = if state.enabled {
        [0x7f, 0x84, 0x9c, 0xff]
    } else {
        [0x45, 0x47, 0x5a, 0xff]
    };
    let text = if state.text[3] > 0 {
        state.text
    } else if state.enabled {
        [0xcd, 0xd6, 0xf4, 0xff]
    } else {
        [0x6c, 0x70, 0x86, 0xff]
    };
    let size = if state.font_size > 0.0 {
        state.font_size
    } else {
        9.0
    };
    let top = 7.0;
    let gap = if state.caption.is_empty() {
        None
    } else {
        Some((8.0, caption_w))
    };
    let seg = |x0: f32, y0: f32, x1: f32, y1: f32| Command::Line {
        x0,
        y0,
        x1,
        y1,
        color: line,
        width: 1.0,
    };
    let mut out = vec![
        seg(0.5, top, 0.5, h - 0.5),
        seg(0.5, h - 0.5, w - 0.5, h - 0.5),
        seg(w - 0.5, top, w - 0.5, h - 0.5),
    ];
    match gap {
        Some((x, cw)) => {
            out.push(seg(0.5, top, x - 3.0, top));
            out.push(seg((x + cw + 3.0).min(w - 0.5), top, w - 0.5, top));
            out.push(Command::Text {
                text: state.caption.clone(),
                x,
                y: 0.0,
                color: text,
                size,
            });
        }
        None => out.push(seg(0.5, top, w - 0.5, top)),
    }
    for (i, (x, y, _, ch)) in cells(w, h, state.items.len(), state.columns)
        .into_iter()
        .enumerate()
    {
        let cy = y + ch / 2.0;
        out.push(Command::Circle {
            x: x + MARK / 2.0,
            y: cy,
            r: MARK / 2.0,
            fill: Some([0x18, 0x18, 0x25, 0xff]),
            stroke: Some((line, 1.0)),
        });
        if state.index == i as i64 {
            out.push(Command::Circle {
                x: x + MARK / 2.0,
                y: cy,
                r: MARK / 4.0,
                fill: Some(text),
                stroke: None,
            });
        }
        out.push(Command::Text {
            text: state.items[i].clone(),
            x: x + MARK + 6.0,
            y: cy - size * 0.75,
            color: text,
            size,
        });
    }
    out
}

#[crawrad_note::note("radio-group-crawradradiogroup")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradRadioGroup {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_paint: DrawPaint,
    #[live]
    draw_text: DrawText,
    #[rust]
    state: GroupState,
    #[rust]
    area: Area,
}

impl Widget for CrawradRadioGroup {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.state.enabled {
            return;
        }
        if let Hit::FingerUp(fe) = event.hits(cx, self.area) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            let r = self.area.rect(cx);
            let (x, y) = ((fe.abs.x - r.pos.x) as f32, (fe.abs.y - r.pos.y) as f32);
            let hit = cells(
                r.size.x as f32,
                r.size.y as f32,
                self.state.items.len(),
                self.state.columns,
            )
            .into_iter()
            .position(|(cx0, cy0, cw, ch)| x >= cx0 && x < cx0 + cw && y >= cy0 && y < cy0 + ch);
            if let Some(i) = hit {
                self.state.index = i as i64;
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    CrawradRadioGroupAction::Picked(i),
                );
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        // the caption's width, measured, for the gap in the frame
        let size = if self.state.font_size > 0.0 {
            self.state.font_size
        } else {
            9.0
        };
        self.draw_text.text_style.font_size = size;
        let caption_w = self
            .draw_text
            .layout(
                cx,
                0.0,
                0.0,
                None,
                false,
                Align::default(),
                &self.state.caption,
            )
            .size_in_lpxs
            .width;
        let commands = commands(
            &self.state,
            rect.size.x as f32,
            rect.size.y as f32,
            caption_w,
        );
        paint(
            cx,
            &mut self.draw_paint,
            &mut self.draw_text,
            rect.pos,
            &commands,
        );
        DrawStep::done()
    }
}

impl CrawradRadioGroupRef {
    #[crawrad_note::note("radio-group-crawradradiogroupref-set-state")]
    pub fn set_state(&self, cx: &mut Cx, state: GroupState) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.state != state
        {
            inner.state = state;
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("radio-group-crawradradiogroupref-picked")]
    pub fn picked(&self, actions: &Actions) -> Option<usize> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradRadioGroupAction::Picked(i) => Some(i),
            CrawradRadioGroupAction::None => None,
        }
    }
}

#[cfg(test)]
#[path = "test/radio_group/test.rs"]
mod tests;
