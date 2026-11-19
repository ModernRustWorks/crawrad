//! The Translations glue: model in `crawrad_designer::translations::Workbook`, files in
//! `crawrad_project::locales`, drawing in `TranslationsView`; here only the shell wiring.

use std::path::PathBuf;

use crawrad_actions::Panel;
use crawrad_designer::translations::{CellState, Workbook};
use crawrad_project::locales;
use crawrad_runtime_makepad::translations_view::{
    CellView, GridGroup, GridRow, LangChip, TranslationsAction, TranslationsViewRef,
    TranslationsViewWidgetRefExt,
};
use makepad_widgets::*;

use super::{Center, IdeApp};

impl IdeApp {
    fn translations_panel(&self) -> TranslationsViewRef {
        self.painel(Panel::Design)
            .translations_view(id!(designer_row.translations_container.translations_panel))
    }

    /// The project's root and its forms' texts with every locale on disk.
    fn workbook(&self) -> (Option<PathBuf>, Workbook) {
        let root = self.desktop_root.clone().or_else(|| self.raiz());
        let reg = self.designer.registry.clone().unwrap_or_default();
        let files = root.as_deref().map(locales::read_all).unwrap_or_default();
        let book = Workbook::new(&self.host.project_forms(), &reg, files);
        (root, book)
    }

    /// Rebuilds the grid from the forms and the locale files.
    pub(super) fn paint_translations(&mut self, cx: &mut Cx) {
        self.translations_rev = self.host.doc().map_or(0, |d| d.revision());
        let (_, book) = self.workbook();
        let chips: Vec<LangChip> = book
            .chips()
            .into_iter()
            .map(|(name, done, total)| LangChip { name, done, total })
            .collect();
        let cell = |c: CellState| match c {
            CellState::Done(s) => CellView::Done(s),
            CellState::Stale(s) => CellView::Stale(s),
            CellState::Missing => CellView::Missing,
        };
        let row = |r: crawrad_designer::translations::TableRow| GridRow {
            key: r.key,
            label: r.label,
            source: r.source,
            cells: r.cells.into_iter().map(cell).collect(),
        };
        let groups: Vec<GridGroup> = book
            .table(self.translations_missing_only)
            .into_iter()
            .map(|g| GridGroup {
                form: g.form,
                rows: g.rows.into_iter().map(row).collect(),
            })
            .collect();
        let only = self.translations_missing_only;
        let files = book.files_line();
        self.translations_panel()
            .set_view(cx, &chips, &groups, only, book.missing(), &files);
    }

    /// Dispatches user gestures from the Translations tab.
    pub(super) fn translations_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.translations_panel();
        match panel.action(cx, actions) {
            Some(TranslationsAction::ToggleMissingOnly) => {
                self.translations_missing_only = !self.translations_missing_only;
                self.paint_translations(cx);
            }
            Some(TranslationsAction::AddLanguage) => panel.toggle_add_card(cx),
            Some(TranslationsAction::Commit { key, lang, text }) => {
                let code = self.workbook().1.code(lang).map(str::to_string);
                if let Some(code) = code {
                    let _ = self.strings_set(cx, &key, &code, &text);
                }
            }
            _ => {}
        }
        if panel.add_card_add_clicked(actions) {
            self.add_language_from_card(cx);
        }
        if panel.add_card_cancel_clicked(actions) {
            panel.set_add_card_visible(cx, false);
        }
    }

    fn add_language_from_card(&mut self, cx: &mut Cx) {
        let panel = self.translations_panel();
        let code = panel.add_card_code();
        let Some(root) = self.desktop_root.clone().or_else(|| self.raiz()) else {
            return;
        };
        if !crawrad_designer::translations::valid_code(&code) {
            panel.set_add_error(cx, "Not a language code (e.g. pt-BR, de, fr)");
        } else if locales::list_locales(&root).contains(&code) {
            panel.set_add_error(cx, "That language is already there");
        } else if let Err(e) = locales::write_locale(&root, &code, &[]) {
            panel.set_add_error(cx, &e);
        } else {
            panel.set_add_card_visible(cx, false);
            self.language_codes = self.valid_languages();
            self.paint_translations(cx);
        }
    }

    /// Follows document edits while the Translations tab is in front.
    pub(super) fn check_translations(&mut self, cx: &mut Cx) {
        let rev = self.host.doc().map_or(0, |d| d.revision());
        if self.host.center() == Center::Translations && rev != self.translations_rev {
            self.paint_translations(cx);
        }
    }

    /// The `strings.list` answer.
    pub(crate) fn strings_list_json(&self, args: &serde_json::Value) -> serde_json::Value {
        let get = |k: &str| args.get(k).and_then(serde_json::Value::as_str);
        self.workbook().1.report(get("form"), get("lang"))
    }

    /// Sets one translation (from the grid or `strings.set`) and repaints.
    pub(crate) fn strings_set(
        &mut self,
        cx: &mut Cx,
        key: &str,
        lang: &str,
        text: &str,
    ) -> Result<String, String> {
        if !crawrad_designer::translations::valid_code(lang) {
            return Err(format!("Invalid language code '{lang}'"));
        }
        let (root, book) = self.workbook();
        let root = root.ok_or("No project open")?;
        let from = book
            .source_text(key)
            .ok_or_else(|| format!("Unknown key '{key}'"))?;
        locales::set_entry(&root, lang, key, text, from)?;
        self.paint_translations(cx);
        Ok(format!("Translation for '{key}' in '{lang}' set."))
    }
}
