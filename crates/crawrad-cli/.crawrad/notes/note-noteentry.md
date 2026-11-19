---
id: note-noteentry
kind: doc
target_file: src/note.rs
line_hint: 29
author: 
created: 
title: "NoteEntry"
tags: []
signature: "pub struct NoteEntry"
---

Summary of an existing note file and its anchor status.

# Examples

```rust
use crawrad_cli::note::NoteEntry;
use crawrad_note_format::NoteKind;

let entry = NoteEntry {
    id: "button_click".to_string(),
    kind: NoteKind::Doc,
    anchor: Some("src/button.rs:42".to_string()),
};
assert_eq!(entry.id, "button_click");
```
