use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-icons-ideapp-draw-icons")]
    pub(in super::super) fn draw_icons(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(icons_dialog), self.icones);
        if !self.icones {
            return;
        }
        let dlg = self.ui.view(id!(icons_dialog.dlg));
        dlg.label(id!(ic_nota)).set_text(cx, IC_NOTA);
        dlg.button(id!(ic_fechar)).set_text(cx, ACT_CLOSE);
    }

    pub(in super::super) fn atender_icones(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.icones {
            return;
        }
        if self
            .ui
            .view(id!(icons_dialog.dlg))
            .button(id!(ic_fechar))
            .clicked(actions)
        {
            self.icones = false;
            self.refresh(cx);
        }
    }

    // --- NEW PROJECT e CREATING PROJECT — o `cargo` faz, a IDE pergunta ---
}
