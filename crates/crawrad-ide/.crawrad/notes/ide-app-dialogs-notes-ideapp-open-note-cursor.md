---
id: ide-app-dialogs-notes-ideapp-open-note-cursor
kind: doc
target_file: src/ide_app/dialogs/notes.rs
line_hint: 281
author: 
created: 
title: "IdeApp::open_note_cursor"
tags: []
signature: "pub (in super :: super) fn open_note_cursor (& mut self , cx : & mut Cx , kind : Option < NoteKindChoice >)"
---

Opens the note popup or dialog at the editor caret.

The note at the caret: the popup while no kind is chosen, else that
kind's dialog (its anchor written into the code when it is new).
