//! The document: where it comes from, where it goes; MCP pumping and tab actions. Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;

impl IdeApp {
    // --- THE DOCUMENT — where it comes from, and where it goes ---

    #[note("ide-app-document-flow-ideapp-carregar")]
    pub(super) fn carregar(&mut self, cx: &mut Cx) {
        let recent = crawrad_project::workspace::read_recent();
        let last = crawrad_project::workspace::last_project_form(
            &crawrad_project::workspace::read_preferences(),
            &recent,
            crawrad_project::workspace::documents_in,
        );
        let root = last.as_ref().and(recent.first()).cloned();
        let mut welcome = false;
        match documento::abrir(last) {
            Ok(None) => {
                self.host.set_doc(None);
                self.host.set_active_file(None);
                self.falha = None;
                welcome = true;
            }
            Ok(Some(aberto)) => {
                let doc = crawrad_project::form_file::design_doc(&aberto.caminho, aberto.doc);
                self.rev_salva = doc.revision();
                self.host.set_doc(Some(doc));
                self.host.set_active_file(Some(aberto.caminho));
                self.falha = None;
            }
            Err(e) => {
                self.host.set_doc(None);
                self.host.set_active_file(None);
                self.falha = Some(e);
                welcome = true;
            }
        }
        self.design_language = None;
        self.design_locale = None;
        self.status = None;
        self.reabrir_no_canvas(cx);
        if welcome {
            self.show_welcome(cx);
        }
        // Its desktop is given back once the chrome is up (`handle_startup`).
        let inside = |r: &PathBuf| self.host.active_file().is_some_and(|f| f.starts_with(r));
        self.desktop_root = root.filter(inside);
        self.language_codes = self.valid_languages();
        self.sincronizar(cx);
    }

    #[note("ide-app-document-flow-ideapp-window-id")]
    pub(super) fn window_id(&self, cx: &Cx) -> makepad_widgets::WindowId {
        let p = self.ui.as_window().get_position(cx);
        cx.windows.window_id_contains(p + dvec2(1.0, 1.0)).0
    }

    /// The code editor of the centre.
    pub(in crate::ide_app) fn code_view(&self) -> WidgetRef {
        self.painel(Panel::Design).widget(id!(code_view))
    }

    pub(super) fn canvas(&self) -> WidgetRef {
        self.painel(Panel::Design)
            .widget(id!(designer_row.designer_canvas.canvas))
    }

    #[note("ide-app-document-flow-ideapp-reabrir-no-canvas")]
    pub(crate) fn reabrir_no_canvas(&mut self, cx: &mut Cx) {
        let canvas = self.canvas();
        {
            let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() else {
                return;
            };
            let rt = c.runtime_mut();
            rt.com_desenhos(crawrad_project::installed::declared_drawings());
            match self.host.doc() {
                Some(d) => {
                    let opened = match &self.design_locale {
                        Some(loc) => {
                            let reg = self
                                .designer
                                .registry
                                .as_deref()
                                .unwrap_or(&crawrad_project::installed::ide_catalog().catalog);
                            let ir = crawrad_designer::translations::localized(
                                d.ir(),
                                &d.ir().root,
                                reg,
                                loc,
                            );
                            rt.open(&ir)
                        }
                        None => rt.open(d.ir()),
                    };
                    if let Err(e) = opened {
                        self.status = Some(format!("runtime: {e}"));
                    }
                    rt.set_selection(d.selection());
                    rt.set_inherited(&d.inherited_nodes());
                }
                None => rt.close(),
            }
            rt.set_viewport(
                ViewportPx {
                    x: 0,
                    y: 0,
                    w: VIEWPORT.0,
                    h: VIEWPORT.1,
                },
                Scale(1.0),
            );
            rt.set_grid(if self.ver_grade {
                self.designer.grid
            } else {
                0.0
            });
            rt.set_zoom(self.zoom);
            rt.set_snap(self.designer.snap);
        }
        canvas.redraw(cx);
    }

    #[note("ide-app-document-flow-ideapp-reacomodar")]
    pub(crate) fn reacomodar(&mut self, _cx: &mut Cx) {
        let Some(doc) = self.host.doc_mut() else {
            return;
        };
        let atual = doc.ir().node(&doc.ir().root).and_then(|n| n.rect());
        let Some(atual) = atual else { return };

        let gesto = self.designer.current_gesture();

        if let Some(antes) = self.form_anterior {
            let (dw, dh) = (atual.w - antes.w, atual.h - antes.h);
            if (dw != 0.0 || dh != 0.0)
                && let Some(mut tx) = crawrad_designer::layout::apply_anchors(doc.ir(), dw, dh)
            {
                tx = tx.no_gesto(gesto);
                let _ = doc.apply(tx);
            }
        }
        self.form_anterior = Some(atual);

        if let Some(mut tx) = crawrad_designer::layout::apply(doc.ir()) {
            tx = tx.no_gesto(gesto);
            let _ = doc.apply(tx);
        }
    }

    #[note("ide-app-document-flow-ideapp-bombear-mcp")]
    pub fn bombear_mcp(&mut self, cx: &mut Cx) {
        let reqs: Vec<McpRequest> = self
            .mcp_rx
            .as_ref()
            .map(|rx| rx.try_iter().collect())
            .unwrap_or_default();
        if !reqs.is_empty() {
            // a review proposed by a tool shows in the editor at once, not on the next tab switch
            let shown = self.pending_review().map(|r| r.predicted_hash.clone());
            let mut srv = self.mcp_server.take();
            if let Some(srv_mut) = srv.as_mut() {
                for req in reqs {
                    let resp = srv_mut.handle_raw_request(self, cx, &req.line);
                    let _ = req.reply_tx.send(resp);
                }
            }
            self.mcp_server = srv;
            if self.pending_review().map(|r| r.predicted_hash.clone()) != shown {
                self.show_center(cx, self.host.center());
            }
            self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
        }
    }

    pub(super) fn iniciar_mcp(&mut self, _cx: &mut Cx) {
        crawrad_os::tasks::set_waker(
            makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal,
        );
        if self.mcp_rx.is_some() {
            return;
        }
        let requested_instance =
            std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
        match crate::ide_app::mcp_host::start_mcp_listener(&requested_instance) {
            Ok(rx) => {
                self.mcp_rx = Some(rx);
                self.mcp_server = Some(McpServer::new(&requested_instance));
                crawrad_os::set_env_for_children("CRAWRAD_INSTANCE", &requested_instance);
            }
            Err(e) => {
                let pid = std::process::id();
                let fallback_instance = format!("inst-{pid}");
                match crate::ide_app::mcp_host::start_mcp_listener(&fallback_instance) {
                    Ok(rx) => {
                        self.mcp_rx = Some(rx);
                        self.mcp_server = Some(McpServer::new(&fallback_instance));
                        crawrad_os::set_env_for_children("CRAWRAD_INSTANCE", &fallback_instance);
                        self.status = Some(format!(
                            "MCP fallback: instance '{fallback_instance}' (primary '{requested_instance}' busy: {e})"
                        ));
                    }
                    Err(e2) => {
                        self.status = Some(format!(
                            "MCP bind failed: primary '{requested_instance}' ({e}), fallback '{fallback_instance}' ({e2})"
                        ));
                    }
                }
            }
        }
        if let Some(raiz) = self.raiz_projeto() {
            let instance =
                std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
            crate::ide_app::mcp_host::sync_workspace_mcp_config(&raiz, &instance);
        }
    }

    #[note("ide-app-document-flow-ideapp-colocar")]
    pub(crate) fn colocar(&mut self, cx: &mut Cx, classe: &str, x: f32, y: f32) {
        let Some(d) = self.host.doc() else { return };
        let (pai, rx, ry) = self.designer.parent_for_drop(d.ir(), x, y);
        let args = crawrad_actions::Args::text("class", classe)
            .and("parent", Value::Str(pai))
            .and("x", Value::Float(rx as f64))
            .and("y", Value::Float(ry as f64));
        self.execute_with(cx, "component.add", args);
    }

    #[note("ide-app-document-flow-ideapp-acao-de-aba")]
    pub(crate) fn acao_de_aba(&mut self, cx: &mut Cx, acao: TabAction) {
        match acao {
            TabAction::Open { control, page } => {
                let args = crawrad_actions::Args::texts("nodes", [control])
                    .and("key", Value::Str("active_page".into()))
                    .and("value", Value::Int(page as i64))
                    .and("label", Value::Str("Change page".into()));
                self.execute_with(cx, "component.set_property", args);
            }
            TabAction::Create { control } => {
                self.execute_with(
                    cx,
                    "page.new",
                    crawrad_actions::Args::text("control", control),
                );
            }
        }
    }

    #[note("ide-app-document-flow-ideapp-collection-selection")]
    pub(crate) fn collection_selection(&self) -> Option<NodeId> {
        let d = self.host.doc()?;
        let id = d.selection().first()?;
        let classe = &d.ir().node(id)?.class;
        let registro = &crawrad_project::installed::ide_catalog().catalog;
        registro.get(classe)?.item_class()?;
        Some(id.clone())
    }
}
