//! Makepad GroupBox component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-group-box-classe")]
pub const CLASSE: &str = "GroupBox";

#[crawrad_note::note("components-group-box-recuo-da-legenda")]
pub const RECUO_DA_LEGENDA: f32 = 8.0;

#[crawrad_note::note("components-group-box-vao-da-legenda")]
pub const VAO_DA_LEGENDA: f32 = 4.0;

#[crawrad_note::note("components-group-box-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[crawrad_note::note("components-group-box-desenho-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let legenda = sem_acelerador(&c.text);
        let espessura = p.medida(1.0).max(1.0);
        let cor = MAKEPAD_PANEL_BORDER;
        let direita = r.pos.x + r.size.x;
        let baixo = r.pos.y + r.size.y;

        // The caption height decides where the top line runs. Without a caption
        // it runs along the border, like any frame.
        let medida = p.measure(&legenda);
        let topo = r.pos.y + medida.y * 0.5;

        // The three sides without a gap.
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, topo),
                size: dvec2(espessura, (baixo - topo).max(0.0)),
            },
            cor,
        );
        p.stroke(
            Rect {
                pos: dvec2(direita - espessura, topo),
                size: dvec2(espessura, (baixo - topo).max(0.0)),
            },
            cor,
        );
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, baixo - espessura),
                size: dvec2(r.size.x, espessura),
            },
            cor,
        );

        if legenda.is_empty() {
            p.stroke(
                Rect {
                    pos: dvec2(r.pos.x, topo),
                    size: dvec2(r.size.x, espessura),
                },
                cor,
            );
            return;
        }

        let x0 = r.pos.x + p.medida(RECUO_DA_LEGENDA);
        let vao = p.medida(VAO_DA_LEGENDA);

        // The left segment, up to the gap.
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, topo),
                size: dvec2((x0 - vao - r.pos.x).max(0.0), espessura),
            },
            cor,
        );

        // The right segment. The `.min(direita)` prevents the **inverted
        // segment**, and the penalty was already paid: in RXP, a caption with 46
        // characters in a 140-point box put the start of the right segment
        // AFTER its end and drew 174 points of line backwards,
        // over its neighbor (`crawrad-runtime-egui/src/lib.rs:3044-3058`).
        // Clamped, the segment simply has no remainder — which is correct: there is no
        // line to draw there, the caption is in its place.
        let inicio_direito = (x0 + medida.x + vao).min(direita);
        p.stroke(
            Rect {
                pos: dvec2(inicio_direito, topo),
                size: dvec2((direita - inicio_direito).max(0.0), espessura),
            },
            cor,
        );

        // The caption, straddling the line: the text is written from the top of the
        // component, and the line passes through its middle.
        p.write_caption(
            Rect {
                pos: dvec2(x0, r.pos.y),
                size: dvec2((direita - x0).max(0.0), medida.y),
            },
            &c.text,
            text_color(c),
            false,
        );
    }
}
