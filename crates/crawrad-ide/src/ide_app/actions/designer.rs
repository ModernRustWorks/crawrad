//! Actions for the visual designer, toolbox, component tree, and Object Inspector.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-actions-designer-ideapp-set-grid-size")]
    pub(in super::super) fn set_grid_size(&mut self, cx: &mut Cx, size: f32) {
        self.mudar_grade(cx, size);
        self.keep_designer_look();
        self.sincronizar(cx);
    }

    #[note("ide-app-actions-designer-ideapp-keep-designer-look")]
    fn keep_designer_look(&self) {
        let (grid, snap, show, tabs) = (
            self.designer.grid as u32,
            self.designer.snap,
            self.ver_grade,
            self.show_tab_order,
        );
        crawrad_project::workspace::update_preferences(|p| {
            (p.designer_grid, p.designer_snap, p.designer_show_grid) = (grid, snap, show);
            p.designer_show_tab_order = tabs;
        });
    }

    #[note("ide-app-actions-designer-ideapp-designer-json")]
    pub(crate) fn designer_json(&self, cx: &Cx) -> serde_json::Value {
        let bar = self.painel(Panel::Design);
        let names = [
            "btn_designer_grid",
            "btn_designer_snap",
            "btn_designer_tab_order",
            "btn_grid_out",
            "btn_grid_in",
        ];
        let controls: Vec<_> = names
            .iter()
            .map(|n| {
                let path = [live_id!(designer_options_bar), LiveId::from_str(n)];
                let r = bar.widget(&path).area().rect(cx);
                serde_json::json!({ "name": n, "rect": xywh(r) })
            })
            .collect();
        let canvas = self.canvas();
        let t = canvas.borrow::<DesignCanvas>().map(|c| c.transformacao());
        let canvas = t.map(|t| [t.origem_x, t.origem_y, t.escala]);
        serde_json::json!({
            "canvas": canvas,
            "snap": self.designer.snap,
            "show_grid": self.ver_grade,
            "show_tab_order": self.show_tab_order,
            "grid_size": self.designer.grid,
            "language": self.design_language.as_deref().unwrap_or("en"),
            "snap_tooltip": self.snap_hint,
            "controls": controls,
        })
    }

    #[note("ide-app-actions-designer-ideapp-hint-toolbox-row")]
    pub(in super::super) fn hint_toolbox_row(&mut self, cx: &mut Cx, at: DVec2) {
        let list = self
            .painel(Panel::Components)
            .portal_list(id!(toolbox_palette.toolbox_list));
        let rows = self.linhas_da_paleta();
        let first = list.first_id();
        let over = (first..first + list.visible_items() + 1).find_map(|i| {
            let (_, item) = list.get_item(i)?;
            let why = item
                .area()
                .rect(cx)
                .contains(at)
                .then(|| rows.get(i)?.hint())??;
            Some((i, why))
        });
        if over.as_ref().map(|(i, _)| *i) == self.toolbox_hint {
            return;
        }
        let tip = self.ui.tooltip(id!(ide_tooltip));
        match &over {
            Some((_, why)) => tip.show_with_options(cx, at + dvec2(12.0, 20.0), why),
            None => tip.hide(cx),
        }
        self.toolbox_hint = over.map(|(i, _)| i);
    }

    #[note("ide-app-actions-designer-ideapp-hint-designer-bar")]
    pub(in super::super) fn hint_designer_bar(&mut self, cx: &mut Cx, at: DVec2) {
        let snap = self
            .painel(Panel::Design)
            .button(id!(designer_options_bar.btn_designer_snap));
        let over = snap.area().rect(cx).contains(at);
        if over != self.snap_hint {
            self.snap_hint = over;
            let tip = self.ui.tooltip(id!(ide_tooltip));
            if over {
                tip.show_with_options(cx, at + dvec2(12.0, 20.0), crawrad_designer::SNAP_HINT);
            } else {
                tip.hide(cx);
            }
        }
    }

    #[note("ide-app-actions-designer-ideapp-handle-designer-actions")]
    pub(in super::super) fn handle_designer_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let p_des = self.painel(Panel::Design);

        // Designer top bar options
        if p_des
            .button(id!(designer_options_bar.btn_designer_grid))
            .clicked(actions)
        {
            self.ver_grade = !self.ver_grade;
            self.keep_designer_look();
            self.reabrir_no_canvas(cx);
            self.sincronizar(cx);
        } else if p_des
            .button(id!(designer_options_bar.btn_designer_snap))
            .clicked(actions)
        {
            self.designer.snap = !self.designer.snap;
            self.keep_designer_look();
            self.reabrir_no_canvas(cx);
            self.sincronizar(cx);
        } else if p_des
            .button(id!(designer_options_bar.btn_designer_tab_order))
            .clicked(actions)
        {
            self.show_tab_order = !self.show_tab_order;
            self.keep_designer_look();
            self.paint_form_check(cx);
            self.sincronizar(cx);
        } else if p_des
            .button(id!(designer_options_bar.btn_grid_out))
            .clicked(actions)
            || p_des
                .button(id!(designer_options_bar.btn_grid_in))
                .clicked(actions)
        {
            let up = p_des
                .button(id!(designer_options_bar.btn_grid_in))
                .clicked(actions);
            self.set_grid_size(cx, crawrad_designer::adjacent_grid(self.designer.grid, up));
        } else if p_des
            .button(id!(designer_options_bar.btn_zoom_out))
            .clicked(actions)
        {
            self.zoom = crawrad_runtime::adjacent_zoom(self.zoom, false);
            crawrad_project::workspace::update_preferences(|p| p.designer_scale = self.zoom as f64);
            self.aplicar_zoom(cx);
            self.sincronizar(cx);
        } else if p_des
            .button(id!(designer_options_bar.btn_zoom_in))
            .clicked(actions)
        {
            self.zoom = crawrad_runtime::adjacent_zoom(self.zoom, true);
            crawrad_project::workspace::update_preferences(|p| p.designer_scale = self.zoom as f64);
            self.aplicar_zoom(cx);
            self.sincronizar(cx);
        } else if p_des
            .button(id!(designer_options_bar.btn_zoom_val))
            .clicked(actions)
            || p_des
                .button(id!(designer_options_bar.btn_zoom_fit))
                .clicked(actions)
        {
            self.zoom = crawrad_runtime::ZOOM_ONE;
            crawrad_project::workspace::update_preferences(|p| p.designer_scale = self.zoom as f64);
            self.aplicar_zoom(cx);
            self.sincronizar(cx);
        }

        if self
            .painel(Panel::Components)
            .button(id!(add_item))
            .clicked(actions)
            && let Some(alvo) = self.collection_selection()
        {
            self.execute_with(
                cx,
                "component.add_item",
                crawrad_actions::Args::text("parent", &alvo),
            );
            self.sincronizar(cx);
        }

        let mut abas = WidgetSet::empty();
        for aba in InspectorTab::ALL {
            abas.push(self.tab_button(*aba));
        }
        if let Some(i) = abas.as_radio_button_set().selected(cx, actions)
            && let Some(nova) = InspectorTab::ALL.get(i).copied()
            && nova != self.aba
        {
            self.aba = nova;
            self.edicao = None;
            self.edicao_de_evento = None;
            self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
        }

        let campo = self
            .painel(Panel::Components)
            .text_input(id!(toolbox_filter.filter_input));
        if let Some(texto) = campo.changed(actions) {
            self.filtro = texto;
            self.ui.redraw(cx);
        }
        if campo.escaped(actions) && !self.filtro.is_empty() {
            self.set_toolbox_filter(cx, &"".into());
            self.ui.redraw(cx);
        }

        let toolbox = self
            .painel(Panel::Components)
            .portal_list(id!(toolbox_palette.toolbox_list));
        if toolbox.any_items_with_actions(actions) {
            let itens = self.linhas_da_paleta();
            let mut armada = None;
            let mut arrastada = None;
            for (i, item) in toolbox.items_with_actions(actions) {
                let alvo = item.button(id!(alvo));
                if alvo.pressed(actions) {
                    armada = itens.get(i).and_then(palette::Row::class);
                }
                if alvo.pressed(actions) {
                    arrastada = itens.get(i).and_then(palette::Row::class);
                }
            }
            if let Some(classe) = armada {
                self.arm_tool(cx, classe);
            }
            if let Some(classe) = arrastada {
                cx.start_dragging(vec![DragItem::FilePath {
                    path: classe.to_string(),
                    internal_id: Some(live_id!(crawrad_paleta)),
                }]);
            }
        }

        // THE SPLITTER: its drag runs the agent's door.
        let divisor = self.painel(Panel::Inspector).widget(id!(name_splitter));
        if let Some(w) = divisor.as_crawrad_column_splitter().dragged(actions) {
            let args = crawrad_actions::Args::one("width", Value::Float(w));
            self.execute_with(cx, "inspector.set_name_width", args);
        }
        let inspetor = self
            .painel(Panel::Inspector)
            .portal_list(id!(inspector_panel.inspector_list));
        if inspetor.any_items_with_actions(actions) {
            let props = self.linhas_do_inspector();
            let mut pedido: Option<(inspector::Outcome, NodeId, &'static str)> = None;
            // The category the user asked to fold, if any.
            let mut dobrar: Option<String> = None;
            let mut rascunho: Option<Edicao> = None;
            let mut rascunho_evento: Option<EventEdit> = None;
            let mut confirmar = false;
            let mut confirmar_evento = false;
            let mut descartar = false;
            for (i, item) in inspetor.items_with_actions(actions) {
                if let Some(inspector::Row::Event(e)) = props.get(i) {
                    let campo = item.text_input(id!(value));
                    if let Some(texto) = campo.changed(actions)
                        && let Some(no) =
                            self.host.doc().and_then(|d| d.selection().first().cloned())
                    {
                        rascunho_evento = Some(EventEdit {
                            no,
                            evento: e.event,
                            texto,
                        });
                    }
                    if campo.returned(actions).is_some() {
                        confirmar_evento = true;
                    }
                    if campo.escaped(actions) {
                        descartar = true;
                    }
                    continue;
                }
                if let Some(inspector::Row::Category { label, .. }) = props.get(i) {
                    if item.button(id!(alvo)).clicked(actions) {
                        dobrar = Some((*label).to_string());
                    }
                    continue;
                }
                let Some(inspector::Row::Prop(p)) = props.get(i) else {
                    continue;
                };
                let Some(no) = self.host.doc().and_then(|d| d.selection().first().cloned()) else {
                    continue;
                };
                for celula in p.editor.cells() {
                    let slot = celula.slot();
                    let caminho = slot_path(slot);
                    match celula {
                        inspector::Cell::Field { .. } => {
                            let campo = item.text_input(caminho);
                            if let Some(texto) = campo.changed(actions) {
                                rascunho = Some(Edicao {
                                    no: no.clone(),
                                    key: p.key,
                                    linha: i,
                                    slot,
                                    texto,
                                });
                            }
                            if campo.returned(actions).is_some() {
                                confirmar = true;
                            }
                            if campo.escaped(actions) {
                                descartar = true;
                            }
                        }
                        inspector::Cell::Check { .. } => {
                            if let Some(v) = item.check_box(caminho).changed(actions) {
                                pedido = Some((
                                    p.editor.gesture(slot, inspector::Gesture::Toggled(v)),
                                    no.clone(),
                                    p.key,
                                ));
                            }
                        }
                        inspector::Cell::Choice { .. } => {
                            if let Some(escolha) = item.drop_down(caminho).selected(actions) {
                                pedido = Some((
                                    p.editor.gesture(slot, inspector::Gesture::Chose(escolha)),
                                    no.clone(),
                                    p.key,
                                ));
                            }
                        }
                        inspector::Cell::Press { .. } => {
                            let btn = item.button(caminho);
                            if btn.clicked(actions) {
                                let r = btn.area().rect(cx);
                                self.pad_pos = Some(dvec2(
                                    r.pos.x + r.size.x - ALIGN_LARGURA,
                                    r.pos.y + r.size.y,
                                ));
                                pedido = Some((
                                    p.editor.gesture(slot, inspector::Gesture::Pressed),
                                    no.clone(),
                                    p.key,
                                ));
                            }
                        }
                        inspector::Cell::Caption { .. } | inspector::Cell::Swatch { .. } => {}
                    }
                }
            }
            if let Some(e) = rascunho {
                self.edicao = Some(e);
            }
            if let Some(e) = rascunho_evento {
                self.edicao_de_evento = Some(e);
            }
            if descartar {
                self.edicao = None;
                self.edicao_de_evento = None;
                self.ui.redraw(cx);
            }
            if confirmar {
                self.confirmar_edicao(cx);
            }
            if confirmar_evento {
                self.confirm_event(cx);
            }
            if let Some((resultado, no, key)) = pedido {
                self.aplicar_resultado(cx, resultado, Some(&no), key);
            }
            if let Some(rotulo) = dobrar {
                self.execute_with(
                    cx,
                    inspector::ACTION_TOGGLE_CATEGORY,
                    crawrad_actions::Args::text("label", rotulo),
                );
            }
        }

        let lista = self
            .painel(Panel::Components)
            .portal_list(id!(structure_tree.structure_list));
        if let Some(i) = clicked_row(&lista, actions)
            && let Some(l) = self.linhas_da_arvore().get(i)
        {
            self.select_node(cx, &l.id.clone());
        }
    }
}
