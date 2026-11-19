---
id: notes-panel-rowtarget
kind: doc
target_file: src/notes_panel.rs
line_hint: 114
author: 
created: 
title: "RowTarget"
tags: []
signature: "pub enum RowTarget"
---

What a row of the list leads to.

# Examples

```rust
use crawrad_project::notes_panel::RowTarget;

let target = RowTarget::Group("src/lib.rs".into());
assert_eq!(target, RowTarget::Group("src/lib.rs".into()));
```
