//! **A column splitter**: a thin vertical strip that drags a column
//! edge — the column-resize cursor over it, and a line while the drag lasts.
//!
//! The owner tells it, every frame, the column's width and where the strip
//! goes ([`CrawradColumnSplitterRef::set_edge`]); a drag hands back the width the
//! finger asks — the width shown when it went down, plus how far it went. The
//! owner clamps that against its own room, because only the owner knows what
//! the column is for. The Object Inspector's name/value splitter is the first
//! one; a grid's column edge is the same gesture.
//!
//! At rest it draws nothing: the cursor is what says the edge moves, and the
//! rows under it keep the look they had.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;

    pub CrawradColumnSplitterBase = {{CrawradColumnSplitter}} {}

    pub CrawradColumnSplitter = <CrawradColumnSplitterBase> {
        width: 7.0, height: Fill,
        draw_line: { color: #f07b46 }
    }
}

#[crawrad_note::note("column-splitter-crawradcolumnsplitteraction")]
#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradColumnSplitterAction {
    /// The column width the finger asks for.
    Dragged(f64),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct CrawradColumnSplitter {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_line: DrawColor,
    #[rust]
    area: Area,
    /// The column width the owner shows now.
    #[rust]
    column: f64,
    /// The finger's x and the width shown when it went down, while it lasts.
    #[rust]
    drag_from: Option<(f64, f64)>,
}

impl Widget for CrawradColumnSplitter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) | Hit::FingerHoverOver(_) => {
                cx.set_cursor(MouseCursor::ColResize)
            }
            Hit::FingerDown(e) => {
                self.drag_from = Some((e.abs.x, self.column));
                self.redraw(cx);
            }
            Hit::FingerMove(e) => {
                cx.set_cursor(MouseCursor::ColResize);
                if let Some((x0, w0)) = self.drag_from {
                    let asked = CrawradColumnSplitterAction::Dragged(w0 + e.abs.x - x0);
                    cx.widget_action(uid, &scope.path, asked);
                }
            }
            Hit::FingerUp(_) => {
                self.drag_from = None;
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if self.drag_from.is_some() {
            // One pixel in the strip's middle, on a whole pixel so it is sharp.
            let x = (rect.pos.x + rect.size.x / 2.0).floor();
            let line = Rect {
                pos: dvec2(x, rect.pos.y),
                size: dvec2(1.0, rect.size.y),
            };
            self.draw_line.draw_abs(cx, line);
        }
        DrawStep::done()
    }
}

impl CrawradColumnSplitterRef {
    #[crawrad_note::note("column-splitter-crawradcolumnsplitterref-set-edge")]
    pub fn set_edge(&self, width: f64, left: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.column = width;
            inner.walk.margin.left = left;
        }
    }

    #[crawrad_note::note("column-splitter-crawradcolumnsplitterref-dragged")]
    pub fn dragged(&self, actions: &Actions) -> Option<f64> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            CrawradColumnSplitterAction::Dragged(w) => Some(w),
            _ => None,
        }
    }
}
