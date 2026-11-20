//! The Form check panel: the Errors and Warnings chips, Fix
//! all, and one row per finding — its pin number, rule, text and fix.

use makepad_widgets::*;

/// A row: (pin number, an error, rule, text, fix button or none, selected).
pub type CheckRowParts = (usize, bool, String, String, Option<String>, bool);

/// The head: the two chips as (label, shown), the note, and Fix all's label.
pub type CheckHeadParts = ([(String, bool); 2], String, Option<String>);

/// What the Form check view was asked this frame.
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum FormCheckAction {
    None,
    /// A chip toggled (0 Errors, 1 Warnings).
    Chip(usize),
    /// A row clicked: select its control.
    Row(usize),
    /// A row's fix button.
    Fix(usize),
    FixAll,
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;

    CheckText = <Label> {
        width: Fit, height: Fit,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } color: #e7e7ea }
        text: ""
    }
    CheckMuted = <CheckText> { draw_text: { color: #8d9093 } }

    CheckChip = <RoundedView> {
        width: Fit, height: 22.0,
        padding: { left: 10.0, right: 10.0 },
        align: { y: 0.5 },
        cursor: Hand,
        draw_bg: { border_radius: 5.5, border_size: 0.0, color: #26292b }
        label = <CheckText> { draw_text: { text_style: { font_size: 8.5 } } }
    }

    // The pin: the number on a disc, red for an error, amber for a warning.
    // The Sdf box doubles its radius: 4.5 is the disc of an 18 px square.
    CheckPin = <RoundedView> {
        width: 18.0, height: 18.0,
        align: { x: 0.5, y: 0.5 },
        draw_bg: { border_radius: 4.5, border_size: 0.0, color: #d29922 }
        number = <CheckText> { draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 8.0 } color: #1b1d1f } }
    }

    CheckFix = <Button> {
        width: 120.0, height: 22.0,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } color: #f3b27a color_hover: #ffd2ab }
        draw_bg: { border_radius: 4.0, border_size: 1.0, color: #00000000, color_hover: #3a2616, color_down: #4a3020, border_color_1: #5a3a20, border_color_2: #5a3a20 }
    }

    CheckRow = <View> {
        width: Fill, height: 30.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 12.0, right: 12.0 },
        spacing: 10.0,
        cursor: Hand,
        show_bg: true,
        draw_bg: { color: #00000000 }
        pin = <CheckPin> {}
        rule = <CheckText> { width: 110.0, draw_text: { color: #a9abae } }
        text = <CheckText> { width: Fill }
        fix = <CheckFix> {}
        none = <CheckMuted> { width: 120.0, text: "no single fix" }
    }

    // The title bar's way in: the warning sign and what the checker found.
    pub FormCheckChip = <Button> {
        width: Fit, height: 26.0,
        margin: { right: 12.0 },
        padding: { left: 10.0, right: 10.0 },
        visible: false,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } color: #f3b27a color_hover: #ffd2ab }
        draw_bg: { border_radius: 6.5, border_size: 1.0, color: #3a2616, color_hover: #4a3020, color_down: #4a3020, border_color_1: #5a3a20, border_color_2: #5a3a20 }
        text: "Form check"
    }

    pub FormCheckView = {{FormCheckView}} {
        width: Fill, height: Fill,
        flow: Down,
        header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 }, spacing: 8.0,
            padding: { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 },
            chip_0 = <CheckChip> {}
            chip_1 = <CheckChip> {}
            <View> { width: Fill, height: 1.0 }
            note = <CheckMuted> {}
            fix_all = <Button> {
                width: Fit, height: 24.0,
                padding: { left: 12.0, right: 12.0 },
                draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 8.5 } color: #1b1d1f color_hover: #1b1d1f }
                draw_bg: { border_radius: 4.0, border_size: 0.0, color: #e8914a, color_hover: #f0a565, color_down: #d47d38, border_color_1: #e8914a, border_color_2: #e8914a }
            }
        }
        list = <PortalList> {
            width: Fill, height: Fill,
            scroll_bar: <ScrollBar> {}
            CheckRow = <CheckRow> {}
        }
        footer = <View> {
            width: Fill, height: 28.0,
            padding: { left: 12.0, right: 12.0 },
            align: { y: 0.5 },
            <CheckMuted> { text: "Click a row selects the control on the form · Fix is one undo step · MCP: form.check" }
        }
    }

    pub FormCheckTip = {{FormCheckTip}} {
        width: Fit, height: Fit,
        visible: false,
        card = <RoundedView> {
            width: 260.0, height: Fit,
            flow: Down,
            padding: { top: 10.0, bottom: 10.0, left: 12.0, right: 12.0 },
            draw_bg: {
                color: #2a2d30,
                border_size: 1.0,
                border_color: #4a4f54,
                border_radius: 3.0, // Makepad doubles it: the mockup's 6 px corner
            }
            text = <CheckText> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 9.0 },
                    color: #f0a39c,
                }
                text: ""
            }
            fix = <CheckText> {
                width: Fill, height: Fit,
                margin: { top: 6.0 },
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 },
                    color: #e8914a,
                }
                text: ""
            }
        }
    }
}

const CHIPS: [LiveId; 2] = [live_id!(chip_0), live_id!(chip_1)];

/// The Form check bottom view.
#[derive(Live, LiveHook, Widget)]
pub struct FormCheckView {
    #[deref]
    view: View,
    #[rust]
    rows: Vec<CheckRowParts>,
    #[rust]
    drawn: Vec<serde_json::Value>,
}

impl Widget for FormCheckView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let act = self.action(&actions);
        if act != FormCheckAction::None {
            cx.widget_action(self.widget_uid(), &scope.path, act);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.drawn.clear();
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            let list = step.as_portal_list();
            let Some(mut list) = list.borrow_mut() else {
                continue;
            };
            list.set_item_range(cx, 0, self.rows.len());
            while let Some(i) = list.next_visible_item(cx) {
                let Some((number, error, rule, text, fix, selected)) = self.rows.get(i) else {
                    continue;
                };
                let item = list.item(cx, i, live_id!(CheckRow));
                let bg = match selected {
                    true => vec4(0.165, 0.176, 0.188, 1.0),
                    false => vec4(0.0, 0.0, 0.0, 0.0),
                };
                item.apply_over(cx, live! { draw_bg: { color: (bg) } });
                let [disc, ink] = match error {
                    true => [vec4(0.898, 0.325, 0.294, 1.0), vec4(1.0, 1.0, 1.0, 1.0)],
                    false => [
                        vec4(0.824, 0.600, 0.133, 1.0),
                        vec4(0.106, 0.114, 0.122, 1.0),
                    ],
                };
                let pin = item.view(id!(pin));
                pin.apply_over(cx, live! { draw_bg: { color: (disc) } });
                let n = pin.label(id!(number));
                n.set_text(cx, &number.to_string());
                n.apply_over(cx, live! { draw_text: { color: (ink) } });
                item.label(id!(rule)).set_text(cx, rule);
                item.label(id!(text)).set_text(cx, text);
                let button = item.button(id!(fix));
                button.set_visible(cx, fix.is_some());
                button.set_text(cx, fix.as_deref().unwrap_or_default());
                // The fix and "no single fix" share the last column.
                let none = if fix.is_none() { 120.0 } else { 0.0 };
                item.label(id!(none))
                    .apply_over(cx, live! { width: (none) });
                item.draw_all(cx, scope);
                let r = item.area().rect(cx);
                self.drawn.push(serde_json::json!({
                    "number": number, "error": error, "rule": rule, "text": text,
                    "fix": fix, "selected": selected,
                    "rect": [r.pos.x, r.pos.y, r.size.x, r.size.y],
                }));
            }
        }
        DrawStep::done()
    }
}

impl FormCheckView {
    /// Paints the head and the rows.
    pub fn set_view(&mut self, cx: &mut Cx, head: CheckHeadParts, rows: Vec<CheckRowParts>) {
        // The mockup's pills: Errors red, Warnings amber; a hidden one fades.
        const LOOK: [([f32; 3], [f32; 3]); 2] = [
            ([0.227, 0.133, 0.141], [0.949, 0.639, 0.627]),
            ([0.227, 0.188, 0.094], [0.902, 0.769, 0.431]),
        ];
        let (chips, note, fix_all) = head;
        for ((chip, (label, shown)), (bg, fg)) in CHIPS.into_iter().zip(chips).zip(LOOK) {
            let view = self.view.view(&[chip]);
            view.label(id!(label)).set_text(cx, &label);
            let alpha = if shown { 1.0 } else { 0.4 };
            let bg = vec4(bg[0], bg[1], bg[2], alpha);
            view.apply_over(cx, live! { draw_bg: { color: (bg) } });
            let fg = vec4(fg[0], fg[1], fg[2], alpha);
            view.label(id!(label))
                .apply_over(cx, live! { draw_text: { color: (fg) } });
        }
        self.view.label(id!(header.note)).set_text(cx, &note);
        let button = self.view.button(id!(header.fix_all));
        button.set_visible(cx, fix_all.is_some());
        button.set_text(cx, fix_all.as_deref().unwrap_or_default());
        self.rows = rows;
        self.view.portal_list(id!(list)).redraw(cx);
        self.redraw(cx);
    }

    /// What the user did this frame.
    pub fn action(&mut self, actions: &Actions) -> FormCheckAction {
        for a in actions {
            if let Some(w) = a.downcast_ref::<WidgetAction>()
                && w.widget_uid == self.widget_uid()
                && let Some(act) = w.action.downcast_ref::<FormCheckAction>()
            {
                return act.clone();
            }
        }
        if self.view.button(id!(header.fix_all)).clicked(actions) {
            return FormCheckAction::FixAll;
        }
        for (i, chip) in CHIPS.into_iter().enumerate() {
            if self
                .view
                .view(&[chip])
                .finger_up(actions)
                .is_some_and(|f| f.is_over)
            {
                return FormCheckAction::Chip(i);
            }
        }
        let list = self.view.portal_list(id!(list));
        for (i, item) in list.items_with_actions(actions) {
            if item.button(id!(fix)).clicked(actions) {
                return FormCheckAction::Fix(i);
            }
            if item.as_view().finger_up(actions).is_some_and(|f| f.is_over) {
                return FormCheckAction::Row(i);
            }
        }
        FormCheckAction::None
    }

    /// The drawn rows, for the MCP read.
    pub fn rows_json(&self) -> String {
        serde_json::json!({
            "note": self.view.label(id!(header.note)).text(),
            "chips": CHIPS.iter().map(|c| self.view.view(&[*c]).label(id!(label)).text()).collect::<Vec<_>>(),
            "rows": self.drawn,
        })
        .to_string()
    }
}

impl FormCheckViewRef {
    pub fn set_view(&self, cx: &mut Cx, head: CheckHeadParts, rows: Vec<CheckRowParts>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, head, rows);
        }
    }

    pub fn action(&self, actions: &Actions) -> FormCheckAction {
        // The view captures its buttons' actions and re-emits one of its own (see
        // `handle_event`): a mouse click only ever reaches the caller as that action.
        let own = actions
            .find_widget_action(self.widget_uid())
            .cast::<FormCheckAction>();
        if own != FormCheckAction::None {
            return own;
        }
        self.borrow_mut()
            .map_or(FormCheckAction::None, |mut inner| inner.action(actions))
    }

    pub fn rows_json(&self) -> String {
        self.borrow()
            .map_or_else(|| "{}".into(), |inner| inner.rows_json())
    }
}

/// The hover tip card for a pin on the canvas.
#[derive(Live, LiveHook, Widget)]
pub struct FormCheckTip {
    #[deref]
    view: View,
}

impl Widget for FormCheckTip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl FormCheckTip {
    pub fn show(&mut self, cx: &mut Cx, at: DVec2, text: &str, error: bool, fix: Option<&str>) {
        let card = self.view.view(id!(card));
        let fg = if error {
            vec4(0.941, 0.639, 0.612, 1.0)
        } else {
            vec4(0.902, 0.769, 0.431, 1.0)
        };
        let lbl_text = card.label(id!(text));
        lbl_text.set_text(cx, text);
        lbl_text.apply_over(cx, live! { draw_text: { color: (fg) } });

        let lbl_fix = card.label(id!(fix));
        match fix {
            Some(f) => {
                lbl_fix.set_text(cx, f);
                lbl_fix.apply_over(
                    cx,
                    live! {
                        margin: { top: 6.0 },
                        height: Fit,
                    },
                );
            }
            None => {
                lbl_fix.set_text(cx, "");
                lbl_fix.apply_over(
                    cx,
                    live! {
                        margin: { top: 0.0 },
                        height: 0.0,
                    },
                );
            }
        }

        self.view.apply_over(
            cx,
            live! {
                margin: { left: (at.x), top: (at.y) },
            },
        );
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }
}

impl FormCheckTipRef {
    pub fn show(&self, cx: &mut Cx, at: DVec2, text: &str, error: bool, fix: Option<&str>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx, at, text, error, fix);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }
}
