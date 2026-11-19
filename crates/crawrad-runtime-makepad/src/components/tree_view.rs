//! Makepad TreeView component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::{Brush, Content, ContentLine, lines_that_fit};
use crate::components::bar::{bars_from_model, paint_inner_bars};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[note("components-tree-view-classe")]
pub const CLASSE: &str = "TreeView";

/// The height of a tree row, in design units. This is `t.medida(17.0)`
/// from the RXP (`crawrad-runtime-egui/src/lib.rs:2215`).
/// **Declared in [`crate::shared_geometry::TREE_ROW_HEIGHT`]**, the file that the
/// code generator also reads. It used to live here, and the generator had its
/// own — the canvas drew one height and the generated application another.
pub use crate::shared_geometry::TREE_ROW_HEIGHT as ALTURA_DA_LINHA;
use crawrad_note::note;

#[note("components-tree-view-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn lines(&self, c: &MakepadCtl) -> Vec<ContentLine> {
        let mut fora = Vec::new();
        achatar(&c.itens, &c.item_symbols, 0, &mut fora);
        fora
    }

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
                selected: None,
            },
        );
    }
}

#[note("components-tree-view-achatar")]
fn achatar(
    itens: &[crawrad_ir::Item],
    symbols: &[String],
    nivel: usize,
    fora: &mut Vec<ContentLine>,
) {
    if nivel > 32 {
        return;
    }
    for it in itens {
        let line_text = match it.symbol(symbols) {
            Some(icon) => format!("{icon} {}", it.text),
            None => it.text.clone(),
        };
        // Nodes with children get an expand marker — Windows `TreeView`'s `[-]`,
        // or `[+]` when collapsed. Without it *"the tree is an indented list"*,
        // which was how the owner saw it. The answer only exists here: once flattened, no one
        // knows who had children.
        if it.children.is_empty() {
            fora.push(ContentLine::leaf(nivel, line_text));
        } else {
            fora.push(ContentLine::branch_expanded(nivel, line_text, it.expanded));
            if it.expanded {
                achatar(&it.children, symbols, nivel + 1, fora);
            }
        }
    }
}
