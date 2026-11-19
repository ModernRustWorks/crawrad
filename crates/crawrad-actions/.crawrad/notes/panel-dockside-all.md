---
id: panel-dockside-all
kind: doc
target_file: src/panel.rs
line_hint: 141
author: 
created: 
title: "DockSide::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Left , Self :: Right , Self :: Bottom]"
---

Every side, in the order the `Move To` menu shows them.

# Examples

```rust
use crawrad_actions::DockSide;

assert_eq!(DockSide::ALL.len(), 3);
```
