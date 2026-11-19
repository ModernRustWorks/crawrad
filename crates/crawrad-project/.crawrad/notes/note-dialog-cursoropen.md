---
id: note-dialog-cursoropen
kind: doc
target_file: src/note_dialog.rs
line_hint: 332
author: 
created: 
title: "CursorOpen"
tags: []
signature: "pub struct CursorOpen"
---

The result of opening a note from the editor cursor position.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let open = NoteDialogModel::from_caret("fn test() {}", 0, "src/lib.rs", Path::new("."));
assert_eq!(open.model.file, "src/lib.rs");
```
