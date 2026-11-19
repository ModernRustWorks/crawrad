//! Makepad Memo component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::{Brush, Content, ContentLine, lines_that_fit};
use crate::components::bar::{Bars, bars_from_model, paint_inner_bars};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[note("components-memo-classe")]
pub const CLASSE: &str = "Memo";

/// The height of a `Memo` line, in design units. It is the step of 15
/// that RXP uses in the design branch (`crawrad-runtime-egui/src/lib.rs:2589`).
/// **Declared in [`crate::shared_geometry::MEMO_LINE_HEIGHT`]**, the file that the
/// code generator also reads. It used to live here, and the generator had its
/// own — the canvas drew one height and the generated application another.
pub use crate::shared_geometry::MEMO_LINE_HEIGHT as ALTURA_DA_LINHA;
use crawrad_note::note;

#[note("components-memo-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[note("components-memo-desenho-lines")]
    fn lines(&self, c: &MakepadCtl) -> Vec<ContentLine> {
        c.text.lines().map(|l| ContentLine::leaf(0, l)).collect()
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let linhas = self.lines(c);
        // The bar stays INSIDE the rectangle and **against the border**, like the
        // screenshot of `TListBox` in a classic RAD IDE sent by the owner — that is why it measures
        // against the box minus the frame, and not against the text inset.
        let caixa = crate::canvas::shrink(r, 1.0);
        let altura = p.medida(ALTURA_DA_LINHA).max(6.0) as f32;
        // How many lines fit is measured on the area that the TEXT will receive, and not
        // on the box: measuring on the box caused the last line to be clipped **without** the
        // bar appearing, which is the worst of both worlds — content disappears and
        // nothing indicates that it exists.
        let cabem = lines_that_fit(p.inset(caixa).size.y as f32, altura);
        let barras = scroll_bars_of(p, c, &linhas, caixa, linhas.len() > cabem);
        let sobrou = paint_inner_bars(p, caixa, barras, 0.0);
        let dentro = p.inset(sobrou);
        p.content(
            dentro,
            &Content {
                lines: &linhas,
                line_height: ALTURA_DA_LINHA,
                color: text_color(c),
                selected: None,
            },
        );
    }
}

#[note("components-memo-scroll-bars-of")]
fn scroll_bars_of(
    p: &mut Brush,
    c: &MakepadCtl,
    lines: &[ContentLine],
    frame: Rect,
    overflow_y: bool,
) -> Bars {
    let asked = bars_from_model(&c.scroll_bars, overflow_y, true);
    if !asked.horizontal.visible() {
        return asked;
    }
    let width = p.inset(frame).size.x;
    let mut overflow_x = false;
    for l in lines {
        if p.measure(&l.text).x > width {
            overflow_x = true;
            break;
        }
    }
    bars_from_model(&c.scroll_bars, overflow_y, overflow_x)
}
