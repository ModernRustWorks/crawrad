---
id: panel-dockside-from-id
kind: doc
target_file: src/panel.rs
line_hint: 162
author: 
created: 
title: "DockSide::from_id"
tags: []
signature: "pub fn from_id (id : & str) -> Option < Self >"
---

The side with this id, or `None`.

# Examples

```rust
use crawrad_actions::DockSide;

assert_eq!(DockSide::from_id("left"), Some(DockSide::Left));
assert_eq!(DockSide::from_id("unknown"), None);
```
