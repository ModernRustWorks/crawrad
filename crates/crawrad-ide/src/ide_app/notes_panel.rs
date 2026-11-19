use crawrad_actions::Panel;
use crawrad_host::navigation::Place;
use crawrad_project::notes_panel::index_project_live;
use makepad_code_editor::notes_panel_view::{
    NotesPanelAction, NotesPanelViewRef, NotesPanelViewWidgetRefExt,
};
use makepad_widgets::*;

use super::{IdeApp, layout};

impl IdeApp {
    pub(super) fn refresh_notes(&mut self, cx: &mut Cx) {
        let notes = index_project_live(&self.raiz_do_projeto(), |p| self.get_editor_buffer(p));
        self.notes.set_notes(notes);
        self.paint_notes(cx);
    }

    fn notes_view(&self) -> NotesPanelViewRef {
        let tab = self.dock().item(layout::tab_of(Panel::Notes));
        tab.notes_panel_view(id!(notes_view))
    }

    pub(super) fn paint_notes(&mut self, cx: &mut Cx) {
        let (head, rows) = (self.notes.view_head(), self.notes.view_rows());
        self.notes_view().set_view(cx, head, rows);
    }

    pub(super) fn notes_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        match self.notes_view().action(actions) {
            NotesPanelAction::None => return,
            NotesPanelAction::Chip(i) => self.notes.pick_chip(i),
            NotesPanelAction::File(choice) => self.notes.pick_file(choice),
            NotesPanelAction::Alert => self.notes.toggle_alert(),
            NotesPanelAction::Group(i) => self.notes.toggle_row(i),
            NotesPanelAction::Open(i) => return self.open_note_row(cx, i, false),
            NotesPanelAction::OpenDialog(i) => return self.open_note_row(cx, i, true),
        }
        self.paint_notes(cx);
    }

    fn open_note_row(&mut self, cx: &mut Cx, row: usize, dialog: bool) {
        let Some((file, line)) = self.notes.note_place(row) else {
            return;
        };
        let file = self.raiz_do_projeto().join(file);
        self.go_to_place(
            cx,
            Place {
                file: file.clone(),
                line,
                byte: 0,
            },
        );
        if dialog {
            self.open_note_dialog_at(cx, &file, line);
        }
    }

    pub(super) fn notes_json(&self) -> serde_json::Value {
        serde_json::from_str(&self.notes_view().rows_json()).unwrap_or_default()
    }
}
