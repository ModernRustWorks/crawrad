---
id: components-panel-panel
kind: doc
target_file: src/components/panel.rs
line_hint: 4
author: 
created: 
title: "panel"
tags: []
signature: "pub fn panel () -> ComponentDesc"
---

A generic container panel with a border and background.

`Panel`: the first container (ADR 0012).

It is the only one whose designer contract is pure — no caption indent, no
item list, no separate design surface. It exercises all four
consequences of the ADR at once (relative coordinates, clipping, Z-order by
parent, grouping by parent) with nothing else mixed in: if `Panel` works, the
model is proven.

# Examples

```rust
use crawrad_registry::panel;

let desc = panel();
assert_eq!(desc.class, "Panel");
```
