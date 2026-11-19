//! Auto save of the editor files: after a delay, or when the editor
//! or the window loses focus. The form is not among them: it saves with
//! Save, since saving it regenerates the designer code.

use crawrad_project::workspace::AutoSave;

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-auto-save-ideapp-auto-save-typed")]
    pub(super) fn auto_save_typed(&mut self, cx: &mut Cx) {
        if self.auto_save == AutoSave::AfterDelay {
            self.auto_save_clock = cx.start_timeout(AutoSave::DELAY_SECONDS);
        }
    }

    #[note("ide-app-auto-save-ideapp-auto-save-event")]
    pub(super) fn auto_save_event(&mut self, cx: &mut Cx, event: &Event) {
        let due = match self.auto_save {
            AutoSave::Off => false,
            AutoSave::AfterDelay => self.auto_save_clock.is_event(event).is_some(),
            AutoSave::EditorFocusLost => {
                let focused = self.code_view_focused(cx);
                std::mem::replace(&mut self.editor_had_focus, focused) && !focused
            }
            AutoSave::WindowFocusLost => matches!(event, Event::AppLostFocus),
        };
        if due && !self.host.open_file_paths().is_empty() {
            let (_, failed) = self.write_editor_buffers(cx);
            if !failed.is_empty() {
                self.status = Some(format!("Auto save failed: {}", failed.join("; ")));
            }
            self.redraw_editor_tabs(cx);
            self.sincronizar(cx);
        }
    }
}
