//! The Problems panel glue: draws the rows, the chips and the stop reason.

use crawrad_actions::Panel;
use crawrad_host::problems::{Filter, Row};
use makepad_code_editor::CodeDocument;
use makepad_code_editor::decoration::{Decoration, DecorationType::Error};
use makepad_code_editor::text::Position;
use makepad_widgets::*;
use std::time::Duration;

use super::{IdeApp, icon};
use crawrad_note::note;

const CHIPS: [(&[LiveId], Filter); 4] = [
    (id!(problems_header.chip_errors), Filter::Errors),
    (id!(problems_header.chip_panics), Filter::Panics),
    (id!(problems_header.chip_warnings), Filter::Warnings),
    (id!(problems_header.chip_output), Filter::Output),
];

impl IdeApp {
    #[note("ide-app-problems-ideapp-refresh-problems")]
    pub(super) fn refresh_problems(&mut self, cx: &mut Cx, failed: Option<(bool, Duration)>) {
        let root = self.alvo_do_cargo().map(|(root, _)| root);
        let diagnostics = self.diagnosticos.as_deref().unwrap_or_default();
        self.problems.populate(diagnostics, root.as_deref());
        self.problems.reason = failed
            .map(|(run, elapsed)| self.problems.stop_text(run, elapsed))
            .unwrap_or_default();
        if self.painel_a_vista(&self.arranjo(), Panel::Problems) {
            self.write_problems(cx);
        }
    }

    pub(super) fn write_problems(&mut self, cx: &mut Cx) {
        let panel = self.painel(Panel::Problems);
        let reason = panel.label(id!(problems_header.reason));
        reason.set_text(cx, &self.problems.reason);
        for (chip, filter) in CHIPS {
            let (text, active) = self.problems.chip(filter);
            let on = f64::from(u8::from(active));
            panel.button(chip).set_text(cx, &text);
            let shown = filter != Filter::Panics || !self.problems.panics.is_empty();
            panel.button(chip).set_visible(cx, shown);
            panel
                .button(chip)
                .apply_over(cx, live! { draw_bg: { active: (on) } });
        }
        let list = id!(problems_body.problems_list);
        self.redraw_panel_list(cx, Panel::Problems, list);
    }

    pub(super) fn write_problem_row(cx: &mut Cx, item: &WidgetRef, row: &Row) {
        if let Some(error) = row.error_icon() {
            let glyph = [icon::Chrome::Warning, icon::Chrome::Error][usize::from(error)];
            item.label(id!(face.icon)).set_text(cx, glyph.glyph());
        }
        for (path, text) in row.labels() {
            let path: Vec<LiveId> = path.split('.').map(LiveId::from_str).collect();
            item.label(&path).set_text(cx, &text);
        }
    }

    pub(super) fn problems_list_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.painel(Panel::Problems);
        if let Some((_, filter)) = CHIPS.iter().find(|(c, _)| panel.button(c).clicked(actions)) {
            self.problems.set_filter(*filter);
            return self.write_problems(cx);
        }
        let list = panel.portal_list(id!(problems_body.problems_list));
        if !list.any_items_with_actions(actions) {
            return;
        }
        let rows = self.problems.rows();
        let picked = list
            .items_with_actions(actions)
            .into_iter()
            .find(|(_, item)| item.button(id!(target)).clicked(actions))
            .and_then(|(i, _)| rows.get(i).cloned());
        let place = picked.and_then(|row| self.problems.row_place(&row));
        if let Some(place) = place {
            self.go_to_place(cx, place);
        }
    }

    /// The app ended in a panic: the panel leads with it, the editor
    /// opens at its line and marks it.
    pub(super) fn report_panics(&mut self, cx: &mut Cx, end: crawrad_project::app_run::AppEnd) {
        let Some(first) = end.panics.first() else {
            return;
        };
        self.status = Some(first.headline(&end.package));
        self.problems
            .show_panics(end.panics, &end.stderr, &end.root);
        self.show_panel(cx, Panel::Problems);
        self.write_problems(cx);
        if let Some(place) = self.problems.panic_place(0, None) {
            self.go_to_place(cx, place);
        }
        self.apply_diagnostics_decorations(cx);
        self.refresh(cx);
    }

    /// Underlines each panic in the open file and returns its line messages.
    pub(super) fn mark_panics(&self, doc: &mut CodeDocument) -> Vec<(usize, String, u8)> {
        let file = self.abas.active_file();
        let marks = file.map(|f| [self.problems.panic_marks(f), self.tests.tree.marks(f)].concat());
        let mut messages = Vec::new();
        for (i, (line, byte, text)) in marks.unwrap_or_default().into_iter().enumerate() {
            let at = |byte_index| Position {
                line_index: line,
                byte_index,
            };
            doc.add_decoration(Decoration::new(9000 + i, at(byte), at(byte + 8), Error));
            messages.push((line, text, 0));
        }
        messages
    }
}
