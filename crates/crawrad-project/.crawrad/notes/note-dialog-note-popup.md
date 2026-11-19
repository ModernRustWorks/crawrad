---
id: note-dialog-note-popup
kind: doc
target_file: src/note_dialog.rs
line_hint: 75
author: 
created: 
title: "note_popup"
tags: []
signature: "pub fn note_popup (source : & str , line : usize) -> NotePopup"
---

Decides the note popup header and rows for a given line in source.

# Examples

```rust
use crawrad_project::note_dialog::note_popup;

let popup = note_popup("fn test() {}", 0);
assert!(popup.header.contains("Line 1"));
```
