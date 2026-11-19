//! **A ListView shows a report-style list with column headers and virtualized rows**.
//!
//! Columns are defined with caption, width and alignment.
//! Rows display their caption in the first column and sub-items in subsequent columns.
//! Virtualization is handled via Makepad's `PortalList`.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use crate::borderless::*;

    pub CrawradListViewBase = {{CrawradListView}} {}

    // A cell whose grid line is an INSTANCE colour: `border_size` and
    // `border_color` are shader uniforms in Makepad 1.0, and `apply_over` at run
    // time cannot reach them. The right and bottom edges take `line_color`;
    // transparent is no line.
    GridCell = <View> {
        draw_bg: {
            instance line_color: #0000
            fn pixel(self) -> vec4 {
                let px = self.pos * self.rect_size;
                if px.x > self.rect_size.x - 1.0 || px.y > self.rect_size.y - 1.0 {
                    return mix(self.color, vec4(self.line_color.xyz, 1.0), self.line_color.w);
                }
                return self.color;
            }
        }
    }

    pub CrawradListView = <CrawradListViewBase> {
        width: Fill, height: Fill,
        flow: Down,
        show_bg: true,
        draw_bg: { color: #181825 }

        header = <View> {
            visible: true,
            width: Fill, height: 24.0, margin: 0.0,
            flow: Right,
            show_bg: true,
            draw_bg: { color: #202030 }

            c0 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c1 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c2 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c3 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c4 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c5 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c6 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
            c7 = <GridCell> { visible: false, cursor: Hand, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #202030 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
        }

        list = <PortalList> {
            width: Fill, height: Fill,
            row = <View> {
                width: Fill, height: 20.0, margin: 0.0,
                flow: Overlay,
                btn = <CrawradBorderlessButton> {
                    width: Fill, height: Fill, margin: 0.0,
                    draw_bg: {
                        color: #181825, color_hover: #262637, color_down: #2e477a, color_focus: #181825,
                    }
                    draw_text: { text_style: { font_size: 0.0 } }
                    text: ""
                }
                cols = <View> {
                    width: Fill, height: Fill,
                    flow: Right,
                    c0 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c1 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c2 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c3 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c4 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c5 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c6 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c7 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #cdd6f4, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                }
            }
            row_selected = <View> {
                width: Fill, height: 20.0, margin: 0.0,
                flow: Overlay,
                btn = <CrawradBorderlessButton> {
                    width: Fill, height: Fill, margin: 0.0,
                    draw_bg: {
                        color: #2e477a, color_hover: #385896, color_down: #385896, color_focus: #2e477a,
                    }
                    draw_text: { text_style: { font_size: 0.0 } }
                    text: ""
                }
                cols = <View> {
                    width: Fill, height: Fill,
                    flow: Right,
                    c0 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c1 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c2 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c3 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c4 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c5 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c6 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                    c7 = <GridCell> { visible: false, width: 80.0, height: Fill, padding: { left: 6.0, right: 6.0 }, align: { x: 0.0, y: 0.5 }, show_bg: false, draw_bg: { color: #0000 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" } }
                }
            }
        }
    }
}

use crate::list::ListLook;

#[note("list-view-listviewcolumn")]
pub type ListViewColumn = (String, f32, String);

#[note("list-view-listviewitem")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListViewItem {
    pub text: String,
    pub sub_items: Vec<String>,
}

#[note("list-view-listviewcolumns")]
pub type ListViewColumns = Vec<ListViewColumn>;

#[note("list-view-listviewitems")]
pub type ListViewItems = Vec<ListViewItem>;

pub use crate::events::ClickRecord;
use crawrad_note::note;

#[note("list-view-rowstamps")]
type RowStamps = std::collections::HashMap<usize, (LiveId, u64, u64)>;

#[note("list-view-aslistviewitem")]
pub trait AsListViewItem {
    fn text(&self) -> &str;
    fn sub_items(&self) -> &[String];
}

impl AsListViewItem for ListViewItem {
    fn text(&self) -> &str {
        &self.text
    }
    fn sub_items(&self) -> &[String] {
        &self.sub_items
    }
}

impl AsListViewItem for (String, Vec<String>) {
    fn text(&self) -> &str {
        &self.0
    }
    fn sub_items(&self) -> &[String] {
        &self.1
    }
}

fn cell_id(idx: usize) -> LiveId {
    match idx {
        0 => live_id!(c0),
        1 => live_id!(c1),
        2 => live_id!(c2),
        3 => live_id!(c3),
        4 => live_id!(c4),
        5 => live_id!(c5),
        6 => live_id!(c6),
        _ => live_id!(c7),
    }
}

#[note("list-view-crawradlistviewaction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradListViewAction {
    /// The user clicked row `.0`.
    Selected(usize),
    /// The user double-clicked row `.0`.
    DoubleClicked(usize),
    /// The user clicked the header of column `.0`.
    ColumnClicked(usize),
    None,
}

#[note("list-view-crawradlistview")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradListView {
    #[deref]
    view: View,
    #[rust]
    items: Vec<ListViewItem>,
    #[rust]
    columns: ListViewColumns,
    #[rust]
    selected: Option<usize>,
    #[rust(true)]
    row_select: bool,
    #[rust(false)]
    grid_lines: bool,
    #[rust(true)]
    show_column_headers: bool,
    #[rust]
    columns_epoch: u64,
    #[rust]
    header_epoch: u64,
    #[rust]
    look: ListLook,
    #[rust]
    look_epoch: u64,
    #[rust]
    styled: RowStamps,
    #[rust]
    last_click_time: ClickRecord,
}

impl Widget for CrawradListView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        if self.show_column_headers {
            let header = self.view.view(id!(header));
            for idx in 0..self.columns.len().min(8) {
                let hit = header.view(&[cell_id(idx)]).finger_up(&actions);
                if hit.is_some_and(|e| e.is_over) {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        CrawradListViewAction::ColumnClicked(idx),
                    );
                }
            }
        }
        let list = self.view.portal_list(id!(list));
        let clicked = list
            .items_with_actions(&actions)
            .into_iter()
            .find(|(_, row)| row.button(id!(btn)).clicked(&actions))
            .map(|(i, _)| i);
        if let Some(i) = clicked {
            self.selected = Some(i);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradListViewAction::Selected(i),
            );

            // The button reports the click on the mouse up, so the clock is
            // the app's, not the event's.
            let now = cx.seconds_since_app_start();
            if crate::events::second_click(&mut self.last_click_time, i, now) {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    CrawradListViewAction::DoubleClicked(i),
                );
            }

            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_header(cx);
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.items.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(item) = self.items.get(i).cloned() else {
                        continue;
                    };
                    let is_sel = self.selected == Some(i);
                    let template = if is_sel {
                        live_id!(row_selected)
                    } else {
                        live_id!(row)
                    };
                    let (row, existed) = list.item_with_existed(cx, i, template);
                    let stamp = (template, self.look_epoch, self.columns_epoch);
                    if !existed || self.styled.get(&i) != Some(&stamp) {
                        self.styled.insert(i, stamp);
                        self.style_row(cx, &row, is_sel);
                    }
                    self.populate_row_text(cx, &row, &item);
                    row.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}

impl CrawradListView {
    fn sync_header(&mut self, cx: &mut Cx) {
        let header = self.view.view(id!(header));
        let show = self.show_column_headers && !self.columns.is_empty();
        header.set_visible(cx, show);
        if !show {
            return;
        }
        if self.header_epoch == self.columns_epoch {
            return;
        }
        self.header_epoch = self.columns_epoch;
        let num_cols = self.columns.len();
        for idx in 0..8 {
            let cell = header.view(&[cell_id(idx)]);
            if idx < num_cols {
                cell.set_visible(cx, true);
                let (caption, width, alignment) = &self.columns[idx];
                let is_last = idx + 1 == num_cols;
                let align_x = match alignment.as_str() {
                    "taCenter" => 0.5,
                    "taRightJustify" => 1.0,
                    _ => 0.0,
                };
                if is_last {
                    cell.apply_over(cx, live! { width: Fill, align: { x: (align_x), y: 0.5 } });
                } else {
                    let w = (*width).max(0.0) as f64;
                    cell.apply_over(cx, live! { width: (w), align: { x: (align_x), y: 0.5 } });
                }
                if self.grid_lines {
                    cell.apply_over(
                        cx,
                        live! { draw_bg: { color: (vec4(0.125, 0.125, 0.188, 1.0)), line_color: (vec4(0.220, 0.220, 0.290, 1.0)) } },
                    );
                } else {
                    cell.apply_over(cx, live! { draw_bg: { color: (vec4(0.125, 0.125, 0.188, 1.0)), line_color: (vec4(0.0, 0.0, 0.0, 0.0)) } });
                }
                cell.label(id!(lbl)).set_text(cx, caption);
            } else {
                cell.set_visible(cx, false);
            }
        }
    }

    fn style_row(&self, cx: &mut Cx, row: &WidgetRef, is_sel: bool) {
        let cols = row.view(id!(cols));
        let num_cols = self.columns.len();
        for idx in 0..8 {
            let cell = cols.view(&[cell_id(idx)]);
            if idx < num_cols {
                cell.set_visible(cx, true);
                let (_, width, alignment) = &self.columns[idx];
                let is_last = idx + 1 == num_cols;
                let align_x = match alignment.as_str() {
                    "taCenter" => 0.5,
                    "taRightJustify" => 1.0,
                    _ => 0.0,
                };
                if is_last {
                    cell.apply_over(cx, live! { width: Fill, align: { x: (align_x), y: 0.5 } });
                } else {
                    let w = (*width).max(0.0) as f64;
                    cell.apply_over(cx, live! { width: (w), align: { x: (align_x), y: 0.5 } });
                }
                if self.grid_lines {
                    cell.apply_over(
                        cx,
                        live! { show_bg: true, draw_bg: { line_color: (vec4(0.169, 0.169, 0.231, 1.0)) } },
                    );
                } else {
                    cell.apply_over(cx, live! { show_bg: false, draw_bg: { line_color: (vec4(0.0, 0.0, 0.0, 0.0)) } });
                }
            } else {
                cell.set_visible(cx, false);
            }
        }

        if is_sel {
            let btn = row.button(id!(btn));
            if self.row_select {
                let sel_bg = vec4(0.18, 0.28, 0.48, 1.0); // #2e477a
                btn.apply_over(
                    cx,
                    live! { draw_bg: { color: (sel_bg), color_hover: (sel_bg), color_focus: (sel_bg) } },
                );
                let c0 = cols.view(id!(c0));
                c0.apply_over(cx, live! { show_bg: false });
                for idx in 0..num_cols.min(8) {
                    cols.view(&[cell_id(idx)]).label(id!(lbl)).apply_over(
                        cx,
                        live! { draw_text: { color: (vec4(1.000, 1.000, 1.000, 1.0)) } },
                    );
                }
            } else {
                let norm_bg = vec4(0.094, 0.094, 0.145, 1.0); // #181825
                btn.apply_over(
                    cx,
                    live! { draw_bg: { color: (norm_bg), color_hover: (norm_bg), color_focus: (norm_bg) } },
                );
                let sel_bg = vec4(0.18, 0.28, 0.48, 1.0); // #2e477a
                let c0 = cols.view(id!(c0));
                c0.apply_over(cx, live! { show_bg: true, draw_bg: { color: (sel_bg) } });
                c0.label(id!(lbl)).apply_over(
                    cx,
                    live! { draw_text: { color: (vec4(1.000, 1.000, 1.000, 1.0)) } },
                );
                for idx in 1..num_cols.min(8) {
                    cols.view(&[cell_id(idx)]).label(id!(lbl)).apply_over(
                        cx,
                        live! { draw_text: { color: (vec4(0.804, 0.839, 0.957, 1.0)) } },
                    );
                }
            }
        } else {
            let look = self.look;
            let text_color = if look.text[3] > 0 {
                let [r, g, b, a] = look.text.map(|v| f32::from(v) / 255.0);
                vec4(r, g, b, a)
            } else {
                vec4(0.804, 0.839, 0.957, 1.0) // #cdd6f4
            };
            for idx in 0..num_cols.min(8) {
                cols.view(&[cell_id(idx)])
                    .label(id!(lbl))
                    .apply_over(cx, live! { draw_text: { color: (text_color) } });
            }
            if look.font_size > 0.0 {
                let s = f64::from(look.font_size);
                for idx in 0..num_cols.min(8) {
                    cols.view(&[cell_id(idx)])
                        .label(id!(lbl))
                        .apply_over(cx, live! { draw_text: { text_style: { font_size: (s) } } });
                }
            }
            if look.background[3] > 0.0 {
                let [r, g, b, a] = look.background;
                let c = vec4(r, g, b, a);
                row.button(id!(btn))
                    .apply_over(cx, live! { draw_bg: { color: (c), color_focus: (c) } });
            }
        }
    }

    fn populate_row_text(&self, cx: &mut Cx, row: &WidgetRef, item: &ListViewItem) {
        let cols = row.view(id!(cols));
        for idx in 0..self.columns.len().min(8) {
            let cell_text = if idx == 0 {
                item.text.as_str()
            } else if idx - 1 < item.sub_items.len() {
                item.sub_items[idx - 1].as_str()
            } else {
                ""
            };
            let cell = cols.view(&[cell_id(idx)]);
            cell.label(id!(lbl)).set_text(cx, cell_text);
        }
    }
}

crate::palette::palette_glue!(CrawradListViewRef);

impl crate::palette::TakesLook for CrawradListView {
    fn take_look(&mut self, cx: &mut Cx, look: ListLook) {
        self.look_epoch += 1;
        if look.background[3] > 0.0 {
            let [r, g, b, a] = look.background;
            let c = vec4(r, g, b, a);
            self.view.apply_over(cx, live! { draw_bg: { color: (c) } });
        }
    }
}

impl CrawradListViewRef {
    pub fn set_columns(&self, cx: &mut Cx, columns: &[(String, f32, String)]) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.columns.as_slice() != columns
        {
            inner.columns = columns.to_vec();
            inner.columns_epoch += 1;
            inner.view.redraw(cx);
        }
    }

    pub fn set_items(&self, cx: &mut Cx, items: &[impl AsListViewItem]) {
        if let Some(mut inner) = self.borrow_mut() {
            let changed =
                inner.items.len() != items.len()
                    || inner.items.iter().zip(items.iter()).any(|(a, b)| {
                        a.text != b.text() || a.sub_items.as_slice() != b.sub_items()
                    });
            if changed {
                inner.items = items
                    .iter()
                    .map(|it| ListViewItem {
                        text: it.text().to_string(),
                        sub_items: it.sub_items().to_vec(),
                    })
                    .collect();
                inner.view.redraw(cx);
            }
        }
    }

    pub fn set_selected(&self, cx: &mut Cx, selected: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.selected != selected
        {
            inner.selected = selected;
            inner.view.redraw(cx);
        }
    }

    pub fn set_row_select(&self, cx: &mut Cx, row_select: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.row_select != row_select
        {
            inner.row_select = row_select;
            inner.columns_epoch += 1;
            inner.view.redraw(cx);
        }
    }

    pub fn set_grid_lines(&self, cx: &mut Cx, grid_lines: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.grid_lines != grid_lines
        {
            inner.grid_lines = grid_lines;
            inner.columns_epoch += 1;
            inner.view.redraw(cx);
        }
    }

    pub fn set_show_column_headers(&self, cx: &mut Cx, show: bool) {
        if let Some(mut inner) = self.borrow_mut()
            && inner.show_column_headers != show
        {
            inner.show_column_headers = show;
            inner.columns_epoch += 1;
            inner.view.redraw(cx);
        }
    }

    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        self.first(actions, |a| match a {
            CrawradListViewAction::Selected(i) => Some(i),
            _ => None,
        })
    }

    pub fn double_clicked(&self, actions: &Actions) -> Option<usize> {
        self.first(actions, |a| match a {
            CrawradListViewAction::DoubleClicked(i) => Some(i),
            _ => None,
        })
    }

    pub fn column_clicked(&self, actions: &Actions) -> Option<usize> {
        self.first(actions, |a| match a {
            CrawradListViewAction::ColumnClicked(i) => Some(i),
            _ => None,
        })
    }

    fn first(
        &self,
        actions: &Actions,
        pick: impl Fn(CrawradListViewAction) -> Option<usize>,
    ) -> Option<usize> {
        first_of(
            actions
                .filter_widget_actions(self.widget_uid())
                .map(|a| a.cast()),
            pick,
        )
    }
}

#[note("list-view-first-of")]
fn first_of(
    actions: impl Iterator<Item = CrawradListViewAction>,
    pick: impl Fn(CrawradListViewAction) -> Option<usize>,
) -> Option<usize> {
    actions.into_iter().find_map(pick)
}

#[cfg(test)]
#[path = "test/list_view/test.rs"]
mod test;
