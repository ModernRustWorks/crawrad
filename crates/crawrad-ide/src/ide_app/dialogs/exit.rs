use super::super::*;
use crawrad_note::note;

/// What the **Unsaved changes** dialog goes on to do once it is answered.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AfterSaving {
    Quit,
    NewProject,
    OpenProject(Option<PathBuf>),
}

impl IdeApp {
    #[note("ide-app-dialogs-exit-ideapp-has-anything-to-save")]
    pub(in super::super) fn has_anything_to_save(&self) -> bool {
        self.host.is_dirty()
            || self
                .host
                .doc()
                .is_some_and(|d| d.revision() != self.rev_salva)
    }

    #[note("ide-app-dialogs-exit-ideapp-draw-quit")]
    pub(in super::super) fn draw_quit(&mut self, cx: &mut Cx) {
        let aberto = self.confirmando_saida.is_some() && self.has_anything_to_save();
        self.show_veil(cx, id!(quit_dialog), aberto);
        if !aberto {
            return;
        }
        let arquivo = self
            .host
            .active_file()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let dlg = self.ui.view(id!(quit_dialog.dlg));
        dlg.label(id!(quit_icone)).set_text(cx, icon::AVISO);
        dlg.label(id!(quit_pergunta)).set_text(cx, QUIT_PERGUNTA);
        dlg.label(id!(quit_arquivo)).set_text(cx, &arquivo);
        dlg.button(id!(quit_save)).set_text(cx, QUIT_SAVE);
        dlg.button(id!(quit_cancel)).set_text(cx, ITEMS_CANCEL);
    }

    #[note("ide-app-dialogs-exit-ideapp-ask-before-closing")]
    pub(in super::super) fn ask_before_closing(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::WindowCloseRequested(e) = event {
            if self.has_anything_to_save() {
                e.accept_close.set(false);
                self.confirmando_saida = Some(AfterSaving::Quit);
                self.refresh(cx);
            } else {
                self.save_desktop(cx);
            }
        }
    }

    #[note("ide-app-dialogs-exit-ideapp-atender-quit")]
    pub(in super::super) fn atender_quit(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(then) = self.confirmando_saida.clone() else {
            return;
        };
        let dlg = self.ui.view(id!(quit_dialog.dlg));
        if dlg.button(id!(quit_cancel)).clicked(actions) {
            self.confirmando_saida = None;
            self.refresh(cx);
            return;
        }
        let salvar = dlg.button(id!(quit_save)).clicked(actions) && self.save_all(cx);
        let descartar = dlg.button(id!(quit_discard)).clicked(actions);
        if salvar || descartar {
            self.confirmando_saida = None;
            if then != AfterSaving::Quit {
                // The project closes; on Discard what was typed in it goes too.
                if descartar {
                    for p in self.host.open_file_paths() {
                        self.host.remove_buffer(&p);
                    }
                    self.host.set_doc(None);
                }
                self.close_current_project(cx);
                self.show_welcome(cx);
            }
            self.proceed(cx, then);
        }
    }

    /// File › New Project and Open Project close the project on screen first, so
    /// unsaved work is asked about here — the same question closing the IDE asks.
    #[note("ide-app-dialogs-exit-ideapp-after-saving")]
    pub(in super::super) fn after_saving(&mut self, cx: &mut Cx, then: AfterSaving) {
        if self.has_anything_to_save() {
            self.confirmando_saida = Some(then);
            return self.refresh(cx);
        }
        self.proceed(cx, then);
    }

    #[note("ide-app-dialogs-exit-ideapp-proceed")]
    fn proceed(&mut self, cx: &mut Cx, then: AfterSaving) {
        match then {
            AfterSaving::Quit => {
                self.save_desktop(cx);
                let janela = self.window_id(cx);
                cx.push_unique_platform_op(CxOsOp::CloseWindow(janela));
            }
            AfterSaving::NewProject => self.open_new_project(cx, None),
            AfterSaving::OpenProject(p) => self.open_project(cx, p),
        }
    }

    /// Closes what the project on screen left open — its saved editor tabs and its
    /// form — so the next project starts clean. Unsaved buffers stay: nothing typed
    /// is dropped here.
    #[note("ide-app-dialogs-exit-ideapp-close-current-project")]
    pub(in super::super) fn close_current_project(&mut self, cx: &mut Cx) {
        self.save_desktop(cx);
        self.desktop_root = None;
        self.close_saved_tabs();
        self.abas.show_design();
        if !self.has_anything_to_save() {
            self.host.set_doc(None);
            self.host.set_active_file(None);
        }
    }

    #[note("ide-app-dialogs-exit-ideapp-close-saved-tabs")]
    pub(in super::super) fn close_saved_tabs(&mut self) {
        let dirty: std::collections::HashSet<_> = self.host.open_file_paths().into_iter().collect();
        self.abas.retain(|p| dirty.contains(p));
    }

    // --- THE IMAGELIST EDITOR and the icon catalogue ---
}
