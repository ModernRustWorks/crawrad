---
id: note-dialog-notedialogmodel-tested
kind: doc
target_file: src/note_dialog.rs
line_hint: 901
author: 
created: 
title: "NoteDialogModel::tested"
tags: []
signature: "pub fn tested (& mut self , outcome : & crate :: note_preview :: DoctestOutcome)"
---

A doctest run answered.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::note_preview::DoctestOutcome;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.tested(&DoctestOutcome::Passed { count: 1, secs: 0.1 });
assert!(model.test_line.is_some());
```
