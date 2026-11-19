//! Makepad ComboBox component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-combo-box-classe")]
pub const CLASSE: &str = "ComboBox";

#[crawrad_note::note("components-combo-box-largura-do-botao")]
pub const LARGURA_DO_BOTAO: f32 = 16.0;

#[crawrad_note::note("components-combo-box-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // Closed, as it starts: the recessed field, the chosen item, and the arrow
    // button against the right edge. The button RISES, like any button — and
    // it is what indicates, without reading the text, that this opens a list.
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let escolhido = c
            .escolhido
            .and_then(|i| c.itens.get(i))
            .map(|i| i.text.clone())
            .unwrap_or_else(|| c.text.clone());
        let lado = p.medida(LARGURA_DO_BOTAO).min(r.size.x * 0.5).max(6.0);
        let botao = Rect {
            pos: dvec2(r.pos.x + r.size.x - lado - 1.0, r.pos.y + 1.0),
            size: dvec2(lado, (r.size.y - 2.0).max(1.0)),
        };
        p.rounded_box(botao, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
        p.arrow(
            crate::components::bar::centered_square(botao),
            dvec2(0.0, 1.0),
            MAKEPAD_TEXT,
        );
        let dentro = p.inset(r);
        let campo = Rect {
            pos: dentro.pos,
            size: dvec2((botao.pos.x - dentro.pos.x - 2.0).max(0.0), dentro.size.y),
        };
        p.write(campo, &escolhido, text_color(c), false);
    }
}
