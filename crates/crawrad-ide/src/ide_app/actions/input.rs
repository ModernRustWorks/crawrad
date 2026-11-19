//! Keyboard focus and shell text input actions.

use super::super::*;

impl IdeApp {
    pub(in super::super) fn handle_input_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for acao in actions {
            let Some(wa) = acao.as_widget_action() else {
                continue;
            };
            match wa.cast::<TextInputAction>() {
                TextInputAction::KeyFocus => self.campo_focado = wa.widgets.first().cloned(),
                TextInputAction::KeyFocusLost => {
                    self.campo_focado = None;
                    self.confirmar_edicao(cx);
                    self.confirm_event(cx);
                }
                // The field says "I didn't use this key". Only the catalog responds:
                // arrow keys and `Esc` remain outside the field (see `teclado::comandos`).
                TextInputAction::KeyDownUnhandled(ke) => self.tecla(cx, &ke, true),
                _ => {}
            }
        }
    }
}
