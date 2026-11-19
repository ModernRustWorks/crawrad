---
id: notes-panel-notespanel-view-rows
kind: doc
target_file: src/notes_panel.rs
line_hint: 346
author: 
created: 
title: "NotesPanel::view_rows"
tags: []
signature: "pub fn view_rows (& self) -> Vec < ViewRowParts >"
---

Returns the rows as the Notes panel view paints them.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert!(panel.view_rows().is_empty());
```
