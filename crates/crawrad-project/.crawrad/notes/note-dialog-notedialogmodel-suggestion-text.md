---
id: note-dialog-notedialogmodel-suggestion-text
kind: doc
target_file: src/note_dialog.rs
line_hint: 681
author: 
created: 
title: "NoteDialogModel::suggestion_text"
tags: []
signature: "pub fn suggestion_text (& self , source : & str) -> String"
---

Returns the formatted suggestion summary text, or empty string when none.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.suggestion_text(""), "");
```
