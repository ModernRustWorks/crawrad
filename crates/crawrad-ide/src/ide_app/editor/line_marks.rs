//! **Extensions' line marks**: what an extension painted on the open
//! file goes to the editor when it changes, and a click on a mark's icon goes
//! back to that extension. The marks and the protocol live in
//! `crawrad_extension_host::line_marks` and `crawrad_extension_api::marks`.

use super::*;
use makepad_code_editor::code_view::CodeView;
use makepad_code_editor::line_mark::LineMark;
use makepad_widgets::makepad_platform::thread::SignalToUI;

impl IdeApp {
    /// Every wake: the marks given to the view when they or the file changed;
    /// a clicked icon told to its extension.
    pub(crate) fn follow_line_marks(&mut self, cx: &mut Cx) {
        let Some(store) = self.extensions.line_marks() else {
            return;
        };
        let file = self.abas.active_file().map(std::path::Path::to_path_buf);
        let view = self.code_view();
        let Ok(mut marks) = store.lock() else {
            return;
        };
        marks.set_waker(SignalToUI::set_ui_signal);
        let seen = (marks.revision(), file.clone());
        if seen != self.marks_seen {
            let on = file.as_deref().map(|f| marks.on(f)).unwrap_or_default();
            let drawn = on.into_iter().map(|(owner, m)| LineMark {
                owner,
                id: m.id,
                line: m.line as usize,
                start: m.start as usize,
                end: m.end as usize,
                label: m.label,
                kind: m.kind,
            });
            if let Some(mut cv) = view.borrow_mut::<CodeView>() {
                cv.set_line_marks(cx, drawn.collect());
            }
            self.marks_seen = seen;
        }
        drop(marks);
        let clicked = view
            .borrow_mut::<CodeView>()
            .and_then(|mut cv| cv.take_line_mark_click());
        if let (Some((owner, id)), Some(file)) = (clicked, file) {
            let _ = self
                .extensions
                .line_mark_clicked(&owner, &id, &file.to_string_lossy());
        }
        // An extension's answer (a subprocess's `window/showMessage`) reaches
        // the status bar on the wake it brings.
        if let Some((_, said)) = self.extensions.take_messages().pop() {
            self.status = Some(said);
        }
    }

    /// What `editor.get_line_marks` reads: each mark drawn on the open file,
    /// with its icon's rect in window pixels.
    pub(super) fn line_marks_json(&self) -> serde_json::Value {
        let view = self.code_view();
        let Some(cv) = view.borrow::<CodeView>() else {
            return serde_json::json!([]);
        };
        let found = cv.line_mark_rects().iter().map(
            |(owner, id, r)| serde_json::json!({ "owner": owner, "id": id, "icon": xywh(*r) }),
        );
        serde_json::Value::Array(found.collect())
    }
}
