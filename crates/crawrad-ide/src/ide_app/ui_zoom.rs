//! **THE ZOOM OF THE WHOLE IDE** — menus, panels and the canvas together.
//!
//! Asked for twice, and why the first attempt bound the keys to the wrong zoom:
//! the shell module notes, under *The zoom of the whole IDE*.

use super::*;

#[cfg(test)]
pub(super) use crawrad_host::ui_scale::STEPS as DEGRAUS;
pub(super) use crawrad_host::ui_scale::{ONE as UM, next_step as degrau_vizinho};
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-ui-zoom-ideapp-aplicar-zoom-da-ide")]
    pub(super) fn aplicar_zoom_da_ide(&mut self, cx: &mut Cx, pedido: crawrad_actions::Zoom) {
        let fator = match pedido {
            crawrad_actions::Zoom::In => degrau_vizinho(self.zoom_da_ide, true),
            crawrad_actions::Zoom::Out => degrau_vizinho(self.zoom_da_ide, false),
            crawrad_actions::Zoom::Reset => UM,
        };
        self.set_ide_zoom(cx, fator);
    }

    #[note("ide-app-ui-zoom-ideapp-set-ide-zoom")]
    pub(super) fn set_ide_zoom(&mut self, cx: &mut Cx, fator: f64) {
        let janela = self.window_id(cx);
        let base = self.dpi_da_area_de_trabalho(cx);
        self.zoom_da_ide = fator;
        if crawrad_project::workspace::read_preferences().ide_scale != fator {
            crawrad_project::workspace::update_preferences(|p| p.ide_scale = fator);
        }
        cx.windows[janela].dpi_override = Some(base * fator);
        crawrad_os::nudge_resize();
        self.status = Some(format!("interface {}%", (fator * 100.0).round()));
        // A zoom asked over MCP comes with no key event behind it to repaint the footer.
        self.refresh_footer(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-ui-zoom-ideapp-dpi-da-area-de-trabalho")]
    fn dpi_da_area_de_trabalho(&mut self, cx: &Cx) -> f64 {
        if self.dpi_base > 0.0 {
            return self.dpi_base;
        }
        let janela = self.window_id(cx);
        let d = cx.windows[janela].window_geom.dpi_factor;
        self.dpi_base = if d > 0.0 { d } else { 1.0 };
        self.dpi_base
    }

    #[note("ide-app-ui-zoom-ideapp-com-ponteiro-na-escala")]
    pub(super) fn com_ponteiro_na_escala(&self, cx: &Cx, event: &Event) -> Option<Event> {
        let escala = 1.0 / self.zoom_da_ide;
        if self.zoom_da_ide == UM {
            return None;
        }
        let _ = cx;
        let escalar = |p: DVec2| p * escala;
        Some(match event {
            Event::MouseDown(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::MouseDown(e)
            }
            Event::MouseMove(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::MouseMove(e)
            }
            Event::MouseUp(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::MouseUp(e)
            }
            Event::MouseLeave(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::MouseLeave(e)
            }
            Event::Scroll(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::Scroll(e)
            }
            Event::LongPress(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::LongPress(e)
            }
            Event::Drag(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::Drag(e)
            }
            Event::Drop(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::Drop(e)
            }
            Event::WindowDragQuery(e) => {
                let mut e = e.clone();
                e.abs = escalar(e.abs);
                Event::WindowDragQuery(e)
            }
            _ => return None,
        })
    }
}

#[cfg(test)]
#[path = "../test/ide_app/ui_zoom.rs"]
mod test;
