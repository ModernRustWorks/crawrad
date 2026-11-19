//! Makepad PageControl component, drawn on the canvas.

use crawrad_ir::Rect as RectIr;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::{Brush, shrink};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-page-control-classe")]
pub const CLASSE: &str = "PageControl";

#[note("components-page-control-desenho")]
pub struct Desenho;

#[note("components-page-control-altura-da-tira-de-abas")]
pub const ALTURA_DA_TIRA_DE_ABAS: f32 = 24.0;

#[note("components-page-control-largura-da-aba")]
pub const LARGURA_DA_ABA: f32 = 92.0;

#[note("components-page-control-largura-do-mais")]
pub const LARGURA_DO_MAIS: f32 = 22.0;

#[note("components-page-control-aba-i")]
pub fn aba_i(r: RectIr, i: usize) -> Option<RectIr> {
    let x = r.x + LARGURA_DA_ABA * i as f32;
    if x + 4.0 > r.x + r.w {
        return None;
    }
    let w = LARGURA_DA_ABA.min(r.x + r.w - x);
    Some(RectIr::new(x, r.y, w, ALTURA_DA_TIRA_DE_ABAS.min(r.h)))
}

#[note("components-page-control-plus-rectangle")]
pub fn plus_rectangle(r: RectIr) -> RectIr {
    RectIr::new(
        r.x + r.w - LARGURA_DO_MAIS,
        r.y,
        LARGURA_DO_MAIS,
        ALTURA_DA_TIRA_DE_ABAS.min(r.h),
    )
}

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[note("components-page-control-desenho-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        let alt = p.medida(ALTURA_DA_TIRA_DE_ABAS).min(r.size.y);
        // `TabPosition` bottom puts the strip under the pages
        let bottom = matches!(
            c.props.get("tab_position"),
            Some(crawrad_ir::Value::Enum(t)) if t == "tpBottom"
        );
        let dy = if bottom {
            (c.rect.h - ALTURA_DA_TIRA_DE_ABAS).max(0.0)
        } else {
            0.0
        };
        let lowered = |x: RectIr| RectIr::new(x.x, x.y + dy, x.w, x.h);
        let corpo = Rect {
            pos: dvec2(r.pos.x, if bottom { r.pos.y } else { r.pos.y + alt }),
            size: dvec2(r.size.x, (r.size.y - alt).max(0.0)),
        };
        super::panel::caixa_de_superficie(p, corpo);

        // The pages are the `TabSheet` children, in document order — what knows
        // the class name is its own file, not this one.
        let paginas: Vec<String> = scene
            .children_of_class(c, super::tab_sheet::CLASSE)
            .map(|f| f.text.clone())
            .collect();

        // Only the window of tabs that fits, holding the open one; the
        // `<` `>` buttons take the right end when the tabs overflow.
        // out of range falls back to the first page, as `Designer::hit_tab` does
        let open_tab = if c.pagina_ativa < paginas.len() {
            c.pagina_ativa
        } else {
            0
        };
        let (first, count, arrows) =
            crate::shared_geometry::tab_window(c.rect.w, paginas.len(), open_tab, LARGURA_DO_MAIS);
        for slot in 0..count {
            let i = first + slot;
            let Some(aba) = aba_i(c.rect, slot) else {
                break;
            };
            let aba = p.na_tela(lowered(aba));
            let ativa = i == c.pagina_ativa;
            let fundo = if ativa { MAKEPAD_PANEL } else { MAKEPAD_PAPER };
            p.rounded_box(aba, fundo, MAKEPAD_PANEL_BORDER, 2.0);
            let rotulo = shrink(aba, 4.0);
            let cor_aba = if ativa {
                MAKEPAD_TEXT
            } else {
                MAKEPAD_TEXT_DIM
            };
            if let Some(nome) = paginas.get(i) {
                p.write_caption(rotulo, nome, cor_aba, true);
            }
        }

        if arrows {
            let btn_w = crate::shared_geometry::TAB_SCROLL_BUTTON_WIDTH;
            let lx = c.rect.x + c.rect.w - LARGURA_DO_MAIS - 2.0 * btn_w;
            let btn_h = ALTURA_DA_TIRA_DE_ABAS.min(c.rect.h);

            let left_rect = p.na_tela(lowered(RectIr::new(lx, c.rect.y, btn_w, btn_h)));
            p.rounded_box(left_rect, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
            p.write(left_rect, "<", MAKEPAD_TEXT, true);

            let right_rect = p.na_tela(lowered(RectIr::new(lx + btn_w, c.rect.y, btn_w, btn_h)));
            p.rounded_box(right_rect, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
            p.write(right_rect, ">", MAKEPAD_TEXT, true);
        }

        let mais = p.na_tela(lowered(plus_rectangle(c.rect)));
        p.rounded_box(mais, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
        p.write(mais, "+", MAKEPAD_TEXT, true);
    }
}
