---
id: notes-panel-notespanel-file-label
kind: doc
target_file: src/notes_panel.rs
line_hint: 307
author: 
created: 
title: "NotesPanel::file_label"
tags: []
signature: "pub fn file_label (& self) -> String"
---

Returns the label for the file filter dropdown.

Returns the label for the file filter dropdown ("All files" or selected file path).

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert_eq!(panel.file_label(), "All files");
```
