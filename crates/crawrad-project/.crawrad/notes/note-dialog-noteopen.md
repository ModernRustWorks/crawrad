---
id: note-dialog-noteopen
kind: doc
target_file: src/note_dialog.rs
line_hint: 161
author: 
created: 
title: "NoteOpen"
tags: []
signature: "pub enum NoteOpen"
---

What the IDE opens for the note at the caret.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::{NoteDialogModel, NoteOpen};

let open = NoteDialogModel::at_caret("fn test() {}", 0, "src/lib.rs", Path::new("."), None);
assert!(matches!(open, NoteOpen::Popup(..)));
```
