//! Makepad ScrollBox component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::bar::{Bars, ScrollBarState, paint_inner_bars};
use crate::components::comum::{MAKEPAD_BORDER, MAKEPAD_WIDGET_INPUT};
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-scroll-box-classe")]
pub const CLASSE: &str = "ScrollBox";

#[note("components-scroll-box-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[note("components-scroll-box-desenho-draw")]
    fn draw(&self, p: &mut Brush, _c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
    }

    fn draws_over_children(&self) -> bool {
        true
    }

    #[note("components-scroll-box-desenho-draw-over")]
    fn draw_over(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        // **THE REGIME IS AUTOMATIC, and the calculation does not live here.**
        //
        // It was previously written here that both bars were shown whenever the
        // document enabled them, with the reason: *"saying WHEN it fits requires a
        // property that `crawrad-registry` does not yet have; inventing it here
        // would mean deciding model inside drawing"*.
        //
        // A property was not missing — CALCULATION was missing, and it was born neutral in
        // `crawrad_ir::rolagem` so that both adapters do the same.
        // The owner saw the defect on screen on 2026-09-17: the designer with both
        // bars and the generated application with none.
        //
        // The two document properties still rule, and rule
        // first: disabled `VertScrollBar`/`HorzScrollBar` is the user
        // saying that this axis does not scroll, so not even overflowing content makes
        // the bar appear.
        let conteudo = crawrad_ir::scrolling::content_extent(
            (c.rect.x, c.rect.y),
            scene
                .children(c)
                .map(|f| (f.rect.x, f.rect.y, f.rect.w, f.rect.h)),
        );
        let precisa = crawrad_ir::scrolling::bars_needed((c.rect.w, c.rect.h), conteudo);
        paint_inner_bars(
            p,
            crate::canvas::shrink(r, 1.0),
            Bars {
                vertical: ScrollBarState::when_needed(c.barra_v && precisa.vertical),
                horizontal: ScrollBarState::when_needed(c.barra_h && precisa.horizontal),
            },
            0.0,
        );
    }
}
