---
id: notes-panel-notespanel-files
kind: doc
target_file: src/notes_panel.rs
line_hint: 187
author: 
created: 
title: "NotesPanel::files"
tags: []
signature: "pub fn files (& self) -> Vec < String >"
---

Returns all distinct anchored file paths, sorted alphabetically.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
assert!(panel.files().is_empty());
```
