---
id: note-dialog-noteview-icon
kind: doc
target_file: src/note_dialog.rs
line_hint: 257
author: 
created: 
title: "NoteView::icon"
tags: []
signature: "pub fn icon (& self) -> i64"
---

The kind's gutter icon in the header: 0 documentation, 1 note, 2 comment.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert!(view.icon() >= 0);
```
