---
id: notes-panel-panelrow
kind: doc
target_file: src/notes_panel.rs
line_hint: 71
author: 
created: 
title: "PanelRow"
tags: []
signature: "pub enum PanelRow"
---

A row of the panel's list.

# Examples

```rust
use crawrad_project::notes_panel::PanelRow;

let row = PanelRow::Group {
    key: "src/lib.rs".into(),
    title: "src/lib.rs".into(),
    count: 1,
    collapsed: false,
};
match row {
    PanelRow::Group { count, .. } => assert_eq!(count, 1),
    _ => unreachable!(),
}
```
