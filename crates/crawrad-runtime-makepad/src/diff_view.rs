//! Generic palette-style side-by-side diff view.
//!
//! Renders two aligned columns of code lines using a virtualized `PortalList`,
//! gutter line numbers, and semantic tint backgrounds for additions, removals,
//! and modifications.

use makepad_widgets::*;

live_design! {
    link crawrad;
    use link::widgets::*;
    use link::theme::*;

    SCROLL_BAR_GAP = 14.0

    DiffCell = <View> {
        width: Fill, height: Fill,
        flow: Right,
        align: { y: 0.5 },
        show_bg: true,
        draw_bg: { color: #1d1f20 }
        // The 3 px mark in the gutter: the colour of the change, on the side
        // that has it. Clear on an unchanged line.
        bar = <View> {
            width: 3.0, height: Fill,
            show_bg: true,
            draw_bg: { color: #0000 }
        }
        // Right-aligned by the text itself (`{:>4}`, the font is monospaced):
        // the Label's own `align: { x: 1.0 }` pushed the last digit past its
        // edge and cut it in half.
        gutter = <Label> {
            width: 39.0, height: Fit,
            flow: Right,
            padding: { left: 2.0 },
            align: { x: 0.0, y: 0.5 },
            draw_text: {
                color: #98989b,
                text_style: <THEME_FONT_CODE> { font_size: 9.0 }
            }
            text: ""
        }
        // A line of code never wraps: the row is 20 pt tall, and a wrapped
        // line drew its second half over the row below. In Makepad 1.0 the
        // wrap is the Label's `flow` (`RightWrap` by default); `Right` keeps
        // the line whole and the cell clips it at its edge.
        content = <Label> {
            width: Fill, height: Fit,
            flow: Right,
            align: { x: 0.0, y: 0.5 },
            draw_text: {
                color: #e7e7ea,
                text_style: <THEME_FONT_CODE> { font_size: 9.0 }
            }
            text: ""
        }
    }

    // One side of a changed line: the IDE's ground with a light tint, the mark
    // in the gutter and the line number in the change's colour. The side that
    // has nothing (an added line's copy side, a removed line's buffer side)
    // stays the plain ground — a whole row painted solid read as a block of
    // colour, not as a difference.
    DiffCellAdded = <DiffCell> {
        draw_bg: { color: #1c2a24 }
        bar = { draw_bg: { color: #7cb46b } }
        gutter = { draw_text: { color: #7cb46b } }
    }
    DiffCellRemoved = <DiffCell> {
        draw_bg: { color: #2a1c24 }
        bar = { draw_bg: { color: #e2726b } }
        gutter = { draw_text: { color: #e2726b } }
    }
    DiffCellChanged = <DiffCell> {
        draw_bg: { color: #1d2d3d }
        bar = { draw_bg: { color: #b3dcff } }
        gutter = { draw_text: { color: #b3dcff } }
    }

    DiffRowBase = <View> {
        width: Fill, height: 20.0,
        flow: Right,
        show_bg: true,
        draw_bg: { color: #1d1f20 }
        left_col = <DiffCell> {}
        sep = <View> {
            width: 1.0, height: Fill,
            show_bg: true,
            draw_bg: { color: #424244 }
        }
        right_col = <DiffCell> {}
        // The list's vertical scroll bar sits over the right edge: the text
        // stops before it instead of running under it.
        bar_gap = <View> { width: (SCROLL_BAR_GAP), height: Fill }
    }

    // One side's horizontal scroll bar. The track starts where the text
    // does (past the 3 px mark and the 39 px gutter); the thumb's place and
    // width are set from the side's offset in `draw_walk`.
    DiffHTrack = <View> {
        width: Fill, height: Fill,
        padding: { left: 42.0, top: 3.0, bottom: 3.0, right: 4.0 },
        show_bg: true,
        draw_bg: { color: #1d1f20 }
        thumb = <View> {
            width: 0.0, height: Fill,
            show_bg: true,
            draw_bg: { color: #4d4f52 }
        }
    }

    pub CrawradDiffViewBase = {{CrawradDiffView}} {}

    pub CrawradDiffView = <CrawradDiffViewBase> {
        width: Fill, height: Fill,
        flow: Down,
        show_bg: true,
        draw_bg: { color: #1d1f20 }

        list = <PortalList> {
            width: Fill, height: Fill,
            row_same = <DiffRowBase> {}
            row_removed = <DiffRowBase> {
                left_col = <DiffCellRemoved> {}
            }
            row_added = <DiffRowBase> {
                right_col = <DiffCellAdded> {}
            }
            row_changed = <DiffRowBase> {
                left_col = <DiffCellChanged> {}
                right_col = <DiffCellChanged> {}
            }
        }

        // A horizontal scroll bar per side, under its own column: each side
        // of a long line is read on its own.
        hbars = <View> {
            width: Fill, height: 12.0,
            flow: Right,
            left_track = <DiffHTrack> {}
            <View> { width: 1.0, height: Fill, show_bg: true, draw_bg: { color: #424244 } }
            right_track = <DiffHTrack> {}
            <View> { width: (SCROLL_BAR_GAP), height: Fill }
        }
    }
}

#[crawrad_note::note("diff-view-diffrowkind")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffRowKind {
    /// Line identical on both sides.
    Same,
    /// Line removed from buffer (present only in copy).
    Removed,
    /// Line added to buffer (present only in buffer).
    Added,
    /// Line modified between copy and buffer.
    Changed,
}

#[crawrad_note::note("diff-view-diff-row-tint")]
pub fn diff_row_tint(kind: DiffRowKind) -> &'static str {
    match kind {
        DiffRowKind::Same => "#1d1f20",
        DiffRowKind::Removed => "#2a1c24",
        DiffRowKind::Added => "#1c2a24",
        DiffRowKind::Changed => "#1d2d3d",
    }
}

#[crawrad_note::note("diff-view-diff-row-accent")]
pub fn diff_row_accent(kind: DiffRowKind) -> Option<&'static str> {
    match kind {
        DiffRowKind::Same => None,
        DiffRowKind::Removed => Some("#e2726b"),
        DiffRowKind::Added => Some("#7cb46b"),
        DiffRowKind::Changed => Some("#b3dcff"),
    }
}

#[crawrad_note::note("diff-view-diffviewrow")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffViewRow<'a> {
    pub kind: DiffRowKind,
    pub left: Option<(usize, &'a str)>,
    pub right: Option<(usize, &'a str)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredDiffRow {
    kind: DiffRowKind,
    left_line: Option<usize>,
    left_text: String,
    right_line: Option<usize>,
    right_text: String,
}

impl<'a> From<&DiffViewRow<'a>> for StoredDiffRow {
    fn from(row: &DiffViewRow<'a>) -> Self {
        Self {
            kind: row.kind,
            left_line: row.left.map(|(line, _)| line),
            left_text: row
                .left
                .map(|(_, text)| text.to_string())
                .unwrap_or_default(),
            right_line: row.right.map(|(line, _)| line),
            right_text: row
                .right
                .map(|(_, text)| text.to_string())
                .unwrap_or_default(),
        }
    }
}

#[crawrad_note::note("diff-view-crawraddiffview")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradDiffView {
    #[deref]
    pub view: View,

    #[rust]
    rows: Vec<StoredDiffRow>,

    /// Per side (left, right): the first column shown, the longest line in
    /// columns, and how many columns the side shows.
    #[rust]
    offset: [usize; 2],
    #[rust]
    longest: [usize; 2],
    #[rust]
    visible: [usize; 2],

    /// A thumb being dragged: the side, where the finger went down and the
    /// offset it had then.
    #[rust]
    drag: Option<(usize, f64, usize)>,

    /// The thumbs' last place and width, so they are set only on a change.
    #[rust]
    thumbs: [(f64, f64); 2],
}

#[crawrad_note::note("diff-view-code-column-width")]
const CODE_COLUMN_WIDTH: f64 = 7.16;

#[crawrad_note::note("diff-view-min-thumb")]
const MIN_THUMB: f64 = 24.0;

#[crawrad_note::note("diff-view-track-padding")]
const TRACK_PADDING: f64 = 46.0;

#[crawrad_note::note("diff-view-gutter-text")]
fn gutter_text(line: usize) -> String {
    format!("{line:>4}")
}

#[crawrad_note::note("diff-view-skip-columns")]
fn skip_columns(text: &str, columns: usize) -> &str {
    text.char_indices()
        .nth(columns)
        .map_or("", |(at, _)| &text[at..])
}

#[crawrad_note::note("diff-view-max-offset")]
fn max_offset(longest: usize, visible: usize) -> usize {
    longest.saturating_sub(visible)
}

#[crawrad_note::note("diff-view-thumb-geometry")]
fn thumb_geometry(inner: f64, visible: usize, longest: usize, offset: usize) -> Option<(f64, f64)> {
    if longest <= visible || inner <= 0.0 {
        return None;
    }
    let width = (inner * visible as f64 / longest as f64)
        .max(MIN_THUMB)
        .min(inner);
    let room = inner - width;
    let max = max_offset(longest, visible).max(1) as f64;
    Some((room * offset as f64 / max, width))
}

impl Widget for CrawradDiffView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.scroll_sideways(cx, event) {
            return;
        }
        for side in 0..2 {
            self.drag_thumb(cx, event, side);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.rows.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some(row_data) = self.rows.get(i) else {
                        continue;
                    };
                    let template = match row_data.kind {
                        DiffRowKind::Same => live_id!(row_same),
                        DiffRowKind::Removed => live_id!(row_removed),
                        DiffRowKind::Added => live_id!(row_added),
                        DiffRowKind::Changed => live_id!(row_changed),
                    };
                    let item = list.item(cx, i, template);
                    let left_gutter = row_data.left_line.map_or(String::new(), gutter_text);
                    let right_gutter = row_data.right_line.map_or(String::new(), gutter_text);
                    item.label(id!(left_col.gutter)).set_text(cx, &left_gutter);
                    item.label(id!(left_col.content))
                        .set_text(cx, skip_columns(&row_data.left_text, self.offset[0]));
                    item.label(id!(right_col.gutter))
                        .set_text(cx, &right_gutter);
                    item.label(id!(right_col.content))
                        .set_text(cx, skip_columns(&row_data.right_text, self.offset[1]));
                    item.draw_all(cx, scope);
                }
                drop(list);
                // The list is drawn and the bars are next: the thumbs take
                // their place now, from the tracks of the frame before.
                self.place_thumbs(cx);
            }
        }
        DrawStep::done()
    }
}

impl CrawradDiffView {
    fn track(&self, side: usize) -> ViewRef {
        if side == 0 {
            self.view.view(id!(left_track))
        } else {
            self.view.view(id!(right_track))
        }
    }

    #[crawrad_note::note("diff-view-crawraddiffview-track-inner")]
    fn track_inner(&self, cx: &Cx, side: usize) -> f64 {
        (self.track(side).area().rect(cx).size.x - TRACK_PADDING).max(0.0)
    }

    fn place_thumbs(&mut self, cx: &mut Cx2d) {
        for side in 0..2 {
            let inner = self.track_inner(cx, side);
            self.visible[side] = (inner / CODE_COLUMN_WIDTH).floor() as usize;
            self.offset[side] =
                self.offset[side].min(max_offset(self.longest[side], self.visible[side]));
            let (x, w) = thumb_geometry(
                inner,
                self.visible[side],
                self.longest[side],
                self.offset[side],
            )
            .unwrap_or((0.0, 0.0));
            if self.thumbs[side] != (x, w) {
                self.thumbs[side] = (x, w);
                self.track(side)
                    .view(id!(thumb))
                    .apply_over(cx, live! { width: (w), margin: { left: (x) } });
            }
        }
    }

    #[crawrad_note::note("diff-view-crawraddiffview-side-at")]
    fn side_at(&self, cx: &Cx, x: f64) -> usize {
        let right = self.track(1).area().rect(cx);
        usize::from(right.size.x > 0.0 && x >= right.pos.x)
    }

    fn scroll_to(&mut self, cx: &mut Cx, side: usize, offset: usize) {
        let offset = offset.min(max_offset(self.longest[side], self.visible[side]));
        if offset != self.offset[side] {
            self.offset[side] = offset;
            self.view.redraw(cx);
        }
    }

    #[crawrad_note::note("diff-view-crawraddiffview-scroll-sideways")]
    fn scroll_sideways(&mut self, cx: &mut Cx, event: &Event) -> bool {
        let Event::Scroll(e) = event else {
            return false;
        };
        if !self.view.area().rect(cx).contains(e.abs) {
            return false;
        }
        let amount = if e.modifiers.shift && e.scroll.x == 0.0 {
            e.scroll.y
        } else {
            e.scroll.x
        };
        if amount == 0.0 {
            return false;
        }
        let side = self.side_at(cx, e.abs.x);
        // A wheel notch is ~120 px: a column per pixel ran to the line's end
        // in a few notches, so a notch moves about six columns.
        let columns = ((amount.abs() / (3.0 * CODE_COLUMN_WIDTH)).round() as usize).max(1);
        let offset = if amount > 0.0 {
            self.offset[side] + columns
        } else {
            self.offset[side].saturating_sub(columns)
        };
        self.scroll_to(cx, side, offset);
        true
    }

    #[crawrad_note::note("diff-view-crawraddiffview-drag-thumb")]
    fn drag_thumb(&mut self, cx: &mut Cx, event: &Event, side: usize) {
        let track = self.track(side);
        let inner = self.track_inner(cx, side);
        let (thumb_x, thumb_w) = self.thumbs[side];
        if thumb_w <= 0.0 {
            return;
        }
        let per_px =
            max_offset(self.longest[side], self.visible[side]) as f64 / (inner - thumb_w).max(1.0);
        match event.hits(cx, track.area()) {
            Hit::FingerDown(f) => {
                let at = f.abs.x - (track.area().rect(cx).pos.x + 42.0);
                if at < thumb_x || at > thumb_x + thumb_w {
                    let centred = (at - thumb_w / 2.0).max(0.0) * per_px;
                    self.scroll_to(cx, side, centred.round() as usize);
                }
                self.drag = Some((side, f.abs.x, self.offset[side]));
            }
            Hit::FingerMove(f) => {
                if let Some((s, x0, o0)) = self.drag
                    && s == side
                {
                    let moved = (f.abs.x - x0) * per_px;
                    let offset = (o0 as f64 + moved).max(0.0).round() as usize;
                    self.scroll_to(cx, side, offset);
                }
            }
            Hit::FingerUp(_) => self.drag = None,
            _ => {}
        }
    }
}

impl CrawradDiffView {
    #[crawrad_note::note("diff-view-crawraddiffview-set-rows")]
    pub fn set_rows(&mut self, cx: &mut Cx, rows: &[DiffViewRow]) {
        self.rows = rows.iter().map(StoredDiffRow::from).collect();
        self.offset = [0, 0];
        let longest = |side: fn(&StoredDiffRow) -> &str| {
            self.rows
                .iter()
                .map(|r| side(r).chars().count())
                .max()
                .unwrap_or(0)
        };
        self.longest = [longest(|r| &r.left_text), longest(|r| &r.right_text)];
        self.view.redraw(cx);
    }

    #[crawrad_note::note("diff-view-crawraddiffview-reveal-row")]
    pub fn reveal_row(&mut self, cx: &mut Cx, row: usize) {
        if row < self.rows.len() {
            self.view
                .portal_list(id!(list))
                .set_first_id_and_scroll(row, 0.0);
            self.view.redraw(cx);
        }
    }
}

impl CrawradDiffViewRef {
    #[crawrad_note::note("diff-view-crawraddiffviewref-set-rows")]
    pub fn set_rows(&self, cx: &mut Cx, rows: &[DiffViewRow]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_rows(cx, rows);
        }
    }

    #[crawrad_note::note("diff-view-crawraddiffviewref-reveal-row")]
    pub fn reveal_row(&self, cx: &mut Cx, row: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reveal_row(cx, row);
        }
    }
}

#[cfg(test)]
#[path = "test/diff_view/test.rs"]
mod tests;
