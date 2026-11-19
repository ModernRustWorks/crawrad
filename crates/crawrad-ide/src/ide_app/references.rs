//! Find All References: the glue between the window and what is
//! decided elsewhere — the request in `crawrad_lsp::references`, the panel's
//! groups, marks and rows in `crawrad_host::references`. This file asks, takes
//! the answer, draws a row and turns a click into a place.

use std::path::Path;

use crawrad_actions::{Args, Panel};
use crawrad_host::references::{Hit, Row, same_file};
use crawrad_lsp::code_lens::{self, LensKind};
use crawrad_lsp::protocol::Location;
use makepad_code_editor::code_view::CodeView;
use makepad_widgets::*;

use super::{IdeApp, icon, layout};
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-references-ideapp-find-references")]
    pub(super) fn find_references(&mut self, cx: &mut Cx) {
        let (Some(lsp), Some(from)) = (self.lsp.clone(), self.here(cx)) else {
            self.status = Some("Find All References needs rust-analyzer and a file".into());
            return;
        };
        let line = self.texto_na_vista.lines().nth(from.line).unwrap_or("");
        let refs = &mut self.nav.references;
        refs.symbol = crawrad_host::navigation::word_at(line, from.byte).to_string();
        let ticket = refs.pending.ask(from.file.clone(), self.lsp_versao);
        let caret = (from.line, from.byte);
        self.lsp_hints.references.request(
            lsp,
            ticket,
            &from.file,
            self.lsp_versao,
            &self.texto_na_vista,
            caret,
        );
        self.status = Some("Looking for references\u{2026}".into());
        let panel = crawrad_actions::Args::text("panel", Panel::References.id());
        self.execute_with(cx, "layout.show_panel", panel);
        self.write_references(cx);
    }

    #[note("ide-app-references-ideapp-poll-references")]
    pub(super) fn poll_references(&mut self, cx: &mut Cx) {
        use crawrad_lsp::references::Outcome;
        let Some(ticket) = self.nav.references.pending.ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.references.take(ticket) else {
            return;
        };
        let Some((file, version)) = self.nav.references.pending.answered(ticket) else {
            return;
        };
        let changed = self.abas.active_file().is_some_and(|f| same_file(f, &file))
            && self.lsp_versao != version;
        let places = match answer.outcome {
            _ if changed => {
                self.status =
                    Some("The file changed while looking; Find All References again".into());
                return self.write_references(cx);
            }
            Outcome::NoAnswer(_) => {
                self.status = Some("Language server did not answer".into());
                return self.write_references(cx);
            }
            Outcome::Nothing => Vec::new(),
            Outcome::Found(p) => p,
        };
        let symbol = self.nav.references.symbol.clone();
        self.show_places(cx, &symbol, places);
    }

    #[note("ide-app-references-ideapp-show-places")]
    fn show_places(&mut self, cx: &mut Cx, symbol: &str, places: Vec<Location>) {
        let enc = self.lsp_encoding();
        let mut read = |f: &Path| self.text_of(f);
        let spans = crawrad_lsp::references::spans(places, enc, &mut read).into_iter();
        let hits = spans.map(|s| (s.file, Hit::new(s.line, s.start, s.end, s.text)));
        let hits = hits.collect();
        let roots = crawrad_host::navigation::foreign_roots();
        let refs = &mut self.nav.references;
        refs.show(symbol, hits, &roots, crawrad_codegen::SUFIXO_GERADO);
        self.status = Some(refs.title());
        self.write_references(cx);
    }

    #[note("ide-app-references-ideapp-follow-code-lenses")]
    pub(super) fn follow_code_lenses(&mut self, cx: &mut Cx) {
        let (lsp, file, worker) = (self.lsp.clone(), self.editavel.clone(), &self.lsp_hints);
        let lenses = lsp
            .zip(file)
            .and_then(|(l, f)| worker.code_lenses.poll(l, &f, self.lsp_versao));
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        if let Some(lenses) = lenses {
            cv.set_lenses(cx, &code_lens::rows(&lenses));
        }
        let Some(line) = cv.take_lens_click() else {
            return;
        };
        drop(cv);
        let shown = self.lsp_hints.code_lenses.shown();
        match code_lens::at_line(&shown, line).map(|c| (&c.title, &c.kind)) {
            Some((title, LensKind::ShowLocations(places))) => {
                let panel = crawrad_actions::Args::text("panel", Panel::References.id());
                self.execute_with(cx, "layout.show_panel", panel);
                self.show_places(cx, title, places.clone());
            }
            Some((_, LensKind::Run)) => self.execute_with(cx, "project.run", Args::none()),
            _ => self.status = Some("This code lens does nothing here yet".into()),
        }
    }

    #[note("ide-app-references-ideapp-code-lenses-json")]
    pub(super) fn code_lenses_json(&self) -> serde_json::Value {
        let bands = self
            .with_code_view(CodeView::lens_bands)
            .unwrap_or_default();
        code_lens::json(&self.lsp_hints.code_lenses.shown(), &bands)
    }

    #[note("ide-app-references-ideapp-text-of")]
    pub(super) fn text_of(&self, file: &Path) -> Option<String> {
        if self.abas.active_file().is_some_and(|f| same_file(f, file)) {
            return Some(self.texto_na_vista.clone());
        }
        let open = self.host.open_file_paths();
        match open.iter().find(|p| same_file(p, file)) {
            Some(p) => self.host.buffer(p).map(str::to_owned),
            None => std::fs::read_to_string(file).ok(),
        }
    }

    #[note("ide-app-references-ideapp-write-references")]
    pub(super) fn write_references(&mut self, cx: &mut Cx) {
        let refs = &self.nav.references;
        let title = match refs.pending.ticket() {
            Some(_) => format!("Looking for `{}`\u{2026}", refs.symbol),
            None => refs.title(),
        };
        self.dock()
            .item(layout::tab_of(Panel::References))
            .label(id!(references_header.title))
            .set_text(cx, &title);
        let list = id!(references_body.references_list);
        self.redraw_panel_list(cx, Panel::References, list);
    }

    #[note("ide-app-references-ideapp-write-reference-row")]
    pub(super) fn write_reference_row(cx: &mut Cx, item: &WidgetRef, row: &Row) {
        match row {
            Row::File {
                name,
                folder,
                count,
                mark,
                open,
                ..
            } => Self::write_file_row(cx, item, [name, folder, mark.unwrap_or("")], *count, *open),
            Row::Hit {
                line,
                before,
                word,
                after,
                ..
            } => {
                item.label(id!(face.line_no)).set_text(cx, line);
                item.label(id!(face.before)).set_text(cx, before);
                item.label(id!(face.highlight.word)).set_text(cx, word);
                item.label(id!(face.after)).set_text(cx, after);
            }
        }
    }

    // A file row of the References or the Problems panel: the same
    // `CrawradRefFileRow`, so both panels fold the same way.
    pub(super) fn write_file_row(
        cx: &mut Cx,
        item: &WidgetRef,
        texts: [&str; 3],
        count: usize,
        open: bool,
    ) {
        let chevron = match open {
            true => icon::Chrome::ChevronDown,
            false => icon::Chrome::ChevronRight,
        };
        item.label(id!(face.chevron)).set_text(cx, chevron.glyph());
        let [name, folder, mark] = texts;
        item.label(id!(face.name)).set_text(cx, name);
        item.label(id!(face.folder)).set_text(cx, folder);
        item.label(id!(face.mark)).set_text(cx, mark);
        item.label(id!(face.count)).set_text(cx, &count.to_string());
    }

    #[note("ide-app-references-ideapp-references-list-clicks")]
    pub(super) fn references_list_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let list = self
            .dock()
            .item(layout::tab_of(Panel::References))
            .portal_list(id!(references_body.references_list));
        if !list.any_items_with_actions(actions) {
            return;
        }
        let rows = self.nav.references.rows();
        let picked = list
            .items_with_actions(actions)
            .into_iter()
            .find(|(_, item)| item.button(id!(target)).clicked(actions))
            .and_then(|(i, _)| rows.get(i).cloned());
        match picked {
            Some(Row::File { group, .. }) => self.nav.references.toggle(group),
            Some(Row::Hit { group, hit, .. }) => {
                if let Some(place) = self.nav.references.place(group, hit) {
                    self.go_to_place(cx, place);
                }
            }
            None => return,
        }
        self.write_references(cx);
    }
}
