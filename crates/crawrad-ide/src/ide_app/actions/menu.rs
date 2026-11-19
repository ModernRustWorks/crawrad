//! Actions for the menu bar, menu popups, window buttons, and toolbar commands.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-actions-menu-ideapp-handle-menu-actions")]
    pub(in super::super) fn handle_menu_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut alternar = None;
        for m in menu::Menu::ALL {
            if self.menu_button(*m).as_button().clicked(actions) {
                alternar = Some(*m);
            }
        }
        if let Some(m) = alternar {
            self.open_menu = if self.open_menu == Some(Aberto::Barra(m)) {
                None
            } else {
                Some(Aberto::Barra(m))
            };
            self.open_submenu = None;
            self.show_menu(cx);
        }

        for (lista, sub) in [
            (id!(menu_popup.menu_list), false),
            (id!(menu_sub_popup.menu_list), true),
        ] {
            let Some(i) = clicked_row(&self.ui.portal_list(lista), actions) else {
                continue;
            };
            let comandos = if sub {
                self.submenu_rows()
            } else {
                self.menu_rows()
            };
            if let Some(linha) = comandos.get(i) {
                self.run_menu_row(cx, linha);
            }
        }

        // The rail's gear opens Settings/Manage; the Run chevron, the Run menu.
        let buttons: [(&[LiveId], Aberto); 3] = [
            (id!(rail_settings), Aberto::SettingsRail),
            (id!(top_titlebar.btn_play_menu), Aberto::RunMenu),
            (id!(top_titlebar.btn_preview_menu), Aberto::LanguageMenu),
        ];
        let mut disparar = None;
        for (button, menu) in buttons {
            if !self.ui.button(button).clicked(actions) {
            } else if menu == Aberto::LanguageMenu
                && !crawrad_actions::menu::shows_preview_language(&self.language_codes)
            {
                disparar = Some(toolbar::Command::Preview); // only the source: the arrow just views
            } else {
                self.toggle_menu(cx, menu);
            }
        }

        if (self.painel(Panel::Inspector))
            .button(id!(inspector_picker.picker_hit))
            .clicked(actions)
        {
            self.toggle_menu(cx, Aberto::InspectorPicker);
        }

        for c in toolbar::Command::ALL {
            if self.bar_button(*c).as_button().clicked(actions) {
                disparar = Some(*c);
            }
        }

        if self.ui.button(id!(top_titlebar.btn_min)).clicked(actions) {
            let janela = self.window_id(cx);
            cx.push_unique_platform_op(CxOsOp::MinimizeWindow(janela));
        }
        let max = self.ui.button(id!(top_titlebar.btn_max)).clicked(actions);
        let restaurar = self
            .ui
            .button(id!(top_titlebar.btn_restore))
            .clicked(actions);
        if max || restaurar {
            self.toggle_maximized(cx);
        }
        if self.ui.button(id!(top_titlebar.btn_close)).clicked(actions) {
            if self.has_anything_to_save() {
                self.confirmando_saida = Some(AfterSaving::Quit);
                self.refresh(cx);
                return;
            }
            let janela = self.window_id(cx);
            cx.push_unique_platform_op(CxOsOp::CloseWindow(janela));
        }

        if let Some(c) = disparar {
            self.close_menu(cx);
            self.execute_action(cx, c.action());
            self.reabrir_no_canvas(cx);
            self.sincronizar(cx);
        }
    }
}
