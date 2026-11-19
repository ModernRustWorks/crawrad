---
id: notes-panel-notespanel-row-target
kind: doc
target_file: src/notes_panel.rs
line_hint: 450
author: 
created: 
title: "NotesPanel::row_target"
tags: []
signature: "pub fn row_target (& self , index : usize) -> Option < RowTarget >"
---

Resolves what row `index` in `rows()` leads to.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert!(panel.row_target(0).is_none());
```
