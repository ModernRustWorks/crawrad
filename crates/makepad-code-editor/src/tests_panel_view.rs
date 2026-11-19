//! The Test panel: chips, summary, name filter and the tree
//! package › file › test; and the Test output view of the picked test.

use makepad_widgets::*;

/// A tree row as the view paints it: (depth 0 package / 1 file / 2 test,
/// mark 0 failed / 1 passed / 2 ignored / 3 not run, text, right note, open, selected).
pub type TestsRowParts = (u8, u8, String, String, bool, bool);

/// The head: the three chips (Passed, Failed, Ignored) as (label, shown), and the summary line.
pub type TestsHeadParts = ([(String, bool); 3], String);

/// The Test output: (mark, test name, `in file · time`, output lines, the assert's `file:line:col`).
pub type TestOutputParts = (u8, String, String, Vec<String>, String);

/// What the Tests view was asked this frame.
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum TestsPanelAction {
    None,
    RunAll,
    RunFailed,
    /// The refresh icon: list the tests again.
    Refresh,
    /// A chip toggled (0 Passed, 1 Failed, 2 Ignored).
    Chip(usize),
    /// The name filter changed.
    Filter(String),
    /// A row clicked once: a file folds, a test opens.
    Row(usize),
    /// A row double-clicked: runs that file or test.
    RunRow(usize),
    /// The output's place link clicked.
    OpenSpot,
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;

    TestText = <Label> {
        width: Fit, height: Fit,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } color: #e7e7ea }
        text: ""
    }
    TestMuted = <TestText> { draw_text: { color: #98989b } }
    TestCode = <TestText> { draw_text: { text_style: <THEME_FONT_CODE> { font_size: 9.0 } } }

    // ✓ ✗ ○ · drawn, not glyphs: the theme font has none of them.
    TestMark = <View> {
        width: 12.0, height: 12.0,
        show_bg: true,
        draw_bg: {
            instance mark: 3.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                if self.mark < 0.5 {
                    sdf.move_to(c.x - 3.5, c.y - 3.5); sdf.line_to(c.x + 3.5, c.y + 3.5);
                    sdf.move_to(c.x + 3.5, c.y - 3.5); sdf.line_to(c.x - 3.5, c.y + 3.5);
                    sdf.stroke(#e2726b, 1.6);
                } else if self.mark < 1.5 {
                    sdf.move_to(c.x - 4.0, c.y); sdf.line_to(c.x - 1.0, c.y + 3.0);
                    sdf.line_to(c.x + 4.0, c.y - 3.5);
                    sdf.stroke(#7cb46b, 1.6);
                } else if self.mark < 2.5 {
                    sdf.circle(c.x, c.y, 3.5);
                    sdf.stroke(#98989b, 1.2);
                } else {
                    sdf.circle(c.x, c.y, 1.5);
                    sdf.fill(#6b6b6e);
                }
                return sdf.result;
            }
        }
    }

    TestChevron = <View> {
        width: 10.0, height: 10.0,
        show_bg: true,
        draw_bg: {
            instance open: 1.0
            instance shown: 1.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                if self.open > 0.5 {
                    sdf.move_to(c.x - 3.0, c.y - 1.5); sdf.line_to(c.x, c.y + 1.5); sdf.line_to(c.x + 3.0, c.y - 1.5);
                } else {
                    sdf.move_to(c.x - 1.5, c.y - 3.0); sdf.line_to(c.x + 1.5, c.y); sdf.line_to(c.x - 1.5, c.y + 3.0);
                }
                sdf.stroke(#98989b * self.shown, 1.2);
                return sdf.result;
            }
        }
    }

    TestChip = <RoundedView> {
        width: Fit, height: 20.0,
        padding: { left: 9.0, right: 9.0 },
        align: { y: 0.5 },
        cursor: Hand,
        // Makepad's RoundedView draws twice the border_radius: a 20 px pill takes 5, not 10.
        draw_bg: { border_radius: 5.0, border_size: 1.0, color: #26292b, border_color: #00000000 }
        label = <TestText> { draw_text: { text_style: { font_size: 8.5 } } }
    }

    TestButton = <Button> {
        width: Fit, height: 22.0,
        padding: { left: 9.0, right: 9.0 },
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } color: #e7e7ea color_hover: #ffffff }
        draw_bg: { border_radius: 4.0, border_size: 1.0, color: #2b2b2d, color_hover: #343437, color_down: #3a3a3d, border_color_1: #4d4f52, border_color_2: #4d4f52 }
    }

    // The refresh arrow, drawn: an open ring and its head.
    TestRefresh = <Button> {
        width: 22.0, height: 22.0, text: ""
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.arc_round_caps(c.x, c.y, 5.5, 0.6, 5.6, 1.4);
                sdf.fill(mix(#b7b7ba, #ffffff, self.hover));
                sdf.move_to(c.x + 5.5, c.y - 4.5); sdf.line_to(c.x + 5.5, c.y - 0.5); sdf.line_to(c.x + 1.5, c.y - 0.5);
                sdf.stroke(mix(#b7b7ba, #ffffff, self.hover), 1.4);
                return sdf.result;
            }
        }
    }

    TestRow = <View> {
        width: Fill, height: 22.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: { color: #00000000 }
        btn = <Button> {
            width: Fill, height: Fill, margin: 0.0, padding: 0.0, text: ""
            draw_bg: { border_size: 0.0, color: #00000000, color_hover: #262a31, color_down: #262a31 }
        }
        face = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 12.0, right: 12.0 },
            spacing: 6.0,
            chevron = <TestChevron> {}
            mark = <TestMark> {}
            name = <TestCode> {}
            spacer = <View> { width: Fill, height: 1.0 }
            note = <TestMuted> {}
        }
    }

    pub TestsPanelView = {{TestsPanelView}} {
        width: Fill, height: Fill,
        flow: Down,
        header = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 }, spacing: 6.0,
            padding: { left: 12.0, right: 12.0, top: 10.0, bottom: 10.0 },
            show_bg: true,
            draw_bg: { color: #00000000 }
            <TestText> { width: Fill, draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 9.5 } } text: "Tests" }
            run_all = <TestButton> {
                text: "Run all"
                draw_bg: { color: #de5a2b, color_hover: #e86d40, color_down: #c94e22, border_color_1: #de5a2b, border_color_2: #de5a2b }
                draw_text: { color: #ffffff }
            }
            run_failed = <TestButton> { text: "Run failed" }
            refresh = <TestRefresh> {}
        }
        <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #2b2e30 } }
        chips = <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 6.0,
            padding: { left: 12.0, right: 12.0, top: 8.0, bottom: 6.0 },
            chip_0 = <TestChip> { draw_bg: { color: #1f3322 } label = { draw_text: { color: #9fd28c } } }
            chip_1 = <TestChip> { draw_bg: { color: #3a2224, border_color: #6b3530 } label = { draw_text: { color: #f2a3a0 } } }
            chip_2 = <TestChip> { label = { draw_text: { color: #b7b7ba } } }
        }
        summary = <TestMuted> { margin: { left: 12.0, bottom: 8.0 } }
        filter = <TextInput> {
            width: Fill, height: 26.0,
            margin: { left: 12.0, right: 12.0, bottom: 8.0 },
            empty_text: "Filter by name"
        }
        list = <PortalList> {
            width: Fill, height: Fill,
            scroll_bar: <ScrollBar> {}
            TestRow = <TestRow> {}
        }
        footer = <View> {
            width: Fill, height: 28.0,
            padding: { left: 12.0, right: 12.0 },
            align: { y: 0.5 },
            show_bg: true,
            draw_bg: { color: #1d1f20 }
            <TestMuted> { draw_text: { color: #7d8087 } text: "Click opens a test · double-click runs it" }
        }
    }

    OutputLine = <View> {
        width: Fill, height: 19.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 14.0 },
        before = <TestCode> { draw_text: { color: #b7b7ba } }
        link = <LinkLabel> {
            width: Fit, height: Fit, padding: 0.0, margin: 0.0,
            draw_text: { text_style: <THEME_FONT_CODE> { font_size: 9.0 } color: #8fb8ff color_hover: #b9d2ff }
            text: ""
        }
        after = <TestCode> { draw_text: { color: #b7b7ba } }
    }

    pub TestOutputView = {{TestOutputView}} {
        width: Fill, height: Fill,
        flow: Down,
        head = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 }, spacing: 10.0,
            padding: { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 },
            <TestText> { draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 9.5 } } text: "Test output" }
            mark = <TestMark> {}
            name = <TestText> {}
            spacer = <View> { width: Fill, height: 1.0 }
            place = <TestMuted> {}
        }
        lines = <PortalList> {
            width: Fill, height: Fill,
            scroll_bar: <ScrollBar> {}
            OutputLine = <OutputLine> {}
        }
    }
}

const CHIPS: [LiveId; 3] = [live_id!(chip_0), live_id!(chip_1), live_id!(chip_2)];

/// The colour of a mark's text: failed red, passed and not run plain, ignored muted.
fn name_color(mark: u8) -> Vec4 {
    match mark {
        2 => vec4(0.596, 0.596, 0.608, 1.0),
        _ => vec4(0.906, 0.906, 0.918, 1.0),
    }
}

/// The Tests side view.
#[derive(Live, LiveHook, Widget)]
pub struct TestsPanelView {
    #[deref]
    view: View,
    #[rust]
    rows: Vec<TestsRowParts>,
    #[rust]
    last_click: Option<(usize, std::time::Instant)>,
    #[rust]
    drawn: Vec<serde_json::Value>,
}

impl Widget for TestsPanelView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let act = self.action(&actions);
        if act != TestsPanelAction::None {
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
                let Some((depth, mark, text, note, open, selected)) = self.rows.get(i) else {
                    continue;
                };
                let item = list.item(cx, i, live_id!(TestRow));
                let indent = [12.0, 28.0, 46.0][(*depth as usize).min(2)];
                let chevron = if *depth == 2 { 0.0 } else { 1.0 };
                let open = if *open { 1.0 } else { 0.0 };
                let bg = if *selected {
                    vec4(0.114, 0.176, 0.239, 1.0)
                } else {
                    vec4(0.0, 0.0, 0.0, 0.0)
                };
                item.apply_over(
                    cx,
                    live! { draw_bg: { color: (bg) } face: { padding: { left: (indent) } } },
                );
                item.view(id!(face.chevron)).apply_over(
                    cx,
                    live! { width: (10.0 * chevron), draw_bg: { open: (open), shown: (chevron) } },
                );
                item.view(id!(face.mark))
                    .apply_over(cx, live! { draw_bg: { mark: (*mark as f64) } });
                let name = item.label(id!(face.name));
                name.set_text(cx, text);
                name.apply_over(cx, live! { draw_text: { color: (name_color(*mark)) } });
                item.label(id!(face.note)).set_text(cx, note);
                item.draw_all(cx, scope);
                let r = item.area().rect(cx);
                self.drawn.push(serde_json::json!({
                    "depth": depth, "mark": mark, "text": text, "note": note,
                    "open": open > 0.5, "selected": selected,
                    "rect": [r.pos.x, r.pos.y, r.size.x, r.size.y],
                }));
            }
        }
        DrawStep::done()
    }
}

impl TestsPanelView {
    /// Paints the head and the tree.
    pub fn set_view(&mut self, cx: &mut Cx, head: TestsHeadParts, rows: Vec<TestsRowParts>) {
        // The mockup's pills: Passed green, Failed red, Ignored grey; a hidden one fades.
        const LOOK: [([f32; 3], [f32; 3]); 3] = [
            ([0.122, 0.200, 0.133], [0.624, 0.824, 0.549]),
            ([0.227, 0.133, 0.141], [0.949, 0.639, 0.627]),
            ([0.149, 0.161, 0.169], [0.718, 0.718, 0.729]),
        ];
        for ((chip, (label, shown)), (bg, fg)) in CHIPS.into_iter().zip(head.0).zip(LOOK) {
            let view = self.view.view(&[chip]);
            view.label(id!(label)).set_text(cx, &label);
            let alpha = if shown { 1.0 } else { 0.4 };
            let bg = vec4(bg[0], bg[1], bg[2], alpha);
            view.apply_over(cx, live! { draw_bg: { color: (bg) } });
            let fg = vec4(fg[0], fg[1], fg[2], alpha);
            view.label(id!(label))
                .apply_over(cx, live! { draw_text: { color: (fg) } });
        }
        self.view.label(id!(summary)).set_text(cx, &head.1);
        self.rows = rows;
        self.view.portal_list(id!(list)).redraw(cx);
        self.redraw(cx);
    }

    /// What the user did this frame.
    pub fn action(&mut self, actions: &Actions) -> TestsPanelAction {
        for a in actions {
            if let Some(w) = a.downcast_ref::<WidgetAction>()
                && w.widget_uid == self.widget_uid()
                && let Some(act) = w.action.downcast_ref::<TestsPanelAction>()
            {
                return act.clone();
            }
        }
        let buttons = [
            (id!(header.run_all), TestsPanelAction::RunAll),
            (id!(header.run_failed), TestsPanelAction::RunFailed),
            (id!(header.refresh), TestsPanelAction::Refresh),
        ];
        for (path, act) in buttons {
            if self.view.button(path).clicked(actions) {
                return act;
            }
        }
        for (i, chip) in CHIPS.into_iter().enumerate() {
            if self
                .view
                .view(&[chip])
                .finger_up(actions)
                .is_some_and(|f| f.is_over)
            {
                return TestsPanelAction::Chip(i);
            }
        }
        if let Some(text) = self.view.text_input(id!(filter)).changed(actions) {
            return TestsPanelAction::Filter(text);
        }
        let list = self.view.portal_list(id!(list));
        for (i, item) in list.items_with_actions(actions) {
            if !item.button(id!(btn)).clicked(actions) || self.rows.get(i).is_none_or(|r| r.0 == 0)
            {
                continue;
            }
            let now = std::time::Instant::now();
            if let Some((last, at)) = self.last_click.take()
                && last == i
                && now.duration_since(at).as_millis() <= 400
            {
                return TestsPanelAction::RunRow(i);
            }
            self.last_click = Some((i, now));
            return TestsPanelAction::Row(i);
        }
        TestsPanelAction::None
    }

    /// The drawn rows, for the MCP read.
    pub fn rows_json(&self) -> String {
        serde_json::json!({
            "summary": self.view.label(id!(summary)).text(),
            "chips": CHIPS.iter().map(|c| self.view.view(&[*c]).label(id!(label)).text()).collect::<Vec<_>>(),
            "rows": self.drawn,
        })
        .to_string()
    }
}

impl TestsPanelViewRef {
    pub fn set_view(&self, cx: &mut Cx, head: TestsHeadParts, rows: Vec<TestsRowParts>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, head, rows);
        }
    }

    pub fn action(&self, actions: &Actions) -> TestsPanelAction {
        self.borrow_mut()
            .map_or(TestsPanelAction::None, |mut inner| inner.action(actions))
    }

    pub fn rows_json(&self) -> String {
        self.borrow()
            .map_or_else(|| "{}".into(), |inner| inner.rows_json())
    }
}

/// The Test output view: the picked test's name, place and output, its
/// assert's place a link, `left` red and `right` green.
#[derive(Live, LiveHook, Widget)]
pub struct TestOutputView {
    #[deref]
    view: View,
    #[rust]
    parts: Option<TestOutputParts>,
}

impl Widget for TestOutputView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let list = self.view.portal_list(id!(lines));
        if list
            .items_with_actions(&actions)
            .iter()
            .any(|(_, item)| item.link_label(id!(link)).clicked(&actions))
        {
            cx.widget_action(self.widget_uid(), &scope.path, TestsPanelAction::OpenSpot);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let (lines, spot) = self
            .parts
            .as_ref()
            .map(|p| (p.3.clone(), p.4.clone()))
            .unwrap_or_default();
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            let list = step.as_portal_list();
            let Some(mut list) = list.borrow_mut() else {
                continue;
            };
            list.set_item_range(cx, 0, lines.len());
            while let Some(i) = list.next_visible_item(cx) {
                let Some(line) = lines.get(i) else { continue };
                let item = list.item(cx, i, live_id!(OutputLine));
                let (before, link, after) = match (!spot.is_empty())
                    .then(|| line.split_once(spot.as_str()))
                    .flatten()
                {
                    Some((b, a)) => (b, spot.as_str(), a),
                    None => (line.as_str(), "", ""),
                };
                let color = match line.trim_start() {
                    l if l.starts_with("left:") => vec4(0.949, 0.639, 0.627, 1.0),
                    l if l.starts_with("right:") => vec4(0.624, 0.824, 0.549, 1.0),
                    _ => vec4(0.718, 0.718, 0.729, 1.0),
                };
                item.label(id!(before)).set_text(cx, before);
                item.label(id!(before))
                    .apply_over(cx, live! { draw_text: { color: (color) } });
                item.link_label(id!(link)).set_text(cx, link);
                item.label(id!(after)).set_text(cx, after);
                item.draw_all(cx, scope);
            }
        }
        DrawStep::done()
    }
}

impl TestOutputView {
    /// Shows a test's output, or nothing.
    pub fn set_output(&mut self, cx: &mut Cx, parts: Option<TestOutputParts>) {
        let (mark, name, place) = parts
            .as_ref()
            .map(|p| (p.0, p.1.clone(), p.2.clone()))
            .unwrap_or((3, String::new(), String::new()));
        self.view
            .view(id!(head.mark))
            .apply_over(cx, live! { draw_bg: { mark: (mark as f64) } });
        let color = if mark == 0 {
            vec4(0.886, 0.447, 0.420, 1.0)
        } else {
            name_color(mark)
        };
        self.view.label(id!(head.name)).set_text(cx, &name);
        self.view
            .label(id!(head.name))
            .apply_over(cx, live! { draw_text: { color: (color) } });
        self.view.label(id!(head.place)).set_text(cx, &place);
        self.parts = parts;
        self.view.portal_list(id!(lines)).redraw(cx);
        self.redraw(cx);
    }
}

impl TestOutputViewRef {
    pub fn set_output(&self, cx: &mut Cx, parts: Option<TestOutputParts>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_output(cx, parts);
        }
    }

    /// Whether its place link was clicked.
    pub fn spot_clicked(&self, actions: &Actions) -> bool {
        let uid = self.widget_uid();
        actions.iter().any(|a| {
            a.downcast_ref::<WidgetAction>().is_some_and(|w| {
                w.widget_uid == uid
                    && w.action.downcast_ref::<TestsPanelAction>()
                        == Some(&TestsPanelAction::OpenSpot)
            })
        })
    }
}
