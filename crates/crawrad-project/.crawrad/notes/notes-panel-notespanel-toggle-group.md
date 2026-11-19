---
id: notes-panel-notespanel-toggle-group
kind: doc
target_file: src/notes_panel.rs
line_hint: 198
author: 
created: 
title: "NotesPanel::toggle_group"
tags: []
signature: "pub fn toggle_group (& mut self , key : & str)"
---

Toggles the collapsed state of a group by its key.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let mut panel = NotesPanel::default();
panel.toggle_group("src/lib.rs");
panel.toggle_group("src/lib.rs");
```
