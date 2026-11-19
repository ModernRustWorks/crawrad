---
id: note-dialog-notedialogmodel-insert-link
kind: doc
target_file: src/note_dialog.rs
line_hint: 628
author: 
created: 
title: "NoteDialogModel::insert_link"
tags: []
signature: "pub fn insert_link (& mut self , at : usize , link : & str)"
---

Inserts a markdown link into note content at the given byte offset.

Writes `link` at byte `at` of the content (Link item…, #659); a byte
past the end or inside a character lands before it.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.insert_link(0, "[link]");
assert_eq!(model.content, "[link]");
```
