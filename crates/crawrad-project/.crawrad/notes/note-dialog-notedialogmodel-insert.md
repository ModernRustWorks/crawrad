---
id: note-dialog-notedialogmodel-insert
kind: doc
target_file: src/note_dialog.rs
line_hint: 623
author: 
created: 
title: "NoteDialogModel::insert"
tags: []
signature: "pub fn insert (& mut self , section : Section)"
---

Inserts a section template into content.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::doc_note::Section;

let mut model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
model.insert(Section::Errors);
assert!(model.content.contains("# Errors"));
```
