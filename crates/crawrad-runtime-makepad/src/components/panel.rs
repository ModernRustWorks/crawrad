//! Makepad Panel component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-panel-classe")]
pub const CLASSE: &str = "Panel";

#[crawrad_note::note("components-panel-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[crawrad_note::note("components-panel-desenho-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        caixa_de_superficie(p, r);
        if c.text.is_empty() {
            return;
        }
        let legenda = Rect {
            pos: dvec2(r.pos.x + p.medida(6.0), r.pos.y + p.medida(2.0)),
            size: dvec2(
                (r.size.x - p.medida(12.0)).max(0.0),
                p.medida(16.0).max(8.0),
            ),
        };
        p.write_caption(legenda, &c.text, text_color(c), false);
    }
}

#[crawrad_note::note("components-panel-caixa-de-superficie")]
pub(crate) fn caixa_de_superficie(p: &mut Brush, r: Rect) {
    p.rounded_box(r, MAKEPAD_PANEL, MAKEPAD_PANEL_BORDER, 2.0);
}
