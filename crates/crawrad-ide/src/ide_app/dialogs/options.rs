//! Options, the IDE's window about the IDE: a piece of `impl IdeApp` (it
//! reaches the shell's private state); each dialog's draw/handle pair together.

use super::super::*;
use crawrad_agent::acp_registry::{self, AgentState};
use crawrad_agent::chat_config::{menu_or_catalogue, pick};
use crawrad_runtime_makepad::drive::{checked_in, fill_drop, picked_in};

#[note("ide-app-dialogs-options-agent-menus")]
const AGENT_MENUS: [(&str, &str); 2] = [
    ("opt_ai_model", "model"),
    ("opt_ai_effort", "thought_level"),
];

#[note("ide-app-dialogs-options-fill-font-drop")]
fn fill_font_drop(cx: &mut Cx, dlg: &ViewRef, drop: &[LiveId], size: u32) {
    let labels = FONT_SIZES.iter().map(|s| format!("{s} pt")).collect();
    fill_drop(
        cx,
        dlg,
        drop,
        labels,
        FONT_SIZES.iter().position(|&s| s == size),
    );
}

#[note("ide-app-dialogs-options-fill-grid-checks")]
fn fill_grid_checks(cx: &mut Cx, dlg: &ViewRef, [show, snap]: [&[LiveId; 1]; 2], on: [bool; 2]) {
    for (b, (text, on)) in [show, snap]
        .into_iter()
        .zip([OPT_VER_GRADE, OPT_SNAP].into_iter().zip(on))
    {
        dlg.check_box(b).set_text(text);
        dlg.check_box(b).set_active(cx, on);
    }
}

use crawrad_note::note;
pub use crawrad_project::options::{FONT_SIZES, TAB_SIZES};

#[note("ide-app-dialogs-options-agentinstall")]
type AgentInstall = std::sync::mpsc::Receiver<Result<String, String>>;

#[note("ide-app-dialogs-options-scheme")]
pub(crate) fn scheme(i: usize, part: &str) -> [LiveId; 1] {
    let stem = crawrad_project::workspace::COLOR_FIELDS[i].0;
    [LiveId::from_str(&format!("opt_cs_{stem}_{part}"))]
}

impl IdeApp {
    #[note("ide-app-dialogs-options-ideapp-apply-history-settings")]
    pub(crate) fn apply_history_settings(&mut self) {
        let p = crawrad_project::workspace::read_preferences();
        self.disk
            .set_history_settings(crawrad_host::history::HistorySettings {
                enabled: p.history_enabled,
                limit: p.history_limit as usize,
                merge_seconds: u64::from(p.history_merge_seconds),
                max_file_kb: u64::from(p.history_max_file_kb),
            });
    }

    #[note("ide-app-dialogs-options-ideapp-apply-scheme-fields")]
    pub(crate) fn apply_scheme_fields(&mut self, cx: &mut Cx) {
        let dlg = self.ui.view(id!(options_dialog.dlg));
        let inputs: [String; 15] =
            std::array::from_fn(|i| dlg.text_input(&scheme(i, "val")).text());
        let errors = crawrad_project::workspace::Preferences::color_field_errors(&inputs);
        for (i, err) in errors.iter().enumerate() {
            let label = dlg.label(&scheme(i, "err"));
            let height = if err.is_some() { 16.0 } else { 0.0 };
            label.apply_over(cx, live! { height: (height) });
            label.set_text(cx, err.unwrap_or(""));
        }
        let mut pref = crawrad_project::workspace::read_preferences();
        if pref.update_color_fields(&inputs) {
            crawrad_project::workspace::save_preferences(pref);
            self.aplicar_tema_editor(cx);
            self.redraw_options(cx);
        }
    }

    fn save_history_pref(
        &mut self,
        cx: &mut Cx,
        change: impl FnOnce(&mut crawrad_project::workspace::Preferences),
    ) {
        crawrad_project::workspace::update_preferences(change);
        self.apply_history_settings();
        self.redraw_options(cx);
    }

    #[note("ide-app-dialogs-options-ideapp-selecionar-provedor-ia")]
    pub fn selecionar_provedor_ia(&mut self, cx: &mut Cx, provider_name: &str) {
        self.acp_config.switch_provider(provider_name);
        self.save_acp_config();
        self.warm_live_agent(cx);
        self.sync_model_button(cx);
        self.redraw_options(cx);
    }

    #[note("ide-app-dialogs-options-ideapp-open-options")]
    pub(in super::super) fn open_options(&mut self, cx: &mut Cx) {
        self.options = true;
        self.chat.opt_ai_catalogue = false;
        self.grupos_options_fechados = options::closed_on_open().into_iter().collect();
        self.options_antes = Some((
            self.designer.grid,
            self.designer.snap,
            self.ver_grade,
            crawrad_project::workspace::read_preferences(),
        ));
        self.acp_antes = Some(self.acp_config.clone());
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-options-ideapp-linhas-das-options")]
    pub(in super::super) fn linhas_das_options(&self) -> Vec<options::OptionRow> {
        options::option_rows(&|g| self.grupos_options_fechados.contains(g))
    }

    #[note("ide-app-dialogs-options-ideapp-draw-options")]
    pub(in super::super) fn draw_options(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(options_dialog), self.options);
        if !self.options {
            return;
        }
        let secao = self.secao;
        let grade = format!("{}", self.designer.grid as i32);
        let ver_grade = self.ver_grade;
        let snap = self.designer.snap;

        let dlg = self.ui.view(id!(options_dialog.dlg));
        dlg.button(id!(opt_save)).set_text(cx, QUIT_SAVE);
        dlg.button(id!(opt_cancel)).set_text(cx, ITEMS_CANCEL);

        // Search bar: updates the text field if necessary
        let search_input = dlg.text_input(id!(opt_search_input));
        if search_input.text() != self.options_busca {
            search_input.set_text(cx, &self.options_busca);
        }

        // Scope tabs: User | Workspace
        let scope_btn = if self.options_scope == 0 {
            dlg.radio_button(id!(opt_scope_user))
        } else {
            dlg.radio_button(id!(opt_scope_workspace))
        };
        scope_btn.select(cx, &mut Scope::empty());

        let tem_busca = !self.options_busca.trim().is_empty();

        // The pages: one in view, the others hidden.
        let shown = Some(secao)
            .filter(|_| !tem_busca)
            .and_then(options::Category::page);
        let at = |page: &str| shown == Some(page);
        let history = at("opt_history");
        dlg.view(id!(opt_search_results)).set_visible(cx, tem_busca);
        dlg.view(id!(opt_registry)).set_visible(cx, false);
        for page in options::Category::PAGES {
            dlg.view(&[LiveId::from_str(page)])
                .set_visible(cx, at(page));
        }

        if tem_busca {
            let titulo_busca = format!("Search Settings: \"{}\"", self.options_busca.trim());
            dlg.label(id!(opt_pagina)).set_text(cx, &titulo_busca);
            dlg.label(id!(opt_nota)).set_text(cx, "");

            let m = crawrad_project::options::OptionsSearchMatches::from_query(&self.options_busca);

            dlg.view(id!(opt_sr_editor_font)).set_visible(cx, m.font);
            dlg.view(id!(opt_sr_editor_line_numbers))
                .set_visible(cx, m.line);
            dlg.view(id!(opt_sr_saving)).set_visible(cx, m.save);
            dlg.view(id!(opt_sr_grid)).set_visible(cx, m.grid);
            dlg.view(id!(opt_sr_ai)).set_visible(cx, m.ai);
            dlg.view(id!(opt_sr_keymap)).set_visible(cx, m.key);
            dlg.view(id!(opt_sr_empty_wrap))
                .set_visible(cx, !m.has_any());

            // Synchronize visible controls in results
            let pref = crawrad_project::workspace::read_preferences();
            if m.font {
                fill_font_drop(cx, &dlg, id!(opt_sr_font_size), pref.editor_font_size);
            }
            if m.grid {
                let boxes = [id!(opt_sr_ver_grade), id!(opt_sr_snap)];
                fill_grid_checks(cx, &dlg, boxes, [ver_grade, snap]);
            }
            if m.ai {
                let providers = crawrad_agent::AcpConfig::provider_labels();
                let at = self.acp_config.provider_index();
                fill_drop(cx, &dlg, id!(opt_sr_ai_provider), providers, at);
            }
            if m.key {
                let schemes = crawrad_actions::KeymapScheme::labels();
                let at = Some(self.esquema_teclado.index());
                fill_drop(cx, &dlg, id!(opt_sr_keymap_scheme), schemes, at);
            }
        } else {
            dlg.label(id!(opt_pagina)).set_text(cx, secao.title());
            // And the phrase for those that only INFORM — empty on the ones above.
            dlg.label(id!(opt_nota))
                .set_text(cx, secao.note().unwrap_or(""));
        }

        if at("opt_commonly_used") {
            let pref = crawrad_project::workspace::read_preferences();
            fill_font_drop(cx, &dlg, id!(opt_cu_font_size), pref.editor_font_size);

            let boxes = [id!(opt_cu_ver_grade), id!(opt_cu_snap)];
            fill_grid_checks(cx, &dlg, boxes, [ver_grade, snap]);

            let providers = crawrad_agent::AcpConfig::provider_labels();
            let at = self.acp_config.provider_index();
            fill_drop(cx, &dlg, id!(opt_cu_ai_provider), providers, at);
        }

        if at("opt_designer") {
            dlg.label(id!(opt_grade_nota)).set_text(cx, OPT_GRADE_NOTA);
            let boxes = [id!(opt_ver_grade), id!(opt_snap)];
            fill_grid_checks(cx, &dlg, boxes, [ver_grade, snap]);
            let campo = dlg.text_input(id!(opt_grade_x));
            if campo.text() != grade {
                campo.set_text(cx, &grade);
            }
        }
        if at("opt_appearance") {
            dlg.label(id!(opt_zoom_nota)).set_text(cx, OPT_ZOOM_NOTA);
            dlg.label(id!(opt_tema_nota)).set_text(cx, OPT_TEMA_NOTA);
            dlg.button(id!(opt_icones)).set_text(cx, OPT_ICONES);
        }
        if at("opt_keyboard") {
            let schemes = crawrad_actions::KeymapScheme::labels();
            let at = Some(self.esquema_teclado.index());
            fill_drop(cx, &dlg, id!(opt_keymap_scheme), schemes, at);
            dlg.label(id!(opt_keymap_summary))
                .set_text(cx, self.esquema_teclado.description());
        }
        if at("opt_ai") {
            dlg.view(id!(opt_ai_main))
                .set_visible(cx, !self.chat.opt_ai_catalogue);
            dlg.view(id!(opt_ai_catalogue))
                .set_visible(cx, self.chat.opt_ai_catalogue);
            let card = self.agent_card();
            let (op, state) = (&card.status, card.state);
            for (label, text) in [
                (id!(opt_ai_active_name), op.active_name.as_str()),
                (id!(opt_ai_active_ver), &op.active_version),
                (id!(opt_ai_status_text), state.label()),
                (id!(opt_ai_status_hint), &card.hint),
                (id!(opt_ai_active_cmd), &op.command),
                (id!(opt_ai_status_detail), &card.detail),
            ] {
                dlg.label(label).set_text(cx, text);
            }
            // One dot per state, each with its own token: the page shows the one that holds.
            for (dot, its) in [
                (id!(opt_ai_dot_ready), AgentState::Ready),
                (id!(opt_ai_dot_signed_out), AgentState::SignedOut),
                (id!(opt_ai_dot_missing), AgentState::NotInstalled),
            ] {
                dlg.view(dot).set_visible(cx, state == its);
            }
            let fix_btn = dlg.button(id!(opt_ai_fix_btn));
            fix_btn.set_visible(cx, state.fix().is_some());
            fix_btn.set_text(cx, state.fix().unwrap_or_default());
            let open = self.chat.opt_ai_advanced;
            let adv = ["Show advanced", "Hide advanced"][usize::from(open)];
            dlg.button(id!(opt_ai_adv_btn)).set_text(cx, adv);
            dlg.view(id!(opt_ai_adv)).set_visible(cx, open);

            // The same menus as the chat buttons; Effort hides when the agent offers none.
            for (drop, category) in AGENT_MENUS {
                let menu = menu_or_catalogue(&self.chat.live, &self.acp_config, category);
                if category == "thought_level" {
                    dlg.view(id!(opt_ai_effort_row))
                        .set_visible(cx, menu.is_some());
                }
                if let Some(menu) = menu {
                    let at = menu.items.iter().position(|it| it.selected);
                    let labels = menu.items.iter().map(|it| it.label.clone()).collect();
                    fill_drop(cx, &dlg, &[LiveId::from_str(drop)], labels, at);
                }
            }

            for flag in crawrad_agent::acp::AcpFlag::ALL {
                let b = dlg.check_box(&[LiveId::from_str(flag.widget())]);
                b.set_text(flag.label());
                b.set_active(cx, flag.get(&self.acp_config));
            }

            let input_search = dlg.text_input(id!(opt_reg_search));
            if input_search.text() != self.chat.acp_registry_query {
                input_search.set_text(cx, &self.chat.acp_registry_query);
            }
            let reg_agents = crawrad_agent::search_registry(&self.chat.acp_registry_query);
            let count = format!(
                "From the official ACP registry · {} agents",
                reg_agents.len()
            );
            let fb = self.chat.acp_registry_feedback.as_deref().unwrap_or(&count);
            dlg.label(id!(opt_reg_feedback)).set_text(cx, fb);

            let link_text =
                crawrad_agent::acp_registry::registry_link_text(&self.chat.acp_registry_query);
            dlg.button(id!(opt_reg_link)).set_text(cx, &link_text);
        }
        if at("opt_editor") {
            let pref = crawrad_project::workspace::read_preferences();
            fill_font_drop(cx, &dlg, id!(opt_editor_font_size), pref.editor_font_size);
            let tabs = TAB_SIZES.iter().map(|s| format!("{s} spaces")).collect();
            let at = TAB_SIZES.iter().position(|&s| s == pref.editor_tab_size);
            fill_drop(cx, &dlg, id!(opt_editor_tab_size), tabs, at);
        }
        if at("opt_color_scheme") {
            let pref = crawrad_project::workspace::read_preferences();

            let presets = crawrad_project::workspace::EditorColorScheme::preset_labels(
                pref.editor_color_base,
            );
            let at = Some(pref.editor_color_scheme.position());
            fill_drop(cx, &dlg, id!(opt_cs_preset), presets, at);

            for (i, value) in pref.color_fields().into_iter().enumerate() {
                let more = dlg.button(&scheme(i, "more"));
                more.set_text(cx, crawrad_designer::inspector_icon::OPEN_EDITOR);
                let input = dlg.text_input(&scheme(i, "val"));
                if input.text() != value {
                    input.set_text(cx, value);
                }
                if let Some([r, g, b, a]) = crawrad_project::workspace::hex_to_rgba(value) {
                    let col = vec4(r, g, b, a);
                    let swatch = dlg.view(&scheme(i, "sw"));
                    swatch.apply_over(cx, live! { draw_bg: { color: (col) } });
                    swatch.redraw(cx);
                }
            }

            use crawrad_runtime_makepad::CrawradSchemePreviewWidgetRefExt;
            let rows = crawrad_project::scheme_preview::painted(&pref);
            let preview_rows: Vec<crawrad_runtime_makepad::PreviewRow> = rows
                .into_iter()
                .map(|l| {
                    crawrad_runtime_makepad::PreviewRow::new(
                        l.number,
                        l.number_color,
                        l.band,
                        l.spans,
                    )
                })
                .collect();
            let bg = crawrad_project::scheme_preview::background(&pref);
            dlg.crawrad_scheme_preview(id!(opt_cs_preview))
                .set(cx, bg, preview_rows);
        }
        if at("opt_folders") {
            let pref = crawrad_project::workspace::read_preferences();
            let input = dlg.text_input(id!(opt_default_dir_input));
            let current = pref
                .default_projects_dir
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            if input.text() != current {
                input.set_text(cx, &current);
            }
        }
        // The Saving and Recovering boxes, wherever they sit.
        let pref = crawrad_project::workspace::read_preferences();
        let at = Some(pref.auto_save.position());
        fill_drop(
            cx,
            &dlg,
            id!(opt_auto_save),
            crawrad_project::workspace::AutoSave::labels(),
            at,
        );
        let names = crawrad_project::profile::all();
        let at = names
            .iter()
            .position(|n| *n == pref.desktop_profile)
            .or(Some(0));
        fill_drop(cx, &dlg, id!(opt_profile), names, at);
        for check in options::SavingCheck::ALL {
            for n in check.widgets() {
                let b = dlg.check_box(&[LiveId::from_str(n)]);
                b.set_text(check.label());
                b.set_active(cx, check.get(&pref));
            }
        }
        for check in options::EditorCheck::ALL {
            for n in check.widgets() {
                let b = dlg.check_box(&[LiveId::from_str(n)]);
                b.set_text(check.label());
                b.set_active(cx, check.get(&pref));
            }
        }
        if history {
            dlg.check_box(id!(opt_history_enabled))
                .set_active(cx, pref.history_enabled);
        }
        for choice in options::NumberChoice::ALL
            .into_iter()
            .filter(|c| history || !c.is_history())
        {
            let drop = &[LiveId::from_str(choice.widget())];
            fill_drop(cx, &dlg, drop, choice.labels(), choice.position(&pref));
        }
    }

    #[note("ide-app-dialogs-options-ideapp-atender-options")]
    pub(in super::super) fn atender_options(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.options {
            return;
        }

        if let Some(ref rx) = self.chat.options_test_rx
            && let Ok(res) = rx.try_recv()
        {
            match res {
                Ok(msg) => self.chat.acp_status_msg = Some(msg),
                Err(err) => self.chat.acp_status_msg = Some(format!("Connection failed: {err}")),
            }
            self.chat.options_test_rx = None;
            self.redraw_options(cx);
        }

        let dlg = self.ui.view(id!(options_dialog.dlg));

        // THE TREE: the group collapses, the item changes page.
        let arvore = dlg.portal_list(id!(opt_tree));
        if arvore.any_items_with_actions(actions) {
            let linhas = self.linhas_das_options();
            let mut dobrar = None;
            let mut ir_para = None;
            for (i, item) in arvore.items_with_actions(actions) {
                match linhas.get(i) {
                    Some(options::OptionRow::Group { name, .. })
                        if item.button(id!(alvo)).clicked(actions) =>
                    {
                        dobrar = Some((*name).to_string());
                    }
                    Some(options::OptionRow::Item(c))
                        if item.button(id!(alvo)).clicked(actions) =>
                    {
                        ir_para = Some(*c);
                    }
                    _ => {}
                }
            }
            if let Some(g) = dobrar {
                if !self.grupos_options_fechados.remove(&g) {
                    self.grupos_options_fechados.insert(g);
                }
                self.redraw_options(cx);
            }
            if let Some(c) = ir_para {
                self.secao = c;
                // When navigating manually through the tree, clear search if active
                if !self.options_busca.is_empty() {
                    self.options_busca.clear();
                    dlg.text_input(id!(opt_search_input)).set_text(cx, "");
                }
                self.redraw_options(cx);
            }
        }

        // TOP SEARCH BAR (modern-editor style)
        if let Some(t) = dlg.text_input(id!(opt_search_input)).changed(actions) {
            self.options_busca = t;
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_search_clear)).clicked(actions) {
            self.options_busca.clear();
            dlg.text_input(id!(opt_search_input)).set_text(cx, "");
            self.redraw_options(cx);
        }

        // SCOPE TABS: User | Workspace
        let mut scope_set = WidgetSet::empty();
        scope_set.push(dlg.widget(id!(opt_scope_user)));
        scope_set.push(dlg.widget(id!(opt_scope_workspace)));
        if let Some(i) = scope_set.as_radio_button_set().selected(cx, actions)
            && self.options_scope != i
        {
            self.options_scope = i;
            self.redraw_options(cx);
        }

        if let Some(v) = checked_in(
            &dlg,
            actions,
            &["opt_ver_grade", "opt_cu_ver_grade", "opt_sr_ver_grade"],
        ) {
            self.ver_grade = v;
            self.reabrir_no_canvas(cx);
            self.redraw_options(cx);
        }
        if let Some(v) = checked_in(&dlg, actions, &["opt_snap", "opt_cu_snap", "opt_sr_snap"]) {
            self.designer.snap = v;
            self.redraw_options(cx);
        }
        let menos = dlg.button(id!(opt_grade_menos)).clicked(actions);
        let mais = dlg.button(id!(opt_grade_mais)).clicked(actions);
        if menos || mais {
            let passo = if mais { 1.0 } else { -1.0 };
            self.mudar_grade(cx, self.designer.grid + passo);
        }
        if let Some((t, _)) = dlg.text_input(id!(opt_grade_x)).returned(actions)
            && let Ok(n) = t.trim().parse::<f32>()
        {
            self.mudar_grade(cx, n);
        }

        // AI / ACP page events
        if let Some(i) = picked_in(
            &dlg,
            actions,
            &[
                "opt_ai_provider",
                "opt_cu_ai_provider",
                "opt_sr_ai_provider",
            ],
        ) && let Some(p) = crawrad_agent::AcpConfig::PROVIDERS.get(i)
        {
            self.selecionar_provedor_ia(cx, p);
        }
        for (btn, catalogue) in [
            (id!(opt_ai_change_btn), true),
            (id!(opt_ai_back_btn), false),
        ] {
            if dlg.button(btn).clicked(actions) {
                self.chat.opt_ai_catalogue = catalogue;
                self.redraw_options(cx);
            }
        }
        if dlg.button(id!(opt_ai_adv_btn)).clicked(actions) {
            self.chat.opt_ai_advanced = !self.chat.opt_ai_advanced;
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_ai_fix_btn)).clicked(actions) {
            let card = self.agent_card();
            match card.state {
                AgentState::SignedOut => self.abrir_url_navegador(&card.status.target_url),
                AgentState::NotInstalled => {
                    let agents = crawrad_agent::search_registry("");
                    if let Some(agent) = acp_registry::active_entry(&self.acp_config, &agents) {
                        let res = acp_registry::click_agent_action(agent, &mut self.acp_config);
                        self.apply_agent_click_result(cx, res);
                    }
                }
                AgentState::Ready => {}
            }
            self.redraw_options(cx);
        }
        for (drop, category) in AGENT_MENUS {
            let Some(i) = dlg.drop_down(&[LiveId::from_str(drop)]).selected(actions) else {
                continue;
            };
            if let Some(menu) = menu_or_catalogue(&self.chat.live, &self.acp_config, category) {
                if let Err(e) = pick(&mut self.chat.live, &mut self.acp_config, &menu, i) {
                    self.chat.acp_status_msg = Some(format!("Error: {e}"));
                }
                self.sync_model_button(cx);
                self.redraw_options(cx);
            }
        }
        if let Some((t, _)) = dlg.text_input(id!(opt_ai_endpoint)).returned(actions) {
            self.acp_config.endpoint = t.trim().to_string();
            self.save_acp_config();
        }
        for flag in crawrad_agent::acp::AcpFlag::ALL {
            let check = dlg.check_box(&[LiveId::from_str(flag.widget())]);
            if let Some(v) = check.changed(actions) {
                *flag.field(&mut self.acp_config) = v;
                self.save_acp_config();
                self.redraw_options(cx);
            }
        }
        if let Some(i) = dlg.drop_down(id!(opt_ai_diff_mode)).selected(actions) {
            let diff_modes = [
                crawrad_agent::diff_review::DiffReviewMode::WaitForApproval,
                crawrad_agent::diff_review::DiffReviewMode::ShowAndApply,
                crawrad_agent::diff_review::DiffReviewMode::ApplyDirectly,
            ];
            if let Some(&mode) = diff_modes.get(i) {
                self.acp_config.diff_review_mode = mode;
                self.save_acp_config();
            }
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_ai_test_btn)).clicked(actions) {
            self.chat.acp_status_msg = Some("Testing connection in background...".to_string());
            let cfg = self.acp_config.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            self.chat.options_test_rx = Some(rx);
            std::thread::spawn(move || {
                let res = cfg.test_connection();
                let _ = tx.send(res);
                crawrad_os::tasks::wake();
            });
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_ai_term_btn)).clicked(actions) {
            self.options = false;
            self.show_panel(cx, Panel::Terminal);
            self.dock().select_tab(cx, layout::tab_of(Panel::Terminal));
            self.ui.redraw(cx);
        }

        // ACP Registry page events
        if let Some((t, _)) = dlg.text_input(id!(opt_reg_search)).returned(actions) {
            self.chat.acp_registry_query = t.trim().to_string();
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_reg_clear)).clicked(actions) {
            self.chat.acp_registry_query.clear();
            dlg.text_input(id!(opt_reg_search)).set_text(cx, "");
            self.redraw_options(cx);
        }
        if dlg.button(id!(opt_reg_link)).clicked(actions) {
            let url =
                crawrad_agent::acp_registry::registry_browse_url(&self.chat.acp_registry_query);
            self.abrir_url_navegador(&url);
        }
        let reg_lista = dlg.portal_list(id!(opt_reg_list));
        if reg_lista.any_items_with_actions(actions) {
            let reg_agents = crawrad_agent::search_registry(&self.chat.acp_registry_query);
            for (i, item) in reg_lista.items_with_actions(actions) {
                if item.button(id!(reg_agent_action)).clicked(actions)
                    && let Some(agent) = reg_agents.get(i)
                {
                    let res = crawrad_agent::acp_registry::click_agent_action(
                        agent,
                        &mut self.acp_config,
                    );
                    self.apply_agent_click_result(cx, res);
                    self.redraw_options(cx);
                }
                if item.button(id!(reg_agent_repo)).clicked(actions)
                    && let Some(agent) = reg_agents.get(i)
                {
                    self.abrir_url_navegador(agent.target_url());
                }
            }
        }

        if let Some(pos) = picked_in(
            &dlg,
            actions,
            &["opt_keymap_scheme", "opt_sr_keymap_scheme"],
        ) {
            let novo = match pos {
                1 => crawrad_actions::KeymapScheme::Classic,
                _ => crawrad_actions::KeymapScheme::Modern,
            };
            if self.esquema_teclado != novo {
                self.esquema_teclado = novo;
                crate::ide_app::teclado::set_active_scheme(novo);
                crawrad_project::workspace::update_preferences(|p| p.keymap_scheme = novo);
                self.redraw_options(cx);
            }
        }

        if let Some(pos) = picked_in(
            &dlg,
            actions,
            &[
                "opt_editor_font_size",
                "opt_cu_font_size",
                "opt_sr_font_size",
            ],
        ) && let Some(&tam) = FONT_SIZES.get(pos)
        {
            let mut pref = crawrad_project::workspace::read_preferences();
            if pref.editor_font_size != tam {
                pref.editor_font_size = tam;
                crawrad_project::workspace::save_preferences(pref);
                self.redraw_options(cx);
            }
        }

        if let Some(pos) = dlg.drop_down(id!(opt_editor_tab_size)).selected(actions)
            && let Some(&tab) = TAB_SIZES.get(pos)
        {
            let mut pref = crawrad_project::workspace::read_preferences();
            if pref.editor_tab_size != tab {
                pref.editor_tab_size = tab;
                crawrad_project::workspace::save_preferences(pref);
                self.redraw_options(cx);
            }
        }

        use options::EditorCheck as Look;
        for check in Look::ALL {
            if let Some(on) = checked_in(&dlg, actions, check.widgets()) {
                crawrad_project::workspace::update_preferences(|p| check.set(p, on));
                match check {
                    Look::InlayHints => self.set_inlay_hints(cx, on),
                    Look::BreadcrumbChevron => self.escrever_breadcrumbs_do_editor(cx),
                    Look::LineNumbers | Look::CurrentLine => self.aplicar_tema_editor(cx),
                }
                self.redraw_options(cx);
            }
        }
        for check in options::SavingCheck::ALL {
            if let Some(on) = checked_in(&dlg, actions, check.widgets()) {
                crawrad_project::workspace::update_preferences(|p| check.set(p, on));
            }
        }
        self.atender_profiles(cx, actions, &dlg);
        if let Some(pos) = dlg.drop_down(id!(opt_auto_save)).selected(actions) {
            self.auto_save = crawrad_project::workspace::AutoSave::ALL[pos.min(3)];
            let chosen = self.auto_save;
            crawrad_project::workspace::update_preferences(|p| p.auto_save = chosen);
        }
        if let Some(on) = dlg.check_box(id!(opt_history_enabled)).changed(actions) {
            self.save_history_pref(cx, |p| p.history_enabled = on);
        }
        for choice in options::NumberChoice::ALL {
            if let Some(pos) = dlg
                .drop_down(&[LiveId::from_str(choice.widget())])
                .selected(actions)
            {
                self.save_history_pref(cx, |p| choice.pick(p, pos));
            }
        }

        if let Some(pos) = dlg.drop_down(id!(opt_cs_preset)).selected(actions) {
            let mut pref = crawrad_project::workspace::read_preferences();
            let novo = crawrad_project::workspace::EditorColorScheme::ALL[pos.min(3)];
            if pref.editor_color_scheme != novo {
                pref.pick_scheme(novo);
                crawrad_project::workspace::save_preferences(pref);
                self.aplicar_tema_editor(cx);
                self.redraw_options(cx);
            }
        }

        if dlg.button(id!(opt_cs_reset_btn)).clicked(actions) {
            crawrad_project::workspace::update_preferences(|p| p.reset_colors());
            self.aplicar_tema_editor(cx);
            self.redraw_options(cx);
        }

        if (0..15).any(|i| {
            dlg.text_input(&scheme(i, "val"))
                .returned(actions)
                .is_some()
        }) {
            self.apply_scheme_fields(cx);
        }

        if let Some(i) = (0..15).find(|&i| dlg.button(&scheme(i, "more")).clicked(actions)) {
            let text = dlg.text_input(&scheme(i, "val")).text();
            let current =
                crawrad_ir::Color::from_hex(text.trim()).unwrap_or(crawrad_ir::Color::rgb(0, 0, 0));
            let title = format!("Color · {}", crawrad_project::workspace::COLOR_FIELDS[i].1);
            let target = crawrad_actions::color_dialog::ColorTarget::SchemeField(i);
            self.open_color(cx, title, current, target);
            return;
        }

        if dlg.button(id!(opt_default_dir_btn)).clicked(actions)
            && let Some(pasta) = rfd::FileDialog::new().pick_folder()
        {
            crawrad_project::workspace::update_preferences(|p| {
                p.default_projects_dir = Some(pasta)
            });
            self.redraw_options(cx);
        }

        if dlg.button(id!(opt_icones)).clicked(actions) {
            self.icones = true;
            self.refresh(cx);
        }
        if dlg.button(id!(opt_save)).clicked(actions) {
            let endp = dlg.text_input(id!(opt_ai_endpoint)).text();
            if !endp.trim().is_empty() {
                self.acp_config.endpoint = endp.trim().to_string();
            }
            self.save_acp_config();

            self.apply_scheme_fields(cx);
            let input_dir = dlg.text_input(id!(opt_default_dir_input)).text();
            let mut pref = crawrad_project::workspace::read_preferences();
            pref.keymap_scheme = self.esquema_teclado;
            if !input_dir.trim().is_empty() {
                pref.default_projects_dir = Some(std::path::PathBuf::from(input_dir.trim()));
            }

            pref.designer_grid = self.designer.grid as u32;
            (pref.designer_snap, pref.designer_show_grid) = (self.designer.snap, self.ver_grade);

            crawrad_project::workspace::save_preferences(pref);
            self.aplicar_tema_editor(cx);
            self.apply_history_settings();

            self.options = false;
            self.options_antes = None;
            self.acp_antes = None;
            self.sync_model_button(cx);
            self.warm_live_agent(cx);
            self.refresh(cx);
        }
        if dlg.button(id!(opt_cancel)).clicked(actions)
            || dlg.button(id!(opt_close)).clicked(actions)
        {
            self.desfazer_options(cx);
        }
    }

    #[note("ide-app-dialogs-options-ideapp-agent-card")]
    fn agent_card(&self) -> acp_registry::AgentCard {
        let msg = self.chat.acp_status_msg.as_deref();
        acp_registry::agent_card(&self.acp_config, msg, self.chat.live.client().is_some())
    }

    #[note("ide-app-dialogs-options-ideapp-apply-agent-click-result")]
    fn apply_agent_click_result(
        &mut self,
        cx: &mut Cx,
        res: crawrad_agent::acp_registry::AgentClickResult,
    ) {
        use crawrad_agent::acp_registry::AgentClickResult as Click;
        match res {
            Click::Connected(msg) => {
                self.chat.acp_registry_feedback = Some(msg);
                self.warm_live_agent(cx);
            }
            Click::NeedsRequirement(msg) => {
                self.chat.acp_registry_feedback = Some(msg);
            }
            Click::InstallStarted(name, rx) => self.watch_agent_install(&name, rx),
            Click::Ignored => {}
        }
    }

    #[note("ide-app-dialogs-options-ideapp-save-acp-config")]
    fn save_acp_config(&mut self) {
        if let Err(e) = self.acp_config.save() {
            self.chat.acp_status_msg = Some(format!("Failed to save ACP config: {e}"));
        }
    }

    #[note("ide-app-dialogs-options-ideapp-atender-profiles")]
    fn atender_profiles(&mut self, cx: &mut Cx, actions: &Actions, dlg: &ViewRef) {
        let wrap = dlg.view(id!(opt_profile_name_wrap));
        let input = dlg.text_input(id!(opt_profile_name));
        let typed = input.text();
        let picked = dlg.drop_down(id!(opt_profile)).selected(actions);
        let save = dlg.button(id!(opt_profile_name_ok)).clicked(actions)
            || input.returned(actions).is_some();
        let names = crawrad_project::profile::all();
        let gestures = [
            (
                picked.and_then(|i| names.get(i).cloned()),
                "layout.apply_profile",
            ),
            (
                dlg.button(id!(opt_profile_default))
                    .clicked(actions)
                    .then(|| crawrad_project::profile::DEFAULT.into()),
                "layout.apply_profile",
            ),
            (
                dlg.button(id!(opt_profile_delete))
                    .clicked(actions)
                    .then(|| crawrad_project::workspace::read_preferences().desktop_profile),
                "layout.delete_profile",
            ),
            (save.then(|| typed.clone()), "layout.save_profile"),
        ];
        for (name, action) in gestures {
            if let Some(name) = name {
                self.execute_with(cx, action, crawrad_actions::Args::text("name", name));
            }
        }
        let saved = save && crawrad_project::profile::check_name(&typed).is_ok();
        if dlg.button(id!(opt_profile_save_as)).clicked(actions) {
            input.set_text(cx, "");
            wrap.set_visible(cx, true);
        } else if saved || dlg.button(id!(opt_profile_name_cancel)).clicked(actions) {
            wrap.set_visible(cx, false);
        }
    }

    #[note("ide-app-dialogs-options-ideapp-mudar-grade")]
    pub(in super::super) fn mudar_grade(&mut self, cx: &mut Cx, nova: f32) {
        self.designer.grid = nova.clamp(OPT_GRADE_MIN, OPT_GRADE_MAX);
        self.reabrir_no_canvas(cx);
        self.redraw_options(cx);
    }

    #[note("ide-app-dialogs-options-ideapp-desfazer-options")]
    pub(in super::super) fn desfazer_options(&mut self, cx: &mut Cx) {
        if let Some((grid, snap, ver, pref_antes)) = self.options_antes.take() {
            self.designer.grid = grid;
            self.designer.snap = snap;
            self.ver_grade = ver;
            self.esquema_teclado = pref_antes.keymap_scheme;
            self.auto_save = pref_antes.auto_save;
            crate::ide_app::teclado::set_active_scheme(pref_antes.keymap_scheme);
            crawrad_project::workspace::save_preferences(pref_antes);
            self.aplicar_tema_editor(cx);
            self.apply_history_settings();
            self.reabrir_no_canvas(cx);
        }
        if let Some(acp_antes) = self.acp_antes.take() {
            self.acp_config = acp_antes;
            self.save_acp_config();
            self.sync_model_button(cx);
        }
        self.options = false;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-options-ideapp-watch-agent-install")]
    pub(crate) fn watch_agent_install(&mut self, name: &str, rx: AgentInstall) {
        self.chat.acp_registry_feedback = Some(format!("Installing {name}…"));
        self.chat.acp_install_rx = Some(rx);
    }

    #[note("ide-app-dialogs-options-ideapp-follow-agent-install")]
    pub(crate) fn follow_agent_install(&mut self, cx: &mut Cx) {
        let got = self
            .chat
            .acp_install_rx
            .as_ref()
            .and_then(|rx| rx.try_recv().ok());
        let Some(res) = got else { return };
        self.chat.acp_registry_feedback =
            Some(res.map(|n| format!("Installed {n}")).unwrap_or_else(|e| e));
        self.chat.acp_install_rx = None;
        self.redraw_options(cx);
    }

    pub(in super::super) fn redraw_options(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        let dlg = self.ui.view(id!(options_dialog.dlg));
        dlg.portal_list(id!(opt_tree)).redraw(cx);
        dlg.portal_list(id!(opt_reg_list)).redraw(cx);
        dlg.portal_list(id!(opt_keymap_list)).redraw(cx);
        self.ui.redraw(cx);
    }

    // --- UNSAVED CHANGES — what prevents closing over work ---
}
