---
id: notes-panel-notespanel-counts
kind: doc
target_file: src/notes_panel.rs
line_hint: 168
author: 
created: 
title: "NotesPanel::counts"
tags: []
signature: "pub fn counts (& self) -> Counts"
---

Computes overall counts over all notes, independent of active filters.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert_eq!(panel.counts().all, 0);
```
