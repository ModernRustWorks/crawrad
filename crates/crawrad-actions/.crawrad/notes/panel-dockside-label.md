---
id: panel-dockside-label
kind: doc
target_file: src/panel.rs
line_hint: 153
author: 
created: 
title: "DockSide::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What a menu row shows.

# Examples

```rust
use crawrad_actions::DockSide;

assert_eq!(DockSide::Bottom.label(), "Bottom");
```
