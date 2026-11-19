---
id: notes-panel-notespanel-note-place
kind: doc
target_file: src/notes_panel.rs
line_hint: 470
author: 
created: 
title: "NotesPanel::note_place"
tags: []
signature: "pub fn note_place (& self , index : usize) -> Option < (String , usize) >"
---

Returns the anchored location of the note at row `index` of `rows()`.

Where the note at row `index` of `rows()` is anchored: its file and
0-based line; `None` for a group or an unanchored note.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert!(panel.note_place(0).is_none());
```
