//! The Notes side panel view widget.

use crate::note_icon::*;
use makepad_widgets::*;

/// A row as the Notes panel view paints it:
/// (is_group, kind 0 doc / 1 note / 2 comment, title, badge text ("" = none), badge is warn,
///  right text (the line, "—", or a group's count), faded, struck, collapsed group).
pub type NotesRowParts = (bool, u8, String, String, bool, String, bool, bool, bool);

/// The panel's head as the view paints it: the four chip labels (All, Documentation, Notes, Comments),
/// the active chip index, the file choices ("All files" first, then `files()`), the selected choice index,
/// the alert label, whether "In alert" is on, and the footer.
pub type NotesHeadParts = ([String; 4], usize, Vec<String>, usize, String, bool, String);

/// User action produced by the Notes panel view during a frame.
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum NotesPanelAction {
    /// No action performed.
    None,
    /// Kind filter chip selected (0 All, 1 Doc, 2 Notes, 3 Comments).
    Chip(usize),
    /// File dropdown choice selected (0 All files, or file index + 1).
    File(usize),
    /// "In alert" toggle clicked.
    Alert,
    /// Group header row clicked to collapse/expand.
    Group(usize),
    /// Note row clicked once to open file at line.
    Open(usize),
    /// Note row double-clicked (< 400ms) to open note dialog.
    OpenDialog(usize),
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;
    use crate::note_icon::*;

    // A whole-area click target over a list row.
    NoteHit = <Button> {
        width: Fill, height: Fill,
        margin: 0.0, padding: 0.0,
        draw_bg: {
            border_size: 0.0,
            color: #00000000,
            color_hover: #00000000,
            color_down: #00000000,
        }
        text: ""
    }

    NoteText = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
            color: #D7DAE0
        }
        text: ""
    }

    NoteMuted = <NoteText> {
        draw_text: { color: #9BA1AB }
    }

    // A filter pill: the kind's icon (none on `All`) and its label.
    NoteChip = <RoundedView> {
        width: Fit, height: 24.0,
        flow: Overlay,
        draw_bg: {
            border_radius: 5.5,
            border_size: 1.0,
            border_color: #3A404B,
            color: #00000000,
        }
        cursor: Hand,
        face = <View> {
            width: Fit, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 10.0 },
            spacing: 4.0,
            icon = <NoteKindIcon> {
                width: 14.0, height: 14.0,
            }
            label = <NoteText> {}
        }
    }

    NoteListRow = <View> {
        width: Fill, height: 26.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: { color: #00000000 }
        btn = <NoteHit> {
            draw_bg: {
                color_hover: #262A31,
                color_down: #262A31,
            }
        }
    }

    NoteGroup = <NoteListRow> {
        face = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0 },
            spacing: 6.0,
            // Drawn, not a glyph: the theme font has no ▾/▸.
            chevron = <View> {
                width: 10.0, height: 10.0,
                show_bg: true,
                draw_bg: {
                    instance open: 1.0
                    color: #9BA1AB
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;
                        if self.open > 0.5 {
                            sdf.move_to(c.x - 3.0, c.y - 1.5);
                            sdf.line_to(c.x, c.y + 1.5);
                            sdf.line_to(c.x + 3.0, c.y - 1.5);
                        } else {
                            sdf.move_to(c.x - 1.5, c.y - 3.0);
                            sdf.line_to(c.x + 1.5, c.y);
                            sdf.line_to(c.x - 1.5, c.y + 3.0);
                        }
                        sdf.stroke(self.color, 1.2);
                        return sdf.result;
                    }
                }
            }
            title = <NoteText> {}
            spacer = <View> { width: Fill, height: 1.0 }
            count = <NoteMuted> {}
        }
    }

    NoteRow = <NoteListRow> {
        face = <View> {
            width: Fill, height: Fill,
            flow: Right, align: { y: 0.5 },
            padding: { left: 22.0, right: 8.0 },
            spacing: 6.0,
            icon = <NoteKindIcon> {
                width: 16.0, height: 16.0,
            }
            title_box = <View> {
                width: Fit, height: Fit,
                flow: Overlay, align: { y: 0.5 },
                title = <NoteText> {
                    draw_text: { text_style: { font_size: 9.5 } }
                }
                strike = <View> {
                    width: Fill, height: 1.0,
                    align: { y: 0.5 },
                    show_bg: true,
                    draw_bg: { color: #00000000 }
                }
            }
            badge = <RoundedView> {
                width: Fit, height: 18.0,
                padding: { left: 6.0, right: 6.0 },
                align: { y: 0.5 },
                draw_bg: {
                    border_radius: 4.5,
                    color: #00000000,
                }
                badge_label = <NoteText> {
                    draw_text: {
                        text_style: { font_size: 8.0 }
                        color: #00000000
                    }
                }
            }
            spacer = <View> { width: Fill, height: 1.0 }
            line_num = <NoteMuted> {
                draw_text: { text_style: <THEME_FONT_CODE> { font_size: 9.0 } }
            }
        }
    }

    pub NotesPanelView = {{NotesPanelView}} {
        width: Fill, height: Fill,
        flow: Down,

        // Chips row
        chips_row = <View> {
            width: Fill, height: Fit,
            flow: RightWrap, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0, top: 8.0, bottom: 4.0 },
            spacing: 6.0,

            chip_0 = <NoteChip> {
                face = {
                    padding: { left: 10.0 },
                    spacing: 0.0,
                    icon = { width: 0.0 }
                }
            }
            chip_1 = <NoteChip> {
                face = { icon = { kind: 0 } }
            }
            chip_2 = <NoteChip> {
                face = { icon = { kind: 1 } }
            }
            chip_3 = <NoteChip> {
                face = { icon = { kind: 2 } }
            }
        }

        // Second row: File dropdown and In alert toggle
        second_row = <View> {
            width: Fill, height: Fit,
            flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 6.0 },
            spacing: 8.0,

            file_drop = <DropDown> {
                width: Fill, height: 26.0,
            }

            alert_toggle = <RoundedView> {
                width: Fit, height: 26.0,
                flow: Overlay,
                draw_bg: {
                    border_radius: 6.0,
                    border_size: 1.0,
                    border_color: #F0883E,
                    color: #00000000,
                }
                cursor: Hand,
                face = <View> {
                    width: Fit, height: Fill,
                    flow: Right, align: { y: 0.5 },
                    padding: { left: 8.0, right: 8.0 },
                    spacing: 5.0,
                    dot = <RoundedView> {
                        width: 6.0, height: 6.0,
                        draw_bg: {
                            color: #F0883E,
                            border_radius: 1.5,
                        }
                    }
                    label = <NoteText> {
                        draw_text: { color: #F0883E }
                    }
                }
            }
        }

        // The list
        list = <PortalList> {
            width: Fill, height: Fill,
            scroll_bar: <ScrollBar> {}
            NoteGroup = <NoteGroup> {}
            NoteRow = <NoteRow> {}
        }

        // Footer
        footer_divider = <View> {
            width: Fill, height: 1.0,
            show_bg: true,
            draw_bg: { color: #2C313A }
        }
        footer_view = <View> {
            width: Fill, height: 26.0,
            flow: Right, align: { y: 0.5 },
            padding: { left: 8.0, right: 8.0 },
            footer_label = <NoteMuted> { width: Fill }
        }
    }
}

/// The four filter pills, in the model's chip order.
const CHIPS: [LiveId; 4] = [
    live_id!(chip_0),
    live_id!(chip_1),
    live_id!(chip_2),
    live_id!(chip_3),
];

/// The last click on a row: its index and when.
type LastClick = Option<(usize, std::time::Instant)>;

/// A drawn element as the MCP read reports it.
type Json = serde_json::Value;

/// A composite Makepad widget for the project Notes side panel.
#[derive(Live, LiveHook, Widget)]
pub struct NotesPanelView {
    #[deref]
    view: View,
    #[rust]
    head: Option<NotesHeadParts>,
    #[rust]
    rows: Vec<NotesRowParts>,
    #[rust]
    last_click: LastClick,
    #[rust]
    drawn_rows: Vec<Json>,
    #[rust]
    chips_info: Vec<Json>,
    #[rust]
    file_info: Json,
    #[rust]
    alert_info: Json,
    #[rust]
    footer_info: Json,
    #[rust]
    row_rects: Vec<(usize, Rect)>,
}

impl Widget for NotesPanelView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let act = self.action(&actions);
        if act != NotesPanelAction::None {
            cx.widget_action(self.widget_uid(), &scope.path, act);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.drawn_rows.clear();
        self.row_rects.clear();
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.rows.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(row_data) = self.rows.get(i) else {
                        continue;
                    };
                    let (
                        is_group,
                        kind,
                        title,
                        badge_text,
                        badge_is_warn,
                        right_text,
                        faded,
                        struck,
                        collapsed,
                    ) = row_data;
                    let template = if *is_group {
                        live_id!(NoteGroup)
                    } else {
                        live_id!(NoteRow)
                    };
                    let item = list.item(cx, i, template);
                    if *is_group {
                        let open = if *collapsed { 0.0 } else { 1.0 };
                        item.view(id!(face.chevron))
                            .apply_over(cx, live! { draw_bg: { open: (open) } });
                        item.label(id!(face.title)).set_text(cx, title);
                        item.label(id!(face.count)).set_text(cx, right_text);
                    } else {
                        item.note_kind_icon(id!(face.icon))
                            .set_kind(cx, *kind as i64);
                        let icon_alpha = if *faded { 0.55 } else { 1.0 };
                        item.note_kind_icon(id!(face.icon))
                            .set_alpha(cx, icon_alpha);

                        let title_color = if *faded {
                            vec4(0.431, 0.459, 0.506, 1.0)
                        } else {
                            vec4(0.843, 0.855, 0.878, 1.0)
                        };
                        item.label(id!(face.title_box.title)).set_text(cx, title);
                        item.label(id!(face.title_box.title)).apply_over(
                            cx,
                            live! {
                                draw_text: { color: (title_color) }
                            },
                        );

                        let strike_color = if *struck {
                            vec4(0.431, 0.459, 0.506, 1.0)
                        } else {
                            vec4(0.0, 0.0, 0.0, 0.0)
                        };
                        item.view(id!(face.title_box.strike)).apply_over(
                            cx,
                            live! {
                                draw_bg: { color: (strike_color) }
                            },
                        );

                        let badge_view = item.view(id!(face.badge));
                        if badge_text.is_empty() {
                            badge_view.set_visible(cx, false);
                            badge_view.label(id!(badge_label)).set_text(cx, "");
                        } else {
                            badge_view.set_visible(cx, true);
                            badge_view.label(id!(badge_label)).set_text(cx, badge_text);
                            let (badge_bg, badge_fg) = if *badge_is_warn {
                                (
                                    vec4(0.941, 0.533, 0.243, 0.16),
                                    vec4(0.941, 0.533, 0.243, 1.0),
                                )
                            } else {
                                (
                                    vec4(0.608, 0.631, 0.671, 0.14),
                                    vec4(0.608, 0.631, 0.671, 1.0),
                                )
                            };
                            badge_view.apply_over(
                                cx,
                                live! {
                                    draw_bg: { color: (badge_bg) }
                                },
                            );
                            badge_view.label(id!(badge_label)).apply_over(
                                cx,
                                live! {
                                    draw_text: { color: (badge_fg) }
                                },
                            );
                        }

                        item.label(id!(face.line_num)).set_text(cx, right_text);
                    }
                    item.draw_all(cx, scope);

                    let r = item.area().rect(cx);
                    if r.size.x > 0.0 && r.size.y > 0.0 {
                        self.row_rects.push((i, r));
                    }
                    self.drawn_rows.push(serde_json::json!({
                        "text": title,
                        "kind": kind,
                        "badge": badge_text,
                        "right": right_text,
                        "group": is_group,
                        "faded": faded,
                        "rect": [r.pos.x, r.pos.y, r.size.x, r.size.y],
                    }));
                }
            }
        }

        self.chips_info.clear();
        for chip_id in CHIPS {
            let chip_view = self.view.view(&[chip_id]);
            let label_text = chip_view.label(id!(face.label)).text();
            let r = chip_view.area().rect(cx);
            self.chips_info.push(serde_json::json!({
                "text": label_text,
                "rect": [r.pos.x, r.pos.y, r.size.x, r.size.y],
            }));
        }

        let file_r = self.view.view(id!(file_drop)).area().rect(cx);
        let selected_file_text = self
            .head
            .as_ref()
            .and_then(|h| h.2.get(h.3).cloned())
            .unwrap_or_else(|| "All files".to_string());
        self.file_info = serde_json::json!({
            "text": selected_file_text,
            "rect": [file_r.pos.x, file_r.pos.y, file_r.size.x, file_r.size.y],
        });

        let alert_r = self.view.view(id!(alert_toggle)).area().rect(cx);
        let alert_text = self.head.as_ref().map(|h| h.4.clone()).unwrap_or_default();
        let alert_on = self.head.as_ref().map(|h| h.5).unwrap_or(false);
        self.alert_info = serde_json::json!({
            "text": alert_text,
            "active": alert_on,
            "rect": [alert_r.pos.x, alert_r.pos.y, alert_r.size.x, alert_r.size.y],
        });

        let footer_r = self.view.view(id!(footer_view)).area().rect(cx);
        let footer_text = self.view.label(id!(footer_view.footer_label)).text();
        self.footer_info = serde_json::json!({
            "text": footer_text,
            "rect": [footer_r.pos.x, footer_r.pos.y, footer_r.size.x, footer_r.size.y],
        });

        DrawStep::done()
    }
}

impl NotesPanelView {
    /// Updates the view with latest head and rows model data.
    pub fn set_view(&mut self, cx: &mut Cx, head: NotesHeadParts, rows: Vec<NotesRowParts>) {
        for (i, chip_id) in CHIPS.into_iter().enumerate() {
            let chip = self.view.view(&[chip_id]);
            chip.label(id!(face.label)).set_text(cx, &head.0[i]);

            let is_active = head.1 == i;
            let (bg, border, text_col) = if is_active {
                (
                    vec4(0.192, 0.220, 0.267, 1.0),
                    vec4(0.290, 0.322, 0.384, 1.0),
                    vec4(0.949, 0.953, 0.961, 1.0),
                )
            } else {
                (
                    vec4(0.0, 0.0, 0.0, 0.0),
                    vec4(0.227, 0.251, 0.294, 1.0),
                    vec4(0.843, 0.855, 0.878, 1.0),
                )
            };
            chip.apply_over(
                cx,
                live! {
                    draw_bg: {
                        color: (bg),
                        border_color: (border)
                    }
                },
            );
            chip.label(id!(face.label)).apply_over(
                cx,
                live! {
                    draw_text: { color: (text_col) }
                },
            );
        }

        let drop = self.view.drop_down(id!(file_drop));
        drop.set_labels(cx, head.2.clone());
        drop.set_selected_item(cx, head.3);

        let alert_toggle = self.view.view(id!(alert_toggle));
        alert_toggle.label(id!(face.label)).set_text(cx, &head.4);
        let alert_bg = if head.5 {
            vec4(0.941, 0.533, 0.243, 0.16)
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        alert_toggle.apply_over(
            cx,
            live! {
                draw_bg: { color: (alert_bg) }
            },
        );

        self.view
            .label(id!(footer_view.footer_label))
            .set_text(cx, &head.6);

        self.head = Some(head);
        self.rows = rows;
        self.view.portal_list(id!(list)).redraw(cx);
        self.redraw(cx);
    }

    /// Evaluates user actions against the view and subwidgets.
    pub fn action(&mut self, actions: &Actions) -> NotesPanelAction {
        for a in actions {
            if let Some(w) = a.downcast_ref::<WidgetAction>()
                && w.widget_uid == self.widget_uid()
                && let Some(act) = w.action.downcast_ref::<NotesPanelAction>()
            {
                return act.clone();
            }
        }

        for (i, chip_id) in CHIPS.into_iter().enumerate() {
            let chip = self.view.view(&[chip_id]);
            if chip.finger_up(actions).is_some_and(|f| f.is_over) {
                return NotesPanelAction::Chip(i);
            }
        }

        let alert = self.view.view(id!(alert_toggle));
        if alert.finger_up(actions).is_some_and(|f| f.is_over) {
            return NotesPanelAction::Alert;
        }

        if let Some(choice) = self.view.drop_down(id!(file_drop)).selected(actions) {
            return NotesPanelAction::File(choice);
        }

        let list = self.view.portal_list(id!(list));
        if list.any_items_with_actions(actions) {
            for (i, item) in list.items_with_actions(actions) {
                if !item.button(id!(btn)).clicked(actions) {
                    continue;
                }
                let Some(row) = self.rows.get(i) else {
                    continue;
                };
                if row.0 {
                    return NotesPanelAction::Group(i);
                }
                let now = std::time::Instant::now();
                if let Some((last_i, last_time)) = self.last_click
                    && last_i == i
                    && now.duration_since(last_time).as_millis() <= 400
                {
                    self.last_click = None;
                    return NotesPanelAction::OpenDialog(i);
                }
                self.last_click = Some((i, now));
                return NotesPanelAction::Open(i);
            }
        }

        NotesPanelAction::None
    }

    /// Where each visible row was drawn during the last frame.
    pub fn captured_rects(&self) -> Vec<(usize, Rect)> {
        self.row_rects.clone()
    }

    /// Number of total rows currently loaded in the panel.
    pub fn rows_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns drawn row geometry and header state as a serialized JSON string for MCP inspect.
    pub fn rows_json(&self) -> String {
        self.rows_json_with_rects(&[])
    }

    /// Serializes every row with its on-screen rectangle (null when not drawn or off-screen).
    pub fn rows_json_with_rects(&self, rects: &[(usize, [f64; 4])]) -> String {
        let rows: Vec<serde_json::Value> = self
            .rows
            .iter()
            .enumerate()
            .map(
                |(
                    i,
                    (
                        is_group,
                        kind,
                        title,
                        badge_text,
                        _badge_is_warn,
                        right_text,
                        faded,
                        _struck,
                        _collapsed,
                    ),
                )| {
                    let rect = rects.iter().find(|(n, _)| *n == i).map(|(_, r)| *r);
                    serde_json::json!({
                        "text": title,
                        "kind": kind,
                        "badge": badge_text,
                        "right": right_text,
                        "group": is_group,
                        "faded": faded,
                        "rect": rect,
                    })
                },
            )
            .collect();
        serde_json::json!({
            "chips": self.chips_info,
            "file": self.file_info,
            "alert": self.alert_info,
            "footer": self.footer_info,
            "rows": rows,
        })
        .to_string()
    }
}

impl NotesPanelViewRef {
    /// Sets view model data onto the underlying widget.
    pub fn set_view(&self, cx: &mut Cx, head: NotesHeadParts, rows: Vec<NotesRowParts>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, head, rows);
        }
    }

    /// Where each visible row was drawn during the last frame.
    pub fn captured_rects(&self) -> Vec<(usize, Rect)> {
        self.borrow()
            .map(|inner| inner.captured_rects())
            .unwrap_or_default()
    }

    /// Returns the internal portal list widget for direct geometry inspection.
    pub fn portal_list(&self) -> PortalListRef {
        if let Some(inner) = self.borrow() {
            inner.view.portal_list(id!(list))
        } else {
            PortalListRef::default()
        }
    }

    /// Number of total rows currently loaded in the panel.
    pub fn rows_count(&self) -> usize {
        self.borrow().map(|inner| inner.rows_count()).unwrap_or(0)
    }

    /// Reads back whatever action occurred during this frame.
    pub fn action(&self, actions: &Actions) -> NotesPanelAction {
        // The view captures its buttons' actions and re-emits one of its own (see
        // `handle_event`): a mouse click only ever reaches the caller as that action.
        let own = actions
            .find_widget_action(self.widget_uid())
            .cast::<NotesPanelAction>();
        if own != NotesPanelAction::None {
            return own;
        }
        if let Some(mut inner) = self.borrow_mut() {
            inner.action(actions)
        } else {
            NotesPanelAction::None
        }
    }

    /// Serializes rows and header state for MCP inspect tool.
    pub fn rows_json(&self) -> String {
        self.rows_json_with_rects(&[])
    }

    /// Serializes rows with measured rectangle geometry.
    pub fn rows_json_with_rects(&self, rects: &[(usize, [f64; 4])]) -> String {
        if let Some(inner) = self.borrow() {
            inner.rows_json_with_rects(rects)
        } else {
            "{}".to_string()
        }
    }
}
