---
id: note-dialog-notedialogmodel-from-caret
kind: doc
target_file: src/note_dialog.rs
line_hint: 340
author: 
created: 
title: "NoteDialogModel::from_caret"
tags: []
signature: "pub fn from_caret (source : & str , line : usize , file : & str , project_root : & Path) -> CursorOpen"
---

Opens or creates a note model based on editor caret position.

From the caret: `source` = editor text, `line` = 0-based caret line, `file` = active file path,
`project_root` = project root. Decides everything `open_note_cursor` decides today:
- caret on an item that already has `#[note("id")]` -> load that doc note from crate_dir (or a doc template if missing);
- caret on an item without a note -> NEW Documentation note: id = suggested_note_id, kind Doc, content = target.existing_docs (the `///` text moves in), title = `"<owner>::<name>"` or name;
- caret on a line with a `// [crawrad:note#id]` anchor -> load that internal note (as today);
- otherwise -> NEW internal note (id = sidecar_notes::generate_note_id(file)), and `CursorOpen::insert_anchor` tells the IDE to add the anchor (as today).

# Examples

```rust
use std::path::Path;
use crawrad_project::note_dialog::NoteDialogModel;

let open = NoteDialogModel::from_caret("fn test() {}", 0, "src/lib.rs", Path::new("."));
assert_eq!(open.model.file, "src/lib.rs");
```
