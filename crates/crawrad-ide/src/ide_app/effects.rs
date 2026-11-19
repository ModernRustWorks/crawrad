//! apply_effect and execute_with: an action's `Efeito` applied to the window.
//! Cut out of `ide_app.rs` as a pure move, as `chat.rs` and `build.rs` were.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-effects-ideapp-execute-action")]
    pub fn execute_action(&mut self, cx: &mut Cx, action_id: &str) {
        self.execute_with(cx, action_id, crawrad_actions::Args::none());
    }

    #[note("ide-app-effects-ideapp-execute-with")]
    pub(super) fn execute_with(
        &mut self,
        cx: &mut Cx,
        action_id: &str,
        args: crawrad_actions::Args,
    ) {
        // 1. Dynamic dispatch via unified CapabilityRegistry
        if self.capabilities.contains(action_id) {
            let empty = crawrad_actions::capability::CapabilityRegistry::new();
            let mut caps = std::mem::replace(&mut self.capabilities, empty);
            let dispatched = caps.dispatch(action_id, &self.action_context(args), None);
            self.capabilities = caps;
            match dispatched {
                Ok(efeito) => {
                    // Also executes any procedural handler active in the extension host
                    let _ = self.extensions.execute_command(action_id);
                    let _ = self.apply_effect(cx, efeito);
                    return;
                }
                Err(crawrad_actions::capability::CapabilityError::Disabled(_)) => {
                    let label = self
                        .capabilities
                        .get(action_id)
                        .map(|c| c.label.as_str())
                        .unwrap_or(action_id);
                    self.status = Some(format!("{} is not available now", label));
                    return;
                }
                Err(err) => {
                    self.status = Some(format!("{err}"));
                    return;
                }
            }
        }

        let ctx = self.action_context(args);
        // 2. If it is a command registered directly in ShellExtensionHost
        if self.extensions.execute_command(action_id).is_ok() {
            self.sincronizar_extensoes();
            return;
        }

        // 3. Fallback to legacy native catalogue
        let Some(action) = crawrad_actions::catalogue()
            .into_iter()
            .find(|a| a.id == action_id)
        else {
            self.status = Some(format!("unknown action: {action_id}"));
            return;
        };
        if !action.enabled.allows(&ctx) {
            self.status = Some(format!("{} is not available now", action.label));
            return;
        }
        let efeito = (action.execute)(&ctx);
        let _ = self.apply_effect(cx, efeito);
    }

    #[note("ide-app-effects-ideapp-action-context")]
    pub(super) fn action_context(
        &self,
        args: crawrad_actions::Args,
    ) -> crawrad_actions::Context<'_> {
        crawrad_actions::Context {
            doc: self.host.doc(),
            registry: &crawrad_project::installed::ide_catalog().catalog,
            clipboard: &self.transferencia,
            args,
            project: self.projeto_contexto(),
            author: crawrad_actions::Author::User,
            editor_dirty: self.editor_sujo(),
            text: self.editor_text(),
        }
    }

    #[note("ide-app-effects-ideapp-refusal")]
    pub(super) fn refusal(&self, action_id: &str, args: &crawrad_actions::Args) -> Option<String> {
        let ctx = self.action_context(args.clone());
        let (label, on) = match self.capabilities.get(action_id) {
            Some(c) => (c.label.clone(), c.is_enabled(&ctx)),
            None => {
                let a = crawrad_actions::catalogue()
                    .into_iter()
                    .find(|a| a.id == action_id)?;
                (a.label.to_string(), a.enabled.allows(&ctx))
            }
        };
        if let Some(why) = crawrad_mcp::dialogs::modal_refusal(self.modal_open(), &label) {
            return Some(why.to_string());
        }
        let unmet = crawrad_actions::by_id(action_id)
            .map(|a| a.enabled.need)
            .filter(|n| !n.met(&ctx));
        (!on).then(|| match unmet {
            Some(n) => format!("{label} needs {}", n.reason()),
            None => format!("{label} is not available now"),
        })
    }

    #[note("ide-app-effects-ideapp-apply-effect")]
    pub fn apply_effect(
        &mut self,
        cx: &mut Cx,
        efeito: crawrad_actions::Effect,
    ) -> Result<(), String> {
        let mut err_res = Ok(());
        let antes = self
            .host
            .doc()
            .map(|d| (d.revision(), d.selection().to_vec()));
        if let Some(tx) = efeito.transaction {
            if let Some(d) = self.host.doc_mut() {
                if let Err(e) = d.apply(tx) {
                    self.status = Some(format!("doc: {e}"));
                    err_res = Err(format!("Document error: {e}"));
                }
            } else {
                err_res = Err("No document open to apply transaction".to_string());
            }
            self.reacomodar(cx);
        }
        if let Some(sel) = efeito.selection
            && let Some(d) = self.host.doc_mut()
        {
            d.select(sel);
        }
        match efeito.history {
            Some(crawrad_actions::History::Undo) => {
                if let Some(active_file) = self.active_file_path()
                    && self.desfazer_editor(cx, &active_file)
                {
                    return Ok(());
                }
                if let Some(d) = self.host.doc_mut()
                    && let Err(e) = d.undo()
                {
                    self.status = Some(format!("undo: {e}"));
                    err_res = Err(format!("Undo error: {e}"));
                }
                self.form_anterior = self
                    .host
                    .doc()
                    .and_then(|d| d.ir().node(&d.ir().root))
                    .and_then(|n| n.rect());
            }
            Some(crawrad_actions::History::Redo) => {
                if let Some(d) = self.host.doc_mut()
                    && let Err(e) = d.redo()
                {
                    self.status = Some(format!("redo: {e}"));
                    err_res = Err(format!("Redo error: {e}"));
                }
                self.form_anterior = self
                    .host
                    .doc()
                    .and_then(|d| d.ir().node(&d.ir().root))
                    .and_then(|n| n.rect());
            }
            None => {}
        }
        match efeito.file {
            Some(crawrad_actions::FileEffect::Save) => self.salvar(cx),
            Some(crawrad_actions::FileEffect::SaveAll) => _ = self.save_all(cx),
            Some(crawrad_actions::FileEffect::Reload) => self.reload_form_from_disk(cx),
            Some(crawrad_actions::FileEffect::Close) => self.fechar_documento(cx),
            Some(crawrad_actions::FileEffect::Generate) => self.gerar_projeto(cx),
            Some(crawrad_actions::FileEffect::NewProject) => {
                self.after_saving(cx, AfterSaving::NewProject)
            }
            Some(crawrad_actions::FileEffect::OpenProject(p)) => {
                self.after_saving(cx, AfterSaving::OpenProject(p));
            }
            Some(crawrad_actions::FileEffect::OpenForm(p)) => self.open_form_file(cx, p),
            Some(crawrad_actions::FileEffect::OpenBaseForm) => {
                let base = self
                    .host
                    .doc()
                    .and_then(|d| d.ir().inherits().map(str::to_string));
                if let (Some(form), Some(base)) = (self.host.active_file(), base) {
                    let path = form.with_file_name(format!("{base}.crawform"));
                    self.open_form_file(cx, Some(path));
                }
            }
            Some(crawrad_actions::FileEffect::ForgetRecent(p)) => self.forget_recent(cx, &p),
            Some(
                c @ (crawrad_actions::FileEffect::AddProject { .. }
                | crawrad_actions::FileEffect::AddExistingProject(_)),
            ) => self.workspace_command(cx, c),
            Some(crawrad_actions::FileEffect::SetActiveProject(member)) => {
                self.set_active_project(cx, member)
            }
            Some(crawrad_actions::FileEffect::Run) => self.run_or_continue(cx),
            Some(crawrad_actions::FileEffect::OpenInEditor(_)) => {
                self.status = Some(Self::SEM_ABRIR_NO_EDITOR.into())
            }
            Some(crawrad_actions::FileEffect::NewInheritedForm) => self.open_inherit_form(cx),
            Some(crawrad_actions::FileEffect::NewComponent) => self.open_new_component(cx),
            Some(crawrad_actions::FileEffect::NewForm) => {
                self.criar_novo_formulario(cx, None);
            }
            Some(crawrad_actions::FileEffect::NewFile) => {
                self.create_new_file(cx, None);
            }
            Some(crawrad_actions::FileEffect::Recover(f)) => self.answer_recovery(cx, f, false),
            Some(crawrad_actions::FileEffect::DiscardRecovery(f)) => {
                self.answer_recovery(cx, f, true)
            }
            Some(crawrad_actions::FileEffect::OpenInTab(rel)) => self.abrir_aba_relativa(cx, &rel),
            Some(crawrad_actions::FileEffect::BuildAll) => {
                self.execute_build_all(cx, BuildMode::Build)
            }
            Some(crawrad_actions::FileEffect::CleanAll) => {
                self.execute_build_all(cx, BuildMode::Clean)
            }
            None => {}
        }
        if let Some(comando) = efeito.layout {
            self.aplicar_arranjo(cx, comando);
        }
        if let Some(comando) = efeito.profile {
            self.run_profile(cx, comando);
        }
        if let Some(comando) = efeito.explorer {
            self.aplicar_dobra_do_explorer(cx, comando);
        }
        if let Some(comando) = efeito.inspector {
            self.aplicar_comando_do_inspector(cx, comando);
        }
        match efeito.debug {
            Some(crawrad_actions::DebugCommand::ToggleBreakpoint) => self.toggle_breakpoint(cx),
            Some(crawrad_actions::DebugCommand::StepOver) => {
                self.debug_command(crawrad_dap::Command::StepOver)
            }
            Some(crawrad_actions::DebugCommand::StepInto) => {
                self.debug_command(crawrad_dap::Command::StepInto)
            }
            Some(crawrad_actions::DebugCommand::StepOut) => {
                self.debug_command(crawrad_dap::Command::StepOut)
            }
            Some(crawrad_actions::DebugCommand::Pause) => {
                self.debug_command(crawrad_dap::Command::Pause)
            }
            Some(crawrad_actions::DebugCommand::Stop) if self.web_run.stop() => {
                self.status = Some("stopped".into())
            }
            Some(crawrad_actions::DebugCommand::Stop) => {
                self.debug_command(crawrad_dap::Command::Stop)
            }
            Some(crawrad_actions::DebugCommand::AddWatch(expressao)) => {
                self.add_watch(cx, expressao)
            }
            Some(crawrad_actions::DebugCommand::RemoveWatch(i)) => self.remove_watch(cx, i),
            Some(crawrad_actions::DebugCommand::RunWithoutDebugging) => {
                self.host.debug_mut().run_without_debugger_once();
                self.rodar_projeto(cx);
            }
            None => {}
        }
        if let Some(command) = efeito.lsp {
            self.lsp_command(command);
        }
        match efeito.component {
            Some(crawrad_actions::ComponentEffect::Install(p)) => self.instalar_componente(cx, p),
            Some(crawrad_actions::ComponentEffect::Installed) => self.show_installed(cx),
            None => {}
        }
        match efeito.view {
            Some(crawrad_actions::ViewEffect::ToggleDesignCode) => self.toggle_view(cx),
            Some(crawrad_actions::ViewEffect::ToggleFormSource) => self.toggle_form_source(cx),
            Some(crawrad_actions::ViewEffect::ToggleProblems) => {
                if self.painel_a_vista(&self.arranjo(), Panel::Problems) {
                    self.esconder_painel(cx, Panel::Problems);
                } else {
                    self.show_panel(cx, Panel::Problems);
                }
            }
            Some(crawrad_actions::ViewEffect::ToggleWelcome) => self.show_welcome(cx),
            Some(crawrad_actions::ViewEffect::Options) => self.open_options(cx),
            Some(crawrad_actions::ViewEffect::ProjectOptions) => self.open_project_options(cx),
            Some(crawrad_actions::ViewEffect::Note(kind)) => self.open_note_cursor(cx, kind),
            Some(crawrad_actions::ViewEffect::Mcp) => self.open_mcp(cx),
            Some(crawrad_actions::ViewEffect::AiMemory) => self.open_memory(cx),
            Some(crawrad_actions::ViewEffect::EnvironmentCheck) => self.open_environment(cx),
            Some(crawrad_actions::ViewEffect::QuickOpen) => self.open_quick_open(cx),
            Some(crawrad_actions::ViewEffect::CommandPalette) => self.open_command_palette(cx),
            Some(crawrad_actions::ViewEffect::Build) => self.open_build(cx),
            Some(crawrad_actions::ViewEffect::ManageTemplates) => self.open_manage_templates(cx),
            None => {}
        }
        self.editor_command(cx, efeito.editor);
        if let Some(jump) = &efeito.handler {
            self.open_handler(cx, jump);
        }
        if let Some(crawrad_actions::DesignerMode::ToggleLive) = efeito.designer {
            self.vivo = !self.vivo;
            let mode = if self.vivo {
                crawrad_runtime::Mode::Live
            } else {
                crawrad_runtime::Mode::Design
            };
            let canvas = self.canvas();
            if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
                c.runtime_mut().set_mode(mode);
            }
            canvas.redraw(cx);
            // The eye next to Run is lit while live: the document does not change, so no sync runs.
            let colour = if self.vivo {
                vec4(1.0, 0.612, 0.416, 1.0)
            } else {
                vec4(0.906, 0.906, 0.918, 1.0)
            };
            self.bar_button(toolbar::Command::Preview)
                .apply_over(cx, live! { draw_text: { color: (colour) } });
            self.status = Some(
                if self.vivo {
                    "preview — the form as the product shows it; Run to operate it"
                } else {
                    "design — ruler, handles and grid are back"
                }
                .into(),
            );
            self.ui.redraw(cx);
        }
        if let Some(crawrad_actions::DesignerMode::ToggleLock) = efeito.designer {
            self.designer.locked = !self.designer.locked;
            self.status = Some(
                if self.designer.locked {
                    "controls locked — the mouse no longer moves or resizes them"
                } else {
                    "controls unlocked"
                }
                .into(),
            );
        }
        if let Some(pedido) = efeito.ui_zoom {
            self.aplicar_zoom_da_ide(cx, pedido);
        }
        if let Some(pedido) = efeito.zoom {
            let novo = match pedido {
                crawrad_actions::Zoom::In => crawrad_runtime::adjacent_zoom(self.zoom, true),
                crawrad_actions::Zoom::Out => crawrad_runtime::adjacent_zoom(self.zoom, false),
                crawrad_actions::Zoom::Reset => crawrad_runtime::ZOOM_ONE,
            };
            self.zoom = novo;
            crawrad_project::workspace::update_preferences(|p| p.designer_scale = self.zoom as f64);
            self.aplicar_zoom(cx);
            self.status = Some(format!("zoom {}%", (novo * 100.0).round()));
        }
        if let Some(texto) = efeito.copy_text {
            if let Some(a) = Clipboard::from_text(&texto) {
                self.transferencia = a;
            }
            cx.copy_to_clipboard(&texto);
        }
        if let Some(s) = efeito.status {
            self.status = Some(s);
        }
        if let Some(code) = efeito.language {
            let before = self.design_language.clone();
            let chosen = self.set_design_language(Some(code.as_str()));
            if chosen.is_ok() && self.design_language != before {
                self.reabrir_no_canvas(cx);
                self.check_form(cx);
                self.check_sizes(cx);
                self.sincronizar(cx);
                self.ui.redraw(cx);
            }
        }
        if let Some(code) = efeito.run_language {
            let next_run = if code.is_empty() || code == "system" {
                None
            } else {
                Some(code)
            };
            if self.run_language != next_run {
                self.run_language = next_run;
                self.sincronizar(cx);
                self.ui.redraw(cx);
            }
        }
        let depois = self
            .host
            .doc()
            .map(|d| (d.revision(), d.selection().to_vec()));
        if antes != depois {
            self.reabrir_no_canvas(cx);
            self.sincronizar(cx);
            self.ui.redraw(cx);
        }
        err_res
    }

    /// One active project: the designer follows it (approved mockup, 13/11).
    pub(super) fn set_active_project(&mut self, cx: &mut Cx, member: std::path::PathBuf) {
        self.workspace_command(
            cx,
            crawrad_actions::FileEffect::SetActiveProject(member.clone()),
        );
        if let Some(form) = crawrad_project::workspace::member_documents(&member).first() {
            self.abrir_formulario(cx, form);
        }
    }
}
