---
id: notes-panel-viewrowparts
kind: doc
target_file: src/notes_panel.rs
line_hint: 108
author: 
created: 
title: "ViewRowParts"
tags: []
signature: "pub type ViewRowParts = (bool , u8 , String , String , bool , String , bool , bool , bool)"
---

The parts of a row as painted by the Notes panel view.

A row as the Notes panel view paints it:
(is_group, kind 0 doc / 1 note / 2 comment, title, badge text ("" = none), badge is warn,
 right text (the line, "—", or a group's count), faded, struck, collapsed group).

# Examples

```rust
use crawrad_project::notes_panel::ViewRowParts;

let parts: ViewRowParts = (true, 0, "Group".into(), String::new(), false, "1".into(), false, false, false);
assert!(parts.0);
```
