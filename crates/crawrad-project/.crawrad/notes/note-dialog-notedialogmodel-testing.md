---
id: note-dialog-notedialogmodel-testing
kind: doc
target_file: src/note_dialog.rs
line_hint: 896
author: 
created: 
title: "NoteDialogModel::testing"
tags: []
signature: "pub fn testing (& mut self)"
---

A doctest run started.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.testing();
assert!(model.test_line.is_some());
```
