---
id: panel-dockside
kind: doc
target_file: src/panel.rs
line_hint: 132
author: 
created: 
title: "DockSide"
tags: []
signature: "pub enum DockSide"
---

Where a panel can be docked.

Three, and `Second Window` is deliberately missing: Amendment 10 of [ADR 0014]
lists it as a destination and a tracked issue as the work, and a variant that
no shell can honour would be a tool an agent calls into silence. `Top` is
missing for a different reason — nothing in this IDE docks above the menu
bar, and the dock of the framework reaches it by dragging anyway.


# Examples

```rust
use crawrad_actions::DockSide;

assert_eq!(DockSide::Left.id(), "left");
```
