---
id: note-dialog-noteview-labels
kind: doc
target_file: src/note_dialog.rs
line_hint: 266
author: 
created: 
title: "NoteView::labels"
tags: []
signature: "pub fn labels (& self) -> Vec < LabelPaint < '_ > >"
---

Every label of the dialog, in the order the shell paints them.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert!(!view.labels().is_empty());
```
