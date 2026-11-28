//! Shell BUILD AND PROJECT: compilation, code generation, saving, and lifecycle.
//!
//! Split from `ide_app.rs` per ADR 0043.
//! This module is a part of `impl IdeApp`, preserving visibility and private fields.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-build-ideapp-bombear")]
    pub(super) fn bombear(&mut self, cx: &mut Cx) {
        self.check_build_run_cancel(cx);
        let canvas = self.canvas();
        let eventos = {
            let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() else {
                return;
            };
            c.runtime_mut().drain_events()
        };
        if eventos.is_empty() {
            return;
        }

        let mut marquee = None;
        let mut guias = Vec::new();
        let mut ghost = None;

        for ev in eventos {
            if let RuntimeEvent::PointerDown { x, y, .. } = ev {
                self.status = None;
                if let Some(classe) = self.tool_armed.take() {
                    self.colocar(cx, &classe, x, y);
                    continue;
                }
                if let Some(acao) = self
                    .host
                    .doc()
                    .and_then(|d| self.designer.hit_tab(d.ir(), x, y))
                {
                    self.acao_de_aba(cx, acao);
                    continue;
                }
            }
            if let RuntimeEvent::PointerDblClick { x, y } = ev {
                if let Some(id) = self
                    .host
                    .doc()
                    .and_then(|d| self.designer.hit(d.ir(), x, y))
                {
                    self.abrir_handler_do_no(cx, &id);
                }
                continue;
            }

            let sel: Vec<NodeId> = self.host.selection().to_vec();
            let Some(d) = self.host.doc() else { break };
            let out = match ev {
                RuntimeEvent::PointerDown { x, y, ctrl } => {
                    self.designer.pointer_down(d.ir(), &sel, x, y, ctrl)
                }
                RuntimeEvent::PointerMove { x, y, free } => {
                    self.designer.free = free;
                    self.designer.pointer_move(d.ir(), x, y)
                }
                RuntimeEvent::PointerUp { .. } => self.designer.pointer_up(d.ir()),
                // Handled ABOVE, with `&mut self` in hand.
                RuntimeEvent::PointerDblClick { .. } => continue,
                RuntimeEvent::Cancel => self.designer.cancel(),
            };

            if let Some(nova) = out.select
                && let Some(dm) = self.host.doc_mut()
            {
                dm.select(nova);
            }
            if let Some(tx) = out.tx {
                if let Some(dm) = self.host.doc_mut()
                    && let Err(e) = dm.apply(tx)
                {
                    self.status = Some(format!("doc: {e}"));
                }
                self.reacomodar(cx);
            }
            marquee = out.marquee;
            guias = out.guides;
            ghost = out.ghost;
            if out.status.is_some() {
                self.status = out.status;
            }
        }

        {
            let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() else {
                return;
            };
            let rt = c.runtime_mut();
            if let Some(d) = self.host.doc() {
                if let Err(e) = rt.open(d.ir()) {
                    self.status = Some(format!("runtime: {e}"));
                }
                rt.set_selection(d.selection());
                rt.set_inherited(&d.inherited_nodes());
                rt.set_snap_ghost(ghost.and_then(|g| g.rects(d.ir())));
            }
            rt.set_marquee(marquee);
            rt.set_guides(&guias);
        }
        self.refresh(cx);
        canvas.redraw(cx);
    }

    #[note("ide-app-build-ideapp-salvar")]
    pub(super) fn salvar(&mut self, cx: &mut Cx) {
        if self.save_editor_file(cx) {
            return;
        }
        if self.has_any_pending_diff_review() {
            self.save_waits_for_review();
            return;
        }
        self.save_form(cx);
    }

    #[note("ide-app-build-ideapp-save-form")]
    pub(super) fn save_form(&mut self, cx: &mut Cx) -> bool {
        let falhou = match self.save_form_file() {
            Some(Ok(p)) => {
                self.status = Some(format!("saved {}", p.display()));
                false
            }
            Some(Err(e)) => {
                self.status = Some(format!("save failed: {e}"));
                true
            }
            None => {
                self.status = Some("no file to save".into());
                return false;
            }
        };
        if !crawrad_project::generator::should_generate_on_save(falhou, self.host.has_doc()) {
            return !falhou;
        }
        let salvo = self.status.take().unwrap_or_default();
        let antes = self.host.doc().map(|d| d.revision());
        self.gerar_projeto(cx);
        let gerado = self.status.take().unwrap_or_default();
        self.status = Some(format!("{salvo} — {gerado}"));
        if self.host.doc().map(|d| d.revision()) != antes
            && let Some(Err(_)) = self.save_form_file()
        {
            self.status = Some(format!("{salvo} — {gerado} (the unlink is not on disk)"));
        }
        if let Some(d) = self.host.doc() {
            self.host.set_saved_revision(d.revision());
        }
        self.reabrir_no_canvas(cx);
        self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
        self.sincronizar(cx);
        true
    }

    #[note("ide-app-build-ideapp-save-form-file")]
    fn save_form_file(&mut self) -> Option<Result<std::path::PathBuf, String>> {
        let d = self.host.doc()?;
        let saved = crawrad_project::form_file::save(&d.own_ir(), self.host.active_file()?);
        if let Ok(p) = &saved {
            self.host.set_saved_revision(d.revision());
            self.host.set_active_file(Some(p.clone()));
        }
        Some(saved)
    }

    #[note("ide-app-build-ideapp-save-all")]
    pub(super) fn save_all(&mut self, cx: &mut Cx) -> bool {
        let (saved, mut failed) = self.write_editor_buffers(cx);
        let form = self
            .host
            .doc()
            .is_some_and(|d| d.revision() != self.host.saved_revision());
        let review = form && self.has_any_pending_diff_review();
        if review {
            failed.push(format!("the form: {}", super::diff_review::REVIEW_PENDING));
        }
        let form_ok = !form || (!review && self.save_form(cx));
        self.status = Some(match (failed.is_empty() && form_ok, form) {
            (true, false) => format!("Saved {saved} file(s)"),
            (true, true) => format!("Saved {saved} file(s) and the form"),
            (false, _) if failed.is_empty() => "The form could not be saved".into(),
            (false, _) => format!("Save failed: {}", failed.join("; ")),
        });
        self.redraw_editor_tabs(cx);
        self.sincronizar(cx);
        failed.is_empty() && form_ok
    }

    #[note("ide-app-build-ideapp-write-editor-buffers")]
    pub(super) fn write_editor_buffers(&mut self, cx: &mut Cx) -> (usize, Vec<String>) {
        let reviews = &self.diff_reviews;
        let report = self.host.save_buffers(&mut |path, text| {
            if reviews.has_pending_review(path) {
                return Err(super::diff_review::REVIEW_PENDING.into());
            }
            let bytes = super::editor_footer::file_encoding(Some(path)).to_bytes(text);
            std::fs::write(path, bytes).map_err(|e| e.to_string())
        });
        for (path, outcome) in &report {
            if outcome.is_ok() {
                let text = std::fs::read_to_string(path)
                    .map(|c| c.replace("\r\n", "\n"))
                    .unwrap_or_default();
                self.disk.saved(path, text);
            }
        }
        self.update_disk_change_bar(cx);
        let mut failed = Vec::new();
        for (path, outcome) in &report {
            match outcome {
                Ok(()) => self.lsp_notificar_salvamento(path),
                Err(e) => failed.push(format!("{}: {e}", path.display())),
            }
        }
        if self
            .editavel
            .as_ref()
            .is_some_and(|f| self.host.buffer(f).is_none())
        {
            self.base_na_vista = self.texto_na_vista.clone();
            self.refresh_change_bars(cx);
        }
        (report.len() - failed.len(), failed)
    }

    #[note("ide-app-build-ideapp-reload-form-from-disk")]
    pub(super) fn reload_form_from_disk(&mut self, cx: &mut Cx) {
        let Some(caminho) = self.host.active_file().map(|p| p.to_path_buf()) else {
            self.status = Some("nothing to reload — no form open".into());
            return;
        };
        if !caminho.exists() {
            self.status = Some(format!("{} is gone from disk", caminho.display()));
            return;
        }
        if self.abrir_formulario(cx, &caminho) {
            self.status = Some(format!("reloaded from {}", caminho.display()));
        }
    }

    #[note("ide-app-build-ideapp-fechar-documento")]
    pub(super) fn fechar_documento(&mut self, cx: &mut Cx) {
        self.confirmar_edicao(cx);
        self.confirm_event(cx);
        let Some(caminho) = self.host.active_file().map(|p| p.to_path_buf()) else {
            self.status = Some("nothing to close — no form open".into());
            return;
        };
        if self
            .host
            .doc()
            .is_some_and(|d| d.revision() != self.host.saved_revision())
        {
            self.status = Some(Self::TRABALHO_NAO_SALVO.into());
            return;
        }
        self.host.set_doc(None);
        self.host.set_active_file(None);
        self.design_language = None;
        self.design_locale = None;
        self.host.set_saved_revision(0);
        self.reabrir_no_canvas(cx);
        self.sincronizar(cx);
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        self.status = Some(format!("closed {}", caminho.display()));
    }

    #[note("ide-app-build-ideapp-save-before-build")]
    pub(super) fn save_before_build(&mut self, cx: &mut Cx) -> bool {
        // With the option on, EVERY open file goes, not only the one in view.
        let all = crawrad_project::workspace::read_preferences().save_before_build;
        if (all && !self.save_all(cx)) || (self.save_editor_file(cx) && self.editor_sujo()) {
            return false;
        }
        // The form reaches the disk with its code, or a crash loses the design.
        if let (Some(d), Some(p)) = (self.host.doc(), self.host.active_file())
            && crawrad_project::form_file::is_form(p)
            && (d.revision() != self.host.saved_revision() || !p.exists())
            && let Some(Err(e)) = self.save_form_file()
        {
            self.status = Some(format!("save failed: {e}"));
            return false;
        }
        true
    }

    #[note("ide-app-build-ideapp-prepare-build")]
    pub(super) fn prepare_build(&mut self, cx: &mut Cx) -> bool {
        if self.host.doc().is_none() {
            return self.save_before_build(cx);
        }
        self.gerar_projeto(cx);
        let status = self.status.as_deref();
        !status.is_some_and(crawrad_project::generator::refused)
    }

    #[note("ide-app-build-ideapp-gerar-projeto")]
    pub(super) fn gerar_projeto(&mut self, cx: &mut Cx) {
        // Generating rewrites the user's files. With a diff review pending it
        // threw the review's own rendering away — the proposed line vanished
        // from the editor while the bar at the top went on offering to accept
        // it — and accepting afterwards would have written a stale original
        // over the freshly generated code. Saving already refuses here; so
        // does generating.
        if self.has_any_pending_diff_review() {
            // Through `refusal`, so `generator::refused` recognises it: otherwise
            // `run.start` over MCP would sail past a generation that never ran.
            self.status = Some(crawrad_project::generator::refusal(format!(
                "cannot generate: {}",
                super::diff_review::REVIEW_PENDING
            )));
            return;
        }
        if !self.save_before_build(cx) {
            return;
        }
        let Some(destino) = self.membro() else {
            self.status = Some(crawrad_project::generator::refusal(
                crawrad_project::generator::NO_MEMBER,
            ));
            return;
        };
        // The package name comes out BEFORE the document: it requires all of `&self`.
        let pacote = crawrad_project::generator::package_name_in(&destino);
        let plano = match self.host.doc() {
            Some(d) => crawrad_project::generator::prepare(&destino, &mut (), &d.own_ir(), &pacote),
            None => {
                self.status = Some(crawrad_project::generator::refusal(
                    crawrad_project::generator::NO_DOCUMENT,
                ));
                return;
            }
        };
        let plano = match plano {
            Ok(p) => p,
            Err(e) => {
                self.status = Some(e);
                return;
            }
        };
        if let Some(tx) = plano.unlink()
            && let Some(d) = self.host.doc_mut()
            && let Err(e) = d.apply(tx)
        {
            self.status = Some(format!("doc: {e}"));
        }
        let had_target = crawrad_project::generator::has_a_target(&destino);
        self.status = Some(crawrad_project::generator::write(&destino, plano, &mut ()));
        if let Some(aviso) = self
            .host
            .doc()
            .and_then(|d| crawrad_project::creation::third_party_dependencies(d.ir(), &destino))
        {
            self.status = Some(match self.status.take() {
                Some(s) => format!("{s} — {aviso}"),
                None => aviso,
            });
            self.lsp_reload_workspace();
        }
        if !had_target && crawrad_project::generator::has_a_target(&destino) {
            self.lsp_reload_workspace();
        }
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        if self.host.center().is_text() {
            let texto = self.texto_do_centro();
            self.code_view().set_text(cx, &texto);
            // The unit exists on disk now, so it becomes editable.
            self.editavel = self.editable_file();
            self.aplicar_somente_leitura(cx);
            self.base_na_vista = texto.clone();
            self.texto_na_vista = texto;
            self.refresh_change_bars(cx);
        }
    }

    #[note("ide-app-build-ideapp-rodar-projeto")]
    pub(super) fn rodar_projeto(&mut self, cx: &mut Cx) {
        if self.build_tasks.is_running("build") {
            self.status = Some(self.build.refuse_busy());
            return;
        }
        self.build.mode = BuildMode::Run;
        if !self.prepare_build(cx) {
            return;
        }
        let Some((raiz, pacote)) = self.alvo_do_cargo() else {
            self.status = Some("no Cargo project open — nothing to build".into());
            return;
        };
        let call = crawrad_os::process::CargoCall::build(&pacote, &raiz);
        let cmd_line = format!("cargo build -p {pacote}");
        self.build.start_run(
            BuildMode::Run,
            crawrad_project::build_progress::BuildProgress::new("Running", &pacote, &cmd_line),
        );
        let now = std::time::Instant::now();
        let id = self.build_tasks.spawn("build", move |reporter| {
            let mut cmd = call.progress_command();
            let rep = &reporter;
            let res = crawrad_os::process::stream(
                &mut cmd,
                |pipe, line| rep.progress(crawrad_project::build_progress::encode_line(pipe, line)),
                || rep.is_cancelled(),
            );
            match res {
                Ok(outcome) => reporter.done(outcome),
                Err(error) => reporter.failed(error),
            }
        });
        self.build.running = Some((BuildMode::Run, id, now));
        self.build_clock = cx.start_timeout(0.1);
        self.status = Some("building… (first time takes a minute)".into());
        self.draw_build(cx);
        self.draw_build_run(cx);
    }

    fn finish_build_mode(&mut self, cx: &mut Cx, mode: crate::ide_app::dialogos::build::BuildMode) {
        if self.build.after_finish(mode, std::time::Instant::now()) {
            self.open_build(cx);
        }
        if self.build.needs_tick() {
            self.build_clock = cx.start_timeout(0.1);
        }
    }

    #[note("ide-app-build-ideapp-bombear-build")]
    pub(super) fn bombear_build(&mut self, cx: &mut Cx) {
        if let Some((status, url)) = self.web_run.pump() {
            url.map(|u| crawrad_runtime::shell::open_url(&u));
            self.status = Some(status);
            self.refresh(cx);
        }
        for end in self.app_run.pump() {
            self.report_panics(cx, end);
        }
        self.pump_tests(cx);
        let events = self.build_tasks.drain();
        for (_, event) in events {
            match event {
                crawrad_os::tasks::TaskEvent::Progress(text) => {
                    if let Some((pipe, line)) = crawrad_project::build_progress::decode_line(&text)
                        && let Some((mode, _, _)) = self.build.running
                    {
                        if let Some(run) = self.build.run_mut(mode) {
                            run.feed(pipe, line);
                        }
                        self.draw_build(cx);
                        self.draw_build_run(cx);
                    }
                }
                crawrad_os::tasks::TaskEvent::Done(outcome) => {
                    let (mode, elapsed) = self.build.take_running();
                    let diags =
                        crawrad_project::diagnostics_model::parse_diagnostics(&outcome.stdout);
                    self.build.update_diagnostics(&diags);
                    self.diagnosticos = Some(diags.clone());

                    let is_success = outcome.success && self.build.errors_count == 0;
                    let (errors, warnings) = (self.build.errors_count, self.build.warnings_count);
                    if let Some(run) = self.build.run_mut(mode) {
                        run.finish(crawrad_project::build_progress::Finish {
                            success: is_success,
                            cancelled: outcome.cancelled,
                            errors,
                            warnings,
                            elapsed,
                        });
                    }
                    self.finish_build_mode(cx, mode);

                    let (raiz_opt, pacote) = self
                        .alvo_do_cargo()
                        .map(|(r, p)| (Some(r), p))
                        .unwrap_or_else(|| (self.raiz(), "project".into()));
                    let artefatos =
                        crawrad_project::diagnostics_model::extract_artifacts(&outcome.stdout);
                    let manifest = crawrad_project::build_report::manifest_path(
                        raiz_opt.as_deref(),
                        &pacote,
                        &artefatos,
                    );
                    let platform =
                        crawrad_project::build_report::platform_label(&self.build.selected_targets);
                    let configuration = self.build.profile.label().to_string();
                    let raw_exes = crawrad_project::build_report::executables(
                        raiz_opt.as_deref(),
                        &pacote,
                        self.build.profile,
                        &self.build.selected_targets,
                        &artefatos,
                    );
                    let executables = raw_exes
                        .into_iter()
                        .map(|exe| {
                            let size = std::fs::metadata(&exe).ok().map(|m| m.len());
                            (exe, size)
                        })
                        .collect();
                    let error_lines = crawrad_project::build_report::error_lines(&diags);
                    let report = crawrad_project::build_report::Report {
                        manifest,
                        platform,
                        configuration,
                        executables,
                        mode,
                        package: pacote,
                        success: is_success,
                        errors: self.build.errors_count,
                        warnings: self.build.warnings_count,
                        error_lines,
                    };
                    let summary_lines =
                        self.build
                            .closing_summary(&report, outcome.cancelled, raiz_opt.as_deref());
                    if let Some(run) = self.build.run_mut(mode) {
                        run.append_log_line("");
                        run.append_log_lines(summary_lines);
                    }

                    // The Problems panel: the full log, why it stopped, the list.
                    let log = self.build.run(mode).map(|r| r.log().to_vec());
                    self.problems.log = log.unwrap_or_default();
                    let failed = (!is_success).then_some((mode == BuildMode::Run, elapsed));
                    self.refresh_problems(cx, failed);
                    if !outcome.cancelled {
                        match crawrad_os::process::after_build(Ok(outcome.clone())) {
                            crawrad_os::process::AfterBuild::Launch => {
                                if mode == BuildMode::Run {
                                    self.esconder_painel(cx, Panel::Problems);
                                }
                                self.lancar_projeto(cx, &artefatos)
                            }
                            crawrad_os::process::AfterBuild::Report(frase) => {
                                self.status = Some(frase);
                                if mode == BuildMode::Run {
                                    self.show_panel(cx, Panel::Problems);
                                }
                            }
                        }
                    } else {
                        self.status = Some("build cancelled".into());
                    }

                    self.draw_build(cx);
                    self.draw_build_run(cx);
                    self.apply_diagnostics_decorations(cx);
                    self.sincronizar(cx);
                }
                crawrad_os::tasks::TaskEvent::Failed(err) => {
                    let (mode, elapsed) = self.build.take_running();
                    self.build.finish_failed_run(mode, elapsed, &err);
                    self.finish_build_mode(cx, mode);
                    self.status = Some(if err == crawrad_os::tasks::NO_ANSWER {
                        "the build thread ended without answering".into()
                    } else {
                        err
                    });
                    if mode == BuildMode::Run {
                        self.show_panel(cx, Panel::Problems);
                    }
                    self.draw_build(cx);
                    self.draw_build_run(cx);
                    self.sincronizar(cx);
                }
            }
        }
    }

    #[note("ide-app-build-ideapp-lancar-projeto")]
    pub(super) fn lancar_projeto(
        &mut self,
        cx: &mut Cx,
        artefatos: &[crate::view_models::diagnostics_model::CompiledArtifact],
    ) {
        let Some((raiz, pacote)) = self.alvo_do_cargo() else {
            self.status = Some("built — but the project folder is no longer there".into());
            return;
        };
        if self.build.mode != crate::ide_app::dialogos::build::BuildMode::Run {
            let acao = match self.build.mode {
                crate::ide_app::dialogos::build::BuildMode::Check => "checked",
                crate::ide_app::dialogos::build::BuildMode::Clean => "cleaned",
                _ => "built",
            };
            self.status = Some(format!("{acao} — {pacote} ready"));
            return;
        }
        if crawrad_project::web_run::is_web_project(&raiz, &pacote) {
            self.status = Some(self.web_run.start(&raiz, &pacote));
            return;
        }
        if !crawrad_project::workspace::has_binary(&raiz, &pacote) {
            self.status = Some(format!(
                "built — {pacote} is a library, so there is nothing to run"
            ));
            return;
        }
        let mut sem_depurador = String::new();
        if !self.host.debug_mut().take_run_without_debugger() {
            let exe = artefatos
                .iter()
                .find(|a| a.package_name == pacote && a.executable.is_some())
                .and_then(|a| a.executable.clone());
            match exe.map(|exe| self.start_debugging(cx, exe, &raiz)) {
                Some(Ok(())) => {
                    self.status = Some(format!("built — {pacote} running under the debugger"));
                    return;
                }
                Some(Err(why)) => sem_depurador = format!(" without the debugger ({why})"),
                None => {
                    sem_depurador =
                        " without the debugger (no executable in the build output)".into()
                }
            }
        }
        let mut cmd = crawrad_os::process::CargoCall::run(&pacote, &raiz).command();
        if let Some(ref lang) = self.run_language
            && !lang.is_empty()
            && lang != "system"
        {
            cmd.env(crawrad_codegen::locales::LANG_ENV, lang);
        }
        match self.app_run.start(cmd, &pacote, &raiz) {
            Ok(()) => self.status = Some(format!("built — {pacote} running{sem_depurador}")),
            Err(e) => self.status = Some(format!("built, but could not launch: {e}")),
        }
    }
}
