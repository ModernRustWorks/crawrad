//! Composite Local History panel widget.
//!
//! Provides the complete history view with header mode buttons (Information,
//! Contents, Differences), copies list with badges, side-by-side diff
//! integration, block navigation, and copy inspection.

use makepad_widgets::*;

use crate::diff_view::{CrawradDiffViewWidgetRefExt, DiffViewRow};

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;
    use crate::borderless::*;
    use crate::diff_view::*;

    InfoRow = <View> {
        show_bg: true,
        draw_bg: { color: #1d1f20 }
        width: Fill, height: 26.0,
        flow: Overlay,
        btn = <CrawradBorderlessButton> {
            width: Fill, height: Fill,
            draw_bg: {
                color: #0000, color_hover: #ffffff10, color_down: #ffffff18, color_focus: #0000,
            }
            draw_text: { text_style: { font_size: 0.0 } }
            text: ""
        }
        cols = <View> {
            width: Fill, height: Fill,
            flow: Right,
            align: { y: 0.5 },
            c_saved = <Label> { width: 120.0, height: Fit, padding: { left: 8.0 }, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
            c_when = <Label> { width: 100.0, height: Fit, draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
            c_origin = <View> {
                width: 80.0, height: Fill,
                align: { y: 0.5 },
                badge = <View> {
                    width: Fit, height: Fit,
                    padding: { left: 6.0, right: 6.0, top: 2.0, bottom: 2.0 },
                    show_bg: true,
                    draw_bg: { color: #3a2a24 }
                    lbl_badge = <Label> { draw_text: { color: #ff9c6a, text_style: <THEME_FONT_BOLD> { font_size: 8.0 } }, text: "" }
                }
            }
            c_size = <Label> { width: 90.0, height: Fit, draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
            c_lines = <Label> { width: 80.0, height: Fit, draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
        }
    }

    CopyRow = <View> {
        width: Fill, height: 54.0,
        flow: Overlay,
        show_bg: true,
        draw_bg: { color: #1d1f20 }
        btn = <CrawradBorderlessButton> {
            width: Fill, height: Fill,
            draw_bg: {
                color: #0000, color_hover: #ffffff10, color_down: #ffffff18, color_focus: #0000,
            }
            draw_text: { text_style: { font_size: 0.0 } }
            text: ""
        }
        accent_bar = <View> {
            visible: false,
            width: 3.0, height: Fill,
            show_bg: true,
            draw_bg: { color: #de5a2b }
        }
        info_box = <View> {
            width: Fill, height: Fill,
            flow: Down,
            padding: { left: 12.0, right: 8.0, top: 4.0, bottom: 4.0 },
            spacing: 2.0,
            row_top = <View> {
                width: Fill, height: Fit,
                flow: Right,
                align: { y: 0.5 },
                lbl_time = <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        color: #e7e7ea,
                        text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                    }
                    text: ""
                }
                badge = <View> {
                    width: Fit, height: Fit,
                    padding: { left: 5.0, right: 5.0, top: 1.0, bottom: 1.0 },
                    show_bg: true,
                    draw_bg: { color: #3a2a24 }
                    lbl_badge = <Label> {
                        draw_text: {
                            color: #ff9c6a,
                            text_style: <THEME_FONT_BOLD> { font_size: 8.0 }
                        }
                        text: ""
                    }
                }
            }
            lbl_meta = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #98989b,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                }
                text: ""
            }
        }
    }

    pub CrawradHistoryPanelBase = {{CrawradHistoryPanel}} {}

    pub CrawradHistoryPanel = <CrawradHistoryPanelBase> {
        width: Fill, height: Fill,
        flow: Down,

        header = <CrawradTopRounded> {
            width: Fill, height: 38.0,
            flow: Right,
            align: { y: 0.5 },
            padding: { left: 16.0, right: 16.0 },
            draw_bg: { color: #161819 }

            lbl_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #e7e7ea,
                    text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                }
                text: "History"
            }

            mode_buttons = <View> {
                width: Fit, height: Fit,
                flow: Right,
                spacing: 4.0,

                btn_mode_info = <Button> {
                    width: Fit, height: 26.0,
                    padding: { left: 10.0, right: 10.0 },
                    text: "Information"
                }
                btn_mode_contents = <Button> {
                    width: Fit, height: 26.0,
                    padding: { left: 10.0, right: 10.0 },
                    text: "Contents"
                }
                btn_mode_diff = <Button> {
                    width: Fit, height: 26.0,
                    padding: { left: 10.0, right: 10.0 },
                    text: "Differences"
                }
            }
        }

        body_split = <View> {
            show_bg: true,
            draw_bg: { color: #1d1f20 }
            width: Fill, height: Fill,
            flow: Right,

            left_pane = <View> {
                width: 240.0, height: Fill,
                flow: Down,
                show_bg: true,
                draw_bg: { color: #1d1f20 }

                copies_list = <PortalList> {
                    width: Fill, height: Fill,
                    row = <CopyRow> {}
                }
            }

            split_sep = <View> {
                width: 1.0, height: Fill,
                show_bg: true,
                draw_bg: { color: #424244 }
            }

            diff_container = <View> {
                visible: true,
                width: Fill, height: Fill,
                flow: Down,

                diff_view = <CrawradDiffView> {
                    width: Fill, height: Fill,
                }

                diff_bar = <View> {
                    width: Fill, height: 36.0,
                    flow: Right,
                    align: { y: 0.5 },
                    padding: { left: 12.0, right: 12.0 },
                    spacing: 8.0,
                    show_bg: true,
                    draw_bg: { color: #161819 }

                    lbl_changes = <Label> {
                        width: Fit, height: Fit,
                        draw_text: {
                            color: #e7e7ea,
                            text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                        }
                        text: "No changes"
                    }
                    <View> { width: 8.0, height: Fit }
                    btn_prev = <Button> { width: 90.0, height: 24.0, text: "◀ Previous" }
                    btn_next = <Button> { width: 80.0, height: 24.0, text: "Next ▶" }
                    btn_restore = <Button> { width: 130.0, height: 24.0, text: "Restore this block" }
                    <View> { width: 8.0, height: Fit }
                    legend = <View> {
                        width: Fit, height: Fit,
                        flow: Right,
                        align: { y: 0.5 },
                        spacing: 6.0,
                        <View> { width: 10.0, height: 10.0, show_bg: true, draw_bg: { color: #7cb46b } }
                        <Label> { draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "added" }
                        <View> { width: 6.0, height: Fit }
                        <View> { width: 10.0, height: 10.0, show_bg: true, draw_bg: { color: #e2726b } }
                        <Label> { draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "removed" }
                        <View> { width: 6.0, height: Fit }
                        <View> { width: 10.0, height: 10.0, show_bg: true, draw_bg: { color: #b3dcff } }
                        <Label> { draw_text: { color: #b7b7ba, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "changed" }
                    }
                    <View> { width: Fill, height: Fit }
                    btn_revert = <Button> {
                        width: 160.0, height: 26.0,
                        draw_bg: {
                            color: #de5a2b, color_hover: #f07b46, color_down: #b83d16,
                            border_color_1: #0000, border_color_2: #0000,
                        }
                        draw_text: { color: #f5f5f8, text_style: <THEME_FONT_BOLD> { font_size: 9.5 } }
                        text: "Revert to this version"
                    }
                }
            }

            contents_container = <View> {
                visible: false,
                width: Fill, height: Fill,
                flow: Down,

                contents_text = <TextInput> {
                    width: Fill, height: Fill,
                    is_read_only: true,
                    draw_text: {
                        color: #e7e7ea,
                        text_style: <THEME_FONT_CODE> { font_size: 9.5 }
                    }
                }

                contents_bar = <View> {
                    width: Fill, height: 36.0,
                    flow: Right,
                    align: { y: 0.5 },
                    padding: { left: 12.0, right: 12.0 },
                    show_bg: true,
                    draw_bg: { color: #161819 }

                    btn_compare = <Button> { width: 140.0, height: 24.0, text: "Compare with buffer" }
                    <View> { width: Fill, height: Fit }
                    btn_revert_contents = <Button> {
                        width: 160.0, height: 26.0,
                        draw_bg: {
                            color: #de5a2b, color_hover: #f07b46, color_down: #b83d16,
                            border_color_1: #0000, border_color_2: #0000,
                        }
                        draw_text: { color: #f5f5f8, text_style: <THEME_FONT_BOLD> { font_size: 9.5 } }
                        text: "Revert to this version"
                    }
                }
            }
        }

        info_container = <View> {
            show_bg: true,
            draw_bg: { color: #1d1f20 }
            visible: false,
            width: Fill, height: Fill,
            flow: Down,

            table_header = <View> {
                width: Fill, height: 26.0,
                flow: Right,
                align: { y: 0.5 },
                show_bg: true,
                draw_bg: { color: #232628 }
                th_saved = <Label> { width: 120.0, height: Fit, padding: { left: 8.0 }, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "Saved" }
                th_when = <Label> { width: 100.0, height: Fit, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "When" }
                th_origin = <Label> { width: 80.0, height: Fit, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "Origin" }
                th_size = <Label> { width: 90.0, height: Fit, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "Size" }
                th_lines = <Label> { width: 80.0, height: Fit, draw_text: { color: #e7e7ea, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "Lines" }
            }

            info_list = <PortalList> {
                width: Fill, height: Fill,
                row = <InfoRow> {}
            }

            lbl_note = <Label> {
                width: Fill, height: 26.0,
                padding: { left: 12.0 },
                draw_text: { color: #98989b, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } },
                text: ""
            }
        }
    }
}

#[crawrad_note::note("history-panel-historymode")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HistoryMode {
    Information,
    Contents,
    #[default]
    Differences,
}

impl HistoryMode {
    #[crawrad_note::note("history-panel-historymode-as-str")]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Information => "information",
            Self::Contents => "contents",
            Self::Differences => "differences",
        }
    }
}

#[crawrad_note::note("history-panel-format-changes-text")]
pub fn format_changes_text(blocks: usize, current_block: Option<usize>) -> String {
    if blocks == 0 {
        return "No changes".to_string();
    }
    let i = current_block.map_or(1, |b| b + 1);
    let noun = if blocks == 1 { "change" } else { "changes" };
    format!("{blocks} {noun} · block {i} of {blocks}")
}

#[crawrad_note::note("history-panel-can-navigate-blocks")]
pub fn can_navigate_blocks(blocks: usize) -> bool {
    blocks > 0
}

#[crawrad_note::note("history-panel-origin-badge-colors")]
pub fn origin_badge_colors(origin: &str) -> ([f32; 4], [f32; 4]) {
    if origin.eq_ignore_ascii_case("external") {
        (
            [179.0 / 255.0, 220.0 / 255.0, 1.0, 1.0],
            [45.0 / 255.0, 74.0 / 255.0, 99.0 / 255.0, 1.0],
        )
    } else {
        (
            [1.0, 156.0 / 255.0, 106.0 / 255.0, 1.0],
            [58.0 / 255.0, 42.0 / 255.0, 36.0 / 255.0, 1.0],
        )
    }
}

#[crawrad_note::note("history-panel-historyentryview")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryEntryView {
    pub time: String,
    pub when: String,
    pub origin: String,
    pub size: String,
    pub lines: usize,
}

#[crawrad_note::note("history-panel-historypanelaction")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryPanelAction {
    Select(usize),
    Mode(HistoryMode),
    Previous,
    Next,
    RestoreBlock,
    Revert,
    Compare,
}

#[crawrad_note::note("history-panel-crawradhistorypanel")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradHistoryPanel {
    #[deref]
    pub view: View,

    #[rust]
    entries: Vec<HistoryEntryView>,

    #[rust]
    selected: Option<usize>,

    #[rust]
    mode: HistoryMode,

    #[rust]
    row_areas: Vec<Area>,

    /// The legend's width when it was last drawn, to know when it fits back.
    #[rust]
    legend_width: f64,

    /// The frame after the legend left or came back, to draw the bar again.
    #[rust]
    refit: NextFrame,
}

#[crawrad_note::note("history-panel-legend-gap")]
const LEGEND_GAP: f64 = 40.0;

impl Widget for CrawradHistoryPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.refit.is_event(event).is_some() {
            self.view.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let copies_list = self.view.portal_list(id!(copies_list));
        let info_list = self.view.portal_list(id!(info_list));

        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = copies_list.borrow_mut_if_eq(&step) {
                list.set_item_range(cx, 0, self.entries.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(entry) = self.entries.get(i) else {
                        continue;
                    };
                    let item = list.item(cx, i, live_id!(row));
                    let is_selected = self.selected == Some(i);
                    item.view(id!(accent_bar)).set_visible(cx, is_selected);
                    if is_selected {
                        item.apply_over(
                            cx,
                            live! { draw_bg: { color: (vec4(0.173, 0.271, 0.365, 1.0)) } },
                        );
                    } else {
                        item.apply_over(
                            cx,
                            live! { draw_bg: { color: (vec4(0.114, 0.122, 0.125, 1.0)) } },
                        );
                    }
                    item.label(id!(lbl_time)).set_text(cx, &entry.time);
                    let meta = format!("{} · {}", entry.when, entry.size);
                    item.label(id!(lbl_meta)).set_text(cx, &meta);
                    item.label(id!(lbl_badge)).set_text(cx, &entry.origin);
                    let (badge_text_col, badge_bg_col) = origin_badge_colors(&entry.origin);
                    item.view(id!(badge)).apply_over(cx, live! { draw_bg: { color: (vec4(badge_bg_col[0], badge_bg_col[1], badge_bg_col[2], badge_bg_col[3])) } });
                    item.label(id!(lbl_badge)).apply_over(cx, live! { draw_text: { color: (vec4(badge_text_col[0], badge_text_col[1], badge_text_col[2], badge_text_col[3])) } });
                    item.draw_all(cx, scope);
                    if self.row_areas.len() <= i {
                        self.row_areas.resize(i + 1, Area::Empty);
                    }
                    self.row_areas[i] = item.area();
                }
            } else if let Some(mut list) = info_list.borrow_mut_if_eq(&step) {
                list.set_item_range(cx, 0, self.entries.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(entry) = self.entries.get(i) else {
                        continue;
                    };
                    let item = list.item(cx, i, live_id!(row));
                    let is_selected = self.selected == Some(i);
                    if is_selected {
                        item.apply_over(
                            cx,
                            live! { draw_bg: { color: (vec4(0.173, 0.271, 0.365, 1.0)) } },
                        );
                    } else {
                        item.apply_over(
                            cx,
                            live! { draw_bg: { color: (vec4(0.114, 0.122, 0.125, 1.0)) } },
                        );
                    }
                    item.label(id!(c_saved)).set_text(cx, &entry.time);
                    item.label(id!(c_when)).set_text(cx, &entry.when);
                    item.label(id!(lbl_badge)).set_text(cx, &entry.origin);
                    let (badge_text_col, badge_bg_col) = origin_badge_colors(&entry.origin);
                    item.view(id!(c_origin.badge)).apply_over(cx, live! { draw_bg: { color: (vec4(badge_bg_col[0], badge_bg_col[1], badge_bg_col[2], badge_bg_col[3])) } });
                    item.label(id!(lbl_badge)).apply_over(cx, live! { draw_text: { color: (vec4(badge_text_col[0], badge_text_col[1], badge_text_col[2], badge_text_col[3])) } });
                    item.label(id!(c_size)).set_text(cx, &entry.size);
                    item.label(id!(c_lines))
                        .set_text(cx, &entry.lines.to_string());
                    item.draw_all(cx, scope);
                    if self.mode == HistoryMode::Information {
                        if self.row_areas.len() <= i {
                            self.row_areas.resize(i + 1, Area::Empty);
                        }
                        self.row_areas[i] = item.area();
                    }
                }
            }
        }
        self.fit_diff_bar(cx);
        DrawStep::done()
    }
}

impl CrawradHistoryPanel {
    #[crawrad_note::note("history-panel-crawradhistorypanel-fit-diff-bar")]
    fn fit_diff_bar(&mut self, cx: &mut Cx2d) {
        let bar = self.view.view(id!(diff_bar)).area().rect(cx);
        let revert = self.view.button(id!(btn_revert)).area().rect(cx);
        if bar.size.x <= 0.0 || revert.size.x <= 0.0 {
            return;
        }
        let restore = self.view.button(id!(btn_restore)).area().rect(cx);
        let legend = self.view.view(id!(legend));
        let show = if legend.visible() {
            self.legend_width = legend.area().rect(cx).size.x;
            revert.pos.x + revert.size.x <= bar.pos.x + bar.size.x
        } else {
            // The `Fill` spacer pins `Revert` to the right, so the room is
            // what lies between it and the last button on the left.
            let room = revert.pos.x - (restore.pos.x + restore.size.x);
            room >= self.legend_width + LEGEND_GAP
        };
        if show != legend.visible() {
            legend.set_visible(cx, show);
            self.refit = cx.new_next_frame();
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-title")]
    pub fn set_title(&mut self, cx: &mut Cx, title: &str) {
        self.view.label(id!(lbl_title)).set_text(cx, title);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-entries")]
    pub fn set_entries(&mut self, cx: &mut Cx, entries: &[HistoryEntryView]) {
        self.entries = entries.to_vec();
        self.row_areas.clear();
        self.row_areas.resize(entries.len(), Area::Empty);
        self.view.redraw(cx);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-selected")]
    pub fn set_selected(&mut self, cx: &mut Cx, selected: Option<usize>) {
        self.selected = selected;
        self.view.portal_list(id!(copies_list)).redraw(cx);
        self.view.portal_list(id!(info_list)).redraw(cx);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-mode")]
    pub fn set_mode(&mut self, cx: &mut Cx, mode: HistoryMode) {
        self.mode = mode;
        let is_diff = mode == HistoryMode::Differences;
        let is_info = mode == HistoryMode::Information;
        let is_contents = mode == HistoryMode::Contents;

        self.view.view(id!(body_split)).set_visible(cx, !is_info);
        self.view.view(id!(info_container)).set_visible(cx, is_info);

        if !is_info {
            self.view.view(id!(diff_container)).set_visible(cx, is_diff);
            self.view
                .view(id!(contents_container))
                .set_visible(cx, is_contents);
        }

        for (btn_id, active) in [
            (id!(btn_mode_info), is_info),
            (id!(btn_mode_contents), is_contents),
            (id!(btn_mode_diff), is_diff),
        ] {
            let btn = self.view.button(btn_id);
            if active {
                btn.apply_over(cx, live! { draw_bg: { color: (vec4(0.251, 0.259, 0.271, 1.0)), color_hover: (vec4(0.290, 0.298, 0.310, 1.0)), color_down: (vec4(0.290, 0.298, 0.310, 1.0)) } });
            } else {
                btn.apply_over(cx, live! { draw_bg: { color: (vec4(0.169, 0.169, 0.176, 1.0)), color_hover: (vec4(0.208, 0.216, 0.227, 1.0)), color_down: (vec4(0.208, 0.216, 0.227, 1.0)) } });
            }
        }
        self.view.redraw(cx);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-diff")]
    pub fn set_diff(
        &mut self,
        cx: &mut Cx,
        diff: &[DiffViewRow],
        blocks: usize,
        current_block: Option<usize>,
    ) {
        let enabled = can_navigate_blocks(blocks);
        for btn_id in [id!(btn_prev), id!(btn_next), id!(btn_restore)] {
            let btn = self.view.button(btn_id);
            btn.as_button().set_enabled(cx, enabled);
            btn.clone().set_disabled(cx, !enabled);
        }
        let text = format_changes_text(blocks, current_block);
        self.view.label(id!(lbl_changes)).set_text(cx, &text);
        self.view
            .widget(id!(diff_view))
            .as_crawrad_diff_view()
            .set_rows(cx, diff);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-reveal-row")]
    pub fn reveal_row(&mut self, cx: &mut Cx, row: usize) {
        self.view
            .widget(id!(diff_view))
            .as_crawrad_diff_view()
            .reveal_row(cx, row);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-contents")]
    pub fn set_contents(&mut self, cx: &mut Cx, contents: &str) {
        self.view
            .text_input(id!(contents_text))
            .set_text(cx, contents);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-set-note")]
    pub fn set_note(&mut self, cx: &mut Cx, note: &str) {
        self.view.label(id!(lbl_note)).set_text(cx, note);
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-drawn-controls")]
    pub fn drawn_controls(&self, cx: &Cx) -> Vec<(String, Rect)> {
        let mut controls = Vec::new();

        let btn_info = self.view.button(id!(btn_mode_info));
        controls.push((
            "history_mode_information".to_string(),
            btn_info.area().rect(cx),
        ));

        let btn_contents = self.view.button(id!(btn_mode_contents));
        controls.push((
            "history_mode_contents".to_string(),
            btn_contents.area().rect(cx),
        ));

        let btn_diff = self.view.button(id!(btn_mode_diff));
        controls.push((
            "history_mode_differences".to_string(),
            btn_diff.area().rect(cx),
        ));

        let btn_prev = self.view.button(id!(btn_prev));
        controls.push(("history_prev".to_string(), btn_prev.area().rect(cx)));

        let btn_next = self.view.button(id!(btn_next));
        controls.push(("history_next".to_string(), btn_next.area().rect(cx)));

        let btn_restore = self.view.button(id!(btn_restore));
        controls.push(("history_restore".to_string(), btn_restore.area().rect(cx)));

        let btn_revert = if self.mode == HistoryMode::Contents {
            self.view.button(id!(btn_revert_contents))
        } else {
            self.view.button(id!(btn_revert))
        };
        controls.push(("history_revert".to_string(), btn_revert.area().rect(cx)));

        let btn_compare = self.view.button(id!(btn_compare));
        controls.push(("history_compare".to_string(), btn_compare.area().rect(cx)));

        for (i, area) in self.row_areas.iter().enumerate() {
            controls.push((format!("history_row_{i}"), area.rect(cx)));
        }

        controls
    }

    #[crawrad_note::note("history-panel-crawradhistorypanel-clicked")]
    pub fn clicked(&self, actions: &Actions) -> Option<HistoryPanelAction> {
        if self.view.button(id!(btn_mode_info)).clicked(actions) {
            return Some(HistoryPanelAction::Mode(HistoryMode::Information));
        }
        if self.view.button(id!(btn_mode_contents)).clicked(actions) {
            return Some(HistoryPanelAction::Mode(HistoryMode::Contents));
        }
        if self.view.button(id!(btn_mode_diff)).clicked(actions) {
            return Some(HistoryPanelAction::Mode(HistoryMode::Differences));
        }
        if self.view.button(id!(btn_prev)).clicked(actions) {
            return Some(HistoryPanelAction::Previous);
        }
        if self.view.button(id!(btn_next)).clicked(actions) {
            return Some(HistoryPanelAction::Next);
        }
        if self.view.button(id!(btn_restore)).clicked(actions) {
            return Some(HistoryPanelAction::RestoreBlock);
        }
        if self.view.button(id!(btn_revert)).clicked(actions)
            || self.view.button(id!(btn_revert_contents)).clicked(actions)
        {
            return Some(HistoryPanelAction::Revert);
        }
        if self.view.button(id!(btn_compare)).clicked(actions) {
            return Some(HistoryPanelAction::Compare);
        }

        let copies = self.view.portal_list(id!(copies_list));
        for (i, item) in copies.items_with_actions(actions) {
            if item.button(id!(btn)).clicked(actions) {
                return Some(HistoryPanelAction::Select(i));
            }
        }

        if self.mode == HistoryMode::Information {
            let info = self.view.portal_list(id!(info_list));
            for (i, item) in info.items_with_actions(actions) {
                if item.button(id!(btn)).clicked(actions) {
                    return Some(HistoryPanelAction::Select(i));
                }
            }
        }

        None
    }
}

impl CrawradHistoryPanelRef {
    #[crawrad_note::note("history-panel-crawradhistorypanelref-mode")]
    pub fn mode(&self) -> HistoryMode {
        self.borrow().map_or(HistoryMode::Differences, |p| p.mode)
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-title")]
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(cx, title);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-entries")]
    pub fn set_entries(&self, cx: &mut Cx, entries: &[HistoryEntryView]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_entries(cx, entries);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-selected")]
    pub fn set_selected(&self, cx: &mut Cx, selected: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected(cx, selected);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-mode")]
    pub fn set_mode(&self, cx: &mut Cx, mode: HistoryMode) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_mode(cx, mode);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-diff")]
    pub fn set_diff(
        &self,
        cx: &mut Cx,
        diff: &[DiffViewRow],
        blocks: usize,
        current_block: Option<usize>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_diff(cx, diff, blocks, current_block);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-reveal-row")]
    pub fn reveal_row(&self, cx: &mut Cx, row: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reveal_row(cx, row);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-contents")]
    pub fn set_contents(&self, cx: &mut Cx, contents: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_contents(cx, contents);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-set-note")]
    pub fn set_note(&self, cx: &mut Cx, note: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_note(cx, note);
        }
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-drawn-controls")]
    pub fn drawn_controls(&self, cx: &Cx) -> Vec<(String, Rect)> {
        self.borrow()
            .map_or(Vec::new(), |inner| inner.drawn_controls(cx))
    }

    #[crawrad_note::note("history-panel-crawradhistorypanelref-clicked")]
    pub fn clicked(&self, actions: &Actions) -> Option<HistoryPanelAction> {
        self.borrow().and_then(|inner| inner.clicked(actions))
    }
}

#[cfg(test)]
#[path = "test/history_panel/test.rs"]
mod tests;
