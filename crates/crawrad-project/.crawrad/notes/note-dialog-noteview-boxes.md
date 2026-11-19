---
id: note-dialog-noteview-boxes
kind: doc
target_file: src/note_dialog.rs
line_hint: 310
author: 
created: 
title: "NoteView::boxes"
tags: []
signature: "pub fn boxes (& self) -> [(& 'static str , bool) ; 6]"
---

The boxes that show only for some kinds.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert_eq!(view.boxes().len(), 6);
```
