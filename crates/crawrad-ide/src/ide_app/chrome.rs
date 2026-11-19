//! Chrome of the shell: language, title bar, menus, and panel tabs.
//!
//! What is WRITTEN to the screen and not read from the document — button label, panel
//! title, menu item. Split from `ide_app.rs` on 2026-09-17, when the file
//! exceeded ten thousand lines and the owner asked: *"create a folder and break this up"*.
//!
//! It is part of `impl IdeApp`, not a new type: the shell remains
//! ONE, and the child module accesses its private fields.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-chrome-ideapp-language")]
    pub fn language(&self) -> Language {
        self.language
    }

    // --- THE RAIL AND THE DOCK — one destination at a time ---

    #[note("ide-app-chrome-ideapp-show-window-button")]
    pub(super) fn show_window_button(&mut self, cx: &mut Cx) {
        self.ui
            .button(id!(top_titlebar.btn_max))
            .set_visible(cx, !self.maximizada);
        self.ui
            .button(id!(top_titlebar.btn_restore))
            .set_visible(cx, self.maximizada);
    }

    #[note("ide-app-chrome-ideapp-montar-cromo")]
    pub(super) fn montar_cromo(&mut self, cx: &mut Cx) {
        self.show_window_button(cx);
        self.escrever_icones_do_rodape(cx);
        for aba in InspectorTab::ALL {
            self.tab_button(*aba).set_text(cx, aba.label());
        }
        for m in menu::Menu::ALL {
            self.menu_button(*m).set_text(cx, m.title());
        }
        for c in toolbar::Command::ALL {
            let button = self.bar_button(*c);
            match c.caption() {
                Some(word) => {
                    button.set_text(cx, word);
                    let glyph = self.bar_widget(*c, live_id!(glyph));
                    glyph.set_text(cx, c.icon().glyph());
                }
                None => button.set_text(cx, c.icon().glyph()),
            }
        }
        self.write_debug_bar(cx);
        // The rest of the chrome: the gear at the bottom of the rail and the chevron menu of the Run button.
        let glifos: [(&[LiveId], icon::Chrome); 3] = [
            (id!(rail_settings), icon::Chrome::Settings),
            (id!(top_titlebar.btn_play_menu), icon::Chrome::ChevronDown),
            (
                id!(top_titlebar.btn_preview_menu),
                icon::Chrome::ChevronDown,
            ),
        ];
        for (caminho, papel) in glifos {
            self.ui.widget(caminho).set_text(cx, papel.glyph());
        }
        self.load_arrangement(cx);
        // The IDE's default on open is always to display the EXPLORER selected on the left
        self.show_panel(cx, Panel::Explorer);
        self.write_panel_chrome(cx);
    }

    #[note("ide-app-chrome-ideapp-write-panel-chrome")]
    pub(super) fn write_panel_chrome(&mut self, cx: &mut Cx) {
        for aba in InspectorTab::ALL {
            self.tab_button(*aba).set_text(cx, aba.label());
        }
        self.tab_button(self.aba)
            .as_radio_button()
            .select(cx, &mut Scope::empty());
        self.escrever_x_dos_paineis(cx);
        self.write_center_tabs(cx);
        let (text, design) = (self.host.center().is_text(), self.painel(Panel::Design));
        for (bar, on) in [
            (id!(editor_tabs_bar), text),
            (id!(sep_editor_tabs), text),
            (id!(designer_options_bar), !text),
            (id!(sep_designer_options), !text),
        ] {
            design.view(bar).set_visible(cx, on);
        }
        self.sincronizar_barra_do_designer(cx);
        self.escrever_cromo_dos_pacotes(cx);
        self.write_extension_views(cx);

        // THE AI PANEL CHROME: header and tools footer
        let chat = self.painel(Panel::Chat);
        chat.widget(id!(ai_header.btn_ai_new))
            .set_text(cx, icon::Chrome::New.glyph());
        chat.widget(id!(ai_header.btn_ai_clear))
            .set_text(cx, icon::Chrome::ClearChat.glyph());
        chat.widget(id!(ai_header.btn_ai_history))
            .set_text(cx, icon::Chrome::History.glyph());
        chat.widget(id!(ai_header.btn_ai_settings))
            .set_text(cx, icon::Chrome::Settings.glyph());

        let badge_text = if self.acp_config.provider.contains("Claude") {
            "Claude"
        } else if self.acp_config.provider.contains("Gemini") {
            "Gemini"
        } else if !self.acp_config.provider.is_empty() {
            "Active"
        } else {
            "No agent"
        };
        chat.label(id!(ai_header.ai_mock_badge.ai_mock_text))
            .set_text(cx, badge_text);

        chat.widget(id!(ai_tools_bar.btn_ai_attach))
            .set_text(cx, "+");

        let auto_lbl = if self.acp_config.auto_approve_plans {
            "Auto-approve: ON"
        } else {
            "Auto-approve: OFF"
        };
        chat.widget(id!(ai_messages_view
            .plan_card
            .plan_actions_row
            .btn_plan_auto_approve))
            .set_text(cx, auto_lbl);

        if std::env::var_os("DEMO_PLAN").is_some() {
            chat.view(id!(ai_messages_view.ai_hero_view))
                .set_visible(cx, false);
            chat.view(id!(ai_messages_view.plan_card))
                .set_visible(cx, true);
        }
        self.warm_live_agent(cx);
        self.alvo_na_tela.clear();
        self.sincronizar(cx);
    }

    // --- TOP MENUS — a view over the catalog, never a second list ---

    #[note("ide-app-chrome-ideapp-escrever-linha-com-icone")]
    pub(super) fn escrever_linha_com_icone(
        cx: &mut Cx,
        item: &WidgetRef,
        kind: Option<f64>,
        count: f64,
        nome: &str,
        classe: &str,
        strip: &Texture,
    ) {
        let k = kind.unwrap_or(NO_ICON);
        item.widget(id!(face.icone))
            .apply_over(cx, live! { draw_bg: { kind: (k), count: (count) } });
        item.view(id!(face.icone)).set_texture(0, strip);
        item.label(id!(face.nome)).set_text(cx, nome);
        item.label(id!(face.classe)).set_text(cx, classe);
    }

    #[note("ide-app-chrome-ideapp-bar-button")]
    pub(super) fn bar_button(&self, c: toolbar::Command) -> WidgetRef {
        self.bar_widget(c, live_id!(button))
    }

    // A captioned command (Build, Run) is a `CrawradCaptionButton`: its button and
    // its glyph are children named `part`; every other command IS its button.
    fn bar_widget(&self, c: toolbar::Command, part: LiveId) -> WidgetRef {
        let id = LiveId::from_str(c.widget());
        match c.caption() {
            Some(_) => self.ui.widget(&[live_id!(top_titlebar), id, part]),
            None => self.ui.widget(&[live_id!(top_titlebar), id]),
        }
    }

    #[note("ide-app-chrome-ideapp-menu-button")]
    pub(super) fn menu_button(&self, m: menu::Menu) -> WidgetRef {
        match m {
            menu::Menu::File => self.ui.widget(id!(top_titlebar.menu_file)),
            menu::Menu::Edit => self.ui.widget(id!(top_titlebar.menu_edit)),
            menu::Menu::View => self.ui.widget(id!(top_titlebar.menu_view)),
            menu::Menu::Go => self.ui.widget(id!(top_titlebar.menu_go)),
            menu::Menu::Refactor => self.ui.widget(id!(top_titlebar.menu_refactor)),
            menu::Menu::Project => self.ui.widget(id!(top_titlebar.menu_project)),
            menu::Menu::Run => self.ui.widget(id!(top_titlebar.menu_run)),
            menu::Menu::Component => self.ui.widget(id!(top_titlebar.menu_component)),
            menu::Menu::Tools => self.ui.widget(id!(top_titlebar.menu_tools)),
            menu::Menu::Help => self.ui.widget(id!(top_titlebar.menu_help)),
        }
    }

    #[note("ide-app-chrome-ideapp-menu-rows")]
    pub(super) fn menu_rows(&self) -> Vec<menu::Row> {
        let Some(aberto) = self.open_menu else {
            return Vec::new();
        };
        self.with_menu_context(|ctx| self.rows_of(aberto, ctx))
    }

    #[note("ide-app-chrome-ideapp-with-menu-context")]
    fn with_menu_context(
        &self,
        f: impl FnOnce(&crawrad_actions::Context) -> Vec<menu::Row>,
    ) -> Vec<menu::Row> {
        let Some(registro) = self.designer.registry.clone() else {
            return Vec::new();
        };
        f(&crawrad_actions::Context {
            doc: self.host.doc(),
            registry: &registro,
            clipboard: &self.transferencia,
            args: crawrad_actions::Args::none(),
            project: self.projeto_contexto(),
            author: crawrad_actions::Author::User,
            editor_dirty: self.editor_sujo(),
            text: self.editor_text(),
        })
    }

    fn rows_of(&self, aberto: Aberto, ctx: &crawrad_actions::Context) -> Vec<menu::Row> {
        match aberto {
            Aberto::Barra(m) => {
                let contributed_menus = self
                    .extensions
                    .contributed_menus()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>();
                menu::rows_with_scheme_and_contributions(
                    m,
                    ctx,
                    self.esquema_teclado,
                    Some(&self.capabilities),
                    &contributed_menus,
                )
            }
            Aberto::Painel { painel, .. } => menu::panel_rows(painel, ctx),
            Aberto::SettingsRail => menu::settings_rail_rows(ctx),
            Aberto::EditorMore => menu::editor_more_rows(ctx),
            Aberto::RunMenu => menu::run_menu_rows(
                ctx,
                self.esquema_teclado,
                &self.language_codes,
                self.run_language.as_deref(),
            ),
            Aberto::InspectorPicker => {
                let doc = self.host.doc().map(|d| d.ir());
                let current = self.host.doc().and_then(|d| match d.selection() {
                    [one] => Some(one.clone()),
                    _ => None,
                });
                let choices = crawrad_actions::inspector_model::component_choices(doc);
                menu::inspector_target_rows(&choices, current.as_deref())
            }
            Aberto::LanguageMenu => {
                menu::designer_language_rows(&self.language_codes, self.design_language.as_deref())
            }
            Aberto::LspStatus => {
                menu::lsp_status_rows(self.lsp.as_ref().and_then(|l| l.server_version()), ctx)
            }
            Aberto::EditorContext { .. } => menu::editor_context_rows(ctx, self.esquema_teclado),
            Aberto::CodeActions { teach, .. } => self.code_action_rows(teach),
            Aberto::EventContext { row, .. } => match self.linhas_do_inspector().get(row) {
                Some(inspector::Row::Event(e)) => {
                    self.host.selection().first().map_or_else(Vec::new, |n| {
                        menu::event_context_rows(n, e.event, e.is_override(), ctx)
                    })
                }
                _ => Vec::new(),
            },
            Aberto::ComponentContext { .. } => match (self.host.doc(), self.host.selection()) {
                (Some(d), [id]) => {
                    let name = crawrad_host::document::nome_de(d.ir(), id);
                    let base = d.ir().inherits().filter(|_| d.is_inherited(id));
                    menu::component_context_rows(&name, base, ctx)
                }
                _ => Vec::new(),
            },
            Aberto::ExplorerContext { is_dir, .. } => {
                let p = self
                    .explorer_context_target
                    .as_ref()
                    .map(|(p, _)| p.as_path())
                    .unwrap_or_else(|| std::path::Path::new(""));
                let role = crawrad_project::workspace_edit::role_of_item(p, is_dir);
                menu::explorer_context_rows(is_dir, p, role, ctx)
            }
        }
    }

    #[note("ide-app-chrome-ideapp-menu-rects-json")]
    pub(super) fn menu_rects_json(&self, cx: &Cx) -> serde_json::Value {
        let rect = |w: WidgetRef| {
            let r = w.area().rect(cx);
            xywh(r)
        };
        let bar = menu::Menu::ALL
            .iter()
            .map(|m| (m.title(), rect(self.menu_button(*m))));
        let (popup, sub) = (
            self.ui.widget(id!(menu_popup)),
            self.ui.widget(id!(menu_sub_popup)),
        );
        let bar: std::collections::BTreeMap<_, _> = bar.collect();
        serde_json::json!({ "bar": bar, "popup": rect(popup), "submenu_popup": rect(sub) })
    }

    #[note("ide-app-chrome-ideapp-submenu-rows")]
    pub(super) fn submenu_rows(&self) -> Vec<menu::Row> {
        let (Some(Aberto::Barra(m)), Some((name, ..))) = (self.open_menu, &self.open_submenu)
        else {
            return Vec::new();
        };
        self.with_menu_context(|ctx| menu::submenu_rows(m, name, ctx, self.esquema_teclado))
    }

    #[note("ide-app-chrome-ideapp-show-menu")]
    pub(super) fn show_menu(&mut self, cx: &mut Cx) {
        let painel = self.ui.widget(id!(menu_popup));
        let sub_panel = self.ui.widget(id!(menu_sub_popup));
        sub_panel.set_visible(cx, false);
        let Some(aberto) = self.open_menu else {
            painel.set_visible(cx, false);
            self.ui.redraw(cx);
            return;
        };
        let comandos = self.menu_rows();
        let height = |rows: &[menu::Row]| {
            menu::panel_height(rows, MENU_ROW_HEIGHT, MENU_SEPARATOR_HEIGHT, MENU_PADDING)
        };
        let altura = height(&comandos);
        let (x, y) = match aberto {
            Aberto::Barra(m) => {
                let x = self.menu_button(m).area().rect(cx).pos.x;
                let barra = self.ui.widget(id!(top_titlebar)).area().rect(cx);
                (x, barra.pos.y + barra.size.y)
            }
            // The PANEL menu opens AT THE POINTER.
            Aberto::Painel { em, .. } => (em.x, em.y),
            // The EXPLORER context menu opens AT THE POINTER.
            Aberto::ExplorerContext { em, .. }
            | Aberto::EventContext { em, .. }
            | Aberto::ComponentContext { em }
            | Aberto::EditorContext { em }
            | Aberto::CodeActions { em, .. } => (em.x, em.y),
            // The GEAR menu (Settings/Manage) opens above it in the Activity Rail.
            Aberto::SettingsRail => {
                let rect = self.ui.button(id!(rail_settings)).area().rect(cx);
                let x = rect.pos.x + rect.size.x + 4.0;
                let y = rect.pos.y + rect.size.y - altura;
                (x, y)
            }
            // rust-analyzer's menu opens ABOVE its cell, as a modern editor's.
            Aberto::LspStatus => {
                let rect = self
                    .ui
                    .view(id!(bottom_status_bar.cell_lsp))
                    .area()
                    .rect(cx);
                (rect.pos.x, rect.pos.y - altura - 2.0)
            }
            // The More Actions (...) menu of the editor tabs.
            Aberto::EditorMore => {
                let p_des = self.painel(Panel::Design);
                let rect = p_des
                    .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_more))
                    .area()
                    .rect(cx);
                let x = rect.pos.x + rect.size.x - MENU_WIDTH;
                let y = rect.pos.y + rect.size.y + 2.0;
                (x, y)
            }
            Aberto::RunMenu => {
                let play = self.ui.widget(id!(top_titlebar.btn_play)).area().rect(cx);
                let bar = self.ui.widget(id!(top_titlebar)).area().rect(cx);
                (play.pos.x, bar.pos.y + bar.size.y)
            }
            Aberto::InspectorPicker => {
                let rect = (self.painel(Panel::Inspector))
                    .widget(id!(inspector_picker))
                    .area()
                    .rect(cx);
                (rect.pos.x, rect.pos.y + rect.size.y)
            }
            Aberto::LanguageMenu => {
                let rect = self
                    .ui
                    .widget(id!(top_titlebar.btn_preview_menu))
                    .area()
                    .rect(cx);
                let bar = self.ui.widget(id!(top_titlebar)).area().rect(cx);
                (rect.pos.x, bar.pos.y + bar.size.y)
            }
        };
        let janela = self.ui.area().rect(cx);
        let place = |panel: &WidgetRef, cx: &mut Cx, (x, y): (f64, f64), h: f64| {
            let x = x.min((janela.pos.x + janela.size.x - MENU_WIDTH).max(janela.pos.x));
            let y = y.min((janela.pos.y + janela.size.y - h).max(janela.pos.y));
            panel.apply_over(cx, live! { height: (h), margin: { left: (x), top: (y) } });
            panel.set_visible(cx, true);
            (x, y)
        };
        let (x, y) = place(&painel, cx, (x, y), altura);
        if let Some((_, row, _)) = &self.open_submenu {
            let above = &comandos[..(*row).min(comandos.len())];
            let top =
                y + MENU_PADDING + menu::rows_height(above, MENU_ROW_HEIGHT, MENU_SEPARATOR_HEIGHT);
            place(
                &sub_panel,
                cx,
                (x + MENU_WIDTH, top),
                height(&self.submenu_rows()),
            );
        }

        self.ui.redraw(cx);
    }

    #[note("ide-app-chrome-ideapp-run-menu-row")]
    pub(super) fn run_menu_row(&mut self, cx: &mut Cx, linha: &menu::Row) {
        if !linha.enabled {
            return;
        }
        // A submenu row runs nothing: it opens its submenu beside it.
        if let (None, Some(name)) = (&linha.action, &linha.submenu) {
            let row = self
                .menu_rows()
                .iter()
                .position(|r| r == linha)
                .unwrap_or(0);
            self.open_submenu = Some((name.clone(), row, None));
            self.show_menu(cx);
            return;
        }
        self.open_menu = None;
        self.open_submenu = None;
        self.show_menu(cx);
        match linha.action.as_deref() {
            Some(id) => self.execute_with(cx, id, linha.args.clone()),
            None => self.execute_custom_settings_menu(cx, &linha.label),
        }
    }

    #[note("ide-app-chrome-ideapp-menu-highlight")]
    pub(super) fn menu_highlight(&self) -> Option<usize> {
        let (menu, row) = self.menu_keyboard?;
        (Some(menu) == self.open_menu).then_some(row)
    }

    #[note("ide-app-chrome-ideapp-menu-keys")]
    pub(super) fn menu_keys(&mut self, cx: &mut Cx, event: &Event) -> bool {
        let (Event::KeyDown(ke), Some(open)) = (event, self.open_menu) else {
            return false;
        };
        let sub = self.open_submenu.as_ref().map(|s| s.2);
        let (rows, at) = match sub {
            Some(at) => (self.submenu_rows(), at),
            None => (self.menu_rows(), self.menu_highlight()),
        };
        match ke.key_code {
            KeyCode::ArrowLeft | KeyCode::Escape if sub.is_some() => self.open_submenu = None,
            KeyCode::ArrowRight if sub.is_none() => {
                let Some((row, name)) = at.and_then(|h| Some((h, rows.get(h)?.submenu.clone()?)))
                else {
                    return false;
                };
                self.open_submenu = Some((name, row, None));
                let first = menu::next_live_row(&self.submenu_rows(), None, true);
                if let Some(open) = &mut self.open_submenu {
                    open.2 = first;
                }
            }
            KeyCode::ReturnKey => {
                if let Some(row) = at.and_then(|h| rows.get(h)).cloned() {
                    self.run_menu_row(cx, &row);
                }
                return true;
            }
            KeyCode::ArrowDown | KeyCode::ArrowUp => {
                let next = menu::next_live_row(&rows, at, ke.key_code == KeyCode::ArrowDown);
                match &mut self.open_submenu {
                    Some((_, _, h)) => *h = next,
                    None => self.menu_keyboard = next.map(|row| (open, row)),
                }
            }
            _ => return false,
        }
        self.show_menu(cx);
        true
    }

    #[note("ide-app-chrome-ideapp-toggle-menu")]
    pub(super) fn toggle_menu(&mut self, cx: &mut Cx, menu: Aberto) {
        if self.open_menu == Some(menu) {
            self.close_menu(cx);
        } else {
            if matches!(menu, Aberto::LanguageMenu | Aberto::RunMenu) {
                self.language_codes = self.valid_languages();
                if self.language_codes.is_empty() && self.design_language.is_some() {
                    self.design_language = None;
                    self.design_locale = None;
                }
            }
            self.open_menu = Some(menu);
            self.show_menu(cx);
        }
    }

    pub(super) fn close_menu(&mut self, cx: &mut Cx) -> bool {
        if self.open_menu.is_none() {
            return false;
        }
        self.open_menu = None;
        self.open_submenu = None;
        if self.menu_hint.take().is_some() {
            self.ui.tooltip(id!(ide_tooltip)).hide(cx);
        }
        self.show_menu(cx);
        true
    }

    #[note("ide-app-chrome-ideapp-hint-menu-row")]
    pub(super) fn hint_menu_row(&mut self, cx: &mut Cx, at: DVec2) {
        let over = self.open_menu.and_then(|_| {
            [(id!(menu_sub_popup), true), (id!(menu_popup), false)]
                .into_iter()
                .filter(|(_, sub)| !sub || self.open_submenu.is_some())
                .find_map(|(panel, sub)| {
                    let rect = self.ui.widget(panel).area().rect(cx);
                    let rows = if sub {
                        self.submenu_rows()
                    } else {
                        self.menu_rows()
                    };
                    let y = at.y - rect.pos.y - MENU_PADDING;
                    let i = rect.contains(at).then(|| {
                        menu::row_at(&rows, y, MENU_ROW_HEIGHT, MENU_SEPARATOR_HEIGHT)
                    })??;
                    let row = rows.get(i)?;
                    let why = (!row.enabled && !row.separator)
                        .then(|| self.refusal(row.action.as_deref()?, &row.args))??;
                    Some(((sub, i), why))
                })
        });
        if over.as_ref().map(|(k, _)| k) == self.menu_hint.as_ref().map(|(k, _)| k) {
            return;
        }
        let tip = self.ui.tooltip(id!(ide_tooltip));
        match &over {
            Some((_, why)) => tip.show_with_options(cx, at + dvec2(12.0, 20.0), why),
            None => tip.hide(cx),
        }
        self.menu_hint = over;
    }

    #[note("ide-app-chrome-ideapp-point-in-menu")]
    pub(super) fn point_in_menu(&self, cx: &Cx, p: DVec2) -> bool {
        if self.open_menu.is_none() {
            return false;
        }
        let barra = self.ui.widget(id!(top_titlebar)).area().rect(cx);
        let painel = self.ui.widget(id!(menu_popup)).area().rect(cx);
        let submenu = self.ui.widget(id!(menu_sub_popup)).area().rect(cx);
        let btn_settings = self.ui.button(id!(rail_settings)).area().rect(cx);
        let btn_more = self
            .painel(Panel::Design)
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_more))
            .area()
            .rect(cx);
        let btn_language = self
            .ui
            .button(id!(top_titlebar.btn_preview_menu))
            .area()
            .rect(cx);
        barra.contains(p)
            || painel.contains(p)
            || (self.open_submenu.is_some() && submenu.contains(p))
            || btn_settings.contains(p)
            || btn_more.contains(p)
            || btn_language.contains(p)
    }

    #[note("ide-app-chrome-ideapp-point-in-window-drag")]
    pub(super) fn point_in_window_drag(&self, cx: &Cx, pos: DVec2) -> bool {
        let barra = self.ui.widget(id!(top_titlebar)).area().rect(cx);
        if barra.size.x <= 0.0 || barra.size.y <= 0.0 || !barra.contains(pos) {
            return false;
        }

        // If the popup menu is open, do not interfere with the menu
        if self.open_menu.is_some() {
            return false;
        }

        // Exclude the top menu buttons
        for &m in menu::Menu::ALL {
            let r = self.menu_button(m).area().rect(cx);
            if r.size.x > 0.0 && r.contains(pos) {
                return false;
            }
        }

        // The toolbar's commands, the Run chevron and the window buttons.
        let others = [
            id!(top_titlebar.btn_play_menu),
            id!(top_titlebar.btn_preview_menu),
            id!(top_titlebar.btn_min),
            id!(top_titlebar.btn_max),
            id!(top_titlebar.btn_restore),
            id!(top_titlebar.btn_close),
        ];
        let commands = toolbar::Command::ALL
            .iter()
            .map(|c| self.bar_widget(*c, live_id!(button)));
        let mut buttons = commands.chain(others.iter().map(|b| self.ui.widget(*b)));
        !buttons.any(|w| {
            let r = w.area().rect(cx);
            r.size.x > 0.0 && r.contains(pos)
        })
    }

    #[note("ide-app-chrome-ideapp-painel-do-trilho-em")]
    pub(super) fn painel_do_trilho_em(&self, p: DVec2) -> Option<Panel> {
        self.rail_rects
            .iter()
            .find(|(_, r)| r.contains(p))
            .map(|(painel, _)| *painel)
    }

    #[note("ide-app-chrome-ideapp-painel-no-corpo-em")]
    pub(super) fn painel_no_corpo_em(
        &self,
        cx: &Cx,
        arranjo: &std::collections::HashMap<LiveId, DockItem>,
        ponto: DVec2,
    ) -> Option<Panel> {
        Panel::ALL
            .iter()
            .copied()
            .filter(|p| self.painel_a_vista(arranjo, *p))
            .find(|p| self.painel(*p).area().rect(cx).contains(ponto))
    }

    #[note("ide-app-chrome-ideapp-tab-button")]
    pub(super) fn tab_button(&self, aba: InspectorTab) -> WidgetRef {
        match aba {
            InspectorTab::Properties => self
                .painel(Panel::Inspector)
                .widget(id!(inspector_tabs.tab_props)),
            InspectorTab::Events => self
                .painel(Panel::Inspector)
                .widget(id!(inspector_tabs.tab_events)),
        }
    }

    #[note("ide-app-chrome-ideapp-rail-json")]
    pub(crate) fn rail_json(&self) -> serde_json::Value {
        let panels: Vec<_> = (self.rail_rects.iter())
            .map(|(p, r)| serde_json::json!({ "panel": p.label(), "rect": xywh(*r) }))
            .collect();
        serde_json::json!({ "panels": panels })
    }

    pub(crate) fn set_toolbox_filter(
        &mut self,
        cx: &mut Cx,
        text: &serde_json::Value,
    ) -> ExplorerDrawn {
        self.filtro = text.as_str().unwrap_or_default().to_string();
        let field = self
            .painel(Panel::Components)
            .text_input(id!(toolbox_filter.filter_input));
        field.set_text(cx, &self.filtro);
        self.toolbox_json(cx)
    }

    #[note("ide-app-chrome-ideapp-toolbox-json")]
    pub(crate) fn toolbox_json(&self, cx: &Cx) -> serde_json::Value {
        if !self.no_form_in_project() {
            return palette::to_json(&self.linhas_da_paleta(), &self.filtro);
        }
        let empty = self.painel(Panel::Components).view(id!(no_form));
        let text = |name: &[LiveId]| empty.label(name).text();
        let btn = empty.button(id!(new_form));
        serde_json::json!({
            "empty_state": {
                "title": text(id!(title)),
                "body": text(id!(body)),
                "hint": text(id!(hint)),
                "button": { "text": btn.text(), "rect": xywh(btn.area().rect(cx)) }
            }
        })
    }
}
