---
id: note-dialog-notedialogmodel-view
kind: doc
target_file: src/note_dialog.rs
line_hint: 768
author: 
created: 
title: "NoteDialogModel::view"
tags: []
signature: "pub fn view (& self , source : & str , testing : bool) -> NoteView"
---

Everything the dialog shows.

`testing` is whether a doctest run is going.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert!(!view.title_text.is_empty());
```
