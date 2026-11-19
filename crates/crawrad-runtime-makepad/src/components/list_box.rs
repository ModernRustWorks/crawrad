//! Makepad ListBox component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::{Brush, Content, ContentLine, lines_that_fit};
use crate::components::bar::{bars_from_model, paint_inner_bars};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[note("components-list-box-classe")]
pub const CLASSE: &str = "ListBox";

/// The height of a `ListBox` row, in design units.
///
/// **Mirrors `LINHA_DA_LISTA` from the RXP** (`crawrad-runtime-egui/src/lib.rs:3008`),
/// and the number is not arbitrary: it is egui's default `interact_size.y`, which is the
/// height with which the live form actually draws the row and with which `rxp.rs`
/// emits the application. A designer drawing that disagreed would show eight
/// items where the application shows four — that was divergence **D3** in the
/// background study.
/// **Declared in [`crate::shared_geometry::LIST_ROW_HEIGHT`]**, the file that the
/// code generator also reads. It used to live here, and the generator had its
/// own — the canvas drew one height and the generated application another.
pub use crate::shared_geometry::LIST_ROW_HEIGHT as ALTURA_DA_LINHA;
use crawrad_note::note;

#[note("components-list-box-order")]
fn order(c: &MakepadCtl) -> Vec<usize> {
    if matches!(c.props.get("sorted"), Some(crawrad_ir::Value::Bool(true))) {
        let texts: Vec<&str> = c.itens.iter().map(|i| i.text.as_str()).collect();
        crawrad_runtime::list_selection::sorted_order(&texts)
    } else {
        (0..c.itens.len()).collect()
    }
}

#[note("components-list-box-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn lines(&self, c: &MakepadCtl) -> Vec<ContentLine> {
        order(c)
            .into_iter()
            .map(|i| ContentLine::leaf(0, c.itens[i].text.clone()))
            .collect()
    }

    // The field SINKS — background darker than paper — and has an outline. It gains
    // the scrollbar when content does not fit: the bar is not just fidelity, it
    // INFORMS that there is more list below.
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let linhas = self.lines(c);
        // The bar stays INSIDE the rectangle and **flush against the border**, like
        // the screenshot of `TListBox` from a classic RAD IDE that the owner sent — which is why it measures
        // against the box minus the frame, not against text indentation.
        let caixa = crate::canvas::shrink(r, 1.0);
        let altura = p.medida(ALTURA_DA_LINHA).max(6.0) as f32;
        // How many lines fit is measured on the area that the TEXT will receive, not
        // on the box: measuring on the box caused the last row to be cut off **without** the
        // bar appearing, which is the worst of both worlds — content disappears and
        // nothing indicates it exists.
        let cabem = lines_that_fit(p.inset(caixa).size.y as f32, altura);
        // Horizontal axis is left out: neither `ListBox` nor `TreeView` has the
        // `ScrollBars` property in the catalogue, so both operate in AUTOMATIC
        // mode and their horizontal bar was never drawn.
        let barras = bars_from_model(&c.scroll_bars, linhas.len() > cabem, false);
        let sobrou = paint_inner_bars(p, caixa, barras, 0.0);
        let dentro = p.inset(sobrou);
        p.content(
            dentro,
            &Content {
                lines: &linhas,
                line_height: ALTURA_DA_LINHA,
                color: text_color(c),
                // with `Sorted` the item picked is drawn where it lands
                selected: c
                    .escolhido
                    .and_then(|e| order(c).iter().position(|&i| i == e)),
            },
        );
    }
}
