---
id: note-dialog-notedialogmodel-typed
kind: doc
target_file: src/note_dialog.rs
line_hint: 889
author: 
created: 
title: "NoteDialogModel::typed"
tags: []
signature: "pub fn typed (& mut self , text : String)"
---

Updates note content from editor text when on the edit tab.

Takes the editor's text, which is the content only while the Edit tab
shows it (on Preview the editor is hidden and the model holds the text).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.typed("hello".to_string());
assert_eq!(model.content, "hello");
```
