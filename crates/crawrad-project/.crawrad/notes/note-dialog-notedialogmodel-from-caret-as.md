---
id: note-dialog-notedialogmodel-from-caret-as
kind: doc
target_file: src/note_dialog.rs
line_hint: 362
author: 
created: 
title: "NoteDialogModel::from_caret_as"
tags: []
signature: "pub fn from_caret_as (source : & str , line : usize , file : & str , project_root : & Path , kind : NoteKind ,) -> CursorOpen"
---

Opens or creates a note model for a specific chosen note kind.

Like `from_caret` but the `kind` is explicitly chosen by the user (or caller):
- `Doc`: loads or creates doc note for item at line;
- `Internal`: loads existing `[crawrad:note#id]` on the line, or creates a new note anchor;
- `Comment`: loads existing `[crawrad:comment#id]` on the line, or creates a new comment anchor.

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;
use crawrad_project::sidecar_notes::NoteKind;

let open = NoteDialogModel::from_caret_as("fn test() {}", 0, "src/lib.rs", Path::new("."), NoteKind::Doc);
assert_eq!(open.model.kind(), NoteKind::Doc);
```
