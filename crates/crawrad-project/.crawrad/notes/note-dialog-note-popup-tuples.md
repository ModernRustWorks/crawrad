---
id: note-dialog-note-popup-tuples
kind: doc
target_file: src/note_dialog.rs
line_hint: 201
author: 
created: 
title: "note_popup_tuples"
tags: []
signature: "pub fn note_popup_tuples (source : & str , line : usize) -> (String , Vec < PopupRowParts >)"
---

Helper that converts note_popup into plain tuples for the code view API.

# Examples

```rust
use crawrad_project::note_dialog::note_popup_tuples;

let (header, rows) = note_popup_tuples("fn test() {}", 0);
assert_eq!(rows.len(), 3);
```
