---
id: note-dialog-notedialogmodel-toggle-resolved
kind: doc
target_file: src/note_dialog.rs
line_hint: 611
author: 
created: 
title: "NoteDialogModel::toggle_resolved"
tags: []
signature: "pub fn toggle_resolved (& mut self)"
---

Flips `note.resolved` for comment notes (no-op for other kinds).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::sidecar_notes::NoteKind;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.set_kind(NoteKind::Comment);
model.toggle_resolved();
assert!(model.note.resolved);
```
