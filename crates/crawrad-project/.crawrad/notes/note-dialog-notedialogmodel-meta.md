---
id: note-dialog-notedialogmodel-meta
kind: doc
target_file: src/note_dialog.rs
line_hint: 654
author: 
created: 
title: "NoteDialogModel::meta"
tags: []
signature: "pub fn meta (& self) -> String"
---

Returns metadata string for this note.

Returns `"<file>:<line>"` for Doc notes, or today's full metadata for Internal notes.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert!(model.meta().contains("Target:"));
```
