---
id: notes-panel-notespanel
kind: doc
target_file: src/notes_panel.rs
line_hint: 130
author: 
created: 
title: "NotesPanel"
tags: []
signature: "pub struct NotesPanel"
---

The panel's state: the index plus the filters the user set.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert_eq!(panel.rows().len(), 0);
```
