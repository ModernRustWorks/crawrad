---
id: notes-panel-notespanel-set-notes
kind: doc
target_file: src/notes_panel.rs
line_hint: 155
author: 
created: 
title: "NotesPanel::set_notes"
tags: []
signature: "pub fn set_notes (& mut self , notes : Vec < IndexedNote >)"
---

Updates the panel's notes, preserving active filters and collapsing state.

Updates the panel's notes, preserving active filters and collapsing state.
Drops a file filter if the selected file no longer contains any notes.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let mut panel = NotesPanel::default();
panel.set_notes(Vec::new());
assert_eq!(panel.counts().all, 0);
```
