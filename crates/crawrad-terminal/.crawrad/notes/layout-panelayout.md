---
id: layout-panelayout
kind: doc
target_file: src/layout.rs
line_hint: 19
author: 
created: 
title: "PaneLayout"
tags: []
signature: "pub struct PaneLayout"
---

Multi-pane layout manager.

# Examples

```rust
use crawrad_terminal::PaneLayout;

let layout = PaneLayout::default();
assert_eq!(layout.panes.len(), 1);
```
