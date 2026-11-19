---
id: note-dialog-noteview
kind: doc
target_file: src/note_dialog.rs
line_hint: 219
author: 
created: 
title: "NoteView"
tags: []
signature: "pub struct NoteView"
---

Everything the Note dialog shows, decided here so the shell only paints.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert!(!view.title_text.is_empty());
```
