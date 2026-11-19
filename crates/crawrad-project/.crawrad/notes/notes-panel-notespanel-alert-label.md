---
id: notes-panel-notespanel-alert-label
kind: doc
target_file: src/notes_panel.rs
line_hint: 302
author: 
created: 
title: "NotesPanel::alert_label"
tags: []
signature: "pub fn alert_label (& self) -> String"
---

Returns the label for the "In alert" toggle with its count.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert_eq!(panel.alert_label(), "In alert 0");
```
