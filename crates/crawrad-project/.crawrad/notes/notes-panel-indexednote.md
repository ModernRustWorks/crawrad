---
id: notes-panel-indexednote
kind: doc
target_file: src/notes_panel.rs
line_hint: 23
author: 
created: 
title: "IndexedNote"
tags: []
signature: "pub struct IndexedNote"
---

One note found in the project, with where its anchor is.

# Examples

```rust
use crawrad_project::notes_panel::IndexedNote;
use crawrad_project::sidecar_notes::NoteKind;

let note = IndexedNote {
    id: "n1".into(),
    kind: NoteKind::Doc,
    title: "Foo".into(),
    file: Some("src/lib.rs".into()),
    line: Some(10),
    resolved: false,
    alert: None,
};
assert_eq!(note.id, "n1");
```
