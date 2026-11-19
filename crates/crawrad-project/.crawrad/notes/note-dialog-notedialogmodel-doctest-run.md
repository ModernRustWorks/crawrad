---
id: note-dialog-notedialogmodel-doctest-run
kind: doc
target_file: src/note_dialog.rs
line_hint: 753
author: 
created: 
title: "NoteDialogModel::doctest_run"
tags: []
signature: "pub fn doctest_run (& self) -> Option < crate :: note_preview :: DoctestRun >"
---

Constructs a doctest runner for testing examples in doc notes.

The doctest run of Test example, or `None` when there is nothing to
run: an internal note, no item, no code block, or no crate around it.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert!(model.doctest_run().is_none());
```
