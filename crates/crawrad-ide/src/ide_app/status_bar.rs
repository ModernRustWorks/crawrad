//! THE STATUS BAR of the shell

use super::*;
use crawrad_note::note;

#[note("ide-app-status-bar-slots")]
pub(super) const SLOTS: &[&[LiveId]] = &[
    id!(bottom_status_bar.cell_problems.lbl_errors),
    id!(bottom_status_bar.cell_problems.lbl_warnings),
    id!(bottom_status_bar.cell_lsp.lbl_lsp),
    id!(bottom_status_bar.cell_lsp_progress.lbl_lsp_progress),
    id!(bottom_status_bar.cell_target.lbl_target),
    id!(bottom_status_bar.cell_project.lbl_project),
    id!(bottom_status_bar.cell_message.opened_file_label),
    id!(bottom_status_bar.cell_selection.status_nodes_label),
    id!(bottom_status_bar.cell_nodes.lbl_nodes),
    id!(bottom_status_bar.cell_rev.lbl_rev),
    id!(bottom_status_bar.cell_zoom.lbl_zoom),
    id!(bottom_status_bar.cell_framework.pill.lbl_framework),
];

#[note("ide-app-status-bar-cell-names")]
const CELL_NAMES: &str =
    "errors warnings lsp lsp_progress target project message selection nodes rev zoom framework";

#[note("ide-app-status-bar-icons")]
pub(super) const ICONS: &[(&[LiveId], icon::Chrome)] = &[
    (
        id!(bottom_status_bar.cell_problems.ic_errors),
        icon::Chrome::Error,
    ),
    (
        id!(bottom_status_bar.cell_problems.ic_warnings),
        icon::Chrome::Warning,
    ),
    (
        id!(bottom_status_bar.cell_target.ic_target),
        icon::Chrome::Packages,
    ),
    (id!(bottom_status_bar.cell_zoom.ic_zoom), icon::Chrome::Zoom),
];

#[note("ide-app-status-bar-nao-medido")]
pub(super) const NAO_MEDIDO: &str = "—";

#[note("ide-app-status-bar-ide-framework")]
pub(super) const IDE_FRAMEWORK: &str = "Makepad";

impl IdeApp {
    #[note("ide-app-status-bar-ideapp-celulas-do-rodape")]
    pub(super) fn celulas_do_rodape(&self) -> Vec<String> {
        let (erros, avisos) = self.conta_dos_problemas();
        let mensagem = match &self.status {
            Some(s) => s.clone(),
            None => documento::file_label(self.host.active_file(), self.falha.as_deref()),
        };
        let target = (self.project_kind.clone()).unwrap_or_else(|| {
            crawrad_project::generator::active_target()
                .alvo_curto()
                .to_string()
        });
        let project_name = self
            .raiz()
            .as_deref()
            .and_then(crawrad_project::active_project::active_project_name);
        let project = crawrad_project::active_project::status_project_text(project_name.as_deref());
        documento::status_bar_cells(documento::StatusBarInputs {
            errors: erros,
            warnings: avisos,
            lsp_name: self.lsp_shown.0.name.clone(),
            lsp_progress: self.lsp_shown.1.clone(),
            target,
            project,
            message: mensagem,
            counters: documento::counters(self.host.doc()),
            zoom_cell: self.celula_do_zoom(),
            framework: IDE_FRAMEWORK,
        })
    }

    #[note("ide-app-status-bar-ideapp-project-kind-now")]
    pub(super) fn project_kind_now(&self) -> (Option<&'static str>, bool) {
        let p = self.projeto_contexto();
        let ui = self
            .membro()
            .as_deref()
            .and_then(crawrad_project::generator::project_target);
        crawrad_project::generator::project_kind_now(
            p.open,
            p.has_forms,
            p.has_binary,
            ui.is_some(),
        )
    }

    #[note("ide-app-status-bar-ideapp-celula-do-zoom")]
    fn celula_do_zoom(&self) -> String {
        crawrad_host::ui_scale::zoom_label(f64::from(self.zoom), self.zoom_da_ide)
    }

    #[note("ide-app-status-bar-ideapp-conta-dos-problemas")]
    fn conta_dos_problemas(&self) -> (String, String) {
        let lsp_totals = self.lsp.as_ref().and_then(|l| l.problem_totals());
        crawrad_project::diagnostics_model::count_problems_or_diagnostics(
            lsp_totals,
            self.diagnosticos.as_deref(),
            NAO_MEDIDO,
        )
    }

    #[note("ide-app-status-bar-ideapp-refresh-footer")]
    pub(super) fn refresh_footer(&mut self, cx: &mut Cx) {
        let celulas = self.celulas_do_rodape();
        if celulas != self.barra_na_tela {
            self.escrever_rodape(cx, &celulas);
            self.barra_na_tela = celulas;
        }
    }

    pub(crate) fn measured_status_bar_lsp_rect(&self) -> Option<[f64; 4]> {
        self.status_bar_rects
            .iter()
            .find(|(k, _)| k == "lsp")
            .filter(|(_, r)| r.size.x > 0.0 && r.size.y > 0.0)
            .map(|(_, r)| xywh(*r))
    }

    #[note("ide-app-status-bar-ideapp-status-bar-json")]
    pub(crate) fn status_bar_json(&self) -> serde_json::Value {
        let pairs = CELL_NAMES.split(' ').zip(&self.barra_na_tela);
        let mut cells: serde_json::Map<_, _> = pairs
            .map(|(k, v)| (k.to_string(), serde_json::json!(v)))
            .collect();
        let phase = self.lsp_shown.0.phase.name();
        cells.insert("lsp_phase".into(), phase.into());
        // ONE key, and it says WHOSE rectangle it is. Writing the same value into
        // `rect` as well would have the caller read "the status bar's rectangle"
        // where the LSP cell's actually is — and whoever navigates only through the
        // MCP has no way to suspect the mistake. Today the LSP is the bar's only
        // clickable cell; when another becomes one it gets its own key, by the same
        // rule: a rectangle is a promise that clicking there does something.
        cells.insert(
            "lsp_rect".into(),
            serde_json::json!(self.measured_status_bar_lsp_rect()),
        );
        cells.into()
    }

    #[note("ide-app-status-bar-ideapp-escrever-rodape")]
    pub(super) fn escrever_rodape(&mut self, cx: &mut Cx, celulas: &[String]) {
        for (slot, texto) in SLOTS.iter().zip(celulas) {
            self.ui.label(slot).set_text(cx, texto);
        }
    }

    #[note("ide-app-status-bar-ideapp-escrever-icones-do-rodape")]
    pub(super) fn escrever_icones_do_rodape(&mut self, cx: &mut Cx) {
        for (slot, papel) in ICONS {
            self.ui.label(slot).set_text(cx, papel.glyph());
        }
    }
}

#[cfg(test)]
#[path = "../test/ide_app/status_bar.rs"]
mod test;
