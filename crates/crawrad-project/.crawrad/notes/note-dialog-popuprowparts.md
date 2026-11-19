---
id: note-dialog-popuprowparts
kind: doc
target_file: src/note_dialog.rs
line_hint: 150
author: 
created: 
title: "PopupRowParts"
tags: []
signature: "pub type PopupRowParts = (u8 , String , String , & 'static str , bool)"
---

Tuple representing a note popup row for the code view.

A popup row as the code view takes it: kind (0 doc, 1 note, 2 comment),
title, description, shortcut and whether it can be chosen.

# Examples

```rust
use crawrad_project::note_dialog::PopupRowParts;

let row: PopupRowParts = (0, "Title".into(), "Desc".into(), "Ctrl+D", true);
assert_eq!(row.0, 0);
```
