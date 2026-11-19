//! The breakpoint dots in the editor, kept equal to the session's.
//!
//! What is decided lives in `crawrad_host::Breakpoints::sync_view`; this is the
//! glue to the view.

use makepad_code_editor::code_view::CodeView;
use makepad_widgets::*;

use super::IdeApp;

impl IdeApp {
    /// Once a frame, beside `colher_do_editor`: the file just put in the view
    /// gets its breakpoints; a click or an edit in the view reaches the host.
    pub(super) fn sync_breakpoints(&mut self, cx: &mut Cx) {
        let Some(file) = self.abas.active_file().map(std::path::Path::to_path_buf) else {
            return;
        };
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        let lines = cv.breakpoint_lines();
        // A breakpoint changed while a program is being debugged reaches the
        // debugger at once: the host sends it.
        let step = self
            .host
            .sync_breakpoint_view(&file, cv.breakpoints_owner(), &lines);
        if let crawrad_host::ViewSync::Push(lines) = step {
            cv.set_breakpoint_lines(cx, &file, &lines);
        }
        let marks = cv.bookmarks();
        let owner = cv.bookmarks_owner().map(std::path::Path::to_path_buf);
        if let crawrad_host::BookmarkSync::Push(marks) =
            self.host
                .bookmarks_mut()
                .sync_view(&file, owner.as_deref(), &marks)
        {
            cv.set_bookmarks(cx, &file, &marks);
        }
        // The yellow arrow, where the program stands if it is in this file.
        cv.show_stop(
            cx,
            self.host
                .debug_mut()
                .stop_view_in(&file, &self.texto_na_vista),
        );
    }

    /// `debug.toggle_breakpoint`: the caret's line, as a click on the margin.
    pub(super) fn toggle_breakpoint(&mut self, cx: &mut Cx) {
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        cv.toggle_breakpoint_at_cursor(cx);
    }
}
