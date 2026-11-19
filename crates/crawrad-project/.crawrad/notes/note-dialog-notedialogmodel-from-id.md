---
id: note-dialog-notedialogmodel-from-id
kind: doc
target_file: src/note_dialog.rs
line_hint: 582
author: 
created: 
title: "NoteDialogModel::from_id"
tags: []
signature: "pub fn from_id (id : & str , file : & str , project_root : & Path) -> Self"
---

Loads a note by id from project root or creates a fallback template.

Loads a note by id from project root, or creates a fallback template if missing (open_note_by_id).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.file, "src/lib.rs");
```
