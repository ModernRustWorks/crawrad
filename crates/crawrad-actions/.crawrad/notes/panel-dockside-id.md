---
id: panel-dockside-id
kind: doc
target_file: src/panel.rs
line_hint: 144
author: 
created: 
title: "DockSide::id"
tags: []
signature: "pub fn id (self) -> & 'static str"
---

The stable identifier — the `side` argument of `layout.move_panel`.

# Examples

```rust
use crawrad_actions::DockSide;

assert_eq!(DockSide::Right.id(), "right");
```
