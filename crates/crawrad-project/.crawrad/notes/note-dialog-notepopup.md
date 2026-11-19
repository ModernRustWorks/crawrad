---
id: note-dialog-notepopup
kind: doc
target_file: src/note_dialog.rs
line_hint: 68
author: 
created: 
title: "NotePopup"
tags: []
signature: "pub struct NotePopup"
---

The state of the note popup shown under the caret line.

# Examples

```rust
use crawrad_project::note_dialog::note_popup;

let popup = note_popup("fn test() {}", 0);
assert!(popup.rows.len() > 0);
```
