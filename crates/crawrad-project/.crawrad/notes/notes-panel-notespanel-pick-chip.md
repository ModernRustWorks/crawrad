---
id: notes-panel-notespanel-pick-chip
kind: doc
target_file: src/notes_panel.rs
line_hint: 425
author: 
created: 
title: "NotesPanel::pick_chip"
tags: []
signature: "pub fn pick_chip (& mut self , index : usize)"
---

Selects a filter chip by index (0 All, 1 Doc, 2 Internal, 3 Comment).

# Examples

```rust
use crawrad_project::notes_panel::{KindFilter, NotesPanel};

let mut panel = NotesPanel::default();
panel.pick_chip(1);
assert_eq!(panel.kind, KindFilter::Doc);
```
