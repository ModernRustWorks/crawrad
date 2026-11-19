//! **A ListBox shows the items the form holds now**.
//!
//! The Makepad target used to emit one `<Button>` per DESIGN-TIME item, and
//! dispatch clicks per design-time index: an item the form added at run time
//! had no row at all, and the selected row was told apart by a `"> "` in its
//! text. Makepad 1.0.0 has the right widget, `PortalList`
//! (`makepad-widgets-1.0.0/src/portal_list.rs:65`), but it only draws the rows
//! its OWNER feeds while drawing (`set_item_range` / `next_visible_item` /
//! `item`, `:648` / `:348` / `:545`) — and a generated app has no draw loop of
//! its own, only `self.ui.handle_event`. So the feeding lives here, in a
//! widget: [`CrawradListBox`] holds the rows, draws as many as there are (the
//! visible ones), marks the selected one by colour, and reports a click as
//! [`CrawradListBoxAction::Selected`].

use crate::events::ClickRecord;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use crate::borderless::*;

    pub CrawradListBoxBase = {{CrawradListBox}} {}

    pub CrawradListBox = <CrawradListBoxBase> {
        width: Fill, height: Fill,
        show_bg: true,
        draw_bg: { color: #181825 }
        draw_detail: {
            color: #7f849c
            text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
        }
        detail_color_selected: #ffffff
        list = <PortalList> {
            width: Fill, height: Fill,
            row = <CrawradBorderlessButton> {
                width: Fill, height: 18.0, margin: 0.0,
                align: { x: 0.0, y: 0.5 },
                padding: { left: 6.0, right: 6.0 },
                draw_bg: {
                    color: #181825, color_hover: #262637, color_down: #2e477a, color_focus: #181825,
                }
                draw_text: { color: #cdd6f4, color_hover: #ffffff, color_focus: #cdd6f4 }
            }
            row_selected = <CrawradBorderlessButton> {
                width: Fill, height: 18.0, margin: 0.0,
                align: { x: 0.0, y: 0.5 },
                padding: { left: 6.0, right: 6.0 },
                draw_bg: {
                    color: #2e477a, color_hover: #385896, color_down: #385896, color_focus: #2e477a,
                }
                draw_text: { color: #ffffff, color_hover: #ffffff, color_focus: #ffffff }
            }
        }
    }
}

#[crawrad_note::note("list-rowstamps")]
type RowStamps = std::collections::HashMap<usize, (LiveId, u64)>;

#[crawrad_note::note("list-crawradlistboxaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradListBoxAction {
    /// The user clicked row `.0`.
    Selected(usize),
    /// The user double-clicked row `.0`; its `Selected` came first.
    DoubleClicked(usize),
    /// The keys held on the click `Selected` reports: `toggle` is Ctrl (Cmd
    /// on macOS), `extend` is Shift (`MultiSelect`).
    Keys {
        toggle: bool,
        extend: bool,
    },
    None,
}

#[crawrad_note::note("list-crawradlistbox")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradListBox {
    #[deref]
    view: View,
    /// The text at the right end of a row — a count, a shortcut.
    #[live]
    draw_detail: DrawText,
    #[live]
    detail_color_selected: Vec4,
    #[rust]
    items: Vec<String>,
    /// Each row's detail; a missing or empty one draws nothing.
    #[rust]
    details: Vec<String>,
    /// Each row's left padding indent.
    #[rust]
    indents: Vec<f64>,
    /// Whether the scroll bar is hidden because every row fits;
    /// `None` until a frame measured the list.
    #[rust]
    bar_hidden: Option<bool>,
    /// The list's height on the last frame: at the start of a draw the
    /// area is not measured yet.
    #[rust]
    list_h: f64,
    /// The frame booked after the first one, to measure and hide the bar.
    #[rust]
    second_frame: NextFrame,
    #[rust]
    selected: Option<usize>,
    /// Every row marked, when the form hands them; `None` marks only
    /// `selected`.
    #[rust]
    selection: Option<Vec<usize>>,
    // The font and colour the form gave it; the default is the
    // theme's for each.
    #[rust]
    look: ListLook,
    // bumped when `look` changes, so every row takes it again.
    #[rust]
    look_epoch: u64,
    // The epoch each visible row was last styled with. `apply_over`
    // redraws, so a row is styled once per change and not on every frame.
    #[rust]
    styled: RowStamps,
    /// The rows drawn on the last frame, with where they landed: what
    /// a host reports to MCP (`view.get_dialog`) and what a test clicks.
    #[rust]
    drawn: Vec<(String, Rect)>,
    #[rust]
    last_click: ClickRecord,
}

#[crawrad_note::note("list-listlook")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ListLook {
    pub font_size: f32,
    pub text: [u8; 4],
    pub background: [f32; 4],
}

impl Widget for CrawradListBox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.second_frame.is_event(event).is_some() {
            self.view.redraw(cx);
        }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let list = self.view.portal_list(id!(list));
        let clicked = list
            .items_with_actions(&actions)
            .into_iter()
            .find_map(|(i, row)| row.as_button().clicked_modifiers(&actions).map(|m| (i, m)));
        if let Some((i, keys)) = clicked {
            self.selected = Some(i);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradListBoxAction::Selected(i),
            );
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradListBoxAction::Keys {
                    toggle: keys.control || keys.logo,
                    extend: keys.shift,
                },
            );
            let now = cx.seconds_since_app_start();
            if crate::events::second_click(&mut self.last_click, i, now) {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    CrawradListBoxAction::DoubleClicked(i),
                );
            }
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Makepad's PortalList draws its scroll bar even when every row fits
        // (its `view_window` is the visible rows minus 3), over the rows'
        // right end; it shows only when the rows overflow.
        let row_h = self.drawn.first().map_or(0.0, |(_, r)| r.size.y);
        let bar = sync_scroll_bar(
            cx,
            &mut self.view,
            &mut self.bar_hidden,
            (self.items.len(), row_h, self.list_h),
        );
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.items.len());
                // drawn after every row: a row's background is its own draw
                // call when its look differs (the selected one), and a detail
                // drawn before it would end up under it
                let mut details = Vec::new();
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(text) = self.items.get(i) else {
                        continue;
                    };
                    let template = if self.marked(i) {
                        live_id!(row_selected)
                    } else {
                        live_id!(row)
                    };
                    let (row, existed) = list.item_with_existed(cx, i, template);
                    let stamp = (template, self.look_epoch);
                    if !existed || self.styled.get(&i) != Some(&stamp) {
                        self.styled.insert(i, stamp);
                        self.style_row(cx, i, &row, template == live_id!(row));
                    }
                    row.set_text(cx, text);
                    row.draw_all(cx, scope);
                    if let Some(detail) = self.details.get(i).filter(|d| !d.is_empty()) {
                        let mut r = row.area().rect(cx);
                        r.size.x -= bar;
                        details.push((r, detail.clone(), self.marked(i)));
                    }
                    // by index, not cleared per frame: a frame that skips the
                    // rows must not leave the list looking empty
                    if self.drawn.len() <= i {
                        self.drawn.resize(i + 1, (String::new(), Rect::default()));
                    }
                    self.drawn[i] = (text.clone(), row.area().rect(cx));
                }
                draw_details(
                    cx,
                    &mut self.draw_detail,
                    self.detail_color_selected,
                    self.look.font_size,
                    details,
                );
            }
        }
        self.list_h = self.view.area().rect(cx).size.y;
        // the first frame measures the list; a still app sends no event that
        // would draw a second one, and a redraw asked for DURING a draw is
        // dropped with it — so the next frame is booked, and it hides the bar
        if self.bar_hidden.is_none() && self.list_h > 0.0 && !self.drawn.is_empty() {
            self.second_frame = cx.new_next_frame();
        }
        DrawStep::done()
    }
}

#[crawrad_note::note("list-scroll-bar-size")]
const SCROLL_BAR_SIZE: f64 = 10.0;

#[crawrad_note::note("list-sync-scroll-bar")]
pub fn sync_scroll_bar(
    cx: &mut Cx,
    view: &mut View,
    bar_hidden: &mut Option<bool>,
    rows: (usize, f64, f64),
) -> f64 {
    let (count, row_h, list_h) = rows;
    if row_h > 0.0 && list_h > 0.0 {
        let fits = rows_fit(count, row_h, list_h);
        if *bar_hidden != Some(fits) {
            *bar_hidden = Some(fits);
            let size = if fits { 0.0 } else { SCROLL_BAR_SIZE };
            view.portal_list(id!(list))
                .apply_over(cx, live! { scroll_bar: { bar_size: (size) } });
            view.redraw(cx);
        }
    }
    if *bar_hidden == Some(true) {
        0.0
    } else {
        SCROLL_BAR_SIZE
    }
}

#[crawrad_note::note("list-draw-details")]
pub fn draw_details(
    cx: &mut Cx2d,
    draw: &mut DrawText,
    selected_color: Vec4,
    font_size: f32,
    details: Vec<(Rect, String, bool)>,
) {
    let size = if font_size > 0.0 {
        font_size - 1.0
    } else {
        DETAIL_FONT_SIZE
    };
    draw.text_style.font_size = size;
    let plain = draw.color;
    for (r, detail, selected) in details {
        draw.color = if selected { selected_color } else { plain };
        draw.draw_abs(cx, detail_pos(r, &detail, size), &detail);
    }
    draw.color = plain;
}

#[crawrad_note::note("list-rows-fit")]
pub fn rows_fit(count: usize, row_h: f64, list_h: f64) -> bool {
    count as f64 * row_h <= list_h + 0.5
}

#[crawrad_note::note("list-detail-font-size")]
pub const DETAIL_FONT_SIZE: f32 = 8.0;
#[crawrad_note::note("list-detail-char-w-per-pt")]
const DETAIL_CHAR_W_PER_PT: f64 = 0.75;
const DETAIL_PAD_RIGHT: f64 = 8.0;

#[crawrad_note::note("list-detail-pos")]
pub fn detail_pos(row: Rect, detail: &str, font_size: f32) -> DVec2 {
    let size = f64::from(font_size);
    let width = detail.chars().count() as f64 * size * DETAIL_CHAR_W_PER_PT;
    dvec2(
        row.pos.x + row.size.x - DETAIL_PAD_RIGHT - width,
        row.pos.y + (row.size.y - size * 1.4) / 2.0,
    )
}

#[crawrad_note::note("list-to-list-space")]
pub fn to_list_space(rows: &[(usize, (String, Rect))], top: DVec2) -> Vec<(String, Rect)> {
    let Some((i0, (_, r0))) = rows.first() else {
        return Vec::new();
    };
    let offset = top - dvec2(r0.pos.x, r0.pos.y - r0.size.y * *i0 as f64);
    rows.iter()
        .map(|(_, (t, r))| {
            (
                t.clone(),
                Rect {
                    pos: r.pos + offset,
                    size: r.size,
                },
            )
        })
        .collect()
}

impl CrawradListBox {
    #[crawrad_note::note("list-crawradlistbox-marked")]
    fn marked(&self, i: usize) -> bool {
        match &self.selection {
            Some(rows) => rows.contains(&i),
            None => self.selected == Some(i),
        }
    }

    #[crawrad_note::note("list-crawradlistbox-style-row")]
    fn style_row(&self, cx: &mut Cx, i: usize, row: &WidgetRef, plain: bool) {
        if let Some(&left) = self.indents.get(i) {
            row.apply_over(cx, live! { padding: { left: (left) } });
        }
        let look = self.look;
        if look.font_size > 0.0 {
            let s = f64::from(look.font_size);
            row.apply_over(cx, live! { draw_text: { text_style: { font_size: (s) } } });
        }
        if !plain {
            return;
        }
        if look.text[3] > 0 {
            let [r, g, b, a] = look.text.map(|v| f32::from(v) / 255.0);
            let c = vec4(r, g, b, a);
            row.apply_over(
                cx,
                live! { draw_text: { color: (c), color_hover: (c), color_focus: (c) } },
            );
        }
        if look.background[3] > 0.0 {
            let [r, g, b, a] = look.background;
            let c = vec4(r, g, b, a);
            row.apply_over(cx, live! { draw_bg: { color: (c), color_focus: (c) } });
        }
    }
}

crate::palette::palette_glue!(CrawradListBoxRef);

impl crate::palette::TakesLook for CrawradListBox {
    fn take_look(&mut self, cx: &mut Cx, look: ListLook) {
        self.look_epoch += 1;
        if look.background[3] > 0.0 {
            let [r, g, b, a] = look.background;
            let c = vec4(r, g, b, a);
            self.view.apply_over(cx, live! { draw_bg: { color: (c) } });
        }
    }
}

impl CrawradListBoxRef {
    #[crawrad_note::note("list-crawradlistboxref-set-items")]
    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
            inner.drawn.clear();
            inner.view.redraw(cx);
        }
    }

    #[crawrad_note::note("list-crawradlistboxref-set-details")]
    pub fn set_details(&self, cx: &mut Cx, details: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.details != details
        {
            inner.details = details.to_vec();
            inner.view.redraw(cx);
        }
    }

    #[crawrad_note::note("list-crawradlistboxref-set-indents")]
    pub fn set_indents(&self, cx: &mut Cx, indents: &[f64]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.indents != indents
        {
            inner.indents = indents.to_vec();
            inner.look_epoch += 1;
            inner.view.redraw(cx);
        }
    }

    #[crawrad_note::note("list-crawradlistboxref-set-selected")]
    pub fn set_selected(&self, cx: &mut Cx, selected: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.selected != selected
        {
            inner.selected = selected;
            inner.view.redraw(cx);
        }
    }

    #[crawrad_note::note("list-crawradlistboxref-set-selection")]
    pub fn set_selection(&self, cx: &mut Cx, rows: &[usize]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.selection.as_deref() != Some(rows)
        {
            inner.selection = Some(rows.to_vec());
            inner.view.redraw(cx);
        }
    }

    #[crawrad_note::note("list-crawradlistboxref-drawn-rows")]
    pub fn drawn_rows(&self, cx: &Cx) -> Vec<(String, Rect)> {
        let Some(inner) = self.borrow() else {
            return Vec::new();
        };
        let rows: Vec<(usize, (String, Rect))> = (inner.drawn.iter().cloned().enumerate())
            .take(inner.items.len())
            .filter(|(_, (t, _))| !t.is_empty())
            .collect();
        let top = inner.view.area().rect(cx).pos;
        to_list_space(&rows, top)
    }

    #[crawrad_note::note("list-crawradlistboxref-selected")]
    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        self.first(actions, |a| match a {
            CrawradListBoxAction::Selected(i) => Some(i),
            _ => None,
        })
    }

    #[crawrad_note::note("list-crawradlistboxref-double-clicked")]
    pub fn double_clicked(&self, actions: &Actions) -> Option<usize> {
        self.first(actions, |a| match a {
            CrawradListBoxAction::DoubleClicked(i) => Some(i),
            _ => None,
        })
    }

    #[crawrad_note::note("list-crawradlistboxref-keys")]
    pub fn keys(&self, actions: &Actions) -> Option<(bool, bool)> {
        actions
            .filter_widget_actions(self.widget_uid())
            .map(|a| a.cast())
            .find_map(|a| match a {
                CrawradListBoxAction::Keys { toggle, extend } => Some((toggle, extend)),
                _ => None,
            })
    }

    #[crawrad_note::note("list-crawradlistboxref-first")]
    fn first(
        &self,
        actions: &Actions,
        pick: impl Fn(CrawradListBoxAction) -> Option<usize>,
    ) -> Option<usize> {
        actions
            .filter_widget_actions(self.widget_uid())
            .map(|a| a.cast())
            .find_map(pick)
    }
}

#[cfg(test)]
#[path = "test/list/test.rs"]
mod tests;
