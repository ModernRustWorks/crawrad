---
id: note-dialog-notedialogmodel-from-note
kind: doc
target_file: src/note_dialog.rs
line_hint: 555
author: 
created: 
title: "NoteDialogModel::from_note"
tags: []
signature: "pub fn from_note (note : SidecarNote , project_root : & Path) -> Self"
---

Creates a model from an existing note (open_note).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::default();
let model = NoteDialogModel::from_note(note, Path::new("."));
assert!(model.status.is_none());
```
