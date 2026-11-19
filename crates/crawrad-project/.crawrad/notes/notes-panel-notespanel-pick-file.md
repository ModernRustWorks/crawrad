---
id: notes-panel-notespanel-pick-file
kind: doc
target_file: src/notes_panel.rs
line_hint: 435
author: 
created: 
title: "NotesPanel::pick_file"
tags: []
signature: "pub fn pick_file (& mut self , choice : usize)"
---

Selects a file filter choice.

Selects a file filter choice (0 = all files, n = files()[n-1]).
Out of range resets to all files.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let mut panel = NotesPanel::default();
panel.pick_file(0);
assert!(panel.file.is_none());
```
