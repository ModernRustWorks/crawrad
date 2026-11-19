//! **A CheckListBox: a list whose every row has a check box**.
//!
//! The classic RAD palette has one; ours had none, so the IDE's Build dialog
//! drew its eight targets as eight hand-made check boxes. Here the rows are
//! Makepad's own `CheckBox`, fed by a `PortalList` the way [`CrawradListBox`]
//! feeds its buttons: the widget holds the items, which are checked, and an
//! optional detail at each row's right end (a target triple, a size). A click
//! toggles the row and reports [`CrawradCheckListBoxAction::Toggled`].
//!
//! [`CrawradListBox`]: crate::list::CrawradListBox

use makepad_widgets::*;

use crate::list::{ListLook, draw_details, sync_scroll_bar, to_list_space};

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;

    pub CrawradCheckListBoxBase = {{CrawradCheckListBox}} {}

    pub CrawradCheckListBox = <CrawradCheckListBoxBase> {
        width: Fill, height: Fill,
        show_bg: true,
        draw_bg: { color: #181825 }
        draw_detail: {
            color: #7f849c
            text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
        }
        list = <PortalList> {
            width: Fill, height: Fill,
            row = <CheckBox> {
                // 6 px in from the list's left edge, as the designer draws it
                width: Fill, height: 22.0, margin: { left: 6.0 },
                align: { x: 0.0, y: 0.5 },
            }
        }
    }
}

#[crawrad_note::note("check-list-stamps")]
type Stamps = std::collections::HashMap<usize, u64>;

#[crawrad_note::note("check-list-crawradchecklistboxaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradCheckListBoxAction {
    /// Row `.0` was clicked and is now checked (`true`) or not.
    Toggled(usize, bool),
    None,
}

#[crawrad_note::note("check-list-crawradchecklistbox")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradCheckListBox {
    #[deref]
    view: View,
    /// The text at the right end of a row.
    #[live]
    draw_detail: DrawText,
    #[rust]
    items: Vec<String>,
    #[rust]
    details: Vec<String>,
    #[rust]
    checked: Vec<bool>,
    /// Born enabled, like a Button.
    #[rust(true)]
    enabled: bool,
    #[rust]
    look: ListLook,
    // bumped when `look` changes, so every row takes the font again
    #[rust]
    look_epoch: u64,
    #[rust]
    styled: Stamps,
    #[rust]
    bar_hidden: Option<bool>,
    #[rust]
    list_h: f64,
    /// The frame booked after the first one, to measure and hide the bar.
    #[rust]
    second_frame: NextFrame,
    /// The rows drawn on the last frame, with where they landed: what a host
    /// reports to MCP (`view.get_dialog`) and what a test clicks.
    #[rust]
    drawn: Vec<(String, Rect)>,
}

impl Widget for CrawradCheckListBox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.second_frame.is_event(event).is_some() {
            self.view.redraw(cx);
        }
        if !self.enabled {
            return;
        }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let list = self.view.portal_list(id!(list));
        let toggled = list
            .items_with_actions(&actions)
            .into_iter()
            .find_map(|(i, row)| row.as_check_box().changed(&actions).map(|on| (i, on)));
        if let Some((i, on)) = toggled.filter(|(i, _)| *i < self.items.len()) {
            self.checked.resize(self.items.len(), false);
            self.checked[i] = on;
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradCheckListBoxAction::Toggled(i, on),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
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
                let mut details = Vec::new();
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(text) = self.items.get(i) else {
                        continue;
                    };
                    let (row, existed) = list.item_with_existed(cx, i, live_id!(row));
                    if !existed || self.styled.get(&i) != Some(&self.look_epoch) {
                        self.styled.insert(i, self.look_epoch);
                        crate::look::font_size(cx, &row, self.look.font_size);
                    }
                    let check = row.as_check_box();
                    check.set_text(text);
                    let on = self.checked.get(i).copied().unwrap_or(false);
                    if check.active(cx) != on {
                        check.set_active(cx, on);
                    }
                    row.draw_all(cx, scope);
                    let rect = row.area().rect(cx);
                    if let Some(detail) = self.details.get(i).filter(|d| !d.is_empty()) {
                        let mut r = rect;
                        r.size.x -= bar;
                        details.push((r, detail.clone(), false));
                    }
                    if self.drawn.len() <= i {
                        self.drawn.resize(i + 1, (String::new(), Rect::default()));
                    }
                    self.drawn[i] = (text.clone(), rect);
                }
                let plain = self.draw_detail.color;
                draw_details(
                    cx,
                    &mut self.draw_detail,
                    plain,
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

crate::palette::palette_glue!(CrawradCheckListBoxRef, enabled);

impl crate::palette::TakesLook for CrawradCheckListBox {
    fn take_look(&mut self, cx: &mut Cx, look: ListLook) {
        self.look_epoch += 1;
        if look.background[3] > 0.0 {
            let [r, g, b, a] = look.background;
            let c = vec4(r, g, b, a);
            self.view.apply_over(cx, live! { draw_bg: { color: (c) } });
        }
    }
}

impl CrawradCheckListBoxRef {
    #[crawrad_note::note("check-list-crawradchecklistboxref-set-items")]
    pub fn set_items(&self, cx: &mut Cx, items: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.items != items
        {
            inner.items = items.to_vec();
            inner.drawn.clear();
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("check-list-crawradchecklistboxref-set-details")]
    pub fn set_details(&self, cx: &mut Cx, details: &[String]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.details != details
        {
            inner.details = details.to_vec();
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("check-list-crawradchecklistboxref-set-checked")]
    pub fn set_checked(&self, cx: &mut Cx, checked: &[bool]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.checked != checked
        {
            inner.checked = checked.to_vec();
            inner.redraw(cx);
        }
    }

    #[crawrad_note::note("check-list-crawradchecklistboxref-checked")]
    pub fn checked(&self) -> Vec<bool> {
        self.borrow().map_or_else(Vec::new, |inner| {
            let mut out = inner.checked.clone();
            out.resize(inner.items.len(), false);
            out
        })
    }

    #[crawrad_note::note("check-list-crawradchecklistboxref-drawn-rows")]
    pub fn drawn_rows(&self, cx: &Cx) -> Vec<(String, Rect)> {
        let Some(inner) = self.borrow() else {
            return Vec::new();
        };
        let rows: Vec<(usize, (String, Rect))> = (inner.drawn.iter().cloned().enumerate())
            .take(inner.items.len())
            .filter(|(_, (t, _))| !t.is_empty())
            .collect();
        to_list_space(&rows, inner.view.area().rect(cx).pos)
    }

    #[crawrad_note::note("check-list-crawradchecklistboxref-toggled")]
    pub fn toggled(&self, actions: &Actions) -> Option<(usize, bool)> {
        let uid = self.widget_uid();
        actions.iter().find_map(|a| {
            let a = a.as_widget_action()?;
            if a.widget_uid != uid {
                return None;
            }
            match a.cast() {
                CrawradCheckListBoxAction::Toggled(i, on) => Some((i, on)),
                CrawradCheckListBoxAction::None => None,
            }
        })
    }
}

#[crawrad_note::note("check-list-checked-flags")]
pub fn checked_flags(items: &[String], checked: &[String]) -> Vec<bool> {
    items
        .iter()
        .map(|it| checked.iter().any(|c| c == it))
        .collect()
}

#[crawrad_note::note("check-list-set-checked-item")]
pub fn set_checked_item(items: &[String], checked: &mut Vec<String>, index: usize, on: bool) {
    let Some(item) = items.get(index) else {
        return;
    };
    if on {
        if !checked.iter().any(|c| c == item) {
            checked.push(item.clone());
        }
    } else {
        checked.retain(|c| c != item);
    }
}

#[cfg(test)]
#[path = "test/check_list/test.rs"]
mod tests;
