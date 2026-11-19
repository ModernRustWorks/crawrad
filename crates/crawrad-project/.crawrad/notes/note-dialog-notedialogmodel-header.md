---
id: note-dialog-notedialogmodel-header
kind: doc
target_file: src/note_dialog.rs
line_hint: 637
author: 
created: 
title: "NoteDialogModel::header"
tags: []
signature: "pub fn header (& self) -> String"
---

Returns the dialog header title: `"Note — <id>"`.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.header(), "Note \u{2014} sample");
```
