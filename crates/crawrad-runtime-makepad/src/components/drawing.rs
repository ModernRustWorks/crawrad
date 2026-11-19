//! A canvas component's contract — **one file per class**.
//!
//! The house rule (`CONTRIBUTING.md`, *Makepad component rules*) forbids a monolithic
//! `match` block over classes: every component settles its own cycle in
//! `src/componentes/<class>.rs` and hands the result back to the canvas. This
//! file is only the contract; the catalogue of whoever honours it is in
//! [`super::DESENHOS`](crate::components).
//!
//! What the component RECEIVES comes already resolved, and that is on purpose:
//!
//! - the [`Brush`] already carries the frame's [`crate::Transformacao`] — the
//!   component never adds an origin nor multiplies a scale (ADR 0019);
//! - the [`Scene`] gives read access to the neighbours, for the sake of the
//!   `PageControl`, which needs the captions of its own pages.
//!
//! What the component does NOT do, and may not start doing (ADR 0027 §3):
//! hit-test, selection decisions, resize arithmetic and writing to the document.
//! The one that decides is `crawrad-designer`, which is headless and has 155 tests.

use std::collections::BTreeMap;

use crawrad_ir::{NodeId, Rect as RectIr};
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::{Brush, ContentLine};
use crate::components::comum::*;
use crawrad_note::note;

#[note("components-drawing-text-color")]
pub fn text_color(c: &MakepadCtl) -> [f32; 4] {
    if c.enabled {
        MAKEPAD_TEXT
    } else {
        MAKEPAD_TEXT_DIM
    }
}

#[note("components-drawing-scene")]
pub struct Scene<'a> {
    todos: &'a BTreeMap<NodeId, MakepadCtl>,
}

impl<'a> Scene<'a> {
    #[note("components-drawing-scene-new")]
    pub fn new(todos: &'a BTreeMap<NodeId, MakepadCtl>) -> Self {
        Self { todos }
    }

    #[note("components-drawing-scene-ctl")]
    pub fn ctl(&self, id: &str) -> Option<&'a MakepadCtl> {
        self.todos.get(id)
    }

    #[note("components-drawing-scene-find")]
    pub fn find(&self, name_or_id: &str) -> Option<&'a MakepadCtl> {
        self.todos
            .get(name_or_id)
            .or_else(|| self.todos.values().find(|c| c.name == name_or_id))
    }

    #[note("components-drawing-scene-children")]
    pub fn children<'b>(&'b self, c: &'b MakepadCtl) -> impl Iterator<Item = &'a MakepadCtl> + 'b {
        c.children.iter().filter_map(|f| self.todos.get(f))
    }

    #[note("components-drawing-scene-children-of-class")]
    pub fn children_of_class<'b>(
        &'b self,
        c: &'b MakepadCtl,
        class: &'b str,
    ) -> impl Iterator<Item = &'a MakepadCtl> + 'b {
        self.children(c).filter(move |f| f.class == class)
    }
}

#[note("components-drawing-componentdrawing")]
pub trait ComponentDrawing: Sync {
    #[note("components-drawing-componentdrawing-class-name")]
    fn class_name(&self) -> &'static str;

    #[note("components-drawing-componentdrawing-visual")]
    fn visual(&self) -> bool {
        true
    }

    #[note("components-drawing-componentdrawing-drawn-by-owner")]
    fn drawn_by_owner(&self) -> Option<&'static str> {
        None
    }

    #[note("components-drawing-componentdrawing-lines")]
    fn lines(&self, _c: &MakepadCtl) -> Vec<ContentLine> {
        Vec::new()
    }

    #[note("components-drawing-componentdrawing-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene);

    #[note("components-drawing-componentdrawing-clips-to-own-box")]
    fn clips_to_own_box(&self) -> bool {
        true
    }

    #[note("components-drawing-componentdrawing-draws-over-children")]
    fn draws_over_children(&self) -> bool {
        false
    }

    #[note("components-drawing-componentdrawing-draw-over")]
    fn draw_over(&self, _p: &mut Brush, _c: &MakepadCtl, _r: Rect, _scene: &Scene) {}

    #[note("components-drawing-componentdrawing-draw-form-strip")]
    fn draw_form_strip(
        &self,
        _p: &mut Brush,
        _c: &MakepadCtl,
        _form: RectIr,
        _scene: &Scene,
    ) -> bool {
        false
    }
}
