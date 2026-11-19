//! USER INTERFACE DRAWING — `draw_2d` and dispatch per screen region.
//!
//! # Why division by region
//!
//! Each `PortalList` requests its drawing step via `step()`.
//! Splitting the loop by screen region makes navigation modular and keeps each
//! region isolated (rail, editor, inspector, dialogs, menus, and dock panels).

use crawrad_ir::NodeId;
use makepad_widgets::*;

use super::icon;
use super::*;
use crawrad_note::note;

/// Draws `$rows` into the portal list `$list` when it is the one being drawn:
/// one `$template` item per visible row, written by the body; `before` runs
/// first, `then` after each item is drawn (its area is final there), `finally`
/// after the last. Every list of this file had
/// the same eight lines of frame around a different body.
macro_rules! draw_rows {
    ($cx:ident, $next:ident, $list:expr, $rows:expr, $template:expr,
     $(before { $($pre:tt)* })?
     |$item:ident, $i:ident, $row:pat_param| $body:block
     $(then $after:block)? $(finally $post:block)?) => {
        if let Some(mut list) = $list.borrow_mut_if_eq(&$next) {
            $($($pre)*)?
            list.set_item_range($cx, 0, $rows.len());
            while let Some($i) = list.next_visible_item($cx) {
                let Some($row) = $rows.get($i) else { continue };
                let $item = list.item($cx, $i, $template);
                $body
                $item.draw_all($cx, &mut Scope::empty());
                $($after)?
            }
            $($post)?
            continue;
        }
    };
}

/// A project name longer than this ends in "…" on its Explorer line.
const PROJECT_NAME_MAX: usize = 18;

impl IdeApp {
    #[note("ide-app-draw-ideapp-draw-2d")]
    pub(super) fn draw_2d(&mut self, cx: &mut Cx2d) {
        self.follow_save_buttons(cx);
        let linhas = self.linhas_da_arvore();
        let inherited = self
            .host
            .doc()
            .map(|d| d.inherited_nodes())
            .unwrap_or_default();
        let sel: Vec<NodeId> = self.host.selection().to_vec();
        let itens = self.linhas_da_paleta();
        // the Data > Field columns of the bound datasets, the grids' rows
        let dir = self.membro();
        self.host.probe_dataset_columns(dir.as_deref());
        let rows = self.host.design_rows(dir.as_deref());
        if let Some(mut c) = self.canvas().borrow_mut::<DesignCanvas>() {
            c.runtime_mut().set_design_rows(rows);
        }
        let props = self.linhas_do_inspector();

        let toolbox = self
            .painel(Panel::Components)
            .portal_list(id!(toolbox_palette.toolbox_list));
        let arvore = self
            .painel(Panel::Components)
            .portal_list(id!(structure_tree.structure_list));
        let inspetor = self
            .painel(Panel::Inspector)
            .portal_list(id!(inspector_panel.inspector_list));
        let trilho = self.ui.portal_list(id!(activity_rail.rail_list));
        let abas_do_editor = self
            .painel(Panel::Design)
            .portal_list(id!(editor_tabs_bar.editor_tabs));
        let alheios = crawrad_host::navigation::foreign_roots();
        let gerado = crawrad_codegen::SUFIXO_GERADO;
        let rotulos_das_abas: Vec<String> = self
            .abas
            .labels()
            .iter()
            .zip(self.abas.files())
            .map(|(rotulo, arquivo)| {
                let icone = crawrad_host::navigation::tab_icon(arquivo, &alheios, gerado);
                let deleted = if !arquivo.exists() { "  deleted" } else { "" };
                format!("{icone}{}{deleted}", rotulo.one_line())
            })
            .collect();
        // Unsaved work shows where the X is (the tab's `dirty`), as a modern editor does.
        let dirty_tabs: Vec<bool> = (self.abas.files().iter())
            .map(|arquivo| self.host.buffer(arquivo).is_some())
            .collect();
        let aba_a_frente = self.abas.active_index();
        let items_lista = self
            .ui
            .view(id!(items_dialog.dlg))
            .portal_list(id!(items_list));
        let acoes_dlg = self.ui.view(id!(actions_dialog.dlg));
        let cat_lista = acoes_dlg.portal_list(id!(cat_list));
        let act_lista = acoes_dlg.portal_list(id!(act_list));
        let std_lista = self
            .ui
            .view(id!(standard_dialog.dlg))
            .portal_list(id!(std_list));
        let (cat_linhas, act_linhas) = self.linhas_das_acoes();
        let cat_atual = self.categoria_de_acao.clone();
        let act_sel = self.host.doc().and_then(|d| d.selection().first().cloned());
        let iml_dlg = self.ui.view(id!(imagelist_dialog.dlg));
        let iml_grade = iml_dlg.portal_list(id!(iml_grid));
        let iml_fileiras = self.fileiras_das_imagens();
        let iml_imagens = self
            .imagelist
            .as_ref()
            .map(|e| e.entries.clone())
            .unwrap_or_default();
        let iml_sel = self.imagelist.as_ref().and_then(|e| e.selected);
        let add_lista = self
            .ui
            .view(id!(addicon_dialog.dlg))
            .portal_list(id!(add_list));
        let add_sel = self.imagelist.as_ref().and_then(|e| e.add_selected_preset);
        let galeria = self.galeria.clone();
        let opt_arvore = self
            .ui
            .view(id!(options_dialog.dlg))
            .portal_list(id!(opt_tree));
        let opt_reg_lista = self
            .ui
            .view(id!(options_dialog.dlg))
            .portal_list(id!(opt_reg_list));
        let opt_keymap_lista = self
            .ui
            .view(id!(options_dialog.dlg))
            .portal_list(id!(opt_keymap_list));
        let opt_keymap_atalhos = self.esquema_teclado.shortcut_table();
        let opt_linhas = self.linhas_das_options();
        let opt_secao = self.secao;
        let reg_agents = crawrad_agent::search_registry(&self.chat.acp_registry_query);
        let active_acp_provider = self.acp_config.provider.clone();
        let active_acp_endpoint = self.acp_config.endpoint.trim().to_string();
        let ic_lista = self
            .ui
            .view(id!(icons_dialog.dlg))
            .portal_list(id!(ic_list));
        let quick_open_dlg = self.ui.view(id!(quick_open_dialog.dlg));
        let quick_open_lista = quick_open_dlg.portal_list(id!(quick_list));
        let inherit_form_list =
            (self.ui.view(id!(inherit_form_dialog.dlg))).portal_list(id!(inh_list));
        let nc_props_list =
            (self.ui.view(id!(new_component_dialog.dlg))).portal_list(id!(nc_props_list));
        let tpl_list = (self.ui.view(id!(templates_dialog.dlg))).portal_list(id!(tpl_list));
        let quick_open_is_cmd = !self.nav.links.is_open()
            && crawrad_host::quick_open::is_command_mode(&self.quick_open_filtro);
        let quick_open_cmd_items = self.quick_open_comandos.clone();
        let quick_open_rows = self.nav.rows(&self.quick_open_filtrados);
        let std_linhas = self.linhas_padrao();
        let rec_lista = self
            .ui
            .view(id!(recover_dialog.dlg))
            .portal_list(id!(rec_list));
        let rec_rows = self.recover_offer.rows.clone();
        let items_linhas = self
            .items
            .as_ref()
            .map(|e| items_editor::rows(&e.items))
            .unwrap_or_default();
        let items_sel = self.items.as_ref().and_then(|e| e.selected.clone());
        let arranjo = self.arranjo();
        let popup = self.ui.portal_list(id!(menu_popup.menu_list));
        let sub_popup = self.ui.portal_list(id!(menu_sub_popup.menu_list));
        let comandos = self.menu_rows();
        let sub_comandos = self.submenu_rows();

        let explorador = self
            .dock()
            .item(layout::tab_of(Panel::Explorer))
            .portal_list(id!(explorer_body.explorer_list));
        let loja = self
            .dock()
            .item(layout::tab_of(Panel::Packages))
            .portal_list(id!(packages_body.packages_list));
        let depurador = self
            .dock()
            .item(layout::tab_of(Panel::Debug))
            .portal_list(id!(debug_body.debug_list));
        let debug_rows =
            crawrad_host::debug_panel::rows(self.host.debug(), self.host.breakpoints());
        let references_list = self
            .dock()
            .item(layout::tab_of(Panel::References))
            .portal_list(id!(references_body.references_list));
        let reference_rows = self.nav.references.rows();
        let problems_list = self
            .dock()
            .item(layout::tab_of(Panel::Problems))
            .portal_list(id!(problems_body.problems_list));
        let problem_rows = self.problems.rows();
        let views_list = self
            .dock()
            .item(layout::tab_of(Panel::Extensions))
            .portal_list(id!(extensions_body.views_list));
        let view_rows = self.ext_views.shown();
        let menu_highlight = self.menu_highlight();
        let submenu_highlight = self.open_submenu.as_ref().and_then(|s| s.2);
        let mut explorer_rows: Option<Vec<explorer::Row>> = None;
        let mut package_rows: Option<Vec<packages::Row>> = None;

        let mut rail_rects: Vec<(Panel, Rect)> = Vec::new();
        let mut rects_dos_eventos: Option<Vec<(usize, Rect)>> = None;
        let visible_rail = rail::visible(!self.no_form_in_project());

        // Looked up BEFORE the draw loop: inside it the Inspector list is borrowed, and a lookup
        // that misses the cache walks the panel, reaches that list and panics.
        let splitter = self.painel(Panel::Inspector).widget(id!(name_splitter));
        while let Some(proximo) = self.ui.draw(cx, &mut Scope::empty()).step() {
            draw_rows! { cx, proximo, trilho, visible_rail, live_id!(Destino), |item, i, &painel| {
                item.set_text(cx, rail::glyph(painel));
                let marca = f64::from(u8::from(self.painel_a_vista(&arranjo, painel)));
                item.apply_over(cx, live! { draw_bg: { active: (marca) } });
            } then {
                rail_rects.push((painel, item.area().rect(cx)));
            } }
            draw_rows! { cx, proximo, abas_do_editor, rotulos_das_abas, live_id!(EditorTab), |item, i, rotulo| {
                item.button(id!(tab_label)).set_text(cx, rotulo);
                let dirty = f64::from(u8::from(dirty_tabs.get(i) == Some(&true)));
                let close = item.button(id!(tab_close));
                close.set_text(cx, icon::Chrome::Close.glyph());
                close.apply_over(
                    cx,
                    live! { draw_bg: { dirty: (dirty) } draw_text: { dirty: (dirty) } },
                );
                let marca = f64::from(u8::from(aba_a_frente == Some(i)));
                item.apply_over(cx, live! { draw_bg: { active: (marca) } });
            } }
            if let Some(mut lista) = inspetor.borrow_mut_if_eq(&proximo) {
                let eventos = rects_dos_eventos.get_or_insert_with(Vec::new);
                // THE SPLITTER, on the list's own turtle (its area is mid-rebuild).
                let (name_w, left) = self.name_splitter.layout(cx.turtle().rect().size.x);
                splitter.as_crawrad_column_splitter().set_edge(name_w, left);
                lista.set_item_range(cx, 0, props.len());
                let marks = match self.host.selection() {
                    [id] => self.form_check.props_of(id),
                    _ => Vec::new(),
                };
                while let Some(i) = lista.next_visible_item(cx) {
                    let Some(linha) = props.get(i) else { continue };
                    let molde = match linha {
                        inspector::Row::Category { .. } => live_id!(Category),
                        inspector::Row::Notice(_) => live_id!(Notice),
                        inspector::Row::Event(e) => LiveId::from_str(e.template()),
                        inspector::Row::InheritedFrom(_) => live_id!(InheritedFrom),
                        inspector::Row::Prop(p) => LiveId::from_str(p.editor.template()),
                    };
                    // Some(error) when a finding stands behind this property
                    let mark = match linha {
                        inspector::Row::Prop(p) => (marks.iter())
                            .filter(|m| m.0 == p.key)
                            .map(|m| m.1)
                            .reduce(|a, b| a || b),
                        _ => None,
                    };
                    let (item, existia) = lista.item_with_existed(cx, i, molde);
                    Self::escrever_linha(
                        cx,
                        &item,
                        !existia,
                        i,
                        linha,
                        self.edicao.as_ref(),
                        self.edicao_de_evento.as_ref(),
                        &mut self.enums_na_tela,
                        mark,
                    );
                    // Each frame, as the Explorer writes its indent; a row with no name has no box.
                    (item.widget(id!(name_box))).apply_over(cx, live! { width: (name_w) });
                    item.draw_all(cx, &mut Scope::empty());
                    if let inspector::Row::Prop(p) = linha
                        && p.key == "name"
                        && std::mem::take(&mut self.name_focus_pending)
                        && let Some(c) = p.editor.cells().first()
                    {
                        cx.set_key_focus(item.text_input(slot_path(c.slot())).area());
                    }
                    if matches!(linha, inspector::Row::Event(_)) {
                        eventos.push((i, item.area().rect(cx)));
                    }
                }
                continue;
            }
            if let Some(mut lista) = iml_grade.borrow_mut_if_eq(&proximo) {
                lista.set_item_range(cx, 0, iml_fileiras);
                while let Some(f) = lista.next_visible_item(cx) {
                    let item = lista.item(cx, f, live_id!(ImgRow));
                    for (coluna, caminho) in IML_CELULAS.iter().enumerate() {
                        let indice = f * IML_COLUNAS + coluna;
                        let celula = item.view(caminho());
                        let Some(img) = iml_imagens.get(indice) else {
                            celula.set_visible(cx, false);
                            continue;
                        };
                        celula.set_visible(cx, true);
                        celula.label(id!(cel_simbolo)).set_text(cx, &img.symbol);
                        celula
                            .label(id!(cel_indice))
                            .set_text(cx, &indice.to_string());
                        let marca = f64::from(u8::from(iml_sel == Some(indice)));
                        celula
                            .view(id!(cel_fundo))
                            .apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
                    }
                    item.draw_all(cx, &mut Scope::empty());
                }
                continue;
            }
            // THE ICON SHEET: one chrome role per row.
            draw_rows! { cx, proximo, ic_lista, icon::Chrome::ALL, live_id!(IconRow), |item, i, papel| {
                item.label(id!(papel)).set_text(cx, &format!("{papel:?}"));
                for caminho in [id!(g16), id!(g24), id!(g32)] {
                    item.label(caminho).set_text(cx, papel.glyph());
                }
                item.label(id!(nome)).set_text(cx, papel.codicon_name());
            } }
            // THE OPTIONS TREE: group that folds and item that switches pages.
            if let Some(mut lista) = opt_arvore.borrow_mut_if_eq(&proximo) {
                lista.set_item_range(cx, 0, opt_linhas.len());
                self.dialog_rows.clear(); // what view.get_dialog reports as `rows`
                while let Some(i) = lista.next_visible_item(cx) {
                    let Some(linha) = opt_linhas.get(i) else {
                        continue;
                    };
                    match linha {
                        options::OptionRow::Group { name, open } => {
                            let item = lista.item(cx, i, live_id!(OptGrupo));
                            item.label(id!(face.marca)).set_text(
                                cx,
                                if *open {
                                    inspector::icon::FOLD_OPEN
                                } else {
                                    inspector::icon::FOLD_CLOSED
                                },
                            );
                            item.label(id!(face.nome)).set_text(cx, name);
                            item.draw_all(cx, &mut Scope::empty());
                            self.dialog_rows.push((name.to_string(), item.area()));
                        }
                        options::OptionRow::Item(c) => {
                            let item = lista.item(cx, i, live_id!(OptItem));
                            item.label(id!(rotulo)).set_text(cx, c.label());
                            // The INDENT indicates it is a child of the group above.
                            item.label(id!(rotulo))
                                .apply_over(cx, live! { padding: { left: (RECUO_SUB) } });
                            let marca = f64::from(u8::from(*c == opt_secao));
                            item.apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
                            item.draw_all(cx, &mut Scope::empty());
                            self.dialog_rows.push((c.label().to_string(), item.area()));
                        }
                    }
                }
                continue;
            }
            // THE ACP REGISTRY AGENT LIST (Official Portal)
            draw_rows! { cx, proximo, opt_reg_lista, reg_agents, live_id!(RegRow), |item, i, agent| {
                item.label(id!(reg_agent_name)).set_text(cx, &agent.name);
                item.label(id!(reg_agent_badge)).set_text(cx, &agent.badge);
                item.label(id!(reg_agent_desc))
                    .set_text(cx, &agent.description);

                let is_installed = agent.is_installed();
                item.view(id!(reg_installed_box))
                    .set_visible(cx, is_installed);

                let is_active = agent.is_active(&active_acp_endpoint, &active_acp_provider);
                item.view(id!(reg_active_box)).set_visible(cx, is_active);

                let active_flag = if is_active { 1.0 } else { 0.0 };
                item.view(id!(reg_card)).apply_over(
                    cx,
                    live! {
                        draw_bg: {
                            is_active: (active_flag)
                        }
                    },
                );

                item.button(id!(reg_agent_action))
                    .set_text(cx, agent.action_button_text(is_active));

                let tag_chips = [
                    (id!(reg_tag_0), id!(reg_tag_0_lbl)),
                    (id!(reg_tag_1), id!(reg_tag_1_lbl)),
                    (id!(reg_tag_2), id!(reg_tag_2_lbl)),
                    (id!(reg_tag_3), id!(reg_tag_3_lbl)),
                ];
                for (t_idx, (box_id, lbl_id)) in tag_chips.iter().enumerate() {
                    if let Some(tag_text) = agent.tags.get(t_idx) {
                        item.view(*box_id).set_visible(cx, true);
                        item.label(*lbl_id).set_text(cx, tag_text);
                    } else {
                        item.view(*box_id).set_visible(cx, false);
                    }
                }

                item.button(id!(reg_agent_repo))
                    .set_text(cx, agent.repo_button_text());
            } }
            draw_rows! { cx, proximo, opt_keymap_lista, opt_keymap_atalhos, live_id!(KeyRow), |item, i, (cmd, atalho)| {
                item.label(id!(key_cmd)).set_text(cx, cmd);
                item.label(id!(key_binding)).set_text(cx, atalho);
            } }
            if let Some(mut lista) = inherit_form_list.borrow_mut_if_eq(&proximo) {
                self.draw_inherit_form_list(cx, &mut lista);
                continue;
            }
            if let Some(mut lista) = nc_props_list.borrow_mut_if_eq(&proximo) {
                self.draw_new_component_props_list(cx, &mut lista);
                continue;
            }
            if let Some(mut lista) = tpl_list.borrow_mut_if_eq(&proximo) {
                self.draw_templates_list(cx, &mut lista);
                continue;
            }
            if let Some(mut lista) = quick_open_lista.borrow_mut_if_eq(&proximo) {
                if quick_open_is_cmd {
                    lista.set_item_range(cx, 0, quick_open_cmd_items.len());
                    while let Some(i) = lista.next_visible_item(cx) {
                        let Some(cmd) = quick_open_cmd_items.get(i) else {
                            continue;
                        };
                        let item = lista.item(cx, i, live_id!(FileRow));
                        let path = match crawrad_actions::by_id(&cmd.id) {
                            Some(a) => crawrad_actions::menu_path(&a),
                            None if cmd.group.is_empty() => cmd.label.clone(),
                            None => format!("{}: {}", cmd.group, cmd.label),
                        };
                        let display = if let Some(ref sc) = cmd.shortcut {
                            format!("{}  ({})", path, sc)
                        } else {
                            path
                        };
                        item.label(id!(rotulo)).set_text(cx, &display);
                        item.draw_all(cx, &mut Scope::empty());
                    }
                } else {
                    lista.set_item_range(cx, 0, quick_open_rows.len());
                    while let Some(i) = lista.next_visible_item(cx) {
                        let Some(row) = quick_open_rows.get(i) else {
                            continue;
                        };
                        let item = lista.item(cx, i, live_id!(FileRow));
                        item.label(id!(rotulo)).set_text(cx, row);
                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
                continue;
            }
            draw_rows! { cx, proximo, add_lista, galeria, live_id!(PresetRow), |item, i, p| {
                item.label(id!(rotulo))
                    .set_text(cx, &image_list_editor::gallery_label(p));
                let marca = f64::from(u8::from(add_sel == Some(i)));
                item.apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
            } }
            draw_rows! { cx, proximo, cat_lista, cat_linhas, live_id!(CatRow), |item, i, nome| {
                item.label(id!(rotulo)).set_text(cx, nome);
                let marca = f64::from(u8::from(*nome == cat_atual));
                item.apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
            } }
            draw_rows! { cx, proximo, act_lista, act_linhas, live_id!(ActRow), |item, i, linha| {
                item.label(id!(rotulo)).set_text(cx, &linha.label);
                let marca = f64::from(u8::from(act_sel.as_deref() == Some(&linha.id)));
                item.apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
            } }
            draw_rows! { cx, proximo, rec_lista, rec_rows, live_id!(RecRow), |item, i, row| {
                item.check_box(id!(marca)).set_active(cx, row.checked);
                item.label(id!(file))
                    .set_text(cx, &row.file.display().to_string());
                item.label(id!(copy)).set_text(cx, &row.copy);
                item.label(id!(disk)).set_text(cx, &row.disk);
            } }
            if let Some(mut lista) = std_lista.borrow_mut_if_eq(&proximo) {
                lista.set_item_range(cx, 0, std_linhas.len());
                while let Some(i) = lista.next_visible_item(cx) {
                    let Some(linha) = std_linhas.get(i) else {
                        continue;
                    };
                    match linha {
                        action_list::StandardRow::Group { name, open } => {
                            let item = lista.item(cx, i, live_id!(StdGrupo));
                            item.label(id!(face.marca)).set_text(
                                cx,
                                if *open {
                                    inspector::icon::FOLD_OPEN
                                } else {
                                    inspector::icon::FOLD_CLOSED
                                },
                            );
                            item.label(id!(face.nome)).set_text(cx, name);
                            item.draw_all(cx, &mut Scope::empty());
                        }
                        action_list::StandardRow::Class { label, checked, .. } => {
                            let item = lista.item(cx, i, live_id!(StdClasse));
                            item.check_box(id!(marca)).set_active(cx, *checked);
                            item.label(id!(nome)).set_text(cx, label);
                            item.draw_all(cx, &mut Scope::empty());
                        }
                    }
                }
                continue;
            }
            draw_rows! { cx, proximo, items_lista, items_linhas, live_id!(ItemRow), |item, i, linha| {
                item.label(id!(rotulo)).set_text(cx, &linha.text);
                let recuo = linha.level as f64 * RECUO_SUB;
                item.label(id!(rotulo))
                    .apply_over(cx, live! { padding: { left: (recuo) } });
                let marca = f64::from(u8::from(
                    items_sel.as_deref() == Some(linha.path.as_slice()),
                ));
                item.apply_over(cx, live! { draw_bg: { selecionada: (marca) } });
            } }
            draw_rows! { cx, proximo, toolbox, itens, match linha {
                palette::Row::Empty => live_id!(Vazia),
                palette::Row::Section(_) if i == 0 => live_id!(SecaoPrimeira),
                palette::Row::Section(_) => live_id!(Secao),
                palette::Row::Item { blocked: Some(_), .. } => live_id!(ItemOff),
                palette::Row::Item { .. } => live_id!(Item),
            }, before { let strip = self.class_icon_texture(cx); } |item, i, linha| {
                match linha {
                    palette::Row::Item { class, label, .. } => {
                        let kind = self.class_icon_strip.cells().kind_of(class);
                        let count = self.class_icon_strip.cells().count();
                        Self::escrever_linha_com_icone(cx, &item, kind, count, label, "", &strip);
                    }
                    _ => item.set_text(cx, linha.text()),
                }
            } }
            draw_rows! { cx, proximo, explorador, linhas, LiveId::from_str(linha.template()),
                before { let linhas = explorer_rows.get_or_insert_with(|| self.linhas_do_explorer()); let mut exp_rects = Vec::new(); }
                |item, i, linha| {
                Self::escrever_linha_do_explorer(cx, &item, linha);
                match linha.template() {
                    "GroupName" => item.button(id!(btn_ws_more)).set_text(cx, icon::Chrome::More.glyph()),
                    t if t.starts_with("Member") => {
                        Self::write_project_actions(cx, &item);
                        // A long name ends in "…" so the actions never wrap.
                        let name = crawrad_project::workspace::shorten_end(linha.text(), PROJECT_NAME_MAX);
                        // Icon and name live in the project's frame.
                        item.label(id!(face.icone)).set_text(cx, "");
                        item.label(id!(face.nome)).set_text(cx, "");
                        item.label(id!(face.pilula.glifo)).set_text(cx, linha.icon().unwrap_or(""));
                        item.label(id!(face.pilula.texto)).set_text(cx, &name);
                    }
                    _ => {}
                }
            } then {
                exp_rects.push((i, item.area().rect(cx)));
            } finally {
                self.explorer_drawn = Self::explorer_drawn(linhas, &exp_rects);
                self.explorer_rects = exp_rects;
            } }
            // THE PACKAGE MANAGER, via the same path and for the same reason.
            draw_rows! { cx, proximo, depurador, debug_rows, LiveId::from_str(linha.template()), |item, i, linha| {
                Self::write_debug_row(cx, &item, linha);
            } }
            // Where each row lands, for MCP to give a test to click.
            draw_rows! { cx, proximo, references_list, reference_rows, LiveId::from_str(linha.template()),
                before { self.nav.references.rects.clear(); } |item, i, linha| {
                Self::write_reference_row(cx, &item, linha);
            } then {
                self.nav.references.rects.push((i, xywh(item.area().rect(cx))));
            } }
            draw_rows! { cx, proximo, problems_list, problem_rows, LiveId::from_str(linha.template()),
                before { self.problems.rects.clear(); } |item, i, linha| {
                Self::write_problem_row(cx, &item, linha);
            } then {
                self.problems.rects.push((i, xywh(item.area().rect(cx))));
            } }
            draw_rows! { cx, proximo, views_list, view_rows, LiveId::from_str(row.template()),
                before { self.ext_views.rects.clear(); self.ext_views.buttons.clear(); } |item, i, row| {
                Self::write_view_row(cx, &item, row);
            } then {
                self.ext_views.rects.push((i, xywh(item.area().rect(cx))));
                let b = item.button(id!(face.uninstall)).area().rect(cx);
                if b.size.x > 0.0 {
                    self.ext_views.buttons.push((i, xywh(b)));
                }
            } }
            draw_rows! { cx, proximo, loja, linhas, LiveId::from_str(linha.template()),
                before { let linhas = package_rows.get_or_insert_with(|| self.linhas_dos_pacotes()); } |item, i, linha| {
                Self::escrever_linha_de_pacote(cx, &item, linha);
            } }
            if let Some(mut lista) = popup.borrow_mut_if_eq(&proximo) {
                self.draw_menu_list(cx, &mut lista, &comandos, menu_highlight, &arranjo);
                continue;
            }
            if let Some(mut lista) = sub_popup.borrow_mut_if_eq(&proximo) {
                self.draw_menu_list(cx, &mut lista, &sub_comandos, submenu_highlight, &arranjo);
                continue;
            }

            let Some(mut lista) = arvore.borrow_mut_if_eq(&proximo) else {
                continue;
            };
            let strip = self.class_icon_texture(cx);
            lista.set_item_range(cx, 0, linhas.len());
            while let Some(i) = lista.next_visible_item(cx) {
                let Some(linha) = linhas.get(i) else { continue };
                let molde = if linha.nivel == 0 {
                    live_id!(Raiz)
                } else {
                    live_id!(Linha)
                };
                let item = lista.item(cx, i, molde);
                item.label(id!(face.visto)).set_text(
                    cx,
                    if sel.contains(&linha.id) {
                        icon::Chrome::Check.glyph()
                    } else {
                        ""
                    },
                );
                let recuo = RECUO_BASE + RECUO_POR_NIVEL * linha.nivel as f64;
                item.widget(id!(face))
                    .apply_over(cx, live! { padding: { left: (recuo) } });
                let kind = if linha.nivel == 0 {
                    Some(icon::ClassIcon::Form.kind())
                } else {
                    self.class_icon_strip.cells().kind_of(&linha.classe)
                };
                let count = self.class_icon_strip.cells().count();
                Self::escrever_linha_com_icone(
                    cx,
                    &item,
                    kind,
                    count,
                    &linha.nome,
                    &linha.sufixo_de_classe(),
                    &strip,
                );
                let mark = if inherited.contains(&linha.id) {
                    "inherited"
                } else {
                    ""
                };
                item.label(id!(face.inherited)).set_text(cx, mark);
                item.draw_all(cx, &mut Scope::empty());
            }
        }

        if !rail_rects.is_empty() {
            self.rail_rects = rail_rects;
        }
        if let Some(eventos) = rects_dos_eventos {
            self.rects_dos_eventos = eventos;
        }

        // Automatically scrolls AI chat to the bottom when there are new messages or updates
        if self.chat.scroll_to_bottom_frames > 0 {
            self.chat.scroll_to_bottom_frames -= 1;
            let chat = self.painel(Panel::Chat);
            chat.view(id!(ai_messages_view))
                .set_scroll_pos(cx, dvec2(0.0, 100_000.0));
            chat.view(id!(ai_messages_view)).redraw(cx);
        }
    }

    #[note("ide-app-draw-ideapp-draw-menu-list")]
    fn draw_menu_list(
        &self,
        cx: &mut Cx2d,
        lista: &mut PortalList,
        rows: &[menu::Row],
        highlight: Option<usize>,
        arranjo: &std::collections::HashMap<LiveId, DockItem>,
    ) {
        lista.set_item_range(cx, 0, rows.len());
        while let Some(i) = lista.next_visible_item(cx) {
            let Some(linha) = rows.get(i) else { continue };
            // The SEPARATOR is not a command: it has no click target inside.
            if linha.separator {
                let item = lista.item(cx, i, live_id!(Separador));
                item.draw_all(cx, &mut Scope::empty());
                continue;
            }
            // The template comes from `enabled`: the disabled item APPEARS.
            let molde = match (linha.tone, linha.enabled, highlight == Some(i)) {
                (menu::Tone::Title, ..) => live_id!(ComandoTitulo),
                (menu::Tone::Notice, ..) => live_id!(ComandoAviso),
                (_, true, true) => live_id!(ComandoAtivo),
                (_, true, false) => live_id!(Comando),
                _ => live_id!(ComandoCinza),
            };
            let item = lista.item(cx, i, molde);
            let tick = match linha.panel_state {
                Some(p) if self.painel_a_vista(arranjo, p) => icon::Chrome::Check.glyph(),
                Some(_) => "",
                None if linha.tone == menu::Tone::Notice => icon::Chrome::Lock.glyph(),
                None => linha
                    .action
                    .as_deref()
                    .and_then(icon::Chrome::for_action)
                    .map_or("", |i| i.glyph()),
            };
            // A submenu row shows its chevron where a command shows its keys.
            let keys = match linha.submenu {
                Some(_) => crawrad_designer::inspector_icon::FOLD_CLOSED,
                None => linha.shortcut.as_deref().unwrap_or(""),
            };
            item.label(id!(face.marca)).set_text(cx, tick);
            item.label(id!(face.rotulo)).set_text(cx, &linha.label);
            item.label(id!(face.atalho)).set_text(cx, keys);
            item.draw_all(cx, &mut Scope::empty());
        }
    }

    /// The Explorer's actions, on the project's name line (owner, 13/11).
    fn write_project_actions(cx: &mut Cx, item: &WidgetRef) {
        let glyphs = [
            (id!(btn_exp_new_file), icon::Chrome::NewFile),
            (id!(btn_exp_new_form), icon::Chrome::NewForm),
            (id!(btn_exp_new_folder), icon::Chrome::NewFolder),
            (id!(btn_exp_refresh), icon::Chrome::Refresh),
            (id!(btn_exp_collapse), icon::Chrome::CollapseAll),
        ];
        for (button, glyph) in glyphs {
            item.button(button).set_text(cx, glyph.glyph());
        }
    }
}
