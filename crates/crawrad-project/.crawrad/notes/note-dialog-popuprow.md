---
id: note-dialog-popuprow
kind: doc
target_file: src/note_dialog.rs
line_hint: 58
author: 
created: 
title: "PopupRow"
tags: []
signature: "pub struct PopupRow"
---

One row of the note popup.

# Examples

```rust
use crawrad_project::note_dialog::PopupRow;
use crawrad_project::sidecar_notes::NoteKind;

let row = PopupRow {
    kind: NoteKind::Doc,
    title: "Doc".to_string(),
    description: "Documentation".to_string(),
    shortcut: "Ctrl+Alt+D",
    enabled: true,
};
assert_eq!(row.kind, NoteKind::Doc);
```
