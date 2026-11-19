use {
    crate::{
        CodeEditor, CodeEditorAction,
        code_editor::KeepCursorInView,
        gutter_mark::GutterMarkKind,
        makepad_widgets::*,
        {CodeDocument, CodeSession, decoration::DecorationSet},
    },
    std::env,
};

live_design! {
    use crate::code_editor::CodeEditor;

    pub CodeView = {{CodeView}}{
        editor: <CodeEditor>{
            pad_left_top: vec2(0.0, 0.0)
            height:Fit
            empty_page_at_end: false,
            read_only: true,
            show_gutter: false
        }
    }
}

/// A per-change click waiting for the IDE: `(change, accept)`. An alias,
/// since `derive(Live)` rejects a generic path as a field type.
type ReviewChoice = std::cell::Cell<Option<(usize, bool)>>;

#[derive(Live, LiveHook, Widget)]
pub struct CodeView {
    #[wrap]
    #[live]
    pub editor: CodeEditor,
    // alright we have to have a session and a document.
    #[rust]
    pub session: Option<CodeSession>,
    #[live(false)]
    keep_cursor_at_end: bool,

    #[live]
    text: ArcStringMut,
    /// The breakpoint lines (zero-based), kept here as well as in the
    /// document: `set_text` throws the document away and the next draw builds
    /// a new one, and the breakpoints must survive that.
    #[rust]
    breakpoint_lines: Vec<usize>,
    /// The file those breakpoints belong to, as the IDE names it.
    #[rust]
    breakpoints_owner: Option<std::path::PathBuf>,
    /// The numbered bookmarks `(number, zero-based line)`, kept here for the
    /// same reason as the breakpoints: the document is rebuilt by `set_text`.
    #[rust]
    bookmarks: Vec<(u8, usize)>,
    /// The file those bookmarks belong to. `None` after the text changed: the
    /// view then holds no file's bookmarks until the IDE hands them over —
    /// never the previous file's, and never an empty list taken for this one's.
    #[rust]
    bookmarks_owner: Option<std::path::PathBuf>,
    /// The (zero-based) line the program under the debugger stands on, if it
    /// is in this view's file.
    #[rust]
    execution_line: Option<usize>,
    /// The code lens line clicked, waiting for the IDE to act on it.
    #[rust]
    lens_clicked: Option<usize>,
    /// The first change of a review just shown, revealed on the next draw —
    /// after any caret the IDE restores for the tab in the same frame.
    #[rust]
    review_reveal: Option<usize>,
    /// A per-change accept/reject click in a diff review: (change_index, accept).
    #[rust]
    review_choice: ReviewChoice,
}

impl CodeView {
    pub fn lazy_init_session(&mut self) {
        if self.session.is_none() {
            let dec = DecorationSet::new();
            let doc = CodeDocument::new(self.text.as_ref().into(), dec);
            doc.set_gutter_mark_lines(GutterMarkKind::Breakpoint, &self.breakpoint_lines);
            doc.set_bookmarks(&self.bookmarks);
            let point: Vec<usize> = self.execution_line.into_iter().collect();
            doc.set_gutter_mark_lines(GutterMarkKind::ExecutionPoint, &point);
            self.session = Some(CodeSession::new(doc));
            self.session.as_mut().unwrap().handle_changes();
            if self.keep_cursor_at_end {
                self.session.as_mut().unwrap().set_cursor_at_file_end();
                self.editor.keep_cursor_in_view = KeepCursorInView::Once
            }
        }
    }

    pub fn get_scroll_pos(&self) -> DVec2 {
        self.editor.get_scroll_pos()
    }

    pub fn set_scroll_pos(&mut self, cx: &mut Cx, pos: DVec2) -> bool {
        self.editor.set_scroll_pos(cx, pos)
    }

    pub fn get_scroll_view_total(&mut self) -> DVec2 {
        self.editor.get_scroll_view_total()
    }

    pub fn get_scroll_view_visible(&mut self) -> DVec2 {
        self.editor.get_scroll_view_visible()
    }

    pub fn line_start(&self) -> usize {
        self.editor.line_start()
    }

    pub fn line_end(&self) -> usize {
        self.editor.line_end()
    }

    pub fn cell_size(&self) -> DVec2 {
        self.editor.cell_size()
    }

    pub fn scroll_to_line(&mut self, cx: &mut Cx, line_index: usize) {
        let cell_y = if self.editor.cell_size().y > 0.0 {
            self.editor.cell_size().y
        } else {
            18.0
        };
        let target_y = (line_index as f64 * cell_y).max(0.0);
        self.editor.set_scroll_pos(cx, dvec2(0.0, target_y));
        self.editor.redraw(cx);
    }

    pub fn last_cursor_screen_pos(&self) -> Option<DVec2> {
        self.editor.last_cursor_screen_pos()
    }

    pub fn viewport_rect(&self) -> Rect {
        self.editor.viewport_rect()
    }

    /// The whole text replaced as ONE undo step (Format Document); the
    /// caret stays on its line, clamped to it.
    pub fn replace_all(&mut self, cx: &mut Cx, text: &str) {
        let caret = self.caret();
        let Some(session) = self.session.as_ref() else {
            return;
        };
        session.document().replace(crate::text::Text::from(text));
        if let Some((line, byte)) = caret {
            let lines = session.document().as_text().as_lines().len().max(1);
            let line = line.min(lines - 1);
            let len = session.document().as_text().as_lines()[line].len();
            let mut byte = byte.min(len);
            while !session.document().as_text().as_lines()[line].is_char_boundary(byte) {
                byte -= 1;
            }
            session.set_selection(
                crate::text::Position {
                    line_index: line,
                    byte_index: byte,
                },
                crate::selection::Affinity::Before,
                crate::session::SelectionMode::Simple,
                crate::history::NewGroup::Yes,
            );
        }
        self.redraw(cx);
    }

    /// The document as it is NOW, lines joined by `\n` — not `text()`, the
    /// text the view was given: an emptied file is empty, not the old text
    /// again. `None` before the session exists.
    pub fn live_text(&self) -> Option<String> {
        let session = self.session.as_ref()?;
        Some(session.document().as_text().as_lines().join("\n"))
    }

    /// Is anything selected? (Cut and Copy have something to take.)
    pub fn has_selection(&self) -> bool {
        self.session
            .as_ref()
            .is_some_and(|s| s.selections().iter().any(|sel| !sel.is_empty()))
    }

    /// The selection's text, as the system's Copy takes it (`TextCopy`).
    pub fn copy_selection(&self) -> Option<String> {
        let text = self.session.as_ref()?.copy();
        (!text.is_empty()).then_some(text)
    }

    /// The selection's text, deleted, as the system's Cut does (`TextCut`).
    pub fn cut_selection(&mut self, cx: &mut Cx) -> Option<String> {
        self.copy_selection()?;
        if self.editor.is_read_only() {
            return None;
        }
        let text = self.session.as_ref()?.cut();
        self.redraw(cx);
        Some(text)
    }

    /// `text` pasted at the selection, as the system's paste does.
    pub fn paste_text(&mut self, cx: &mut Cx, text: &str) {
        if let (Some(session), false) = (self.session.as_ref(), self.editor.is_read_only()) {
            session.paste(crate::text::Text::from(text));
            self.redraw(cx);
        }
    }

    /// Everything selected, as `Ctrl+A` does.
    pub fn select_all(&mut self, cx: &mut Cx) {
        if let Some(session) = self.session.as_ref() {
            self.editor.select_all(cx, session);
        }
    }

    /// Where the caret is on screen, in window coordinates.
    pub fn caret_anchor(&self) -> Option<crate::code_editor::HoverRest> {
        let (line, byte) = self.caret()?;
        self.editor.anchor_at(self.session.as_ref()?, line, byte)
    }

    /// Where byte `byte` of `line` is on screen, read from the layout and
    /// not from the last frame: the completion list opens there.
    pub fn place_anchor(&self, line: usize, byte: usize) -> Option<crate::code_editor::HoverRest> {
        self.editor.anchor_at(self.session.as_ref()?, line, byte)
    }

    /// Where `line` starts on screen (its first byte): the light bulb of a
    /// code action goes just before it.
    pub fn line_anchor(&self, line: usize) -> Option<crate::code_editor::HoverRest> {
        self.editor.anchor_at(self.session.as_ref()?, line, 0)
    }

    pub fn set_landing_line(&mut self, line_index: usize, start_byte: usize, end_byte: usize) {
        self.editor
            .set_landing_line(line_index, start_byte, end_byte);
    }

    pub fn clear_landing_line(&mut self) {
        self.editor.clear_landing_line();
    }

    /// The breakpoint lines (zero-based), as they are now: moved by the edits
    /// and toggled by clicks on the glyph margin.
    pub fn breakpoint_lines(&self) -> Vec<usize> {
        match &self.session {
            Some(session) => session
                .document()
                .gutter_mark_lines(GutterMarkKind::Breakpoint),
            None => self.breakpoint_lines.clone(),
        }
    }

    /// The file the breakpoints belong to, if the IDE said.
    pub fn breakpoints_owner(&self) -> Option<&std::path::Path> {
        self.breakpoints_owner.as_deref()
    }

    /// Replaces the breakpoint lines (zero-based) with `owner`'s — for the
    /// file just put in the view.
    pub fn set_breakpoint_lines(&mut self, cx: &mut Cx, owner: &std::path::Path, lines: &[usize]) {
        self.breakpoints_owner = Some(owner.to_path_buf());
        self.breakpoint_lines = lines.to_vec();
        if let Some(session) = &self.session {
            session
                .document()
                .set_gutter_mark_lines(GutterMarkKind::Breakpoint, lines);
            self.breakpoint_lines = session
                .document()
                .gutter_mark_lines(GutterMarkKind::Breakpoint);
        }
        self.redraw(cx);
    }

    /// The note whose anchor icon was clicked, once — the IDE opens it.
    pub fn take_note_click(&mut self) -> Option<String> {
        self.editor.take_note_click()
    }

    /// The line of the code lens row that was clicked, once.
    pub fn take_lens_click(&mut self) -> Option<usize> {
        self.lens_clicked.take()
    }

    /// The per-change review decision clicked, once: (change_index, accept).
    pub fn take_review_choice(&self) -> Option<(usize, bool)> {
        self.review_choice.take()
    }

    /// Sets whether per-change review buttons are active on the editor.
    pub fn set_review_buttons(&mut self, on: bool) {
        self.editor.review_buttons = on;
    }

    /// The mouse rested on a name, once — the IDE asks its hover.
    pub fn take_hover_rest(&mut self) -> Option<crate::code_editor::HoverRest> {
        self.editor.take_hover_rest()
    }

    /// The mouse moved to another place in the text, once — the hover closes.
    pub fn take_hover_moved(&mut self) -> bool {
        self.editor.take_hover_moved()
    }

    /// A Ctrl+Click on the text, once — the IDE goes to the definition.
    pub fn take_ctrl_click(&mut self) -> bool {
        self.editor.take_ctrl_click()
    }

    /// Anchors of visible lines that still lack a preview, once each: (id, 0-based line).
    pub fn take_missing_notes(&mut self) -> Vec<(String, usize)> {
        self.editor.take_missing_notes()
    }

    /// The marks extensions painted on this file.
    pub fn set_line_marks(&mut self, cx: &mut Cx, marks: Vec<crate::line_mark::LineMark>) {
        self.editor.set_line_marks(marks);
        self.editor.redraw(cx);
    }

    /// Where each code lens row was last drawn: (the line below it, its
    /// rect as `[x, y, w, h]`).
    pub fn lens_bands(&self) -> Vec<(usize, [f64; 4])> {
        let rects = self.editor.lens_rects().iter();
        rects
            .map(|(l, r)| (*l, [r.pos.x, r.pos.y, r.size.x, r.size.y]))
            .collect()
    }

    /// Where each mark's icon was last drawn: (owner, id, rect).
    pub fn line_mark_rects(&self) -> &[(String, String, makepad_widgets::Rect)] {
        self.editor.line_mark_rects()
    }

    /// The mark whose icon was clicked, once: (owner, id).
    pub fn take_line_mark_click(&mut self) -> Option<(String, String)> {
        self.editor.take_line_mark_click()
    }

    /// A note anchor's preview, whether its icon carries the orange alert
    /// dot (doc out of the rustdoc rule, or signature changed), and whether
    /// its icon is hidden (a resolved comment), in one call.
    pub fn set_note_mark(&mut self, id: &str, (preview, alert, hidden): (String, bool, bool)) {
        self.editor.set_note_preview(id, &preview);
        self.editor.set_note_alert(id, alert);
        self.editor.set_note_hidden(id, hidden);
    }

    /// Clears cached note previews and alerts so the editor re-evaluates them on the next frame.
    pub fn forget_note_previews(&mut self) {
        self.editor.forget_note_previews();
    }

    /// Opens the note popup under the caret with the given header and rows.
    pub fn open_note_popup(&mut self, header: String, rows: Vec<crate::note_popup::PopupRowParts>) {
        self.editor.open_note_popup(header, rows);
    }

    /// The chosen popup row index, once.
    pub fn take_note_popup_choice(&mut self) -> Option<usize> {
        self.editor.take_note_popup_choice()
    }

    /// The numbered bookmarks as they are now — moved by the edits — as
    /// `(number, zero-based line)`.
    pub fn bookmarks(&self) -> Vec<(u8, usize)> {
        match &self.session {
            Some(session) => session.document().bookmarks(),
            None => self.bookmarks.clone(),
        }
    }

    /// The file the bookmarks belong to, if the IDE said since the text changed.
    pub fn bookmarks_owner(&self) -> Option<&std::path::Path> {
        self.bookmarks_owner.as_deref()
    }

    /// Replaces the bookmarks with `owner`'s — for the file in the view.
    pub fn set_bookmarks(
        &mut self,
        cx: &mut Cx,
        owner: &std::path::Path,
        bookmarks: &[(u8, usize)],
    ) {
        self.bookmarks_owner = Some(owner.to_path_buf());
        self.bookmarks = bookmarks.to_vec();
        if let Some(session) = &self.session {
            session.document().set_bookmarks(bookmarks);
            self.bookmarks = session.document().bookmarks();
        }
        self.redraw(cx);
    }

    /// Bookmark `n` on the caret's line, or off it if it is there. `None`
    /// when the line is a note's anchor: it takes no other mark.
    pub fn toggle_bookmark_at_cursor(&mut self, cx: &mut Cx, n: u8) -> Option<bool> {
        self.lazy_init_session();
        let line = self.caret().map_or(0, |(line, _)| line);
        if self.is_note_line(line) {
            return None;
        }
        let session = self.session.as_ref().unwrap();
        let on = session.document().toggle_bookmark(n, line);
        self.bookmarks = session.document().bookmarks();
        self.redraw(cx);
        Some(on)
    }

    /// Whether (zero-based) `line` is a sidecar note's anchor — a comment
    /// line that carries the note's icon and nothing else.
    pub fn is_note_line(&self, line: usize) -> bool {
        self.session
            .as_ref()
            .and_then(|s| s.document().as_text().as_lines().get(line).cloned())
            .is_some_and(|text| crate::note_anchor::id_in(&text).is_some())
    }

    /// The (zero-based) line bookmark `n` is on, if it is set.
    pub fn bookmark_line(&self, n: u8) -> Option<usize> {
        self.bookmarks()
            .into_iter()
            .find(|(m, _)| *m == n)
            .map(|(_, line)| line)
    }

    /// Where the program under the debugger stands (zero-based), or `None`
    /// when it is not stopped in this view's file.
    pub fn set_execution_line(&mut self, cx: &mut Cx, line: Option<usize>) {
        if self.execution_line == line {
            return;
        }
        self.execution_line = line;
        if let Some(session) = &self.session {
            let point: Vec<usize> = line.into_iter().collect();
            session
                .document()
                .set_gutter_mark_lines(GutterMarkKind::ExecutionPoint, &point);
        }
        self.redraw(cx);
    }

    /// Where a debugged program stands in this file: the yellow arrow on
    /// `line`, and the values written after the lines around it.
    pub fn show_stop(
        &mut self,
        cx: &mut Cx,
        (line, values): (Option<usize>, Vec<(usize, String)>),
    ) {
        self.set_execution_line(cx, line);
        self.set_line_end_texts(cx, &values);
    }

    /// Hints inside the lines, in grey — the types rust-analyzer infers
    /// Redraws only when they changed.
    pub fn set_inline_hints(&mut self, cx: &mut Cx, hints: &[(usize, usize, String)]) {
        if let Some(session) = &self.session
            && session.set_inline_hints(hints)
        {
            self.redraw(cx);
        }
    }

    /// Lights the occurrences of the name under the caret (`(line, start
    /// byte, end byte, written)`), replacing the previous ones. Redraws only
    /// when they changed.
    pub fn set_occurrences(&mut self, cx: &mut Cx, found: &[(usize, usize, usize, bool)]) {
        if let Some(session) = &self.session
            && session.document().set_occurrences(found)
        {
            self.redraw(cx);
        }
    }

    /// The caret as `(line, byte)`, zero-based — where the occurrences are
    /// asked about.
    pub fn caret(&self) -> Option<(usize, usize)> {
        let session = self.session.as_ref()?;
        let selections = session.selections();
        let index = session.last_added_selection_index().unwrap_or(0);
        let p = selections.get(index)?.cursor.position;
        Some((p.line_index, p.byte_index))
    }

    /// Text after the end of lines, in grey — the debugger's values on the
    /// lines around the stop. Redraws only when it changed.
    pub fn set_line_end_texts(&mut self, cx: &mut Cx, texts: &[(usize, String)]) {
        if let Some(session) = &self.session
            && session.set_line_end_texts(texts)
        {
            self.redraw(cx);
        }
    }

    /// Shows a diff review in the editor; `rows` empty ends it. A new review
    /// opens on its first change. Redraws only when it changed.
    pub fn set_review(&mut self, cx: &mut Cx, rows: &[crate::review::ReviewRow]) {
        self.lazy_init_session();
        if let Some(session) = &self.session
            && session.set_review(rows)
        {
            let same = crate::review::ReviewRowKind::Same;
            self.review_reveal = rows.iter().position(|r| r.kind != same);
            self.redraw(cx);
        }
    }

    /// Puts the caret at the start of `line` and scrolls it into view (the
    /// editor keeps the caret in view, so scrolling alone would be undone).
    pub fn reveal_line(&mut self, cx: &mut Cx, line: usize) {
        if let Some(session) = self.session.as_mut() {
            let pos = crate::text::Position {
                line_index: line,
                byte_index: 0,
            };
            self.editor.set_cursor_and_scroll(cx, pos, session);
        }
    }

    /// Code lens lines above source lines. Redraws only when changed.
    pub fn set_lenses(&mut self, cx: &mut Cx, lenses: &[(usize, String)]) {
        if let Some(session) = &self.session
            && session.set_lenses(lenses)
        {
            self.redraw(cx);
        }
    }

    /// Adds or removes the breakpoint on the caret's line — the keyboard's
    /// way to do what a click on the glyph margin does.
    pub fn toggle_breakpoint_at_cursor(&mut self, cx: &mut Cx) {
        self.lazy_init_session();
        let session = self.session.as_ref().unwrap();
        let line = session
            .selections()
            .first()
            .map(|s| s.cursor.position.line_index)
            .unwrap_or(0);
        self.toggle_breakpoint(cx, line);
    }

    /// Adds or removes the breakpoint on `line` (zero-based). A note's anchor
    /// line takes none.
    pub fn toggle_breakpoint(&mut self, cx: &mut Cx, line: usize) {
        self.lazy_init_session();
        if self.is_note_line(line) {
            return;
        }
        let session = self.session.as_ref().unwrap();
        session
            .document()
            .toggle_gutter_mark(GutterMarkKind::Breakpoint, line);
        self.breakpoint_lines = session
            .document()
            .gutter_mark_lines(GutterMarkKind::Breakpoint);
        self.redraw(cx);
    }
}

impl Widget for CodeView {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // alright so.
        self.lazy_init_session();
        // alright we have a scope, and an id, so now we can properly draw the editor.
        if let Some(line) = self.review_reveal.take() {
            self.reveal_line(cx, line);
        }
        let session = self.session.as_mut().unwrap();
        if session.document().reviewing() {
            // A landing marks the file's lines; the review shows merged ones.
            self.editor.landing_indicator = None;
            self.editor.clear_landing_line();
        }

        self.editor.draw_walk_editor(cx, session, walk);

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.lazy_init_session();
        let session = self.session.as_mut().unwrap();
        for action in self
            .editor
            .handle_event(cx, event, &mut Scope::empty(), session)
        {
            if let CodeEditorAction::LensClicked(line) = action {
                self.lens_clicked = Some(line);
            }
            if let CodeEditorAction::ReviewChoice { change, accept } = action {
                self.review_choice.set(Some((change, accept)));
            }
            //cx.widget_action(uid, &scope.path, action);
            session.handle_changes();
        }
        // Edits move the marks and a click on the glyph margin toggles one:
        // the copy that outlives the document follows.
        self.breakpoint_lines = session
            .document()
            .gutter_mark_lines(GutterMarkKind::Breakpoint);
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        if self.text.as_ref() != v {
            self.text.as_mut_empty().push_str(v);
            self.session = None;
            // Another text, maybe another file: its bookmarks come from the IDE.
            self.bookmarks.clear();
            self.bookmarks_owner = None;
            self.editor.landing_indicator = None;
            self.redraw(cx);
        }
    }
}
