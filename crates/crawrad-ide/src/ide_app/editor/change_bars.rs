//! Glue connecting the editor's change tracker to the code view widget.

use makepad_code_editor::code_view::CodeView;

use super::*;

impl IdeApp {
    /// Recomputes the change bars of the editable file in front and hands
    /// them to the code view; redraws it only when they changed.
    pub(crate) fn refresh_change_bars(&mut self, cx: &mut Cx) {
        let changes = match self.editavel.as_deref() {
            Some(file) => {
                self.change_tracker
                    .changes(file, &self.base_na_vista, &self.texto_na_vista)
            }
            None => Vec::new(),
        };
        let view = self.code_view();
        let changed = match view.borrow_mut::<CodeView>() {
            Some(mut cv) => {
                cv.forget_note_previews();
                cv.lazy_init_session();
                cv.session
                    .as_ref()
                    .is_some_and(|s| s.document().set_line_changes(&changes))
            }
            None => false,
        };
        if changed {
            view.redraw(cx);
        }
    }
}
