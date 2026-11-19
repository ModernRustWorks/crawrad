---
id: notes-panel-notespanel-toggle-row
kind: doc
target_file: src/notes_panel.rs
line_hint: 463
author: 
created: 
title: "NotesPanel::toggle_row"
tags: []
signature: "pub fn toggle_row (& mut self , index : usize)"
---

Collapses or expands the group at row `index` of `rows()`.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let mut panel = NotesPanel::default();
panel.toggle_row(0);
```
