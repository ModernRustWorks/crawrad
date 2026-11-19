---
id: notes-panel-notespanel-rows
kind: doc
target_file: src/notes_panel.rs
line_hint: 207
author: 
created: 
title: "NotesPanel::rows"
tags: []
signature: "pub fn rows (& self) -> Vec < PanelRow >"
---

Generates the list of rows to display according to filters and state.

Generates the list of rows to display according to current filters and collapse states.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert!(panel.rows().is_empty());
```
