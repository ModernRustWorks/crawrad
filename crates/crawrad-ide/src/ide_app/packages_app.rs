//! THE PACKAGE MANAGER: panel rows and cargo polling.
//!
//! It is part of `impl IdeApp`, not a new type: the shell remains
//! ONE, and the child module accesses its private fields.

use super::*;
use crawrad_note::note;

#[note("ide-app-packages-app-package-slices")]
pub(crate) const PACKAGE_SLICES: &[&[LiveId]] = &[
    id!(direita.stat_1),
    id!(direita.stat_2),
    id!(direita.stat_3),
];

impl IdeApp {
    #[note("ide-app-packages-app-ideapp-acoes-dos-pacotes")]
    pub(super) fn acoes_dos_pacotes(&mut self, cx: &mut Cx, actions: &Actions) {
        let painel = self.dock().item(layout::tab_of(Panel::Packages));

        let mut abas = WidgetSet::empty();
        for caminho in [
            id!(packages_tabs.tab_browse),
            id!(packages_tabs.tab_installed),
        ] {
            abas.push(painel.widget(caminho));
        }
        if let Some(i) = abas.as_radio_button_set().selected(cx, actions)
            && let Some(nova) = packages::model::Tab::ALL.get(i).copied()
        {
            self.packages.tab = nova;
            if nova == packages::model::Tab::Installed {
                let membro = self.membro();
                let alvo = self.nome_do_pacote();
                self.packages
                    .reload_installed(membro.as_deref(), alvo.as_deref());
            }
            self.show_package_job_started(cx);
            self.redraw_panel_list(cx, Panel::Packages, id!(packages_body.packages_list));
        }

        let campo = painel.text_input(id!(packages_search.packages_query));
        if let Some(t) = campo.changed(actions) {
            self.packages_query = t;
        }
        if let Some((t, _)) = campo.returned(actions) {
            self.packages_query = t;
            self.packages.tab = packages::model::Tab::Browse;
            self.marcar_aba_dos_pacotes(cx);
            self.packages.start_search(&self.packages_query);
            self.show_package_job_started(cx);
        }

        let lista = painel.portal_list(id!(packages_body.packages_list));
        if !lista.any_items_with_actions(actions) {
            return;
        }
        let linhas = self.linhas_dos_pacotes();
        let mut escolhida = None;
        for (i, item) in lista.items_with_actions(actions) {
            if item.button(id!(direita.acao)).clicked(actions)
                || item.button(id!(acao)).clicked(actions)
            {
                escolhida = linhas.get(i).cloned();
            }
        }
        let membro = self.membro();
        let alvo = self.nome_do_pacote();
        match escolhida {
            Some(packages::Row::Found {
                name,
                installed: false,
                ..
            }) => {
                self.packages
                    .start_addition(&name, membro.as_deref(), alvo.as_deref());
                self.show_package_job_started(cx);
            }
            Some(packages::Row::Found {
                name,
                version,
                installed: true,
                can_update: true,
                ..
            }) => {
                let clean_ver = version.trim_start_matches('v');
                let pkg_spec = format!("{name}@{clean_ver}");
                self.packages
                    .start_addition(&pkg_spec, membro.as_deref(), alvo.as_deref());
                self.show_package_job_started(cx);
            }
            Some(packages::Row::Installed { name, .. }) => {
                self.packages
                    .start_removal(&name, membro.as_deref(), alvo.as_deref());
                self.show_package_job_started(cx);
            }
            _ => {}
        }
    }

    #[note("ide-app-packages-app-ideapp-escrever-linha-do-explorer")]
    pub(super) fn escrever_linha_do_explorer(cx: &mut Cx, item: &WidgetRef, linha: &explorer::Row) {
        let recuo = RECUO_BASE + RECUO_POR_NIVEL * linha.indent() as f64;
        let glifo = |i: Option<explorer::icon::RowIcon>| i.unwrap_or("");
        item.widget(id!(face))
            .apply_over(cx, live! { padding: { left: (recuo) } });
        item.label(id!(face.marca))
            .set_text(cx, glifo(linha.chevron()));
        item.label(id!(face.icone))
            .set_text(cx, glifo(linha.icon()));
        item.label(id!(face.nome)).set_text(cx, linha.text());
        item.label(id!(face.selo)).set_text(cx, linha.badge());
    }

    #[note("ide-app-packages-app-ideapp-escrever-linha-de-pacote")]
    pub(super) fn escrever_linha_de_pacote(cx: &mut Cx, item: &WidgetRef, linha: &packages::Row) {
        match linha {
            packages::Row::Note(t) | packages::Row::Hint(t) | packages::Row::Error(t) => {
                item.label(id!(texto)).set_text(cx, t);
            }
            packages::Row::Found {
                name,
                version,
                description,
                stats,
                installed,
                can_update,
            } => {
                item.label(id!(esquerda.topo.nome)).set_text(cx, name);
                item.label(id!(esquerda.topo.versao)).set_text(cx, version);
                item.label(id!(esquerda.descricao.texto))
                    .set_text(cx, description);
                for (slot, caixa) in PACKAGE_SLICES.iter().enumerate() {
                    let fatia = item.view(caixa);
                    match stats.get(slot) {
                        Some(t) => {
                            fatia.set_visible(cx, true);
                            let mut dentro = caixa.to_vec();
                            dentro.push(live_id!(texto));
                            item.label(&dentro).set_text(cx, t);
                        }
                        None => fatia.set_visible(cx, false),
                    }
                }
                if *installed && !*can_update {
                    item.label(id!(direita.acao))
                        .set_text(cx, packages::model::ALREADY);
                } else if *can_update {
                    item.button(id!(direita.acao)).set_text(cx, "Update");
                }
            }
            packages::Row::Installed { name, req } => {
                item.label(id!(nome)).set_text(cx, name);
                item.label(id!(requisito)).set_text(cx, req);
            }
        }
    }

    #[note("ide-app-packages-app-ideapp-bombear-pacotes")]
    pub(super) fn bombear_pacotes(&mut self, cx: &mut Cx) {
        let ocupado_antes = self.packages.is_busy();
        let membro = self.membro();
        let alvo = self.nome_do_pacote();
        self.packages
            .update_channels(membro.as_deref(), alvo.as_deref());
        if ocupado_antes {
            self.escrever_cromo_dos_pacotes(cx);
            self.redraw_panel_list(cx, Panel::Packages, id!(packages_body.packages_list));
        }
    }

    #[note("ide-app-packages-app-ideapp-show-package-job-started")]
    pub(super) fn show_package_job_started(&mut self, cx: &mut Cx) {
        self.escrever_cromo_dos_pacotes(cx);
        self.redraw_panel_list(cx, Panel::Packages, id!(packages_body.packages_list));
    }

    #[note("ide-app-packages-app-ideapp-redraw-panel-list")]
    pub(super) fn redraw_panel_list(&mut self, cx: &mut Cx, p: Panel, caminho: &[LiveId]) {
        self.dock()
            .item(layout::tab_of(p))
            .widget(caminho)
            .redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-packages-app-ideapp-marcar-aba-dos-pacotes")]
    pub(super) fn marcar_aba_dos_pacotes(&mut self, cx: &mut Cx) {
        let painel = self.dock().item(layout::tab_of(Panel::Packages));
        let caminho = match self.packages.tab {
            packages::model::Tab::Browse => id!(packages_tabs.tab_browse),
            packages::model::Tab::Installed => id!(packages_tabs.tab_installed),
        };
        painel.radio_button(caminho).select(cx, &mut Scope::empty());
    }

    #[note("ide-app-packages-app-ideapp-escrever-cromo-dos-pacotes")]
    pub(super) fn escrever_cromo_dos_pacotes(&mut self, cx: &mut Cx) {
        let painel = self.dock().item(layout::tab_of(Panel::Packages));
        if painel.is_empty() {
            return;
        }
        self.marcar_aba_dos_pacotes(cx);
        let instalados = self.packages.installed_count();
        for (aba, caminho) in [
            (packages::model::Tab::Browse, id!(packages_tabs.tab_browse)),
            (
                packages::model::Tab::Installed,
                id!(packages_tabs.tab_installed),
            ),
        ] {
            painel.widget(caminho).set_text(cx, &aba.label(instalados));
        }
        let alvo = self
            .nome_do_pacote()
            .map(|p| format!("Project: {p}"))
            .unwrap_or_default();
        let estado = packages::status(&self.packages).to_string();
        if self.packages_na_tela != (alvo.clone(), estado.clone()) {
            painel
                .label(id!(packages_header.packages_target))
                .set_text(cx, &alvo);
            painel
                .label(id!(packages_status.texto))
                .set_text(cx, &estado);
            self.packages_na_tela = (alvo, estado);
        }
    }
}
