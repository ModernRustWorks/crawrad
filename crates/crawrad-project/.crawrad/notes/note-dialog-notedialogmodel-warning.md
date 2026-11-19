---
id: note-dialog-notedialogmodel-warning
kind: doc
target_file: src/note_dialog.rs
line_hint: 703
author: 
created: 
title: "NoteDialogModel::warning"
tags: []
signature: "pub fn warning (& self , source : & str) -> Option < String >"
---

Returns warning text for missing sections or summary issues.

Returns the warning text for Doc notes: first missing section or summary warning.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.warning(""), None);
```
