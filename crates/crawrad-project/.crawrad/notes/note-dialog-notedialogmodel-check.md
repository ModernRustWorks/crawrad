---
id: note-dialog-notedialogmodel-check
kind: doc
target_file: src/note_dialog.rs
line_hint: 741
author: 
created: 
title: "NoteDialogModel::check"
tags: []
signature: "pub fn check (& self , source : & str) -> Option < String >"
---

Confirmation message when all required sections are present.

The line that replaces the warning when nothing is missing: the
summary's length and that every suggested section is there.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.check(""), None);
```
