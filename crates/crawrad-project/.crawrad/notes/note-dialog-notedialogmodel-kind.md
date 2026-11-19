---
id: note-dialog-notedialogmodel-kind
kind: doc
target_file: src/note_dialog.rs
line_hint: 618
author: 
created: 
title: "NoteDialogModel::kind"
tags: []
signature: "pub fn kind (& self) -> NoteKind"
---

Returns the current note kind.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::sidecar_notes::NoteKind;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.kind(), NoteKind::Internal);
```
