---
id: note-dialog-notedialogmodel-footer
kind: doc
target_file: src/note_dialog.rs
line_hint: 725
author: 
created: 
title: "NoteDialogModel::footer"
tags: []
signature: "pub fn footer (& self) -> String"
---

Returns the footer text for the dialog.

Returns the footer text: Doc note attribute write info or internal note file path.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert!(model.footer().contains(".crawrad/notes"));
```
