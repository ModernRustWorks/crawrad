//! Autorecover: every few minutes the unsaved editor files are copied
//! to the project's `.crawrad/recover`; the rules are [`crawrad_project::recover`].

use crawrad_project::workspace::read_preferences;

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-recover-ideapp-follow-recovery")]
    pub(super) fn follow_recovery(&mut self, cx: &mut Cx) {
        let Some(found) = self.recover.follow(self.raiz().as_deref()) else {
            return;
        };
        self.recover_clock = cx.start_timeout(f64::from(read_preferences().recover_minutes) * 60.0);
        let offset = crawrad_os::clock::local_offset_seconds();
        self.recover_offer = RecoveryOffer::new(&found, std::time::SystemTime::now(), offset);
        self.draw_recover(cx);
    }

    #[note("ide-app-recover-ideapp-recovery-tick")]
    pub(super) fn recovery_tick(&mut self, cx: &mut Cx, event: &Event) {
        if self.recover_clock.is_event(event).is_none() {
            return;
        }
        let p = read_preferences();
        self.recover_clock = cx.start_timeout(f64::from(p.recover_minutes) * 60.0);
        let edits = self
            .host
            .editor_buffers()
            .iter()
            .filter(|_| p.recover_copies);
        if let Err(e) = self
            .recover
            .sync(edits.map(|(f, t)| (f.as_path(), t.as_str())))
        {
            self.status = Some(format!("Recovery copy failed: {e}"));
        }
    }

    #[note("ide-app-recover-ideapp-answer-recovery")]
    pub(super) fn answer_recovery(&mut self, cx: &mut Cx, files: Vec<PathBuf>, discard: bool) {
        let answered = self.recover.answer(files, discard);
        let n = answered.len();
        let kept = self.recover.kept();
        self.recover_offer.rows.retain(|r| kept.contains(&r.file));
        for (file, text) in answered.into_iter().filter(|_| !discard) {
            self.host.set_buffer(file.clone(), text);
            self.abrir_no_editor(cx, file);
        }
        let done = if discard { "discarded" } else { "recovered" };
        self.status = Some(format!("{n} recovery copies {done}"));
        self.redraw_editor_tabs(cx);
        self.draw_recover(cx);
    }

    #[note("ide-app-recover-ideapp-draw-recover")]
    pub(super) fn draw_recover(&mut self, cx: &mut Cx) {
        let open = !self.recover_offer.rows.is_empty();
        self.show_veil(cx, id!(recover_dialog), open);
        self.ui.redraw(cx);
    }

    #[note("ide-app-recover-ideapp-handle-recover-dialog")]
    pub(super) fn handle_recover_dialog(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.recover_offer.rows.is_empty() {
            return;
        }
        let dlg = self.ui.view(id!(recover_dialog.dlg));
        for (i, item) in dlg.portal_list(id!(rec_list)).items_with_actions(actions) {
            if let Some(on) = item.check_box(id!(marca)).changed(actions) {
                self.recover_offer.check(i, on);
            }
        }
        let checked = self.recover_offer.checked();
        if dlg.button(id!(rec_recover)).clicked(actions) && !checked.is_empty() {
            self.answer_recovery(cx, checked, false);
        } else if dlg.button(id!(rec_discard)).clicked(actions) {
            self.answer_recovery(cx, self.recover_offer.files(), true);
        }
        if dlg.button(id!(rec_later)).clicked(actions)
            || dlg.button(id!(rec_recover)).clicked(actions)
        {
            self.recover_offer = RecoveryOffer::default();
            self.draw_recover(cx);
        }
    }
}
