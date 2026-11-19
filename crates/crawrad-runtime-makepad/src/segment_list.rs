//! **A generic segment list widget**.
//!
//! Hosts a dynamic sequence of heterogeneous children (text segments and code/block segments)
//! instantiated from templates declared in DSL, laying them out vertically in order.
//!
//! The widget is purely presentational: it manages child instantiation and recycling,
//! forwards drawing and event handling, and exposes helper accessors for content,
//! hit rectangles, and button actions.

use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;

    pub CrawradSegmentListBase = {{CrawradSegmentList}} {}

    pub CrawradSegmentList = <CrawradSegmentListBase> {
        width: Fill,
        height: Fit,
    }
}

#[crawrad_note::note("segment-list-segmentkind")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentKind {
    /// A prose / text segment.
    Text,
    /// A code / content block segment.
    Block,
}

impl SegmentKind {
    #[crawrad_note::note("segment-list-segmentkind-as-str")]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Block => "block",
        }
    }
}

#[crawrad_note::note("segment-list-segment")]
#[derive(Clone, Copy, Debug)]
pub enum Segment<'a> {
    /// A text item.
    Text(&'a str),
    /// A block item: its header `label`, its `body`, and whether it is `tall`
    /// (a fixed 150 px scrolling body instead of `Fit`).
    Block {
        label: &'a str,
        body: &'a str,
        tall: bool,
    },
}

impl<'a> From<(Option<&'a str>, &'a str, bool)> for Segment<'a> {
    #[crawrad_note::note("segment-list-segment-from")]
    fn from((label, body, tall): (Option<&'a str>, &'a str, bool)) -> Self {
        match label {
            None => Segment::Text(body),
            Some(label) => Segment::Block { label, body, tall },
        }
    }
}

#[crawrad_note::note("segment-list-crawradsegmentlist")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradSegmentList {
    #[deref]
    pub view: View,

    #[live]
    pub text_template: Option<LivePtr>,

    #[live]
    pub block_template: Option<LivePtr>,

    #[rust]
    pub items: ComponentMap<usize, WidgetRef>,

    #[rust]
    pub kinds: Vec<SegmentKind>,

    #[rust]
    pub code_strings: Vec<Option<String>>,

    /// The list's own area: the `View` it derefs to never draws.
    #[rust]
    list_area: Area,
}

impl Widget for CrawradSegmentList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_idx, item) in self.items.iter_mut() {
            item.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut layout = self.view.layout;
        layout.flow = Flow::Down;
        cx.begin_turtle(walk, layout);

        for idx in 0..self.kinds.len() {
            if let Some(item) = self.items.get_mut(&idx) {
                item.draw_all(cx, scope);
            }
        }

        cx.end_turtle_with_area(&mut self.list_area);
        DrawStep::done()
    }
}

impl CrawradSegmentList {
    #[crawrad_note::note("segment-list-crawradsegmentlist-update-items")]
    pub fn update_items(&mut self, cx: &mut Cx, kinds: &[SegmentKind]) {
        if self.kinds == kinds && self.items.len() == kinds.len() {
            return;
        }

        // Drop excess widgets
        self.items.retain(|&idx, _| idx < kinds.len());

        for (idx, &kind) in kinds.iter().enumerate() {
            let need_new = match self.kinds.get(idx) {
                Some(&existing_kind) => existing_kind != kind || !self.items.contains_key(&idx),
                None => true,
            };

            if need_new {
                let template_ptr = match kind {
                    SegmentKind::Text => self.text_template,
                    SegmentKind::Block => self.block_template,
                };
                if let Some(ptr) = template_ptr {
                    let child = WidgetRef::new_from_ptr(cx, Some(ptr));
                    self.items.insert(idx, child);
                }
            }
        }

        self.kinds = kinds.to_vec();
        self.code_strings.resize(kinds.len(), None);
        self.list_area.redraw(cx);
    }
}

impl CrawradSegmentListRef {
    #[crawrad_note::note("segment-list-crawradsegmentlistref-set-items")]
    pub fn set_items(&self, cx: &mut Cx, kinds: &[SegmentKind]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_items(cx, kinds);
        }
    }

    #[crawrad_note::note("segment-list-crawradsegmentlistref-item")]
    pub fn item(&self, index: usize) -> WidgetRef {
        self.borrow()
            .and_then(|inner| inner.items.get(&index).cloned())
            .unwrap_or_default()
    }

    #[crawrad_note::note("segment-list-crawradsegmentlistref-set-segments")]
    pub fn set_segments(&self, cx: &mut Cx, segments: &[Segment]) {
        let kinds: Vec<_> = segments
            .iter()
            .map(|s| match s {
                Segment::Text(_) => SegmentKind::Text,
                Segment::Block { .. } => SegmentKind::Block,
            })
            .collect();
        self.set_items(cx, &kinds);
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        for (i, segment) in segments.iter().enumerate() {
            let Some(item) = inner.items.get(&i).cloned() else {
                continue;
            };
            match segment {
                Segment::Text(text) => item.set_text(cx, text),
                Segment::Block { label, body, tall } => {
                    inner.code_strings[i] = Some(body.to_string());
                    item.label(id!(lbl_code_lang)).set_text(cx, label);
                    item.label(id!(code_text)).set_text(cx, body);
                    let scroll = item.view(id!(code_scroll));
                    match tall {
                        true => scroll.apply_over(cx, live! { height: 150.0 }),
                        false => scroll.apply_over(cx, live! { height: Fit }),
                    }
                }
            }
        }
    }

    #[crawrad_note::note("segment-list-crawradsegmentlistref-copy-code-clicked")]
    pub fn copy_code_clicked(&self, actions: &Actions) -> Option<String> {
        let inner = self.borrow()?;
        for (idx, item) in inner.items.iter() {
            if item.button(id!(btn_copy_code)).clicked(actions) {
                if let Some(Some(code)) = inner.code_strings.get(*idx) {
                    return Some(code.clone());
                }
                let text = item.label(id!(code_text)).text();
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
        None
    }

    #[crawrad_note::note("segment-list-crawradsegmentlistref-first-text-state")]
    pub fn first_text_state(&self, cx: &Cx) -> ([f64; 4], String) {
        let Some(inner) = self.borrow() else {
            return ([0.0; 4], String::new());
        };
        let first = inner.kinds.iter().position(|k| *k == SegmentKind::Text);
        let item = first
            .and_then(|i| inner.items.get(&i).cloned())
            .unwrap_or_default();
        (
            rect_of(item.area().rect(cx)),
            item.as_text_input().selected_text(),
        )
    }

    #[crawrad_note::note("segment-list-crawradsegmentlistref-block-rects")]
    pub fn block_rects(&self, cx: &Cx) -> Vec<ItemRect> {
        let Some(inner) = self.borrow() else {
            return Vec::new();
        };
        let rects = inner.kinds.iter().enumerate().filter_map(|(i, kind)| {
            let item = inner.items.get(&i)?;
            let copy = (*kind == SegmentKind::Block)
                .then(|| rect_of(item.button(id!(btn_copy_code)).area().rect(cx)));
            Some((kind.as_str(), rect_of(item.area().rect(cx)), copy))
        });
        rects.collect()
    }
}

#[crawrad_note::note("segment-list-itemrect")]
pub type ItemRect = (&'static str, [f64; 4], Option<[f64; 4]>);

#[crawrad_note::note("segment-list-rect-of")]
pub fn rect_of(r: Rect) -> [f64; 4] {
    [r.pos.x, r.pos.y, r.size.x, r.size.y]
}

#[cfg(test)]
#[path = "test/segment_list/test.rs"]
mod tests;
