---
id: note-dialog-notedialogmodel-preview
kind: doc
target_file: src/note_dialog.rs
line_hint: 736
author: 
created: 
title: "NoteDialogModel::preview"
tags: []
signature: "pub fn preview (& self) -> String"
---

Markdown content for the preview tab.

The markdown of the Preview tab: what `cargo doc` shows for the item.
An internal note has no item, so it previews the body alone.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
assert_eq!(model.preview(), "");
```
