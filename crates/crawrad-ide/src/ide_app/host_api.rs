//! The actions' context and the host API the MCP and extensions call. Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;

impl IdeApp {
    // --- ACTIONS — everything goes through the `crawrad-actions` catalog ---

    #[note("ide-app-host-api-ideapp-projeto-contexto")]
    pub fn projeto_contexto(&self) -> crawrad_actions::Project {
        let cargo_target = self.alvo_do_cargo();
        let tem_bin = (cargo_target.as_ref())
            .is_none_or(|(raiz, pacote)| crawrad_project::workspace::has_binary(raiz, pacote));
        let src = self.explorer_target_folder(None);
        let debug = match self.host.debug().phase() {
            crawrad_host::debug::Phase::Stopped => crawrad_actions::DebugSession::Stopped,
            crawrad_host::debug::Phase::Starting | crawrad_host::debug::Phase::Running => {
                crawrad_actions::DebugSession::Running
            }
            _ if self.web_run.serving() => crawrad_actions::DebugSession::Running,
            _ => crawrad_actions::DebugSession::None,
        };
        crawrad_actions::Project {
            has_binary: tem_bin,
            has_forms: !crawrad_project::inherited_form::forms_in(&src).is_empty(),
            open: cargo_target.is_some(),
            is_workspace: self
                .raiz()
                .as_deref()
                .is_some_and(crawrad_project::active_project::is_workspace),
            debug,
        }
    }

    #[note("ide-app-host-api-ideapp-execute-custom-settings-menu")]
    pub(in crate::ide_app) fn execute_custom_settings_menu(&mut self, cx: &mut Cx, label: &str) {
        if let Some(page) = options::page_of_menu_row(label) {
            self.open_options(cx);
            self.secao = page;
            return self.redraw_options(cx);
        }
        if let Some(note) = menu::placeholder_note(label) {
            self.status = Some(note.into());
            return self.sincronizar(cx);
        }
        match label {
            "Close All" => {
                self.abas.close_all();
                self.show_center(cx, self.centro_das_abas());
            }
            "Close Saved" => {
                self.close_saved_tabs();
                self.show_center(cx, self.centro_das_abas());
            }
            _ => {}
        }
    }

    #[note("ide-app-host-api-ideapp-sem-abrir-no-editor")]
    pub(super) const SEM_ABRIR_NO_EDITOR: &'static str =
        "opening files in an editor is not wired in this shell yet — use the egui shell";
    #[note("ide-app-host-api-ideapp-trabalho-nao-salvo")]
    pub(super) const TRABALHO_NAO_SALVO: &'static str =
        "the open form has unsaved changes — save them or undo them first";

    pub fn center(&self) -> Center {
        self.host.center()
    }

    pub fn set_center(&mut self, center: Center) {
        self.host.set_center(center);
    }

    pub fn doc(&self) -> Option<&Doc> {
        self.host.doc()
    }

    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        self.host.doc_mut()
    }

    pub fn area_transferencia(&self) -> &crawrad_designer::clipboard::Clipboard {
        &self.transferencia
    }

    pub fn capabilities(&self) -> &CapabilityRegistry {
        &self.capabilities
    }

    pub fn capabilities_mut(&mut self) -> &mut CapabilityRegistry {
        &mut self.capabilities
    }

    pub fn extensions(&self) -> &ShellExtensionHost {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut ShellExtensionHost {
        &mut self.extensions
    }

    pub fn execute_extension_command(
        &self,
        action_id: &str,
    ) -> Result<(), crawrad_extension_api::error::ExtensionError> {
        self.extensions.execute_command(action_id)
    }

    pub fn execute_extension_command_with_args(
        &self,
        action_id: &str,
        args: Option<serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, crawrad_extension_api::error::ExtensionError> {
        self.extensions.execute_command_with_args(action_id, args)
    }

    pub fn take_extension_messages(
        &self,
    ) -> Vec<(crawrad_extension_api::traits::MessageLevel, String)> {
        self.extensions.take_messages()
    }

    pub fn extension_card_manager(
        &self,
    ) -> std::sync::Arc<std::sync::Mutex<crawrad_extension_host::ExtensionCardManager>> {
        self.extensions.card_manager()
    }

    pub fn dispatch_extension_ui_event(
        &mut self,
        card_id: &str,
        element_id: &str,
        event: crawrad_extension_api::ui::UiEventKind,
    ) -> Result<(), crawrad_extension_api::error::ExtensionError> {
        self.extensions
            .dispatch_ui_event(card_id, element_id, event)
    }

    pub fn dispatch_capability(
        &mut self,
        cap_id: &str,
        args: crawrad_actions::Args,
        author: crawrad_actions::Author,
        active_view: Option<crawrad_actions::ViewAffinity>,
    ) -> Result<crawrad_actions::Effect, crawrad_actions::capability::CapabilityError> {
        let registry = &crawrad_project::installed::ide_catalog().catalog;
        let ctx = crawrad_actions::Context {
            doc: self.host.doc(),
            registry,
            clipboard: &self.transferencia,
            args,
            project: self.projeto_contexto(),
            author,
            editor_dirty: self.editor_sujo(),
            text: self.editor_text(),
        };
        self.capabilities.dispatch(cap_id, &ctx, active_view)
    }

    #[note("ide-app-host-api-ideapp-land-edit")]
    pub fn land_edit(
        &mut self,
        tool: &str,
        path: std::path::PathBuf,
        (before, after): (String, String),
        gate: bool,
    ) -> Result<Option<crawrad_agent::diff_review::DiffReview>, String> {
        use crawrad_agent::diff_review::DiffReviewMode;
        let mode = self.acp_config.diff_review_mode;
        if gate && mode != DiffReviewMode::ApplyDirectly {
            let review =
                self.propose_diff_review(path.clone(), before.clone(), after.clone(), tool);
            if mode == DiffReviewMode::WaitForApproval {
                return review.map(Some);
            }
        }
        self.editor_undo_stacks
            .entry(path.clone())
            .or_default()
            .push(before);
        self.set_editor_buffer(path, after);
        Ok(None)
    }

    pub fn set_editor_buffer(&mut self, path: std::path::PathBuf, content: String) {
        self.host.set_buffer(path, content);
    }

    pub fn edicoes_paths(&self) -> Vec<std::path::PathBuf> {
        self.host.open_file_paths()
    }

    pub fn caminho_ativo(&self) -> Option<&std::path::Path> {
        self.host.active_file()
    }

    pub fn raiz_projeto(&self) -> Option<std::path::PathBuf> {
        self.host.workspace_root()
    }

    pub fn abas(&self) -> &crate::view_models::editor_tabs::EditorTabs {
        &self.abas
    }

    pub fn active_file_path(&self) -> Option<std::path::PathBuf> {
        self.abas
            .active_file()
            .map(|p| p.to_path_buf())
            .or_else(|| self.host.active_file().map(|p| p.to_path_buf()))
    }

    pub fn close_editor_buffer(&mut self, path: &std::path::Path) {
        self.host.remove_buffer(path);
    }

    #[note("ide-app-host-api-ideapp-aplicar-zoom")]
    pub(super) fn aplicar_zoom(&mut self, cx: &mut Cx) {
        let canvas = self.canvas();
        if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
            c.runtime_mut().set_zoom(self.zoom);
        }
        canvas.redraw(cx);
    }
}
