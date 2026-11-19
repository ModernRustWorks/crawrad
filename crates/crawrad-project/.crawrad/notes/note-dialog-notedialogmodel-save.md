---
id: note-dialog-notedialogmodel-save
kind: doc
target_file: src/note_dialog.rs
line_hint: 906
author: 
created: 
title: "NoteDialogModel::save"
tags: []
signature: "pub fn save (& mut self , source : & str) -> Result < Option < String > , String >"
---

Saves note to disk and returns updated source text if changed.

Saves the note to disk and returns the updated source text if it needs to change.

# Errors

Returns an error message if saving the note file fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let res = model.save("");
assert!(res.is_ok());
```
