//! DOCK ARRANGEMENT: which panel is where, and how it returns to its place.
//!
//! Show, hide, move, stack, and restore — plus saving the arrangement
//! to disk. It is a self-contained topic: nothing here knows about documents, canvas, or
//! code.
//!
//! It is part of `impl IdeApp`, not a new type: the shell remains
//! ONE, and the child module accesses its private fields.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-arrangement-ideapp-load-state")]
    pub(super) fn load_state(
        &mut self,
        cx: &mut Cx,
        arranjo: std::collections::HashMap<LiveId, DockItem>,
    ) {
        if let Some(mut d) = self.dock().borrow_mut() {
            d.load_state(cx, arranjo);
        }
        self.remake_editor = self.host.center().is_text();
        self.repovoar_paineis(cx);
    }

    #[note("ide-app-arrangement-ideapp-dock")]
    pub(super) fn dock(&self) -> DockRef {
        self.ui.dock(layout::DOCK)
    }

    #[note("ide-app-arrangement-ideapp-painel")]
    pub(super) fn painel(&self, p: Panel) -> WidgetRef {
        self.dock().item(layout::tab_of(p))
    }

    #[note("ide-app-arrangement-ideapp-painel-a-vista")]
    pub(super) fn painel_a_vista(
        &self,
        arranjo: &std::collections::HashMap<LiveId, DockItem>,
        p: Panel,
    ) -> bool {
        let aba = layout::tab_of(p);
        arranjo.values().any(|item| match item {
            DockItem::Tabs { tabs, selected, .. } => tabs.get(*selected) == Some(&aba),
            _ => false,
        })
    }

    #[note("ide-app-arrangement-ideapp-arranjo")]
    pub(super) fn arranjo(&self) -> std::collections::HashMap<LiveId, DockItem> {
        self.dock().clone_state().unwrap_or_default()
    }

    #[note("ide-app-arrangement-ideapp-aplicar-arranjo")]
    pub(super) fn aplicar_arranjo(&mut self, cx: &mut Cx, comando: crawrad_actions::LayoutCommand) {
        self.aplicar_arranjo_e_depois(cx, comando);
        self.write_panel_chrome(cx);
    }

    #[note("ide-app-arrangement-ideapp-aplicar-arranjo-e-depois")]
    pub(super) fn aplicar_arranjo_e_depois(
        &mut self,
        cx: &mut Cx,
        comando: crawrad_actions::LayoutCommand,
    ) {
        use crawrad_actions::LayoutCommand as Cmd;
        match comando {
            Cmd::Show(p) => self.show_panel(cx, p),
            Cmd::Hide(p) => self.esconder_painel(cx, p),
            Cmd::Toggle(p) => {
                if self.painel_a_vista(&self.arranjo(), p) {
                    self.esconder_painel(cx, p);
                } else {
                    self.show_panel(cx, p);
                }
            }
            Cmd::Move { panel, side } => self.mover_painel(cx, panel, side),
            Cmd::Stack { panel, onto } => self.empilhar_painel(cx, panel, onto),
            Cmd::Float(p) => {
                self.status = Some(format!(
                    "floating `{}` is not wired yet — the dock of the framework has no Float",
                    p.id()
                ));
            }
            Cmd::Reset => {
                self.restaurar_arranjo(cx);
                self.show_center(cx, self.host.center());
                self.status = Some("panel layout reset".into());
            }
        }
        self.restore_root_if_lost(cx);
        self.escrever_rotulos_das_abas(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-restore-root-if-lost")]
    pub(super) fn restore_root_if_lost(&mut self, cx: &mut Cx) -> bool {
        let Some(arranjo) = layout::with_root_restored(&self.arranjo()) else {
            return false;
        };
        self.load_state(cx, arranjo);
        true
    }

    #[note("ide-app-arrangement-ideapp-show-panel")]
    pub(super) fn show_panel(&mut self, cx: &mut Cx, p: Panel) {
        let aba = layout::tab_of(p);
        let doca = self.dock();
        if self.arranjo().contains_key(&aba) {
            doca.select_tab(cx, aba);
        } else {
            let grupo = Self::grupo_padrao(p);
            let mut criado = false;
            if self.arranjo().contains_key(&grupo)
                && doca
                    .create_and_select_tab(
                        cx,
                        grupo,
                        aba,
                        layout::template_of(p),
                        p.label().to_string(),
                        layout::TAB_TEMPLATE,
                        None,
                    )
                    .is_some()
            {
                criado = true;
            }
            if !criado {
                self.reconstruir_painel_no_arranjo(cx, p);
            }
            if p == Panel::Design {
                self.reabrir_no_canvas(cx);
                self.sincronizar(cx);
            }
            // Problems is born under the editor, not under the whole window.
            if matches!(p, Panel::Problems | Panel::TestOutput | Panel::FormCheck) {
                let editor = self
                    .dock()
                    .item(layout::tab_of(Panel::Design))
                    .area()
                    .rect(cx);
                self.dock_at_edge(cx, p, editor, crawrad_actions::DockSide::Bottom);
            }
            self.paint_notes(cx);
        }
        if p == Panel::Packages {
            let membro = self.membro();
            let alvo = self.nome_do_pacote();
            self.packages.open(membro.as_deref(), alvo.as_deref());
            self.show_package_job_started(cx);
        }
        if p == Panel::Problems {
            self.write_problems(cx);
        }
        if p == Panel::Tests {
            self.open_tests(cx);
        }
        if p == Panel::FormCheck {
            self.paint_form_check(cx);
        }
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-reconstruir-painel-no-arranjo")]
    pub(super) fn reconstruir_painel_no_arranjo(&mut self, cx: &mut Cx, p: Panel) {
        let mut arr = self.arranjo();
        let grupo = Self::grupo_padrao(p);
        let (root, rest) = (live_id!(root), live_id!(dock_rest));
        let chat = SplitterAlign::FromB(layout::CHAT_WIDTH);
        if grupo == layout::RIGHT_GROUP {
            let mut substituiu = false;
            for item in arr.values_mut() {
                if let DockItem::Splitter { a, b, .. } = item {
                    for lado in [a, b] {
                        if *lado == layout::CENTER_GROUP {
                            *lado = rest;
                            substituiu = true;
                        }
                    }
                }
            }
            if substituiu {
                arr.insert(
                    rest,
                    layout::side_by_side(layout::CENTER_GROUP, grupo, chat),
                );
            } else if let Some(item) = arr.remove(&root) {
                arr.insert(layout::CENTER_GROUP, item);
                arr.insert(
                    root,
                    layout::side_by_side(layout::CENTER_GROUP, grupo, chat),
                );
            }
            arr.insert(grupo, layout::lone_group(p));
        } else if grupo == layout::LEFT_GROUP {
            let antigo = live_id!(antigo_root);
            let dock = SplitterAlign::FromA(layout::DOCK_WIDTH);
            let Some(item) = arr.remove(&root) else {
                return;
            };
            arr.insert(antigo, item);
            arr.insert(root, layout::side_by_side(grupo, antigo, dock));
            arr.insert(grupo, layout::lone_group(p));
        } else {
            let aba = layout::tab_of(p);
            let grupo = arr.values_mut().find_map(|item| match item {
                DockItem::Tabs { tabs, selected, .. } => Some((tabs, selected)),
                _ => None,
            });
            let Some((tabs, selected)) = grupo else {
                return;
            };
            if !tabs.contains(&aba) {
                tabs.push(aba);
            }
            *selected = tabs.len().saturating_sub(1);
        }
        arr.insert(layout::tab_of(p), layout::tab_item(p));
        self.load_state(cx, arr);
    }

    #[note("ide-app-arrangement-ideapp-esconder-painel")]
    pub(super) fn esconder_painel(&mut self, cx: &mut Cx, p: Panel) {
        self.dock().close_tab(cx, layout::tab_of(p));
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-mover-painel")]
    pub(super) fn mover_painel(&mut self, cx: &mut Cx, p: Panel, lado: crawrad_actions::DockSide) {
        let r = self.dock().area().rect(cx);
        if r.size.x < 1.0 || r.size.y < 1.0 {
            self.status = Some("the dock has not been drawn yet".into());
            return;
        }
        self.dock_at_edge(cx, p, r, lado);
    }

    fn dock_at_edge(&mut self, cx: &mut Cx, p: Panel, r: Rect, side: crawrad_actions::DockSide) {
        use crawrad_actions::DockSide;
        if r.size.x < 1.0 || r.size.y < 1.0 {
            return;
        }
        let target = match side {
            DockSide::Left => dvec2(r.pos.x + 2.0, r.pos.y + r.size.y * 0.5),
            DockSide::Right => dvec2(r.pos.x + r.size.x - 2.0, r.pos.y + r.size.y * 0.5),
            DockSide::Bottom => dvec2(r.pos.x + r.size.x * 0.5, r.pos.y + r.size.y - 2.0),
        };
        self.show_panel(cx, p);
        self.dock().drop_move(cx, target, layout::tab_of(p));
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-empilhar-painel")]
    pub(super) fn empilhar_painel(&mut self, cx: &mut Cx, p: Panel, sobre: Panel) {
        self.show_panel(cx, sobre);
        let destino = self.dock().item(layout::tab_of(sobre)).area().rect(cx);
        if destino.size.x < 1.0 || destino.size.y < 1.0 {
            self.status = Some(format!("`{}` is not on screen to stack onto", sobre.id()));
            return;
        }
        self.show_panel(cx, p);
        self.dock()
            .drop_move(cx, destino.center(), layout::tab_of(p));
        self.dock().select_tab(cx, layout::tab_of(p));
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-grupo-padrao")]
    pub(super) fn grupo_padrao(p: Panel) -> LiveId {
        match p {
            Panel::Design | Panel::Terminal => layout::CENTER_GROUP,
            Panel::Chat => layout::RIGHT_GROUP,
            _ => layout::LEFT_GROUP,
        }
    }

    #[note("ide-app-arrangement-ideapp-restaurar-arranjo")]
    pub(super) fn restaurar_arranjo(&mut self, cx: &mut Cx) {
        self.load_state(cx, Self::arranjo_padrao());
        self.ui.redraw(cx);
    }

    #[note("ide-app-arrangement-ideapp-repovoar-paineis")]
    pub(super) fn repovoar_paineis(&mut self, cx: &mut Cx) {
        self.escrever_rotulos_das_abas(cx);
        self.write_panel_chrome(cx);
        self.paint_notes(cx);
        self.paint_form_check(cx);
        self.write_problems(cx);
        self.reabrir_no_canvas(cx);
        self.sincronizar(cx);
    }

    #[note("ide-app-arrangement-ideapp-remake-editor-view")]
    pub(super) fn remake_editor_view(&mut self, cx: &mut Cx) {
        if self.remake_editor && self.with_code_view(|_| ()).is_some() {
            self.remake_editor = false;
            self.show_center(cx, self.host.center());
        }
    }

    #[note("ide-app-arrangement-ideapp-warm-up-list-rows")]
    pub(super) fn warm_up_list_rows(&mut self, cx: &mut Cx) {
        let components = self.painel(Panel::Components);
        for (path, templates) in Self::WARM_UP_LISTS {
            let list = components.portal_list(path);
            for (i, template) in templates.iter().enumerate() {
                let _ = list.item(cx, usize::MAX - i, LiveId::from_str(template));
            }
        }
    }

    #[note("ide-app-arrangement-ideapp-warm-up-lists")]
    pub(crate) const WARM_UP_LISTS: &'static [(&'static [LiveId], &'static [&'static str])] = &[
        (
            id!(toolbox_palette.toolbox_list),
            &["Secao", "SecaoPrimeira", "Item", "ItemOff", "Vazia"],
        ),
        (id!(structure_tree.structure_list), &["Linha", "Raiz"]),
    ];

    #[note("ide-app-arrangement-ideapp-arranjo-padrao")]
    pub(super) fn arranjo_padrao() -> std::collections::HashMap<LiveId, DockItem> {
        let (dock, chat) = (layout::DOCK_WIDTH, layout::CHAT_WIDTH);
        let rest = live_id!(dock_rest);
        std::collections::HashMap::from([
            (
                live_id!(root),
                layout::side_by_side(layout::LEFT_GROUP, rest, SplitterAlign::FromA(dock)),
            ),
            (
                rest,
                layout::side_by_side(
                    layout::CENTER_GROUP,
                    layout::RIGHT_GROUP,
                    SplitterAlign::FromB(chat),
                ),
            ),
            (layout::LEFT_GROUP, layout::lone_group(Panel::Components)),
            (layout::CENTER_GROUP, layout::lone_group(Panel::Design)),
            (layout::RIGHT_GROUP, layout::lone_group(Panel::Chat)),
            (
                layout::tab_of(Panel::Components),
                layout::tab_item(Panel::Components),
            ),
            (
                layout::tab_of(Panel::Design),
                layout::tab_item(Panel::Design),
            ),
            (layout::tab_of(Panel::Chat), layout::tab_item(Panel::Chat)),
        ])
    }

    #[note("ide-app-arrangement-ideapp-fechaveis")]
    pub(super) const FECHAVEIS: &'static [Panel] = &[
        Panel::Explorer,
        Panel::Components,
        Panel::Inspector,
        Panel::Packages,
        Panel::Chat,
        Panel::Debug,
        Panel::References,
        Panel::Problems,
        Panel::Extensions,
        Panel::Notes,
    ];

    #[note("ide-app-arrangement-ideapp-escrever-x-dos-paineis")]
    pub(super) fn escrever_x_dos_paineis(&mut self, cx: &mut Cx) {
        for p in Self::FECHAVEIS {
            self.dock()
                .item(layout::tab_of(*p))
                .button(id!(btn_panel_close))
                .set_text(cx, icon::Chrome::Close.glyph());
        }
        self.dock()
            .item(layout::tab_of(Panel::Components))
            .button(id!(add_item))
            .set_text(cx, icon::Chrome::New.glyph());
        // The crab before the menus: the same icon a new project starts with.
        if let Some(png) = crawrad_project::project_options::preview_png(
            crawrad_project::project_options::DEFAULT_ICON,
            48,
        ) {
            let _ = self
                .ui
                .image(id!(top_titlebar.app_icon))
                .load_png_from_data(cx, &png);
        }
        // The same crab drawing as the Welcome page, on the chat's Rust Teacher card.
        // The chat panel is rebuilt as the dock settles, so a texture can be lost.
        let crab = self
            .painel(Panel::Chat)
            .image(id!(teach_content.teach_crab));
        if !crab.is_empty() && !crab.has_texture() {
            let _ = crab.load_png_from_data(cx, crawrad_runtime_makepad::welcome::WELCOME_CRAB_PNG);
        }
        self.write_debug_bar(cx);
        self.escrever_titulos_dos_paineis(cx);
    }

    #[note("ide-app-arrangement-ideapp-escrever-titulos-dos-paineis")]
    pub(super) fn escrever_titulos_dos_paineis(&mut self, cx: &mut Cx) {
        for p in Panel::ALL {
            if matches!(p, Panel::Components | Panel::Chat) {
                continue;
            }
            self.dock()
                .item(layout::tab_of(*p))
                .label(id!(title))
                .set_text(cx, p.label());
        }
    }

    #[note("ide-app-arrangement-ideapp-escrever-rotulos-das-abas")]
    pub(super) fn escrever_rotulos_das_abas(&mut self, cx: &mut Cx) {
        let arranjo = self.arranjo();
        for p in Panel::ALL {
            let aba = layout::tab_of(*p);
            if arranjo.contains_key(&aba) {
                self.dock().set_tab_title(cx, aba, p.label().to_string());
            }
        }
    }

    #[note("ide-app-arrangement-ideapp-gravar-arranjo-se-mudou")]
    pub(super) fn gravar_arranjo_se_mudou(&mut self, cx: &mut Cx) {
        if !self.dock().check_and_clear_need_save() {
            return;
        }
        // A drag goes around `aplicar_arranjo`, so the net is here too.
        self.restore_root_if_lost(cx);
        let Some(caminho) = layout::file_path() else {
            return;
        };
        let arranjo = self.arranjo();
        if arranjo.is_empty() {
            return;
        }
        let _ = std::fs::write(caminho, layout::encode(&arranjo));
    }

    #[note("ide-app-arrangement-ideapp-load-arrangement")]
    pub(super) fn load_arrangement(&mut self, cx: &mut Cx) {
        let Some(caminho) = layout::file_path() else {
            return;
        };
        let Ok(texto) = std::fs::read_to_string(&caminho) else {
            return;
        };
        match layout::decode(&texto) {
            Some(arranjo) => {
                let tinha_pacotes = arranjo.contains_key(&layout::tab_of(Panel::Packages));
                self.load_state(cx, arranjo);
                if tinha_pacotes {
                    let membro = self.membro();
                    let alvo = self.nome_do_pacote();
                    self.packages.open(membro.as_deref(), alvo.as_deref());
                    self.show_package_job_started(cx);
                }
            }
            None => self.status = Some(layout::DISCARDED.into()),
        }
    }
}
