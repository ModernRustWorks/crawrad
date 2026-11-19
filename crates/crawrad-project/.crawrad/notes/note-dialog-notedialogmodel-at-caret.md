---
id: note-dialog-notedialogmodel-at-caret
kind: doc
target_file: src/note_dialog.rs
line_hint: 170
author: 
created: 
title: "NoteDialogModel::at_caret"
tags: []
signature: "pub fn at_caret (source : & str , line : usize , file : & str , project_root : & Path , choice : Option < crawrad_actions :: NoteKindChoice > ,) -> NoteOpen"
---

Opens either the note popup or a specific note dialog at caret.

The popup while `choice` is `None` (popup row or shortcut not taken
yet) or is documentation off an item, else the dialog of that kind.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::{NoteDialogModel, NoteOpen};

let open = NoteDialogModel::at_caret("fn foo() {}", 0, "src/lib.rs", Path::new("."), None);
assert!(matches!(open, NoteOpen::Popup(..)));
```
