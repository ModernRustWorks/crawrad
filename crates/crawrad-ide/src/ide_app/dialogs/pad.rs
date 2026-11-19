use super::super::*;
use crawrad_note::note;

#[note("ide-app-dialogs-pad-pad-margem")]
pub(super) const PAD_MARGEM: f64 = 8.0;

impl IdeApp {
    #[note("ide-app-dialogs-pad-ideapp-draw-pad")]
    pub(in super::super) fn draw_pad(&mut self, cx: &mut Cx) {
        let aberto = self.pad.is_some();
        self.ui.view(id!(align_pad)).set_visible(cx, aberto);
        let Some(pad) = self.pad.as_ref() else {
            return;
        };
        let janela = self.ui.area().rect(cx);
        let painel = self.painel(Panel::Inspector).area().rect(cx);
        let w = ALIGN_LARGURA;
        let h = ALIGN_ALTURA + 34.0;
        let (mut x, mut y) = if let Some(pos) = self.pad_pos {
            (pos.x, pos.y + 2.0)
        } else {
            (painel.pos.x + PAD_MARGEM, painel.pos.y + 100.0)
        };
        if x + w > janela.pos.x + janela.size.x - PAD_MARGEM {
            x = (janela.pos.x + janela.size.x - w - PAD_MARGEM).max(janela.pos.x);
        }
        if y + h > janela.pos.y + janela.size.y - PAD_MARGEM {
            if let Some(pos) = self.pad_pos {
                y = (pos.y - h - 24.0).max(janela.pos.y + PAD_MARGEM);
            } else {
                y = (janela.pos.y + janela.size.y - h - PAD_MARGEM).max(janela.pos.y);
            }
        }
        x = x.max(painel.pos.x + PAD_MARGEM);
        y = y.max(janela.pos.y + PAD_MARGEM);
        let alvo = dvec2(x, y);
        self.ui
            .view(id!(align_pad))
            .apply_over(cx, live! { abs_pos: (alvo) });

        if self.pad_na_tela.as_deref() == Some(&pad.key) {
            return;
        }
        self.pad_na_tela = Some(pad.key.clone());
        if let Some(nome) = crawrad_registry::ALIGN_OPTIONS.first() {
            self.ui.button(id!(align_pad.zone_none)).set_text(cx, nome);
        }
    }

    #[note("ide-app-dialogs-pad-ideapp-atender-pad")]
    pub(in super::super) fn atender_pad(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(pad) = self.pad.as_ref() else { return };
        let (no, key) = (pad.no.clone(), pad.key.clone());

        let mut escolhido: Option<&'static str> = None;
        for (caminho, nome) in ALIGN_ZONES {
            if self.ui.button(caminho()).clicked(actions) {
                escolhido = Some(nome);
            }
        }
        if self.ui.button(id!(align_pad.zone_none)).clicked(actions) {
            escolhido = crawrad_registry::ALIGN_OPTIONS.first().copied();
        }
        if let Some(nome) = escolhido {
            let alvos = self.alvos_da_edicao(&no);
            self.write_property(cx, alvos, &key, Value::Enum(nome.to_string()));
            self.fechar_pad(cx);
        }
    }

    #[note("ide-app-dialogs-pad-ideapp-fechar-pad")]
    pub(in super::super) fn fechar_pad(&mut self, cx: &mut Cx) {
        self.pad_pos = None;
        self.execute_with(
            cx,
            inspector::ACTION_CLOSE_EDITOR,
            crawrad_actions::Args::none(),
        );
        self.refresh(cx);
    }
}
