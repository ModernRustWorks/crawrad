---
id: note-dialog-notedialogmodel-set-kind
kind: doc
target_file: src/note_dialog.rs
line_hint: 601
author: 
created: 
title: "NoteDialogModel::set_kind"
tags: []
signature: "pub fn set_kind (& mut self , kind : NoteKind)"
---

Sets the note kind.

Sets the note kind. Doc is refused (no-op + status set) when target is None.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::sidecar_notes::NoteKind;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.set_kind(NoteKind::Comment);
assert_eq!(model.kind(), NoteKind::Comment);
```
