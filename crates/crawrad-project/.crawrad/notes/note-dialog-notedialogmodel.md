---
id: note-dialog-notedialogmodel
kind: doc
target_file: src/note_dialog.rs
line_hint: 16
author: 
created: 
title: "NoteDialogModel"
tags: []
signature: "pub struct NoteDialogModel"
---

The headless state of the note dialog.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.file, "src/lib.rs");
```
