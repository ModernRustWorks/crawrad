---
id: notes-panel-notespanel-view-head
kind: doc
target_file: src/notes_panel.rs
line_hint: 394
author: 
created: 
title: "NotesPanel::view_head"
tags: []
signature: "pub fn view_head (& self) -> ViewHeadParts"
---

Returns the panel head parts as the view paints them.

# Examples

```rust
use crawrad_project::notes_panel::NotesPanel;

let panel = NotesPanel::default();
let head = panel.view_head();
assert_eq!(head.1, 0);
```
