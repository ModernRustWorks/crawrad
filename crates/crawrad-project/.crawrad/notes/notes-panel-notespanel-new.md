---
id: notes-panel-notespanel-new
kind: doc
target_file: src/notes_panel.rs
line_hint: 144
author: 
created: 
title: "NotesPanel::new"
tags: []
signature: "pub fn new (notes : Vec < IndexedNote >) -> Self"
---

Creates a new panel state with the given index.

Creates a new panel state with the given index; all filters off and nothing collapsed.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::new(Vec::new());
assert_eq!(panel.counts().all, 0);
```
