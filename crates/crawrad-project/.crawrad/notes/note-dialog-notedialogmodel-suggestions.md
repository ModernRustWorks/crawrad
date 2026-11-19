---
id: note-dialog-notedialogmodel-suggestions
kind: doc
target_file: src/note_dialog.rs
line_hint: 671
author: 
created: 
title: "NoteDialogModel::suggestions"
tags: []
signature: "pub fn suggestions (& self , source : & str) -> Vec < Suggestion >"
---

Returns suggested sections based on signature analysis.

Returns suggested sections from signature and body analysis (empty for Internal notes).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert!(model.suggestions("").is_empty());
```
