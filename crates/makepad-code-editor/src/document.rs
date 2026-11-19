use {
    crate::{
        char::CharExt,
        decoration::{Decoration, DecorationSet},
        gutter_mark::{GutterMarkKind, GutterMarkSet},
        history::{EditKind, History},
        inlays::{BlockInlay, InlineInlay},
        iter::IteratorExt,
        selection::SelectionSet,
        session::SessionId,
        settings::Settings,
        str::StrExt,
        text::{Change, Drift, Edit, Length, Position, Text},
        token::{Token, TokenKind},
        tokenizer::Tokenizer,
    },
    std::{
        cell::{Ref, RefCell},
        cmp::Ordering,
        collections::HashMap,
        iter,
        ops::Range,
        rc::Rc,
        sync::mpsc::Sender,
    },
};

#[derive(Clone, Debug)]
pub struct CodeDocument(Rc<DocumentInner>);

impl CodeDocument {
    pub fn new(text: Text, decorations: DecorationSet) -> Self {
        let line_count = text.as_lines().len();
        let tokens: Vec<_> = (0..line_count)
            .map(|line| tokenize(&text.as_lines()[line]).collect::<Vec<_>>())
            .collect();
        let inner = Self(Rc::new(DocumentInner {
            history: RefCell::new(History::from(text)),
            layout: RefCell::new(DocumentLayout {
                indent_state: (0..line_count).map(|_| None).collect(),
                tokens,
                inline_inlays: (0..line_count).map(|_| Vec::new()).collect(),
                block_inlays: Vec::new(),
            }),
            tokenizer: RefCell::new(Tokenizer::new(line_count)),
            decorations: RefCell::new(decorations),
            semantic: RefCell::new(crate::semantic::SemanticSet::default()),
            snippet: RefCell::new(crate::snippet::SnippetStops::default()),
            gutter_marks: RefCell::new(GutterMarkSet::default()),
            occurrences: RefCell::new(Vec::new()),
            line_messages: RefCell::new(Vec::new()),
            line_changes: RefCell::new(Vec::new()),
            review_rows: RefCell::new(Vec::new()),
            edit_senders: RefCell::new(HashMap::new()),
        }));
        inner.update_indent_state();
        inner.0.tokenizer.borrow_mut().update(
            inner.0.history.borrow().as_text(),
            &mut inner.0.layout.borrow_mut().tokens,
        );
        inner
    }

    pub fn replace(&self, new_text: Text) {
        let mut history = self.0.history.borrow_mut();

        // Create an edit that deletes the entire existing text.
        let text_length = history.as_text().length();
        let delete_edit = Edit {
            change: Change::Delete(Position::zero(), text_length),
            drift: Drift::Before,
        };

        // Create an edit that inserts the new text at position zero.
        let insert_edit = Edit {
            change: Change::Insert(Position::zero(), new_text),
            drift: Drift::Before,
        };

        // Apply the edits to history, starting a new group for undo.
        history.force_new_group(); // Start a new undo group.
        history.push_or_extend_group(
            SessionId::default(),
            EditKind::Other,
            &SelectionSet::default(),
        );
        history.apply_edit(delete_edit.clone());
        history.apply_edit(insert_edit.clone());

        drop(history);

        // Now, update the document state after the edits.
        let edits = vec![delete_edit, insert_edit];
        self.update_after_edit(None, None, &edits);
    }

    pub fn as_text(&self) -> Ref<'_, Text> {
        Ref::map(self.0.history.borrow(), |history| history.as_text())
    }

    pub fn layout(&self) -> Ref<'_, DocumentLayout> {
        self.0.layout.borrow()
    }

    /// Text drawn after the end of some lines (the debugger's values):
    /// `texts` REPLACES every end-of-line text the document had; the hints
    /// inside the lines stay. Returns the lines that changed, for the sessions
    /// to lay them out again.
    pub fn set_line_end_texts(&self, texts: &[(usize, String)]) -> Vec<usize> {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let texts = if self.reviewing() { &[][..] } else { texts };
        let ends: Vec<usize> = self.as_text().as_lines().iter().map(String::len).collect();
        self.replace_inlays(
            |inlay| matches!(inlay, InlineInlay::Text(_)),
            |line| {
                texts
                    .iter()
                    .filter(|(l, _)| *l == line)
                    .map(|(_, t)| {
                        (
                            ends.get(line).copied().unwrap_or(0),
                            InlineInlay::Text(t.clone()),
                        )
                    })
                    .collect()
            },
        )
    }

    /// The diagnostics' messages, written after the end of their lines as
    /// `(line, text, severity)` — severity 0 an error, 1 a warning, 2 the
    /// rest. One per line: the first given wins. REPLACES the previous ones;
    /// answers whether anything changed.
    pub fn set_line_messages(&self, messages: &[(usize, String, u8)]) -> bool {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let messages = if self.reviewing() { &[][..] } else { messages };
        let lines = self.as_text().as_lines().len();
        let mut next: Vec<LineMessage> = Vec::new();
        for (line, text, severity) in messages {
            if *line < lines && !next.iter().any(|m| m.line == *line) {
                next.push(LineMessage {
                    line: *line,
                    text: text.clone(),
                    severity: *severity,
                });
            }
        }
        let mut current = self.0.line_messages.borrow_mut();
        if *current == next {
            return false;
        }
        *current = next;
        true
    }

    pub fn line_messages(&self) -> Ref<'_, [LineMessage]> {
        Ref::map(self.0.line_messages.borrow(), Vec::as_slice)
    }

    /// The lines changed in this session, as `(line, kind)`, for the change
    /// bars beside the numbers. REPLACES the previous ones; lines past the end
    /// are dropped. Answers whether anything changed, so the view redraws
    /// only then.
    pub fn set_line_changes(&self, changes: &[(usize, LineChange)]) -> bool {
        let lines = self.as_text().as_lines().len();
        let mut next: Vec<(usize, LineChange)> = changes
            .iter()
            .copied()
            .filter(|(l, _)| *l < lines)
            .collect();
        // Sorted and one per line: the gutter finds each line by binary search.
        next.sort_by_key(|(l, _)| *l);
        next.dedup_by_key(|(l, _)| *l);
        let mut current = self.0.line_changes.borrow_mut();
        if *current == next {
            return false;
        }
        *current = next;
        true
    }

    pub fn line_changes(&self) -> Ref<'_, [(usize, LineChange)]> {
        Ref::map(self.0.line_changes.borrow(), Vec::as_slice)
    }

    /// Diff review rows for inline change display. Replaces the current ones
    /// and returns whether anything changed.
    pub fn set_review(&self, rows: &[crate::review::ReviewRow]) -> bool {
        let mut current = self.0.review_rows.borrow_mut();
        if *current == rows {
            return false;
        }
        *current = rows.to_vec();
        true
    }

    pub fn review_rows(&self) -> Ref<'_, [crate::review::ReviewRow]> {
        Ref::map(self.0.review_rows.borrow(), Vec::as_slice)
    }

    /// A diff review is shown: the text is the merged diff, not the file.
    pub fn reviewing(&self) -> bool {
        !self.0.review_rows.borrow().is_empty()
    }

    /// The occurrences of the name under the caret, as `(line, start byte,
    /// end byte, written there)`: REPLACES the previous ones. Returns whether
    /// anything changed, so the view redraws only then.
    pub fn set_occurrences(&self, found: &[(usize, usize, usize, bool)]) -> bool {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let found = if self.reviewing() { &[][..] } else { found };
        let ends: Vec<usize> = self.as_text().as_lines().iter().map(String::len).collect();
        let next: Vec<Occurrence> = found
            .iter()
            .filter(|(line, start, end, _)| {
                ends.get(*line).is_some_and(|len| start < end && end <= len)
            })
            .map(|&(line, start, end, write)| Occurrence {
                start: Position {
                    line_index: line,
                    byte_index: start,
                },
                end: Position {
                    line_index: line,
                    byte_index: end,
                },
                write,
            })
            .collect();
        let mut current = self.0.occurrences.borrow_mut();
        if *current == next {
            return false;
        }
        *current = next;
        true
    }

    pub fn occurrences(&self) -> Ref<'_, [Occurrence]> {
        Ref::map(self.0.occurrences.borrow(), Vec::as_slice)
    }

    /// The hints inside the lines (types, parameter names) as `(line, byte,
    /// text)`: REPLACES every hint the document had; the end-of-line texts
    /// stay. A byte past its line's end is clamped to the end.
    pub fn set_inline_hints(&self, hints: &[(usize, usize, String)]) -> Vec<usize> {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let hints = if self.reviewing() { &[][..] } else { hints };
        let ends: Vec<usize> = self.as_text().as_lines().iter().map(String::len).collect();
        self.replace_inlays(
            |inlay| matches!(inlay, InlineInlay::Hint(_)),
            |line| {
                let end = ends.get(line).copied().unwrap_or(0);
                hints
                    .iter()
                    .filter(|(l, _, _)| *l == line)
                    .map(|(_, byte, t)| ((*byte).min(end), InlineInlay::Hint(t.clone())))
                    .collect()
            },
        )
    }

    /// Code lens lines shown ABOVE their lines: REPLACES every [`BlockInlay::Lens`]
    /// (keeps other block inlays), one per line at most (joining multiple with `" | "`),
    /// sorted by line, drops lines past the end of the text. Returns whether anything changed.
    pub fn set_lenses(&self, lenses: &[(usize, String)]) -> bool {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let lenses = if self.reviewing() { &[][..] } else { lenses };
        let line_count = self.as_text().as_lines().len();
        let mut grouped: Vec<(usize, String)> = Vec::new();
        for &(line, ref text) in lenses {
            if line >= line_count {
                continue;
            }
            if let Some((_, existing)) = grouped.iter_mut().find(|(l, _)| *l == line) {
                existing.push_str(" | ");
                existing.push_str(text);
            } else {
                grouped.push((line, text.clone()));
            }
        }
        grouped.sort_by_key(|(line, _)| *line);

        let mut next_inlays: Vec<(usize, BlockInlay)> = Vec::new();
        let mut layout = self.0.layout.borrow_mut();
        for (line, inlay) in &layout.block_inlays {
            if !matches!(inlay, BlockInlay::Lens(_)) {
                next_inlays.push((*line, inlay.clone()));
            }
        }
        for (line, text) in grouped {
            next_inlays.push((line, BlockInlay::Lens(text)));
        }
        next_inlays.sort_by_key(|(line, _)| *line);

        if layout.block_inlays == next_inlays {
            return false;
        }
        layout.block_inlays = next_inlays;
        drop(layout);

        for sender in self.0.edit_senders.borrow().values() {
            let _ = sender.send((None, Vec::new()));
        }
        true
    }

    /// Replaces, line by line, the inlays `mine` recognises with what `new`
    /// gives for the line, keeps the others, and orders them by byte.
    fn replace_inlays(
        &self,
        mine: impl Fn(&InlineInlay) -> bool,
        new: impl Fn(usize) -> Vec<(usize, InlineInlay)>,
    ) -> Vec<usize> {
        let mut layout = self.0.layout.borrow_mut();
        let mut changed = Vec::new();
        for (line, inlays) in layout.inline_inlays.iter_mut().enumerate() {
            let mut want: Vec<(usize, InlineInlay)> =
                inlays.iter().filter(|(_, i)| !mine(i)).cloned().collect();
            want.extend(new(line));
            // End-of-line text last on its byte, after any hint on the same byte.
            want.sort_by_key(|(byte, i)| (*byte, matches!(i, InlineInlay::Text(_))));
            if *inlays != want {
                *inlays = want;
                changed.push(line);
            }
        }
        changed
    }

    pub fn decorations(&self) -> Ref<'_, [Decoration]> {
        Ref::map(self.0.decorations.borrow(), |decorations| {
            decorations.as_decorations()
        })
    }

    pub fn edit_selections(
        &self,
        session_id: SessionId,
        kind: EditKind,
        selections: &SelectionSet,
        settings: &Settings,
        mut f: impl FnMut(Editor<'_>, Position, Length),
    ) {
        let mut history = self.0.history.borrow_mut();
        history.push_or_extend_group(session_id, kind, selections);
        let mut edits = Vec::new();
        let mut line_ranges = Vec::new();
        let mut prev_start = Position::zero();
        let mut prev_adjusted_start = Position::zero();
        let mut prev_edit_start = 0;
        for &selection in selections {
            let mut adjusted_start = prev_adjusted_start + (selection.start() - prev_start);
            for edit in &edits[prev_edit_start..] {
                adjusted_start = adjusted_start.apply_edit(edit);
            }
            let edit_start = edits.len();
            f(
                Editor {
                    history: &mut history,
                    edits: &mut edits,
                },
                adjusted_start,
                selection.length(),
            );
            for edit in &edits[edit_start..] {
                if let Change::Insert(position, ref text) = edit.change {
                    if let Some(char) = text.to_single_char() {
                        if char == '}'
                            && history.as_text().as_lines()[position.line_index]
                                [..position.byte_index]
                                .chars()
                                .all(|char| char.is_whitespace())
                        {
                            line_ranges.push(Range {
                                start: position.line_index,
                                end: position.line_index + 1,
                            });
                        }
                    } else if text.as_lines().len() > 1 {
                        line_ranges.push(Range {
                            start: if history.as_text().as_lines()[position.line_index]
                                [..position.byte_index]
                                .chars()
                                .all(|char| char.is_whitespace())
                            {
                                position.line_index
                            } else {
                                position.line_index + 1
                            },
                            end: position.line_index + text.as_lines().len(),
                        });
                    }
                }
            }
            prev_start = selection.start();
            prev_adjusted_start = adjusted_start;
            prev_edit_start = edit_start;
        }
        drop(history);
        self.autoindent(&line_ranges, settings.tab_column_count, &mut edits);
        self.update_after_edit(Some(session_id), None, &edits);
    }

    pub fn edit_linewise(
        &self,
        origin_id: SessionId,
        kind: EditKind,
        selections: &SelectionSet,
        mut f: impl FnMut(Editor, usize),
    ) {
        let mut history = self.0.history.borrow_mut();
        history.push_or_extend_group(origin_id, kind, selections);
        let mut edits = Vec::new();
        for line_range in selections
            .iter()
            .copied()
            .map(|selection| selection.line_range())
            .merge(|line_range_0, line_range_1| {
                if line_range_0.end >= line_range_1.start {
                    Ok(line_range_0.start..line_range_1.end)
                } else {
                    Err((line_range_0, line_range_1))
                }
            })
        {
            for line_index in line_range {
                f(
                    Editor {
                        history: &mut history,
                        edits: &mut edits,
                    },
                    line_index,
                );
            }
        }
        drop(history);
        self.update_after_edit(Some(origin_id), None, &edits);
    }

    pub fn add_decoration(&mut self, decoration: Decoration) {
        self.0.decorations.borrow_mut().add_decoration(decoration);
    }

    /// The snippet's stops: the one to select now, and whether Tab
    /// still has one (`snippet_next`), or none (`end_snippet`).
    pub fn start_snippet(&self, stops: &[[(usize, usize); 2]]) {
        let at = |(line_index, byte_index)| Position {
            line_index,
            byte_index,
        };
        let stops = stops.iter().map(|[a, b]| (at(*a), at(*b))).collect();
        self.0.snippet.borrow_mut().start(stops);
    }

    pub fn snippet_next(&self) -> Option<(Position, Position)> {
        self.0.snippet.borrow_mut().advance()
    }

    pub fn snippet_active(&self) -> bool {
        self.0.snippet.borrow().active()
    }

    pub fn end_snippet(&self) {
        self.0.snippet.borrow_mut().end();
    }

    /// The server's classified spans for the current text.
    pub fn set_semantic(&self, spans: Vec<crate::semantic::SemanticSpan>) {
        // A review shows merged text: the file's decorations would land on the wrong lines.
        let spans = if self.reviewing() { Vec::new() } else { spans };
        self.0.semantic.borrow_mut().set(spans);
    }

    pub fn semantic(&self) -> Ref<'_, [crate::semantic::SemanticSpan]> {
        Ref::map(self.0.semantic.borrow(), |s| s.as_spans())
    }

    /// What `editor.get_semantic_tokens` reads: each coloured span, 1-based
    /// line, its bytes, its kind and its text.
    pub fn semantic_json(&self) -> serde_json::Value {
        let text = self.as_text();
        let lines = text.as_lines();
        let spans = self.semantic();
        let found = spans.iter().map(|s| {
            let line = lines.get(s.start.line_index).map(String::as_str);
            let text = line.and_then(|l| l.get(s.start.byte_index..s.end.byte_index));
            serde_json::json!({ "line": s.start.line_index + 1, "start": s.start.byte_index,
                                "end": s.end.byte_index, "kind": s.kind.map(|k| format!("{k:?}")),
                                "struck": s.struck, "text": text.unwrap_or("") })
        });
        serde_json::Value::Array(found.collect())
    }

    /// What `editor.get_decorations` reads: each squiggle's kind, ends and look.
    pub fn decorations_json(&self) -> serde_json::Value {
        let at = |p: crate::text::Position| [p.line_index, p.byte_index];
        let marks = self.decorations();
        let found = marks.iter().map(|d| {
            serde_json::json!({ "kind": format!("{:?}", d.ty), "start": at(d.start()),
                                "end": at(d.end()), "faded": d.faded, "struck": d.struck })
        });
        serde_json::Value::Array(found.collect())
    }

    pub fn clear_decorations(&mut self) {
        self.0.decorations.borrow_mut().clear()
    }

    /// The marks drawn in the gutter's glyph margin (see [`crate::gutter_mark`]).
    pub fn gutter_marks(&self) -> Ref<'_, GutterMarkSet> {
        self.0.gutter_marks.borrow()
    }

    /// The (zero-based) lines that carry a mark of `kind`.
    pub fn gutter_mark_lines(&self, kind: GutterMarkKind) -> Vec<usize> {
        self.0.gutter_marks.borrow().lines(kind)
    }

    /// Replaces the marks of `kind`; lines past the end of the text are dropped.
    pub fn set_gutter_mark_lines(&self, kind: GutterMarkKind, lines: &[usize]) {
        let line_count = self.0.history.borrow().as_text().as_lines().len();
        let mut marks = self.0.gutter_marks.borrow_mut();
        marks.set_lines(kind, lines);
        marks.clamp(line_count);
    }

    /// Every bookmark as `(number, line)`.
    pub fn bookmarks(&self) -> Vec<(u8, usize)> {
        self.0.gutter_marks.borrow().bookmarks()
    }

    /// Replaces every bookmark; lines past the end of the text are dropped.
    pub fn set_bookmarks(&self, bookmarks: &[(u8, usize)]) {
        let line_count = self.0.history.borrow().as_text().as_lines().len();
        let mut marks = self.0.gutter_marks.borrow_mut();
        marks.set_bookmarks(bookmarks);
        marks.clamp(line_count);
    }

    /// Bookmark `n` on `line`, or off it (see `GutterMarkSet::toggle_bookmark`).
    pub fn toggle_bookmark(&self, n: u8, line: usize) -> bool {
        self.0.gutter_marks.borrow_mut().toggle_bookmark(n, line)
    }

    /// Adds or removes the mark of `kind` on `line`; answers whether it is there now.
    pub fn toggle_gutter_mark(&self, kind: GutterMarkKind, line: usize) -> bool {
        self.0.gutter_marks.borrow_mut().toggle(kind, line)
    }

    pub fn add_session(
        &mut self,
        session_id: SessionId,
        edit_sender: Sender<(Option<SelectionSet>, Vec<Edit>)>,
    ) {
        self.0
            .edit_senders
            .borrow_mut()
            .insert(session_id, edit_sender);
    }

    pub fn remove_session(&mut self, session_id: SessionId) {
        self.0.edit_senders.borrow_mut().remove(&session_id);
    }

    fn autoindent(
        &self,
        line_ranges: &[Range<usize>],
        indent_column_count: usize,
        edits: &mut Vec<Edit>,
    ) {
        fn next_line_indent_column_count(line: &str, tab_column_count: usize) -> Option<usize> {
            if let Some(indent) = line.indent() {
                let mut indent_column_count = indent.column_count();
                if line
                    .chars()
                    .rev()
                    .find_map(|char| {
                        if char.is_opening_delimiter() {
                            return Some(true);
                        }
                        if char.is_closing_delimiter() {
                            return Some(false);
                        }
                        None
                    })
                    .unwrap_or(false)
                {
                    indent_column_count += tab_column_count;
                };
                Some(indent_column_count)
            } else {
                None
            }
        }

        for line_range in line_ranges
            .iter()
            .cloned()
            .merge(|line_range_0, line_range_1| {
                if line_range_0.end >= line_range_1.start {
                    Ok(line_range_0.start..line_range_1.end)
                } else {
                    Err((line_range_0, line_range_1))
                }
            })
        {
            let mut desired_indentation_column_count = self.as_text().as_lines()
                [..line_range.start]
                .iter()
                .rev()
                .find_map(|line| next_line_indent_column_count(line, indent_column_count))
                .unwrap_or(0);
            for line in line_range {
                if self.as_text().as_lines()[line]
                    .chars()
                    .find_map(|char| {
                        if char.is_closing_delimiter() {
                            return Some(true);
                        }
                        if !char.is_whitespace() {
                            return Some(false);
                        }
                        None
                    })
                    .unwrap_or(false)
                {
                    desired_indentation_column_count -= 4;
                }
                self.edit_lines_internal(line, edits, |line| {
                    crate::session::reindent(line, |_| desired_indentation_column_count)
                });
                if let Some(next_line_indentation_column_count) = next_line_indent_column_count(
                    &self.as_text().as_lines()[line],
                    indent_column_count,
                ) {
                    desired_indentation_column_count = next_line_indentation_column_count;
                }
            }
        }
    }

    fn edit_lines_internal(
        &self,
        line: usize,
        edits: &mut Vec<Edit>,
        mut f: impl FnMut(&str) -> (usize, usize, String),
    ) {
        let mut history = self.0.history.borrow_mut();
        let (byte, delete_byte_count, insert_text) = f(&history.as_text().as_lines()[line]);
        if delete_byte_count > 0 {
            let edit = Edit {
                change: Change::Delete(
                    Position {
                        line_index: line,
                        byte_index: byte,
                    },
                    Length {
                        line_count: 0,
                        byte_count: delete_byte_count,
                    },
                ),
                drift: Drift::Before,
            };
            edits.push(edit.clone());
            history.apply_edit(edit);
        }
        if !insert_text.is_empty() {
            let edit = Edit {
                change: Change::Insert(
                    Position {
                        line_index: line,
                        byte_index: byte,
                    },
                    insert_text.into(),
                ),
                drift: Drift::Before,
            };
            edits.push(edit.clone());
            history.apply_edit(edit);
        }
    }

    pub fn force_new_group(&self) {
        self.0.history.borrow_mut().force_new_group()
    }

    pub fn undo(&self, origin_id: SessionId, selections: &SelectionSet) -> bool {
        let mut changes = Vec::new();
        let selections = self.0.history.borrow_mut().undo(selections, &mut changes);
        if let Some(selections) = selections {
            self.update_after_edit(Some(origin_id), Some(selections), &changes);
            true
        } else {
            false
        }
    }

    pub fn redo(&self, origin_id: SessionId, selections: &SelectionSet) -> bool {
        let mut changes = Vec::new();
        let selections = self.0.history.borrow_mut().redo(selections, &mut changes);
        if let Some(selections) = selections {
            self.update_after_edit(Some(origin_id), Some(selections), &changes);
            true
        } else {
            false
        }
    }

    fn update_after_edit(
        &self,
        origin_id: Option<SessionId>,
        selections: Option<SelectionSet>,
        edits: &[Edit],
    ) {
        let mut layout = self.0.layout.borrow_mut();
        for edit in edits {
            match edit.change {
                Change::Insert(position, ref text) => {
                    layout.indent_state[position.line_index] = None;
                    let line_count = text.length().line_count;
                    if line_count > 0 {
                        let line_index = position.line_index + 1;
                        layout
                            .indent_state
                            .splice(line_index..line_index, (0..line_count).map(|_| None));
                    }
                }
                Change::Delete(start, length) => {
                    layout.indent_state[start.line_index] = None;
                    if length.line_count > 0 {
                        let line_start = start.line_index + 1;
                        let line_end = line_start + length.line_count;
                        layout.indent_state.drain(line_start..line_end);
                    }
                }
            }
        }
        drop(layout);
        for edit in edits {
            self.apply_change_to_tokens(&edit.change);
            self.apply_change_to_inline_inlays(&edit.change, edit.drift);
            self.apply_change_to_block_inlays(&edit.change);
            self.0.tokenizer.borrow_mut().apply_change(&edit.change);
        }
        self.update_indent_state();
        self.0.tokenizer.borrow_mut().update(
            self.0.history.borrow().as_text(),
            &mut self.0.layout.borrow_mut().tokens,
        );
        let mut decorations = self.0.decorations.borrow_mut();
        let mut semantic = self.0.semantic.borrow_mut();
        let mut snippet = self.0.snippet.borrow_mut();
        for edit in edits {
            decorations.apply_edit(edit);
            semantic.apply_edit(edit);
            snippet.apply_edit(edit);
        }
        drop((decorations, semantic, snippet));
        let mut gutter_marks = self.0.gutter_marks.borrow_mut();
        for edit in edits {
            gutter_marks.apply_edit(edit);
        }
        drop(gutter_marks);
        // An edit moves the names: the old places would light the wrong
        // text. The next answer, for the new version, brings them back.
        self.0.occurrences.borrow_mut().clear();
        // A message follows its line, as the squiggle under it does: lines
        // added or removed above move it, and it does not blink while typing.
        follow_edits(&mut self.0.line_messages.borrow_mut(), edits);
        for (&session_id, edit_sender) in &*self.0.edit_senders.borrow() {
            if Some(session_id) == origin_id {
                edit_sender
                    .send((selections.clone(), edits.to_vec()))
                    .unwrap();
            } else {
                edit_sender
                    .send((
                        None,
                        edits
                            .iter()
                            .cloned()
                            .map(|edit| Edit {
                                change: edit.change,
                                drift: Drift::Before,
                            })
                            .collect(),
                    ))
                    .unwrap();
            }
        }
    }

    fn apply_change_to_tokens(&self, change: &Change) {
        let mut layout = self.0.layout.borrow_mut();
        let tokens = &mut layout.tokens;
        match *change {
            Change::Insert(point, ref text) => {
                let mut byte = 0;
                let mut index = tokens[point.line_index]
                    .iter()
                    .position(|token| {
                        if byte + token.len > point.byte_index {
                            return true;
                        }
                        byte += token.len;
                        false
                    })
                    .unwrap_or(tokens[point.line_index].len());
                if byte != point.byte_index {
                    let token = tokens[point.line_index][index];
                    let mid = point.byte_index - byte;
                    tokens[point.line_index][index] = Token {
                        len: mid,
                        kind: token.kind,
                    };
                    index += 1;
                    tokens[point.line_index].insert(
                        index,
                        Token {
                            len: token.len - mid,
                            kind: token.kind,
                        },
                    );
                }
                if text.length().line_count == 0 {
                    tokens[point.line_index]
                        .splice(index..index, tokenize(text.as_lines().first().unwrap()));
                } else {
                    let mut new_tokens = (0..text.as_lines().len())
                        .map(|line| tokenize(&text.as_lines()[line]).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    new_tokens
                        .first_mut()
                        .unwrap()
                        .splice(..0, tokens[point.line_index][..index].iter().copied());
                    new_tokens
                        .last_mut()
                        .unwrap()
                        .splice(..0, tokens[point.line_index][index..].iter().copied());
                    tokens.splice(point.line_index..point.line_index + 1, new_tokens);
                }
            }
            Change::Delete(start, length) => {
                let end = start + length;
                let mut byte = 0;
                let mut start_token = tokens[start.line_index]
                    .iter()
                    .position(|token| {
                        if byte + token.len > start.byte_index {
                            return true;
                        }
                        byte += token.len;
                        false
                    })
                    .unwrap_or(tokens[start.line_index].len());
                if byte != start.byte_index {
                    let token = tokens[start.line_index][start_token];
                    let mid = start.byte_index - byte;
                    tokens[start.line_index][start_token] = Token {
                        len: mid,
                        kind: token.kind,
                    };
                    start_token += 1;
                    tokens[start.line_index].insert(
                        start_token,
                        Token {
                            len: token.len - mid,
                            kind: token.kind,
                        },
                    );
                }
                let mut byte = 0;
                let mut end_token = tokens[end.line_index]
                    .iter()
                    .position(|token| {
                        if byte + token.len > end.byte_index {
                            return true;
                        }
                        byte += token.len;
                        false
                    })
                    .unwrap_or(tokens[end.line_index].len());
                if byte != end.byte_index {
                    let token = tokens[end.line_index][end_token];
                    let mid = end.byte_index - byte;
                    tokens[end.line_index][end_token] = Token {
                        len: mid,
                        kind: token.kind,
                    };
                    end_token += 1;
                    tokens[end.line_index].insert(
                        end_token,
                        Token {
                            len: token.len - mid,
                            kind: token.kind,
                        },
                    );
                }
                if length.line_count == 0 {
                    tokens[start.line_index].drain(start_token..end_token);
                } else {
                    let mut new_tokens = tokens[start.line_index][..start_token].to_vec();
                    new_tokens.extend(tokens[end.line_index][end_token..].iter().copied());
                    tokens.splice(start.line_index..end.line_index + 1, iter::once(new_tokens));
                }
            }
        }
    }

    fn apply_change_to_inline_inlays(&self, change: &Change, drift: Drift) {
        let mut layout = self.0.layout.borrow_mut();
        let inline_inlays = &mut layout.inline_inlays;
        match *change {
            Change::Insert(point, ref text) => {
                let index = inline_inlays[point.line_index]
                    .iter()
                    .position(|(byte, _)| match byte.cmp(&point.byte_index) {
                        Ordering::Less => false,
                        Ordering::Equal => match drift {
                            Drift::Before => true,
                            Drift::After => false,
                        },
                        Ordering::Greater => true,
                    })
                    .unwrap_or(inline_inlays[point.line_index].len());
                if text.length().line_count == 0 {
                    for (byte, _) in &mut inline_inlays[point.line_index][index..] {
                        *byte += text.length().byte_count;
                    }
                } else {
                    let mut new_inline_inlays = (0..text.as_lines().len())
                        .map(|_| Vec::new())
                        .collect::<Vec<_>>();
                    new_inline_inlays
                        .first_mut()
                        .unwrap()
                        .splice(..0, inline_inlays[point.line_index].drain(..index));
                    new_inline_inlays.last_mut().unwrap().splice(
                        ..0,
                        inline_inlays[point.line_index]
                            .drain(..)
                            .map(|(byte, inline_inlay)| {
                                (byte + text.length().byte_count, inline_inlay)
                            }),
                    );
                    inline_inlays.splice(point.line_index..point.line_index + 1, new_inline_inlays);
                }
            }
            Change::Delete(start, length) => {
                let end = start + length;
                let start_inlay = inline_inlays[start.line_index]
                    .iter()
                    .position(|&(byte, _)| byte >= start.byte_index)
                    .unwrap_or(inline_inlays[start.line_index].len());
                let end_inlay = inline_inlays[end.line_index]
                    .iter()
                    .position(|&(byte, _)| byte >= end.byte_index)
                    .unwrap_or(inline_inlays[end.line_index].len());
                if length.line_count == 0 {
                    inline_inlays[start.line_index].drain(start_inlay..end_inlay);
                    for (byte, _) in &mut inline_inlays[start.line_index][start_inlay..] {
                        *byte = start.byte_index + (*byte - end.byte_index.min(*byte));
                    }
                } else {
                    let mut new_inline_inlays = inline_inlays[start.line_index]
                        .drain(..start_inlay)
                        .collect::<Vec<_>>();
                    new_inline_inlays.extend(inline_inlays[end.line_index].drain(end_inlay..).map(
                        |(byte, inline_inlay)| {
                            (
                                start.byte_index + byte - end.byte_index.min(byte),
                                inline_inlay,
                            )
                        },
                    ));
                    inline_inlays.splice(
                        start.line_index..end.line_index + 1,
                        iter::once(new_inline_inlays),
                    );
                }
            }
        }
    }

    fn apply_change_to_block_inlays(&self, change: &Change) {
        let mut layout = self.0.layout.borrow_mut();
        match *change {
            Change::Insert(point, ref text) => {
                let line_count = text.length().line_count;
                if line_count > 0 {
                    for (line, _) in &mut layout.block_inlays {
                        if point.line_index < *line
                            || (point.line_index == *line && point.byte_index == 0)
                        {
                            *line += line_count;
                        }
                    }
                }
            }
            Change::Delete(start, length) => {
                let line_count = length.line_count;
                if line_count > 0 {
                    let start_line = start.line_index;
                    let end_line = start_line + line_count;
                    layout.block_inlays.retain_mut(|(line, _)| {
                        if *line < start_line {
                            true
                        } else if *line > end_line {
                            *line -= line_count;
                            true
                        } else if *line == end_line {
                            if start.byte_index == 0 {
                                *line -= line_count;
                                true
                            } else {
                                false
                            }
                        } else if *line == start_line {
                            start.byte_index > 0
                        } else {
                            false
                        }
                    });
                }
            }
        }
    }

    fn update_indent_state(&self) {
        let mut layout = self.0.layout.borrow_mut();
        let indent_state = &mut layout.indent_state;
        let history = self.0.history.borrow();
        let lines = history.as_text().as_lines();
        let mut current_indent_column_count = 0;
        for line_index in 0..lines.len() {
            match indent_state[line_index] {
                Some(IndentState::NonEmpty(_, next_indent_column_count)) => {
                    current_indent_column_count = next_indent_column_count;
                }
                _ => {
                    indent_state[line_index] = Some(match lines[line_index].indent() {
                        Some(indent) => {
                            let indent_column_count = indent.column_count();
                            let mut next_indent_column_count = indent_column_count;
                            if lines[line_index]
                                .chars()
                                .rev()
                                .find_map(|char| {
                                    if char.is_opening_delimiter() {
                                        return Some(true);
                                    }
                                    if char.is_closing_delimiter() {
                                        return Some(false);
                                    }
                                    None
                                })
                                .unwrap_or(false)
                            {
                                next_indent_column_count += 4;
                            }
                            current_indent_column_count = next_indent_column_count;
                            IndentState::NonEmpty(indent_column_count, next_indent_column_count)
                        }
                        None => IndentState::Empty(current_indent_column_count),
                    })
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DocumentLayout {
    pub indent_state: Vec<Option<IndentState>>,
    pub tokens: Vec<Vec<Token>>,
    pub inline_inlays: Vec<Vec<(usize, InlineInlay)>>,
    pub block_inlays: Vec<(usize, BlockInlay)>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IndentState {
    Empty(usize),
    NonEmpty(usize, usize),
}

#[derive(Debug)]
pub struct Editor<'a> {
    history: &'a mut History,
    edits: &'a mut Vec<Edit>,
}

impl<'a> Editor<'a> {
    pub fn as_text(&mut self) -> &Text {
        self.history.as_text()
    }

    pub fn apply_edit(&mut self, edit: Edit) {
        self.history.apply_edit(edit.clone());
        self.edits.push(edit);
    }
}

/// Moves each message with its line through `edits` (the gutter's rule), and
/// keeps one per line when two land on the same one.
pub(crate) fn follow_edits(messages: &mut Vec<LineMessage>, edits: &[Edit]) {
    for m in messages.iter_mut() {
        for edit in edits {
            m.line = crate::gutter_mark::shift_line(m.line, edit);
        }
    }
    let mut seen = Vec::new();
    messages.retain(|m| {
        let first = !seen.contains(&m.line);
        seen.push(m.line);
        first
    });
}

pub use crate::line_change::LineChange;
pub use crate::review::{ReviewRow, ReviewRowKind, ReviewView, review_view};

/// A diagnostic's message on a line: 0 an error, 1 a warning, 2 the rest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineMessage {
    pub line: usize,
    pub text: String,
    pub severity: u8,
}

/// One lit occurrence: a name, on one line, read or written there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Occurrence {
    pub start: Position,
    pub end: Position,
    pub write: bool,
}

#[derive(Debug)]
struct DocumentInner {
    history: RefCell<History>,
    layout: RefCell<DocumentLayout>,
    tokenizer: RefCell<Tokenizer>,
    decorations: RefCell<DecorationSet>,
    /// The language server's classified spans, over the tokenizer.
    semantic: RefCell<crate::semantic::SemanticSet>,
    /// The stops of the snippet being filled.
    snippet: RefCell<crate::snippet::SnippetStops>,
    gutter_marks: RefCell<GutterMarkSet>,
    /// The occurrences of the name under the caret, lit behind the text.
    occurrences: RefCell<Vec<Occurrence>>,
    /// The diagnostics' messages written after the end of their lines.
    line_messages: RefCell<Vec<LineMessage>>,
    /// The change bars: lines changed in this session, saved or not.
    line_changes: RefCell<Vec<(usize, LineChange)>>,
    /// The diff review rows for inline change display.
    review_rows: RefCell<Vec<crate::review::ReviewRow>>,
    edit_senders: RefCell<HashMap<SessionId, Sender<SessionEdit>>>,
}

/// What a document sends each of its sessions after an edit: the selections
/// to take, if any, and the edits.
type SessionEdit = (Option<SelectionSet>, Vec<Edit>);

fn tokenize(text: &str) -> impl Iterator<Item = Token> + '_ {
    text.split_whitespace_boundaries().map(|string| Token {
        len: string.len(),
        kind: if string.chars().next().unwrap().is_whitespace() {
            TokenKind::Whitespace
        } else {
            TokenKind::Unknown
        },
    })
}

#[cfg(test)]
mod inlay_tests {
    use super::CodeDocument;
    use crate::{decoration::DecorationSet, inlays::InlineInlay, text::Text};

    fn inlays(doc: &CodeDocument, line: usize) -> Vec<(usize, InlineInlay)> {
        doc.layout().inline_inlays[line].clone()
    }

    #[test]
    fn type_hints_and_debugger_values_share_a_line_and_replace_only_their_own() {
        let doc = CodeDocument::new(Text::from("let total = 3;\nx;"), DecorationSet::new());
        assert_eq!(doc.set_line_end_texts(&[(0, "total = 3".into())]), vec![0]);
        assert_eq!(doc.set_inline_hints(&[(0, 9, ": usize".into())]), vec![0]);
        assert_eq!(
            inlays(&doc, 0),
            vec![
                (9, InlineInlay::Hint(": usize".into())),
                (14, InlineInlay::Text("total = 3".into())),
            ],
            "the hint did not wipe the value, and both are in byte order"
        );
        assert_eq!(doc.set_line_end_texts(&[]), vec![0]);
        assert_eq!(
            inlays(&doc, 0),
            vec![(9, InlineInlay::Hint(": usize".into()))]
        );
        assert!(
            doc.set_inline_hints(&[(0, 9, ": usize".into())]).is_empty(),
            "same hints: nothing to redo"
        );
        assert_eq!(doc.set_inline_hints(&[(1, 99, ": i32".into())]), vec![0, 1]);
        assert_eq!(
            inlays(&doc, 1),
            vec![(2, InlineInlay::Hint(": i32".into()))],
            "clamped to the line's end"
        );
    }
}

#[cfg(test)]
#[path = "test/document/test.rs"]
mod occurrence_tests;
