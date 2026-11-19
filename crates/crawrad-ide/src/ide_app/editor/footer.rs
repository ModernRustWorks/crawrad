//! **THE EDITOR FOOTER** — where the cursor is, and how the file is written.
//!
//! # What the owner asked for
//!
//! He put the two shells side by side and pointed at the band above
//! `Design | Code`: the egui editor wrote `1: 1   Insert   UTF-8 LF` there, and
//! this one had an empty strip. *"validate the editor footer"*.
//!
//! # Why it is not the same three cells
//!
//! Two of them are, and the third tells the truth instead of copying it.
//!
//! | cell | egui | here |
//! |---|---|---|
//! | position | `1: 1` | the same |
//! | typing mode | `Insert` | **`Read only`** |
//! | encoding | `UTF-8 LF` | the same |
//!
//! The middle one is the difference, and it is not a gap: the Makepad view is
//! a `CodeView`, which is the framework's editor with `read_only: true`
//! hard-wired (`makepad-code-editor-1.0.0/src/code_view.rs:22`). Writing
//! `Insert` under a view that refuses every key would be the bar answering a
//! question with the other shell's answer.
//!
//! The egui footer already had that word for a reason of its own — it writes
//! `Read only` over the `.designer.rs`, because *"it accepts selection and not
//! typing, and without saying why"*. Here the reason covers the whole view.
//!
//! # Where the numbers come from
//!
//! The position is read off the `CodeView`'s own session — `session.selections()`
//! gives a `Selection` whose `cursor.position` is a `Position { line_index,
//! byte_index }` (`makepad-code-editor-1.0.0/src/text.rs:208-211`). It is the
//! editor's truth, not a count this shell keeps in parallel.
//!
//! **And `byte_index` is not a column.** The two only agree in pure ASCII, and
//! the first line of every generated unit in this project is a Portuguese
//! comment. [`crawrad_host::word_motion::column_of_byte`] converts, and it has a gate.

use super::*;
use crawrad_runtime_makepad::breadcrumb::CrawradBreadcrumbWidgetRefExt;

/// Where each cell of the footer is written, in the order
/// [`IdeApp::celulas_do_editor`] fills them.
pub(super) const SLOTS: &[&[LiveId]] = &[
    id!(editor_status_bar.lbl_caret),
    id!(editor_status_bar.lbl_mode),
    id!(editor_status_bar.lbl_modified),
    id!(editor_status_bar.lbl_spaces),
    id!(editor_status_bar.lbl_encoding),
    id!(editor_status_bar.lbl_language),
];

#[cfg(test)]
pub(super) use crawrad_host::editor_footer::SOMENTE_LEITURA;
pub(super) use crawrad_host::editor_footer::{editor_footer_cells, language_label};

impl IdeApp {
    /// The cells of the editor footer, in the order of [`SLOTS`].
    pub(super) fn celulas_do_editor(&self, cx: &Cx) -> Vec<String> {
        let (linha, byte) = self.cursor_do_code_view(cx);
        let texto = self.texto_na_vista.lines().nth(linha).unwrap_or_default();
        let modified = self
            .abas
            .active_file()
            .is_some_and(|f| self.host.buffer(f).is_some());
        editor_footer_cells(
            linha,
            byte,
            texto,
            self.editavel.is_some(),
            modified,
            self.encoding_na_vista.label(),
            language_label(self.abas.active_file()),
        )
    }

    /// Where the `CodeView` says its cursor is — line and byte, both from zero.
    ///
    /// `(0, 0)` while there is no session: the view builds one lazily, on the
    /// first draw (`code_view.rs:40-52`), so the first frame after opening a
    /// file legitimately has none.
    pub(super) fn cursor_do_code_view(&self, _cx: &Cx) -> (usize, usize) {
        self.with_code_view(|cv| {
            let s = cv.session.as_ref()?.selections().first().copied()?;
            Some((s.cursor.position.line_index, s.cursor.position.byte_index))
        })
        .flatten()
        .unwrap_or((0, 0))
    }

    /// Writes the footer, and shows or hides it with the text view.
    ///
    /// Hidden over the canvas, and that is the rule this bar follows: a
    /// position and an encoding are facts about a FILE, and there is no file
    /// in front when the designer is. The egui shell draws the strip only
    /// inside its editor for the same reason.
    pub(super) fn escrever_rodape_do_editor(&mut self, cx: &mut Cx) {
        let mostrar = self.host.center().is_text();
        let panel = self.painel(Panel::Design);
        panel.view(id!(editor_status_bar)).set_visible(cx, mostrar);
        panel
            .view(id!(editor_breadcrumbs_bar))
            .set_visible(cx, mostrar);
        panel
            .view(id!(center_tabs_spacer))
            .set_visible(cx, !mostrar);
        if !mostrar {
            return;
        }
        self.escrever_breadcrumbs_do_editor(cx);
        let rel_text = self.build.status_text().unwrap_or_default();
        let panel = self.painel(Panel::Design);
        panel
            .label(id!(editor_status_bar.lbl_release))
            .set_text(cx, &rel_text);
        panel
            .widget(id!(editor_status_bar.lbl_release))
            .set_visible(cx, !rel_text.is_empty());
        let celulas = self.celulas_do_editor(cx);
        if celulas == self.rodape_do_editor_na_tela {
            return;
        }
        for (slot, texto) in SLOTS.iter().zip(&celulas) {
            let panel = self.painel(Panel::Design);
            panel.label(slot).set_text(cx, texto);
            // An empty cell (Modified on a clean file) leaves no gap behind.
            panel.widget(slot).set_visible(cx, !texto.is_empty());
        }
        self.rodape_do_editor_na_tela = celulas;
    }

    /// Editor breadcrumb: path segments and symbol under caret.
    /// The items as drawn — what `editor.get_active_file` reports.
    pub(crate) fn editor_breadcrumb_items(&self) -> Vec<String> {
        let crumb = self.painel(Panel::Design).widget(id!(editor_breadcrumb));
        crumb.as_crawrad_breadcrumb().items()
    }

    pub(super) fn escrever_breadcrumbs_do_editor(&mut self, cx: &mut Cx) {
        let Some(file) = self.abas.active_file().map(|p| p.to_path_buf()) else {
            return;
        };
        let mut items = crawrad_host::breadcrumb::path_items(&file, self.raiz().as_deref());
        let (line, _) = self.cursor_do_code_view(cx);
        items.extend(crawrad_host::breadcrumb::symbol_item(
            &self.texto_na_vista,
            line,
        ));
        let crumb = self
            .painel(Panel::Design)
            .widget(id!(editor_breadcrumb))
            .as_crawrad_breadcrumb();
        let chevron = crawrad_project::workspace::read_preferences().editor_breadcrumb_chevron;
        crumb.set_style(cx, if chevron { "bsChevron" } else { "bsText" });
        crumb.set_separator(cx, "›");
        crumb.set_items(cx, &items);
        crumb.set_item_index(cx, items.len() as i64 - 1);
    }
}

pub(super) use crawrad_project::encoding::file_encoding;

#[cfg(test)]
#[path = "../../test/ide_app/editor_footer.rs"]
mod test;
