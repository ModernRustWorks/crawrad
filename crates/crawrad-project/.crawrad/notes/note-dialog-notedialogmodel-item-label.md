---
id: note-dialog-notedialogmodel-item-label
kind: doc
target_file: src/note_dialog.rs
line_hint: 642
author: 
created: 
title: "NoteDialogModel::item_label"
tags: []
signature: "pub fn item_label (& self) -> String"
---

Returns the item signature for Doc notes, or empty string for Internal notes.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.item_label(), "");
```
