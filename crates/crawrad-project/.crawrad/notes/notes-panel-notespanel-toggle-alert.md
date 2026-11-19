---
id: notes-panel-notespanel-toggle-alert
kind: doc
target_file: src/notes_panel.rs
line_hint: 445
author: 
created: 
title: "NotesPanel::toggle_alert"
tags: []
signature: "pub fn toggle_alert (& mut self)"
---

Toggles the "In alert" filter.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let mut panel = NotesPanel::default();
panel.toggle_alert();
assert!(panel.in_alert);
```
