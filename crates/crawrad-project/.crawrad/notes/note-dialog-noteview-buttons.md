---
id: note-dialog-noteview-buttons
kind: doc
target_file: src/note_dialog.rs
line_hint: 299
author: 
created: 
title: "NoteView::buttons"
tags: []
signature: "pub fn buttons (& self) -> [(& 'static str , Option < & str >) ; 5]"
---

Buttons whose text the shell sets for the note dialog.

Every button whose text the shell sets: control name and its text,
`None` hiding it (the DSL writes no action words).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let model = NoteDialogModel::from_id("sample", "src/lib.rs", Path::new("."));
let view = model.view("", false);
assert_eq!(view.buttons().len(), 5);
```
