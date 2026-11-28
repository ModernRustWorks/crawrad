//! Diagnostics, the selection, the view toggles and the panels' rows. Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;

impl IdeApp {
    // --- GENERATE, BUILD AND RUN ---

    #[note("ide-app-view-state-ideapp-apply-diagnostics-decorations")]
    pub(crate) fn apply_diagnostics_decorations(&mut self, cx: &mut Cx) {
        let lsp = self.lsp.clone();
        let file = self.abas.active_file().map(std::path::Path::to_path_buf);
        let published = lsp
            .as_ref()
            .zip(file)
            .and_then(|(l, f)| l.diagnostics_of(&f));
        if let (Some(diags), Some(lsp)) = (published, lsp) {
            return self.decorate(cx, &diags, lsp.position_encoding());
        }
        let Some(ref diags) = self.diagnosticos else {
            return;
        };
        let Some(name) = self.abas.active_file().and_then(|f| f.file_name()) else {
            return;
        };
        let marks = crawrad_project::diagnostics_model::marks_in(diags, &name.to_string_lossy());
        let vista = self.code_view();
        let Some(mut cv) = vista.borrow_mut::<makepad_code_editor::code_view::CodeView>() else {
            return;
        };
        cv.lazy_init_session();
        let Some(ref mut session) = cv.session else {
            return;
        };
        let mut doc = session.document().clone();
        doc.clear_decorations();
        use makepad_code_editor::{decoration as deco, text::Position};
        for (idx, (line_index, col, error)) in marks.into_iter().enumerate() {
            let ty = [deco::DecorationType::Warning, deco::DecorationType::Error][error as usize];
            let at = |byte_index| Position {
                line_index,
                byte_index,
            };
            let (start, end) = (at(col), at(col + 8));
            doc.add_decoration(deco::Decoration::new(idx, start, end, ty));
        }
        let panics = self.mark_panics(&mut doc);
        doc.set_line_messages(&panics);
        self.ui.redraw(cx);
    }

    // --- SELECTION AND WHAT THE UI REPORTS ABOUT IT ---

    #[note("ide-app-view-state-ideapp-select-node")]
    pub fn select_node(&mut self, cx: &mut Cx, id: &str) {
        self.status = None;
        if let Some(d) = self.host.doc_mut() {
            d.select_one(id);
        }
        self.selection_to_canvas();
        self.refresh(cx);
        let canvas = self.canvas();
        canvas.redraw(cx);
    }

    #[note("ide-app-view-state-ideapp-clear-selection")]
    pub fn clear_selection(&mut self, cx: &mut Cx) {
        if let Some(d) = self.host.doc_mut() {
            d.clear_selection();
        }
        let canvas = self.canvas();
        {
            if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
                c.runtime_mut().set_selection(&[]);
            }
        }
        self.refresh(cx);
        canvas.redraw(cx);
    }

    #[note("ide-app-view-state-ideapp-arm-tool")]
    pub fn arm_tool(&mut self, cx: &mut Cx, class_name: &str) {
        self.tool_armed = Some(class_name.to_string());
        self.status = Some(format!("{class_name} armed — click the form to place it"));
        self.sincronizar(cx);
    }

    #[note("ide-app-view-state-ideapp-sincronizar-extensoes")]
    pub(super) fn sincronizar_extensoes(&mut self) {
        if let Some(raiz) = self.raiz() {
            let raiz_str = raiz.to_string_lossy().to_string();
            if self.extensions.context.workspace_path.as_deref() != Some(&raiz_str) {
                self.extensions.context.workspace_path = Some(raiz_str);
                self.extensions.load_workspace_extensions(&raiz);
                for manifest in &self.extensions.manifests {
                    let _ = self.capabilities.register_extension_manifest(manifest);
                }
            }
        }
        self.extensions
            .sync_editor_state(self.editavel.clone(), &self.texto_na_vista);
        if let Some(doc) = self.host.doc() {
            let root_name = doc
                .ir()
                .nodes
                .get(&doc.ir().root)
                .map(|n| n.name().to_string());
            let selected = doc
                .selection()
                .iter()
                .filter_map(|id| doc.ir().nodes.get(id).map(|n| n.name().to_string()))
                .collect();
            self.extensions.sync_designer_state(root_name, selected);
        }
        let msgs = self.extensions.take_messages();
        if let Some((_lvl, txt)) = msgs.last() {
            self.status = Some(txt.clone());
        }
    }

    #[note("ide-app-view-state-ideapp-toggle-view")]
    pub fn toggle_view(&mut self, cx: &mut Cx) {
        match self.host.center().toggle_code() {
            Center::Code => {
                let Some(form) = self.host.active_file().map(|p| p.to_path_buf()) else {
                    self.show_center(cx, Center::Code);
                    return;
                };
                self.abrir_no_editor(cx, source::unit_path(&form));
            }
            outra => {
                self.abas.show_design();
                self.show_center(cx, outra);
            }
        }
    }

    #[note("ide-app-view-state-ideapp-toggle-form-source")]
    pub fn toggle_form_source(&mut self, cx: &mut Cx) {
        self.show_center(cx, self.host.center().toggle_form_source());
    }

    #[note("ide-app-view-state-ideapp-sincronizar-barra-do-designer")]
    pub(crate) fn sincronizar_barra_do_designer(&mut self, cx: &mut Cx) {
        let p = self.painel(Panel::Design);
        let lbl_form = p.label(id!(designer_options_bar.lbl_designer_form));
        if let Some(doc) = self.host.doc() {
            let root_name = doc
                .ir()
                .nodes
                .get(&doc.ir().root)
                .map(|n| n.name().to_string())
                .unwrap_or_default();
            lbl_form.set_visible(cx, true);
            lbl_form.set_text(cx, &format!("📐 Form: {root_name}"));
        } else {
            lbl_form.set_visible(cx, false);
        }

        let grid_txt = if self.ver_grade {
            "Grid: ON"
        } else {
            "Grid: OFF"
        };
        p.button(id!(designer_options_bar.btn_designer_grid))
            .set_text(cx, grid_txt);
        let tabs = ["Tab order: OFF", "Tab order: ON"][usize::from(self.show_tab_order)];
        p.button(id!(designer_options_bar.btn_designer_tab_order))
            .set_text(cx, tabs);

        let banner_info = self.design_language.as_deref().and_then(|code| {
            let ov = match (&self.design_locale, self.host.doc()) {
                (Some(loc), Some(d)) => {
                    let reg = self.designer.registry.clone().unwrap_or_default();
                    crawrad_designer::translations::overflowing(d.ir(), code, &reg, loc)
                }
                _ => Vec::new(),
            };
            crawrad_designer::translations::banner_with_fit(code, &ov)
        });
        let banner_view = p.view(id!(banner_language));
        let show_banner = banner_info.is_some() && self.host.center() == Center::Design;
        banner_view.set_visible(cx, show_banner);
        if let Some((text, all_fit)) = banner_info {
            let c = if all_fit {
                vec4(124.0 / 255.0, 180.0 / 255.0, 107.0 / 255.0, 1.0)
            } else {
                vec4(217.0 / 255.0, 164.0 / 255.0, 65.0 / 255.0, 1.0)
            };
            let lbl = p.label(id!(banner_language.lbl_banner_language));
            lbl.set_text(cx, &text);
            lbl.apply_over(cx, live! { draw_text: { color: (c) } });
        }

        let snap_txt = if self.designer.snap {
            "Snap: ON"
        } else {
            "Snap: OFF"
        };
        let snap = p.button(id!(designer_options_bar.btn_designer_snap));
        snap.set_text(cx, snap_txt);
        let v = |c: [f32; 4]| vec4(c[0], c[1], c[2], c[3]);
        let [bg, border, text] =
            crawrad_runtime_makepad::snap_button_look(self.designer.snap).map(v);
        snap.apply_over(
            cx,
            live! {
                draw_bg: { color: (bg), color_hover: (bg), border_color_1: (border), border_color_2: (border) }
                draw_text: { color: (text), color_hover: (text) }
            },
        );
        p.label(id!(designer_options_bar.lbl_grid_val))
            .set_text(cx, &format!("{}", self.designer.grid as u32));

        let zoom_txt = format!("{:.0}%", self.zoom * 100.0);
        p.button(id!(designer_options_bar.btn_zoom_val))
            .set_text(cx, &zoom_txt);
    }

    #[note("ide-app-view-state-ideapp-centro-das-abas")]
    pub(super) fn centro_das_abas(&self) -> Center {
        if self.abas.active_index().is_some() {
            Center::Code
        } else if self.host.center() == Center::FormSource {
            Center::FormSource
        } else {
            Center::Design
        }
    }

    #[note("ide-app-view-state-ideapp-abrir-no-editor")]
    pub(crate) fn abrir_no_editor(&mut self, cx: &mut Cx, arquivo: std::path::PathBuf) {
        self.switch_tab(cx, |a| _ = a.open(arquivo));
        self.show_center(cx, Center::Code);
    }

    #[note("ide-app-view-state-ideapp-texto-do-centro")]
    pub(crate) fn texto_do_centro(&mut self) -> String {
        let (text, status) = source::center_content(
            self.abas.active_file(),
            self.host.active_file(),
            source::to_center_view(self.host.center()),
            self.host.doc().map(|d| d.ir()),
        );
        if let Some(err) = status {
            self.status = Some(err);
        }
        text
    }

    #[note("ide-app-view-state-ideapp-linhas-da-arvore")]
    pub(super) fn linhas_da_arvore(&self) -> Vec<documento::Linha> {
        self.host
            .doc()
            .map(|d| documento::linhas_da_arvore(d.ir()))
            .unwrap_or_default()
    }

    #[note("ide-app-view-state-ideapp-linhas-da-paleta")]
    pub(super) fn linhas_da_paleta(&self) -> Vec<palette::Row> {
        let web = self
            .membro()
            .is_some_and(|m| crawrad_project::web_run::is_web_member(&m));
        self.designer
            .registry
            .as_deref()
            .map(|r| {
                palette::rows(
                    r,
                    crawrad_project::generator::target_key(),
                    &self.filtro,
                    web,
                )
            })
            .unwrap_or_default()
    }

    // --- O OBJECT INSPECTOR ---

    pub(crate) fn pump_dataset_columns(&mut self, cx: &mut Cx) {
        if self.host.pump_dataset_columns() {
            self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
            self.canvas().redraw(cx);
        }
    }

    pub(super) fn linhas_do_inspector(&self) -> Vec<inspector::Row> {
        let sel: Vec<NodeId> = self.host.selection().to_vec();
        let dir = self.membro();
        let mut rows = self
            .designer
            .registry
            .as_deref()
            .map(|r| {
                inspector::rows_with_columns(
                    r,
                    self.host.doc().map(|d| d.ir()),
                    &sel,
                    crawrad_project::generator::target_key(),
                    self.aba,
                    &self.grupos_abertos,
                    &self.categorias_fechadas,
                    &|id| self.host.dataset_columns(dir.as_deref(), id),
                )
            })
            .unwrap_or_default();
        if let Some(d) = self.host.doc() {
            inspector::mark_inherited(&mut rows, d, &sel);
        }
        rows
    }

    #[note("ide-app-view-state-ideapp-abrir-formulario")]
    pub(crate) fn abrir_formulario(&mut self, cx: &mut Cx, caminho: &std::path::Path) -> bool {
        self.confirmar_edicao(cx);
        self.confirm_event(cx);
        // Unsaved work only blocks RELOADING the form in front, which would throw
        // those edits away. Opening a DIFFERENT form no longer does: it gets a tab
        // of its own and the unsaved one stays exactly as it is. Refusing it was
        // right when the IDE held one document and the new one replaced it.
        let e_recarga = self
            .host
            .forms()
            .get(self.host.front().unwrap_or(usize::MAX))
            .and_then(|f| f.path.as_deref())
            == Some(caminho);
        if e_recarga
            && self
                .host
                .doc()
                .is_some_and(|d| d.revision() != self.host.saved_revision())
        {
            self.status = Some(Self::TRABALHO_NAO_SALVO.into());
            self.sincronizar(cx);
            return false;
        }
        let abriu = match crawrad_ir::Document::load(caminho) {
            Ok(ir) => {
                let doc = crawrad_project::form_file::design_doc(caminho, ir);
                self.host.set_saved_revision(doc.revision());
                self.host
                    .set_previous_rect(doc.ir().node(&doc.ir().root).and_then(|n| n.rect()));
                // A TAB, not a replacement. `set_doc` would have put this form
                // where the previous one was and thrown it away - which is what
                // the IDE did before, and why following a reference into another
                // form lost the place you were working in.
                let revisao = doc.revision();
                self.host.open_form(doc, Some(caminho.to_path_buf()));
                self.host.set_saved_revision(revisao);
                self.host.set_active_file(Some(caminho.to_path_buf()));
                self.design_language = None;
                self.design_locale = None;
                self.falha = None;
                self.status = None;
                self.reacomodar(cx);
                if self.host.center() == Center::Welcome {
                    self.show_center(cx, Center::Design);
                }
                // The form that is now open, so reopening brings back THIS one and
                // not whichever form the folder happens to list first.
                let opened = caminho.to_path_buf();
                crawrad_project::workspace::update_preferences(move |p| {
                    p.last_form = Some(opened);
                });
                if let Some(raiz) = self.raiz_projeto() {
                    Self::remember_recent(&raiz);
                    let instance =
                        std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
                    crate::ide_app::mcp_host::sync_workspace_mcp_config(&raiz, &instance);
                }
                true
            }
            Err(e) => {
                self.status = Some(format!("open failed: {e}"));
                false
            }
        };
        self.reabrir_no_canvas(cx);
        self.sincronizar(cx);
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        abriu
    }

    #[note("ide-app-view-state-ideapp-mouse-no-divisor")]
    pub(super) fn mouse_no_divisor(&self, cx: &mut Cx, pos: DVec2) -> bool {
        let center_rect = self
            .dock()
            .item(layout::tab_of(Panel::Design))
            .area()
            .rect(cx);
        if center_rect.size.x <= 0.0 || center_rect.size.y <= 0.0 {
            return false;
        }
        if pos.y < center_rect.pos.y || pos.y > center_rect.pos.y + center_rect.size.y {
            return false;
        }
        let arranjo = self.arranjo();
        let left_open = self.painel_a_vista(&arranjo, Panel::Components)
            || self.painel_a_vista(&arranjo, Panel::Explorer)
            || self.painel_a_vista(&arranjo, Panel::Inspector);
        let right_open = self.painel_a_vista(&arranjo, Panel::Chat)
            || self.painel_a_vista(&arranjo, Panel::Packages);

        // 8 px OUT of the centre only: its scroll bar lives on the inner edge.
        const THRESHOLD: f64 = 8.0;
        let (left, right) = (center_rect.pos.x, center_rect.pos.x + center_rect.size.x);
        if left_open && (left - THRESHOLD..=left).contains(&pos.x) {
            return true;
        }
        if right_open && (right..=right + THRESHOLD).contains(&pos.x) {
            return true;
        }
        false
    }
}
