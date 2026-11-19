//! The Explorer and the Package Manager: rows, file commands, and the Inspector command. Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;
use crawrad_project::form_file::is_form;

impl IdeApp {
    // --- THE EXPLORER AND THE PACKAGE MANAGER — the two panels that the dock gained ---

    #[note("ide-app-explorer-actions-ideapp-abrir-aba-relativa")]
    pub(super) fn abrir_aba_relativa(&mut self, cx: &mut Cx, rel: &crawrad_actions::PathInProject) {
        let Some(raiz) = self.raiz() else {
            self.status = Some("no folder open — use File ▸ Open Folder".into());
            return;
        };
        let completo = self.membro().unwrap_or(raiz).join(rel.as_str());
        if !completo.exists() {
            self.status = Some(format!("{rel} is not generated yet — press Generate first"));
            return;
        }
        self.abrir_no_editor(cx, completo);
        self.status = None;
    }

    pub(crate) fn membro(&self) -> Option<std::path::PathBuf> {
        let host = self.host.active_file();
        host.and_then(crawrad_project::workspace::document_member)
            .or_else(|| self.code_file_place().2)
    }

    #[note("ide-app-explorer-actions-ideapp-code-file-place")]
    fn code_file_place(&self) -> CodePlace {
        let key = (self.active_file_path(), self.desktop_root.clone());
        let cached = self.code_file_place.borrow().clone();
        if (&cached.0, &cached.1) == (&key.0, &key.1) {
            return cached;
        }
        let file = key.0.as_deref();
        let root = file.and_then(explorer::group_root_of);
        let member = file.and_then(crawrad_project::workspace::document_member);
        let place = (key.0, key.1, member, root);
        *self.code_file_place.borrow_mut() = place.clone();
        place
    }

    #[note("ide-app-explorer-actions-ideapp-follow-project-framework")]
    pub(super) fn follow_project_framework(&mut self) {
        let member = self.membro();
        if member == self.framework_of {
            return;
        }
        if let Some(target) = member
            .as_deref()
            .and_then(crawrad_project::generator::project_target)
        {
            crawrad_project::generator::set_active_target(target);
        }
        self.framework_of = member;
    }

    #[note("ide-app-explorer-actions-ideapp-raiz")]
    pub(crate) fn raiz(&self) -> Option<std::path::PathBuf> {
        let host = self.host.active_file().and_then(explorer::group_root_of);
        host.or_else(|| {
            let (_, desktop, _, root) = self.code_file_place();
            root.or(desktop)
        })
    }

    #[note("ide-app-explorer-actions-ideapp-nome-do-pacote")]
    pub(super) fn nome_do_pacote(&self) -> Option<String> {
        let manifesto =
            std::fs::read_to_string(self.membro()?.join(crawrad_project::workspace::MANIFEST))
                .ok()?;
        crawrad_project::workspace::package_name_from_manifest(&manifesto)
    }

    #[note("ide-app-explorer-actions-ideapp-alvo-do-cargo")]
    pub(crate) fn alvo_do_cargo(&self) -> Option<(std::path::PathBuf, String)> {
        let form = self.host.active_file().filter(|f| is_form(f));
        let file = form.map(Into::into).or_else(|| self.active_file_path());
        let file = file.as_deref();
        let root = file
            .and_then(explorer::group_root_of)
            .or_else(|| self.desktop_root.clone())?;
        let membro = file
            .and_then(crawrad_project::workspace::document_member)
            .unwrap_or(root.clone());
        let pacote = crawrad_project::generator::package_name_in(&membro);
        Some((root, pacote))
    }

    #[note("ide-app-explorer-actions-ideapp-plano-de-geracao")]
    pub(super) fn plano_de_geracao(
        &self,
    ) -> Option<Result<Vec<crawrad_codegen::Arquivo>, crawrad_codegen::Erro>> {
        let doc = self.host.doc()?;
        let membro = self.membro();
        Some(crawrad_project::generator::preview_plan(
            membro.as_deref(),
            doc.ir(),
            &|p| self.host.edited_text(p),
        ))
    }

    #[note("ide-app-explorer-actions-ideapp-follow-unsaved")]
    pub(super) fn follow_unsaved(&mut self, cx: &mut Cx) {
        let now: UnsavedTabFiles = (self.abas.files().iter())
            .filter(|f| self.host.buffer(f).is_some())
            .cloned()
            .collect();
        if now != self.unsaved_tabs {
            self.unsaved_tabs = now;
            self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        }
    }

    #[note("ide-app-explorer-actions-ideapp-explorer-drawn")]
    pub(super) fn explorer_drawn(
        rows: &[explorer::Row],
        rects: &[(usize, Rect)],
    ) -> serde_json::Value {
        let at = |(i, r): &(usize, Rect)| (*i, xywh(*r));
        explorer::rows_json(rows, &rects.iter().map(at).collect::<Vec<_>>())
    }

    #[note("ide-app-explorer-actions-ideapp-linhas-do-explorer")]
    pub(super) fn linhas_do_explorer(&mut self) -> Vec<explorer::Row> {
        let raiz = self.raiz();
        let documento = self.host.active_file().map(|p| p.to_path_buf());
        let membro = self.membro();
        let plano = self.plano_de_geracao();
        let timeline_open = self
            .folds
            .sections(documento.as_deref(), membro.as_deref())
            .timeline;
        let timeline_lines = if timeline_open {
            self.timeline_lines()
        } else {
            Vec::new()
        };
        let counts = self.lsp.as_ref().map(|l| l.problem_counts());
        let problems = explorer::problems::map(counts.unwrap_or_default());
        let unsaved: std::collections::BTreeSet<String> = self
            .abas
            .files()
            .iter()
            .filter(|f| self.host.buffer(f).is_some())
            .map(|f| explorer::problems::key(f))
            .collect();
        let s = self.folds.sections(documento.as_deref(), membro.as_deref());
        let rows = explorer::rows(&explorer::View::from_sections(
            raiz.as_deref(),
            documento.as_deref(),
            &plano,
            s,
            &problems,
            &unsaved,
            self.disk.history().map(|_| &timeline_lines[..]),
        ));
        self.explorer_row_targets = rows.iter().map(|r| r.item_target()).collect();
        rows
    }

    pub(super) fn resolver_caminho_do_explorer(&self, p: &std::path::Path) -> std::path::PathBuf {
        let base = self.membro().or_else(|| self.raiz());
        explorer::resolve_path(base.as_deref(), p)
    }

    pub(super) fn explorer_target_folder(&self, p: Option<&std::path::Path>) -> std::path::PathBuf {
        let base = self.membro().or_else(|| self.raiz());
        explorer::target_folder(base.as_deref(), p)
    }

    pub(super) fn create_new_file(&mut self, cx: &mut Cx, p: Option<&std::path::Path>) {
        let pasta = self.explorer_target_folder(p);
        match crawrad_project::creation::create_unit_file(&pasta) {
            Ok(destino) => {
                self.status = Some(format!("Created {}", destino.display()));
                self.abrir_no_editor(cx, destino);
                self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
                self.ui.redraw(cx);
            }
            Err(e) => self.status = Some(format!("Failed to create file: {e}")),
        }
    }

    pub(super) fn criar_novo_formulario(&mut self, cx: &mut Cx, p: Option<&std::path::Path>) {
        let pasta = self.explorer_target_folder(p);
        let reg = crawrad_registry::Registry::builtin();
        match crawrad_project::creation::create_form_file(&pasta, &reg) {
            Ok(destino) => {
                self.status = Some(format!("Created form {}", destino.display()));
                self.abrir_formulario(cx, &destino);
            }
            Err(why) => self.status = Some(why),
        }
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        self.ui.redraw(cx);
    }

    pub(super) fn create_new_folder(&mut self, cx: &mut Cx, p: Option<&std::path::Path>) {
        let pai = self.explorer_target_folder(p);
        match crawrad_project::creation::create_subfolder(&pai) {
            Ok(destino) => {
                self.status = Some(format!("Created {}", destino.display()));
                self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
                self.ui.redraw(cx);
            }
            Err(e) => self.status = Some(format!("Failed to create folder: {e}")),
        }
    }

    pub(super) fn revelar_no_explorador(&mut self, p: &std::path::Path) {
        let target = self.resolver_caminho_do_explorer(p);
        explorer::reveal_in_file_manager(&target);
        self.status = Some(format!("Revealed: {}", target.display()));
    }

    pub(super) fn copiar_caminho(&mut self, cx: &mut Cx, p: &std::path::Path) {
        let target = self.resolver_caminho_do_explorer(p);
        let text = target.display().to_string();
        cx.copy_to_clipboard(&text);
        self.status = Some(format!("Copied path: {text}"));
    }

    pub(super) fn copiar_caminho_relativo(&mut self, cx: &mut Cx, p: &std::path::Path) {
        let base = self.membro().or_else(|| self.raiz());
        let rel = match base {
            Some(ref r) => p.strip_prefix(r).unwrap_or(p),
            None => p,
        };
        let text = rel.display().to_string();
        cx.copy_to_clipboard(&text);
        self.status = Some(format!("Copied relative path: {text}"));
    }

    pub(super) fn apagar_item(&mut self, cx: &mut Cx, p: &std::path::Path) {
        let target = self.resolver_caminho_do_explorer(p);
        match explorer::delete_path(&target) {
            Ok(()) => {
                self.status = Some(format!("Deleted {}", target.display()));
                self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
                self.ui.redraw(cx);
            }
            Err(e) => {
                self.status = Some(format!("Failed to delete {}: {e}", target.display()));
            }
        }
    }

    #[note("ide-app-explorer-actions-ideapp-aplicar-dobra-do-explorer")]
    pub(super) fn aplicar_dobra_do_explorer(
        &mut self,
        cx: &mut Cx,
        comando: crawrad_actions::ExplorerCommand,
    ) {
        let documento = self.host.active_file().map(|p| p.to_path_buf());
        let membro = self.membro();
        match comando {
            crawrad_actions::ExplorerCommand::ToggleFolder(pasta) => {
                self.folds
                    .toggle(documento.as_deref(), membro.as_deref(), &pasta);
            }
            crawrad_actions::ExplorerCommand::ToggleLegend => {
                self.folds
                    .toggle_legend(documento.as_deref(), membro.as_deref());
            }
            crawrad_actions::ExplorerCommand::ToggleOpenEditors => {
                self.folds
                    .toggle_open_editors(documento.as_deref(), membro.as_deref());
            }
            crawrad_actions::ExplorerCommand::ToggleOutline => {
                self.folds
                    .toggle_outline(documento.as_deref(), membro.as_deref());
            }
            crawrad_actions::ExplorerCommand::ToggleTimeline => {
                self.folds
                    .toggle_timeline(documento.as_deref(), membro.as_deref());
            }
            crawrad_actions::ExplorerCommand::OpenTimelineEntry(i) => {
                self.open_history(cx);
                self.history.select(i);
                self.sync_history(cx);
            }
            crawrad_actions::ExplorerCommand::ToggleDependencies => {
                self.folds
                    .toggle_dependencies(documento.as_deref(), membro.as_deref());
            }
            crawrad_actions::ExplorerCommand::NewFile(p) => {
                self.create_new_file(
                    cx,
                    if p.as_os_str().is_empty() {
                        None
                    } else {
                        Some(&p)
                    },
                );
            }
            crawrad_actions::ExplorerCommand::NewForm(p) => {
                self.criar_novo_formulario(
                    cx,
                    if p.as_os_str().is_empty() {
                        None
                    } else {
                        Some(&p)
                    },
                );
            }
            crawrad_actions::ExplorerCommand::NewFolder(p) => {
                self.create_new_folder(
                    cx,
                    if p.as_os_str().is_empty() {
                        None
                    } else {
                        Some(&p)
                    },
                );
            }
            crawrad_actions::ExplorerCommand::Reveal(p) => {
                self.revelar_no_explorador(&p);
            }
            crawrad_actions::ExplorerCommand::CopyPath(p) => {
                self.copiar_caminho(cx, &p);
            }
            crawrad_actions::ExplorerCommand::CopyRelativePath(p) => {
                self.copiar_caminho_relativo(cx, &p);
            }
            crawrad_actions::ExplorerCommand::Rename(p) => {
                let target = self.resolver_caminho_do_explorer(&p);
                self.status = Some(format!("Rename: {}", target.display()));
            }
            crawrad_actions::ExplorerCommand::Delete(p) => {
                self.apagar_item(cx, &p);
            }
            crawrad_actions::ExplorerCommand::Refresh => {
                self.status = Some("Explorer refreshed".into());
            }
            crawrad_actions::ExplorerCommand::CollapseAll => {
                self.folds
                    .collapse_all(documento.as_deref(), membro.as_deref());
                self.status = Some("Folders collapsed".into());
            }
        }
        self.dock()
            .item(layout::tab_of(Panel::Explorer))
            .portal_list(id!(explorer_body.explorer_list))
            .redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-explorer-actions-ideapp-aplicar-comando-do-inspector")]
    pub(super) fn aplicar_comando_do_inspector(
        &mut self,
        cx: &mut Cx,
        comando: crawrad_actions::InspectorCommand,
    ) {
        match comando {
            crawrad_actions::InspectorCommand::OpenEditor { node, key } => {
                match inspector::window_for(&self.linhas_do_inspector(), &key) {
                    inspector::Window::Items => self.open_items(cx, node),
                    inspector::Window::Lines => self.open_lines(cx, node, &key),
                    inspector::Window::Color => self.open_color_for_property(cx, node, &key),
                    inspector::Window::Pad => {
                        self.pad = Some(PadAberto { no: node, key });
                        self.pad_na_tela = None;
                    }
                }
            }
            crawrad_actions::InspectorCommand::OpenCollection { node } => {
                self.open_actions(cx, node)
            }
            crawrad_actions::InspectorCommand::ShowEvents => {
                self.show_inspector_tab(cx, InspectorTab::Events)
            }
            crawrad_actions::InspectorCommand::EditName => {
                self.show_inspector_tab(cx, InspectorTab::Properties);
                self.name_focus_pending = true;
            }
            crawrad_actions::InspectorCommand::CloseEditor => {
                self.pad = None;
                self.pad_na_tela = None;
                self.pad_pos = None;
                self.items = None;
                self.items_na_tela = None;
                self.lines = None;
                self.lines_na_tela = false;
                self.color = None;
            }
            crawrad_actions::InspectorCommand::ToggleGroup { key } => {
                if !self.grupos_abertos.remove(&key) {
                    self.grupos_abertos.insert(key);
                }
            }
            crawrad_actions::InspectorCommand::ToggleCategory { label } => {
                if !self.categorias_fechadas.remove(&label) {
                    self.categorias_fechadas.insert(label);
                }
            }
            crawrad_actions::InspectorCommand::SetNameWidth { width } => {
                self.name_splitter.ask(width)
            }
            crawrad_actions::InspectorCommand::PickPicture { node, key, file } => {
                match crawrad_runtime::file_dialog::pick_picture(file.as_deref()) {
                    Some(Ok(p)) => self.write_property(cx, vec![node], &key, Value::Picture(p)),
                    Some(Err(why)) => self.status = Some(why),
                    None => {}
                }
            }
            crawrad_actions::InspectorCommand::PickFont { node, key, font } => {
                let ir = self.host.doc().map(|d| d.ir());
                match inspector::pick_font(ir, &node, &key, font.as_deref()) {
                    Some(Ok(f)) => self.write_property(cx, vec![node], &key, Value::Font(f)),
                    Some(Err(why)) => self.status = Some(why),
                    None => {}
                }
            }
        }
        self.painel(Panel::Inspector)
            .portal_list(id!(inspector_panel.inspector_list))
            .redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-explorer-actions-ideapp-linhas-dos-pacotes")]
    pub(super) fn linhas_dos_pacotes(&self) -> Vec<packages::Row> {
        packages::rows(&self.packages, self.membro().is_some())
    }
}
